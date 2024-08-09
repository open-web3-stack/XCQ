use syn::spanned::Spanned;
use syn::{Item, ItemFn, Meta, TypePath};
pub struct EntrypointDef {
    pub item_fn: ItemFn,
    pub arg_types: Vec<ArgType>,
}

struct ArgType {
    pub multi: bool,
    pub ty: Box<syn::Type>,
}

impl EntrypointDef {
    pub fn try_from(span: proc_macro2::Span, index: usize, item: &mut Item) -> syn::Result<Self> {
        if let syn::Item::Fn(item_fn) = item {
            let mut arg_types = Vec::new();
            for input in &item_fn.sig.inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    if pat_type.attrs.iter().any(|attr| {
                        if let Meta::Path(path) = &attr.meta {
                            path.segments.first().map_or(false, |segment| segment.ident == "multi")
                        } else {
                            false
                        }
                    }) {
                        arg_types.push(ArgType {
                            multi: true,
                            ty: pat_type.ty.clone(),
                        });
                    } else {
                        arg_types.push(ArgType {
                            multi: false,
                            ty: pat_type.ty.clone(),
                        });
                    }
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "Invalid xcq::entrypoint, expected fn to have typed arguments",
                    ));
                }
            }
            Ok(Self {
                item_fn: item_fn.clone(),
                arg_types,
            })
        } else {
            Err(syn::Error::new(
                item.span(),
                "Invalid xcq::entrypoint, expected item fn",
            ))
        }
    }
}
