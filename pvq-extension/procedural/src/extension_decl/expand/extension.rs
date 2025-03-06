use super::helper;
use crate::extension_decl::parse::extension::ExtensionMethod;
use crate::extension_decl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Replace Self::SomeType with Impl::SomeType
fn replace_self_to_impl(ty: &syn::Type) -> Box<syn::Type> {
    let ty_str = quote!(#ty).to_string();

    let modified_ty_str = ty_str.replace("Self", "Impl");

    let modified_ty =
        syn::parse_str(&modified_ty_str).expect("The replace with Impl::SomeType should not break the syntax");

    Box::new(modified_ty)
}

pub fn expand_extension(def: &mut Def) -> TokenStream2 {
    let pvq_extension = &def.pvq_extension;
    // Set the trait name based on module_name
    let trait_ident = &def.extension.name;

    // Add super trait ExtensionId and ExtensionMetadata to the trait's where clause
    // helper::add_super_trait(&mut item_trait)?;

    // Generate the functions enum definition
    let functions_enum = expand_functions_enum(trait_ident, &def.extension.methods);

    // Generate the dispatchable implementation
    let functions_impl_dispatchable =
        impl_dispatchable_for_functions(&pvq_extension, trait_ident, &def.extension.methods);

    // Generate the extension ID implementation
    let functions_impl_extension_id =
        impl_extension_id_for_functions(&pvq_extension, trait_ident, &def.extension.methods);

    // let extension_runtime_metadata = crate::runtime_metadata::generate_decl_metadata(&item_trait, view_fns.has_config)?;

    // Combine all the generated code
    let expanded = quote! {
        #functions_enum
        #functions_impl_dispatchable
        #functions_impl_extension_id
        // #extension_runtime_metadata
    };

    expanded
}

fn expand_functions_enum(trait_ident: &syn::Ident, methods: &[ExtensionMethod]) -> syn::ItemEnum {
    let mut variants = syn::punctuated::Punctuated::<syn::Variant, syn::token::Comma>::new();

    for method in methods {
        let name = &method.name;
        let mut args = syn::punctuated::Punctuated::<syn::Field, syn::token::Comma>::new();

        for (name, ty) in &method.args {
            let ty = replace_self_to_impl(ty);
            args.push(syn::parse_quote! {
                #name: #ty
            });
        }

        variants.push(syn::parse_quote! {
            #name {
                #args
            }
        });
    }

    // Add phantom data
    variants.push(syn::parse_quote!(
        #[doc(hidden)]
        __marker(core::marker::PhantomData<Impl>)
    ));
    syn::parse_quote!(
        #[derive(parity_scale_codec::Codec)]
        #[allow(non_camel_case_types)]
        pub enum Functions<Impl: #trait_ident> {
            #variants
        }
    )
}

fn impl_dispatchable_for_functions(
    pvq_extension: &syn::Path,
    trait_ident: &syn::Ident,
    methods: &[ExtensionMethod],
) -> syn::ItemImpl {
    let mut pats = Vec::<syn::Pat>::new();

    for method in methods {
        let name = &method.name;
        let mut args = syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma>::new();

        for (ident, _ty) in &method.args {
            args.push(ident.clone());
        }

        pats.push(syn::parse_quote! {
            Self::#name {
                #args
            }
        });
    }

    let mut method_calls = Vec::<syn::ExprCall>::new();

    for method in methods {
        let name = &method.name;
        let mut args = syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma>::new();

        for (ident, _ty) in &method.args {
            args.push(ident.clone());
        }

        method_calls.push(syn::parse_quote! {
            Impl::#name(#args)
        });
    }

    syn::parse_quote! {
        impl<Impl: #trait_ident> #pvq_extension::Dispatchable for Functions<Impl> {
            fn dispatch(self) -> Result<scale_info::prelude::vec::Vec<u8>, #pvq_extension::DispatchError> {
                match self {
                    #( #pats => Ok(#method_calls.encode()),)*
                    Self::__marker(_) => Err(#pvq_extension::DispatchError::PhantomData),
                }
            }
        }
    }
}

fn impl_extension_id_for_functions(
    pvq_extension: &syn::Path,
    trait_ident: &syn::Ident,
    methods: &[ExtensionMethod],
) -> syn::ItemImpl {
    let extension_id = helper::calculate_hash(trait_ident, methods);
    syn::parse_quote! {
        impl<Impl: #trait_ident> #pvq_extension::ExtensionId for Functions<Impl> {
            const EXTENSION_ID: #pvq_extension::ExtensionIdTy = #extension_id;
        }
    }
}
