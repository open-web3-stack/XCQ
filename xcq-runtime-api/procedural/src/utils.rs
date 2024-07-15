use inflector::Inflector;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{spanned::Spanned, Error, Ident, ItemImpl, Path, Result};
/// Should a qualified trait path be required?
///
/// e.g. `path::Trait` is qualified and `Trait` is not.
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
pub fn generate_runtime_mod_name_for_trait(trait_: &Ident) -> Ident {
    Ident::new(
        &format!("runtime_decl_for_{}", trait_.to_string().to_snake_case()),
        Span::call_site(),
    )
}

/// Generate the crate access for the crate using 2018 syntax.
///
/// If `frame` is in scope, it will use `polkadot_sdk_frame::deps::<def_crate>`. Else, it will try
/// and find `<def_crate>` directly.
pub fn generate_access_from_frame_or_crate(def_crate: &str) -> Result<syn::Path, Error> {
    if let Some(path) = get_frame_crate_path(def_crate) {
        Ok(path)
    } else if let Some(path) = get_sdk_crate_path(def_crate) {
        Ok(path)
    } else {
        let ident = match crate_name(def_crate) {
            Ok(FoundCrate::Itself) => {
                let name = def_crate.to_string().replace("-", "_");
                Ok(syn::Ident::new(&name, Span::call_site()))
            }
            Ok(FoundCrate::Name(name)) => Ok(Ident::new(&name, Span::call_site())),
            Err(e) => Err(Error::new(Span::call_site(), e)),
        }?;

        Ok(syn::Path::from(ident))
    }
}
fn import_xcq_types() -> TokenStream2 {
    let found_crate = crate_name("xcq-types").expect("xcq-types not found in Cargo.toml");
    match found_crate {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { ::#name }
        }
    }
}
