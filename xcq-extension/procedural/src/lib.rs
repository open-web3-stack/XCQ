use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

#[derive(Clone)]
struct Method {
    /// Function name
    pub name: syn::Ident,
    /// Information on args: `(name, type)`
    pub args: Vec<(syn::Ident, Box<syn::Type>)>,
}

#[proc_macro_attribute]
pub fn extension(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemTrait);
    let trait_name = &input.ident;
    let trait_items = &input.items;

    let methods = match methods(trait_items) {
        Ok(method) => method,
        Err(e) => return e.to_compile_error().into(),
    };

    // TODO: add phantom data if Impl is not used
    let variants_replace_generics = match methods
        .iter()
        .map(|method| {
            let name = &method.name;
            let args = method
                .args
                .iter()
                .map(|(name, ty)| {
                    let ty = replace_self_to_impl(ty)?;
                    Ok(quote! {
                        #name: #ty
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                #name {
                    #(#args),*
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(variants) => variants,
        Err(e) => return e.to_compile_error().into(),
    };

    let variants_no_arg_type = methods
        .iter()
        .map(|method| {
            let name = &method.name;
            let args = method.args.iter().map(|(name, _ty)| {
                quote! {
                    #name
                }
            });
            quote! {
                #name {
                    #(#args),*
                }
            }
        })
        .collect::<Vec<_>>();

    let methods_no_arg_type = methods
        .iter()
        .map(|method| {
            let name = &method.name;
            let args = method.args.iter().map(|(name, _ty)| {
                quote! {
                    #name
                }
            });
            quote! {
                #name (
                    #(#args),*
                )
            }
        })
        .collect::<Vec<_>>();

    let call_def = quote! {
        #[derive(Decode)]
        pub enum Call<Impl: #trait_name> {
            #(
                #[allow(non_camel_case_types)]
                #variants_replace_generics
            ),*
        }
    };

    let dispatchable_impl = quote! {
        impl<Impl: #trait_name> xcq_extension::Dispatchable for Call<Impl> {
            fn dispatch(self) -> Result<Vec<u8>, xcq_extension::DispatchError> {
                match self {
                    #( Self::#variants_no_arg_type => Ok(Impl::#methods_no_arg_type.encode())),*
                }
            }
        }
    };

    // TODO calculate according to trait definition
    // Currently, specify the extension_id as `args`
    let extension_id = syn::parse_macro_input!(args as syn::LitInt);
    let extension_id_impl = quote! {
        impl<Impl: #trait_name> xcq_extension::ExtensionId for Call<Impl> {
            const EXTENSION_ID: xcq_extension::ExtensionIdTy = #extension_id;
        }
    };

    let expanded = quote! {
        #input
        #call_def
        #dispatchable_impl
        #extension_id_impl
    };
    expanded.into()
}

fn methods(trait_items: &Vec<syn::TraitItem>) -> syn::Result<Vec<Method>> {
    let mut methods = vec![];
    for item in trait_items {
        if let syn::TraitItem::Fn(method) = item {
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

fn replace_self_to_impl(ty: &syn::Type) -> syn::Result<Box<syn::Type>> {
    // replace Self in ty to Impl
    let ty_str = quote!(#ty).to_string();

    let modified_ty_str = ty_str.replace("Self", "Impl");

    let modified_ty = syn::parse_str(&modified_ty_str)?;

    Ok(Box::new(modified_ty))
}
