use super::ExternTypesAttr;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Item, ItemFn};

#[derive(Debug, Clone)]
pub struct CallDef {
    pub item_fn: ItemFn,
    pub extension_id: u64,
    pub call_index: u32,
    #[allow(unused)]
    pub extern_types: Option<ExternTypesAttr>,
}

impl CallDef {
    pub fn try_from(
        span: Span,
        item: &mut Item,
        extension_id: Option<u64>,
        call_index: Option<u32>,
        extern_types: Option<ExternTypesAttr>,
    ) -> syn::Result<Self> {
        let extension_id = extension_id.ok_or_else(|| {
            syn::Error::new(
                span,
                "Missing extension_id for pvq::call_def, expected #[pvq::call_def(extension_id = SOME_U64)]",
            )
        })?;
        let item_fn = if let Item::Fn(item_fn) = item {
            item_fn
        } else {
            return Err(syn::Error::new(item.span(), "Invalid pvq::call_def, expected item fn"));
        };
        let call_index = call_index.ok_or_else(|| {
            syn::Error::new(
                span,
                "Missing call_index for pvq::call_def, expected #[pvq::call_def(call_index = SOME_U32)]",
            )
        })?;
        Ok(Self {
            item_fn: item_fn.clone(),
            extension_id,
            call_index,
            extern_types,
        })
    }
}
