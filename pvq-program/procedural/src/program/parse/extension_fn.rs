use proc_macro2::Span;
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct ExtensionFn {
    pub item_fn: syn::ItemFn,
    pub extension_id: u64,
    pub fn_index: u32,
}

impl ExtensionFn {
    pub fn try_from(
        span: Span,
        item: syn::Item,
        extension_id: Option<u64>,
        fn_index: Option<u32>,
    ) -> syn::Result<Self> {
        let extension_id = extension_id.ok_or_else(|| {
            syn::Error::new(
                span,
                "Missing extension_id for program::extension_fn, expected #[program::extension_fn(extension_id = SOME_U64)]",
            )
        })?;
        let item_fn = if let syn::Item::Fn(item_fn) = item {
            item_fn
        } else {
            return Err(syn::Error::new(
                item.span(),
                "Invalid program::extension_fn, expected item fn",
            ));
        };
        // Check that the inputs of the function are all not self
        if item_fn
            .sig
            .inputs
            .iter()
            .any(|arg| matches!(arg, syn::FnArg::Receiver(_)))
        {
            return Err(syn::Error::new(
                item_fn.span(),
                "Invalid program::extension_fn, expected function inputs to not be receiver",
            ));
        }

        let fn_index = fn_index.ok_or_else(|| {
            syn::Error::new(
                span,
                "Missing fn_index for program::extension_fn, expected #[program::extension_fn(fn_index = SOME_U32)]",
            )
        })?;
        Ok(Self {
            item_fn,
            extension_id,
            fn_index,
        })
    }
}
