use inflector::Inflector;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{spanned::Spanned, Error, Ident, ItemImpl, Path, Result};
/// Should a qualified trait path be required?
///
/// e.g. `path::Trait` is qualified and `Trait` is not.
#[allow(dead_code)]
pub enum RequireQualifiedTraitPath {
    Yes,
    No,
}
/// Extract the trait that is implemented by the given `ItemImpl`.
pub fn extract_impl_trait(impl_: &ItemImpl, require: RequireQualifiedTraitPath) -> Result<&Path> {
    impl_
        .trait_
        .as_ref()
        .map(|v| &v.1)
        .ok_or_else(|| Error::new(impl_.span(), "Only implementation of traits are supported!"))
        .and_then(|p| {
            if p.segments.len() > 1 || matches!(require, RequireQualifiedTraitPath::No) {
                Ok(p)
            } else {
                Err(Error::new(
                    p.span(),
                    "The implemented trait has to be referenced with a path, \
					e.g. `impl xcq_extension_core::ExtensionCore for Runtime`.",
                ))
            }
        })
}

/// Generates the name of the module that contains the trait declaration for the runtime.
pub fn generate_mod_name_for_trait(trait_: &Ident) -> Ident {
    Ident::new(
        &format!("decl_extension_for_{}", trait_.to_string().to_snake_case()),
        Span::call_site(),
    )
}

pub fn generate_crate_access(def_crate: &str) -> Result<TokenStream2> {
    match crate_name(def_crate) {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let name = name.replace('-', "_");
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(#ident))
        }
        Err(e) => Err(Error::new(Span::call_site(), e)),
    }
}
