use crate::utils::{extract_impl_trait, generate_mod_name_for_trait, RequireQualifiedTraitPath};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ItemImpl, Result,
};
pub fn impl_extensions_impl(input: TokenStream) -> TokenStream {
    let XcqExtensionImpls { impls } = parse_macro_input!(input as XcqExtensionImpls);
    impl_extensions_inner(&impls)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

struct XcqExtensionImpls {
    impls: Vec<ItemImpl>,
}

impl Parse for XcqExtensionImpls {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut impls = Vec::new();
        while !input.is_empty() {
            impls.push(input.parse()?);
        }
        Ok(Self { impls })
    }
}

fn impl_extensions_inner(impls: &[ItemImpl]) -> Result<TokenStream2> {
    let runtime_metadata = crate::runtime_metadata::generate_impl_metadata(impls)?;

    let extension_tuple = generate_extension_tuple(impls)?;

    let expanded = quote! {
        #(#impls)*

        #runtime_metadata

        #extension_tuple
    };
    Ok(expanded)
}

fn generate_extension_tuple(impls: &[ItemImpl]) -> Result<TokenStream2> {
    let extension_impl_name = &impls
        .first()
        .expect("Traits should contain at least one implementation; qed")
        .self_ty;
    let mut extensions = Vec::new();
    for impl_ in impls {
        let mut trait_ = extract_impl_trait(impl_, RequireQualifiedTraitPath::Yes)?.clone();

        let trait_name_ident = &trait_
            .segments
            .last()
            .as_ref()
            .expect("Trait path should always contain at least one item; qed")
            .ident;
        let mod_name = generate_mod_name_for_trait(trait_name_ident);
        // Get absolute path to the `runtime_decl_for_` module by replacing the last segment.
        if let Some(segment) = trait_.segments.last_mut() {
            *segment = parse_quote!(#mod_name);
        }
        trait_.segments.push(parse_quote!(Call<#extension_impl_name>));
        extensions.push(trait_);
    }

    Ok(quote! {
        type Extensions = (
            #(#extensions),*
        );
    })
}
