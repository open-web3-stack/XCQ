use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
pub fn generate_crate_access(def_crate: &str) -> syn::Result<syn::Path> {
    let ident = match crate_name(def_crate) {
        Ok(FoundCrate::Itself) => {
            let name = def_crate.replace('-', "_");
            Ok(syn::Ident::new(&name, Span::call_site()))
        }
        Ok(FoundCrate::Name(name)) => Ok(syn::Ident::new(&name, Span::call_site())),
        Err(e) => Err(syn::Error::new(Span::call_site(), e)),
    }?;
    Ok(syn::Path::from(ident))
}
