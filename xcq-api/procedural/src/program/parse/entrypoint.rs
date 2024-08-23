use syn::spanned::Spanned;
use syn::{AngleBracketedGenericArguments, Item, ItemFn, PathArguments, Result, TypePath};
#[derive(Debug)]
pub struct EntrypointDef {
    pub item_fn: ItemFn,
    pub arg_types: Vec<ArgType>,
}

#[derive(Debug)]
pub struct ArgType {
    pub multi: bool,
    pub ty: Box<syn::Type>,
}

impl EntrypointDef {
    pub fn try_from(_span: proc_macro2::Span, _index: usize, item: &mut Item) -> syn::Result<Self> {
        if let syn::Item::Fn(item_fn) = item {
            let mut arg_types = Vec::new();
            for input in &item_fn.sig.inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    // match vec
                    let multi = if let syn::Type::Path(type_path) = pat_type.ty.as_ref() {
                        // TODO: more accurate way to detect vector usage
                        type_path.path.segments.iter().any(|segment| segment.ident == "Vec")
                    } else {
                        return Err(syn::Error::new(input.span(), "entrypoint args must be owned types"));
                    };
                    if multi {
                        let inner_type = extract_inner_type(pat_type.ty.as_ref())?;
                        arg_types.push(ArgType {
                            multi: true,
                            ty: Box::new(inner_type.clone()),
                        });
                    } else {
                        arg_types.push(ArgType {
                            multi: false,
                            ty: pat_type.ty.clone(),
                        })
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

fn extract_inner_type(ty: &syn::Type) -> Result<&syn::Type> {
    if let syn::Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "Vec" {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return Ok(inner_type);
                    }
                }
            }
        }
    }
    Err(syn::Error::new_spanned(ty, "Expected Vec<_>"))
}
