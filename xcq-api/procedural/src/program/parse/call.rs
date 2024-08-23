use super::ExternTypesAttr;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Item, ItemFn};

#[derive(Debug, Clone)]
pub struct CallDef {
    pub index: usize,
    pub item_fn: ItemFn,
    pub extern_types: Option<ExternTypesAttr>,
}

impl CallDef {
    pub fn try_from(
        _span: Span,
        index: usize,
        item: &mut Item,
        extern_types: Option<ExternTypesAttr>,
    ) -> syn::Result<Self> {
        let item_fn = if let Item::Fn(item_fn) = item {
            item_fn
        } else {
            return Err(syn::Error::new(item.span(), "Invalid xcq::call, expected item fn"));
        };
        Ok(Self {
            index,
            item_fn: item_fn.clone(),
            extern_types,
        })
    }
}
