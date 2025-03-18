use syn::spanned::Spanned;
#[derive(Debug)]
pub struct EntrypointDef {
    pub item_fn: syn::ItemFn,
}

impl EntrypointDef {
    pub fn try_from(_span: proc_macro2::Span, item: &mut syn::Item) -> syn::Result<Self> {
        if let syn::Item::Fn(item_fn) = item {
            if item_fn
                .sig
                .inputs
                .iter()
                .any(|arg| matches!(arg, syn::FnArg::Receiver(_)))
            {
                return Err(syn::Error::new(
                    item_fn.span(),
                    "Invalid program::entrypoint, expected fn args are not receiver type",
                ));
            }
            Ok(Self {
                item_fn: item_fn.clone(),
            })
        } else {
            Err(syn::Error::new(
                item.span(),
                "Invalid program::entrypoint, expected item fn",
            ))
        }
    }
}
