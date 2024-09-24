use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::hash::{Hash, Hasher};
use syn::{
    parse_macro_input, parse_quote, parse_str, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, ExprCall,
    Field, FnArg, Ident, ItemEnum, ItemImpl, ItemTrait, Pat, Result, TraitItem, Type, Variant,
};

use crate::utils::{generate_crate_access, generate_mod_name_for_trait};

pub fn decl_extensions_impl(input: TokenStream) -> TokenStream {
    let item_trait = parse_macro_input!(input as ItemTrait);
    decl_extension_inner(&item_trait)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

pub fn decl_extension_inner(item_trait: &ItemTrait) -> Result<TokenStream2> {
    let mut item_trait = item_trait.clone();
    // Generate a separate module for the extension
    let mod_name = generate_mod_name_for_trait(&item_trait.ident);

    // Add super trait ExtensionId and ExtensionMetadata to the trait's where clause
    add_super_trait(&mut item_trait)?;

    // TODO: If trait has associated type, we assume it has a single associated type called `Config`
    let has_config = item_trait
        .items
        .iter()
        .any(|item| matches!(item, syn::TraitItem::Type(_)));

    // Extract methods from the trait
    let methods = methods(&item_trait.items)?;

    let call_enum_def = call_enum_def(&item_trait.ident, &methods)?;
    let call_data_dispatchable_impl = impl_dispatchable(&item_trait.ident, &methods)?;
    let call_data_extension_id_impl = impl_extension_id(&item_trait.ident, &item_trait.items)?;
    let call_data_metadata_impl = impl_metadata(&item_trait.ident)?;
    let extension_runtime_metadata = crate::runtime_metadata::generate_decl_metadata(&item_trait, has_config)?;

    let expanded = quote! {
        #item_trait
        #[doc(hidden)]
        #[allow(dead_code)]
        pub mod #mod_name {
            pub use super::*;
            #call_enum_def
            #call_data_dispatchable_impl
            #call_data_extension_id_impl
            #call_data_metadata_impl
            #extension_runtime_metadata
        }
        pub use #mod_name::*;
    };
    Ok(expanded)
}

fn add_super_trait(item_trait: &mut ItemTrait) -> Result<()> {
    let xcq_extension = generate_crate_access("xcq-extension")?;
    // item_trait.supertraits.push(parse_quote!(#xcq_extension::ExtensionId));
    item_trait
        .supertraits
        .push(parse_quote!(#xcq_extension::ExtensionMetadata));
    Ok(())
}

#[derive(Clone)]
struct Method {
    /// Function name
    pub name: Ident,
    /// Information on args: `(name, type)`
    pub args: Vec<(Ident, Box<Type>)>,
}

fn call_enum_def(trait_ident: &Ident, methods: &[Method]) -> Result<ItemEnum> {
    let xcq_primitives = generate_crate_access("xcq-primitives")?;
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
        __Phantom(core::marker::PhantomData<Impl>)
    ));
    Ok(parse_quote!(
        #[derive(#xcq_primitives::deps::parity_scale_codec::Decode)]
        pub enum Call<Impl: #trait_ident> {
            #variants
        }
    ))
}

fn impl_dispatchable(trait_ident: &Ident, methods: &[Method]) -> Result<ItemImpl> {
    let xcq_extension = generate_crate_access("xcq-extension")?;
    let xcq_primitives = generate_crate_access("xcq-primitives")?;
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

    Ok(parse_quote! {
        impl<Impl: #trait_ident> #xcq_extension::Dispatchable for Call<Impl> {
            fn dispatch(self) -> Result<#xcq_primitives::deps::xcq_types::vec::Vec<u8>, #xcq_extension::DispatchError> {
                match self {
                    #( #pats => Ok(#method_calls.encode()),)*
                    Self::__Phantom(_) => unreachable!(),
                }
            }
        }
    })
}

fn impl_extension_id(trait_ident: &Ident, trait_items: &[TraitItem]) -> Result<TokenStream2> {
    let xcq_extension = generate_crate_access("xcq-extension")?;

    let extension_id = calculate_hash(trait_ident, trait_items);
    Ok(quote! {
        // Defining an trait for extension_id is useful for generic usage
        impl<Impl: #trait_ident> #xcq_extension::ExtensionId for Call<Impl> {
            const EXTENSION_ID: #xcq_extension::ExtensionIdTy = #extension_id;
        }
        // This one is for easier access, since impl doesn't contribute to the extension_id calculation
        pub const EXTENSION_ID: #xcq_extension::ExtensionIdTy = #extension_id;
    })
}

// Delegate the metadata generation to the trait implementation
fn impl_metadata(trait_ident: &Ident) -> Result<ItemImpl> {
    let xcq_extension = generate_crate_access("xcq-extension")?;
    let xcq_primitives = generate_crate_access("xcq-primitives")?;
    Ok(parse_quote! {
        impl<Impl: #trait_ident> #xcq_extension::CallMetadata for Call<Impl> {
            fn metadata() -> #xcq_primitives::metadata_ir::ExtensionMetadataIR {
                Impl::extension_metadata(<Self as #xcq_extension::ExtensionId>::EXTENSION_ID)
            }
        }
    })
}

// helper functions
fn methods(trait_items: &[TraitItem]) -> Result<Vec<Method>> {
    let mut methods = vec![];
    for item in trait_items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let mut method_args = vec![];
            for arg in method.sig.inputs.iter() {
                let arg = if let FnArg::Typed(arg) = arg {
                    arg
                } else {
                    unreachable!("every argument should be typed instead of receiver(self)")
                };
                let arg_ident = if let Pat::Ident(pat) = &*arg.pat {
                    pat.ident.clone()
                } else {
                    let msg = "Invalid call, argument must be ident";
                    return Err(Error::new(arg.pat.span(), msg));
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
fn replace_self_to_impl(ty: &Type) -> Result<Box<Type>> {
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
