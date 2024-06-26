use proc_macro2::TokenStream;
use quote::quote;
use std::hash::{Hash, Hasher};
use syn::token::Comma;
use syn::{parse_macro_input, parse_quote, parse_str, spanned::Spanned};
use syn::{punctuated::Punctuated, ExprCall, Field, Ident, ItemImpl, Pat, TraitItem, Variant};

#[derive(Clone)]
struct Method {
    /// Function name
    pub name: Ident,
    /// Information on args: `(name, type)`
    pub args: Vec<(Ident, Box<syn::Type>)>,
}

#[proc_macro_attribute]
pub fn extension(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemTrait);

    let methods = match methods(&input.items) {
        Ok(method) => method,
        Err(e) => return e.to_compile_error().into(),
    };

    let call_enum_def = match call_enum_def(&input.ident, &methods) {
        Ok(call_enum_def) => call_enum_def,
        Err(e) => return e.to_compile_error().into(),
    };

    let dispatchable_impl = dispatchable_impl(&input.ident, &methods);
    let extension_id_impl = extension_id_impl(&input.ident, &input.items);

    let expanded = quote! {
        #input
        #call_enum_def
        #dispatchable_impl
        #extension_id_impl
    };
    expanded.into()
}

fn call_enum_def(trait_ident: &Ident, methods: &[Method]) -> syn::Result<syn::ItemEnum> {
    let mut variants = Punctuated::<Variant, Comma>::new();
    for method in methods {
        let name = &method.name;
        let mut args = Punctuated::<Field, Comma>::new();
        for (name, ty) in &method.args {
            let ty = replace_self_to_impl(ty)?;
            args.push(parse_quote! {
                #name: #ty
            });
        }
        variants.push(parse_quote! {
            #[allow(non_camel_case_types)]
            #name {
                #args
            }
        });
    }
    // Add phantom data
    variants.push(parse_quote!(
        #[doc(hidden)]
        __Phantom(std::marker::PhantomData<Impl>)
    ));
    Ok(parse_quote!(
        #[derive(Decode)]
        pub enum Call<Impl: #trait_ident> {
            #variants
        }
    ))
}

fn dispatchable_impl(trait_ident: &Ident, methods: &[Method]) -> TokenStream {
    let mut pats = Vec::<Pat>::new();
    for method in methods {
        let name = &method.name;
        let mut args = Punctuated::<Ident, Comma>::new();
        for (ident, _ty) in &method.args {
            args.push(parse_quote! {
                #ident
            });
        }
        pats.push(parse_quote! {
            Self::#name {
                #args
            }
        });
    }

    let mut method_calls = Vec::<ExprCall>::new();
    for method in methods {
        let name = &method.name;
        let mut args = Punctuated::<Ident, Comma>::new();
        for (ident, _ty) in &method.args {
            args.push(parse_quote! {
                #ident
            });
        }
        method_calls.push({
            parse_quote! {
                Impl::#name(#args)
            }
        });
    }

    parse_quote! {
        impl<Impl: #trait_ident> xcq_extension::Dispatchable for Call<Impl> {
            fn dispatch(self) -> Result<Vec<u8>, xcq_extension::DispatchError> {
                match self {
                    #( #pats => Ok(#method_calls.encode()),)*
                    Self::__Phantom(_) => unreachable!(),
                }
            }
        }
    }
}

fn extension_id_impl(trait_ident: &Ident, trait_items: &[TraitItem]) -> ItemImpl {
    let extension_id = calculate_hash(trait_ident, trait_items);
    parse_quote! {
        impl<Impl: #trait_ident> xcq_extension::ExtensionId for Call<Impl> {
            const EXTENSION_ID: xcq_extension::ExtensionIdTy = #extension_id;
        }
    }
}

// helper functions
fn methods(trait_items: &[TraitItem]) -> syn::Result<Vec<Method>> {
    let mut methods = vec![];
    for item in trait_items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let mut method_args = vec![];
            for arg in method.sig.inputs.iter() {
                let arg = if let syn::FnArg::Typed(arg) = arg {
                    arg
                } else {
                    unreachable!("every argument should be typed instead of receiver(self)")
                };
                let arg_ident = if let syn::Pat::Ident(pat) = &*arg.pat {
                    pat.ident.clone()
                } else {
                    let msg = "Invalid call, argument must be ident";
                    return Err(syn::Error::new(arg.pat.span(), msg));
                };
                method_args.push((arg_ident, arg.ty.clone()))
            }
            methods.push(Method {
                name: method_name.clone(),
                args: method_args,
            });
        }
    }
    Ok(methods)
}

// TODO: refine this to make it more stable
fn replace_self_to_impl(ty: &syn::Type) -> syn::Result<Box<syn::Type>> {
    let ty_str = quote!(#ty).to_string();

    let modified_ty_str = ty_str.replace("Self", "Impl");

    let modified_ty = parse_str(&modified_ty_str)?;

    Ok(Box::new(modified_ty))
}

// TODO: currently we only hash on trait ident and function names,
fn calculate_hash(trait_ident: &Ident, trait_items: &[TraitItem]) -> u64 {
    let mut hasher = twox_hash::XxHash64::default();
    // reduce the chance of hash collision
    "xcq-ext$".hash(&mut hasher);
    trait_ident.hash(&mut hasher);
    for trait_item in trait_items {
        if let TraitItem::Fn(method) = trait_item {
            // reduce the chance of hash collision
            "@".hash(&mut hasher);
            method.sig.ident.hash(&mut hasher);
        }
    }
    hasher.finish()
}
