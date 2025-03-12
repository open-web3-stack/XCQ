use syn::spanned::Spanned;
use syn::Error;
pub struct ExtensionImpl {
    pub trait_path: syn::Path,
}

impl ExtensionImpl {
    pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
        let syn::Item::Impl(item) = item else {
            let msg = "Invalid extensions_impl::extension, expected impl definition";
            return Err(syn::Error::new(item.span(), msg));
        };
        // Check if's a impl trait and the trait is qualified
        let path = item
            .trait_
            .as_ref()
            .map(|v| &v.1)
            .ok_or_else(|| Error::new(item.span(), "Only implementation of traits are supported!"))
            .and_then(|p| {
                // The implemented trait has to be referenced with a fully qualified path,
                if p.segments.len() > 1 {
                    Ok(p)
                } else {
                    Err(Error::new(
                        p.span(),
                        "The implemented trait has to be referenced with a fully qualified path, \
					e.g. `impl pvq_extension_core::ExtensionCore for ExtensionsImpl`.",
                    ))
                }
            })?;
        Ok(Self {
            trait_path: path.clone(),
        })
    }
}
