use syn::spanned::Spanned;
use syn::{Error, ItemMod, LitInt, Result};
mod call;
pub use call::CallDef;
mod entrypoint;
pub use entrypoint::EntrypointDef;
mod helper;
// program definition
pub struct Def {
    pub calls: Vec<call::CallDef>,
    pub entrypoint: entrypoint::EntrypointDef,
}

impl Def {
    pub fn try_from(mut item_mod: ItemMod) -> Result<Self> {
        let mod_span = item_mod.span();
        let items = &mut item_mod
            .content
            .as_mut()
            .ok_or_else(|| {
                let msg = "No content inside the XCQ program definition";
                syn::Error::new(mod_span, msg)
            })?
            .1;

        let mut calls = Vec::new();
        let mut entrypoint = None;

        for (index, item) in items.iter_mut().enumerate() {
            let xcq_attr = helper::take_first_xcq_attr(item)?;

            if let Some(attr) = xcq_attr {
                if let Some(last_segment) = attr.path().segments.last() {
                    if last_segment.ident == "call_def" {
                        let mut extern_types = None;
                        let mut extension_id = None;
                        let mut call_index = None;
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("extension_id") {
                                let value = meta.value()?;
                                extension_id = Some(value.parse::<LitInt>()?.base10_parse::<u64>()?);
                            } else if meta.path.is_ident("call_index") {
                                let value = meta.value()?;
                                call_index = Some(value.parse::<LitInt>()?.base10_parse::<u32>()?);
                            } else if meta.path.is_ident("extern_types") {
                                let value = meta.value()?;
                                extern_types = Some(value.parse::<ExternTypesAttr>()?);
                            } else {
                                return Err(Error::new(meta.path.span(), "Invalid attribute for `call_def`"));
                            }
                            Ok(())
                        })?;
                        let call =
                            call::CallDef::try_from(attr.span(), index, item, extension_id, call_index, extern_types)?;
                        calls.push(call);
                    } else if last_segment.ident == "entrypoint" {
                        if entrypoint.is_some() {
                            return Err(Error::new(attr.span(), "Only one entrypoint function is allowed"));
                        }
                        entrypoint = Some(entrypoint::EntrypointDef::try_from(attr.span(), index, item)?);
                    } else {
                        return Err(Error::new(
                            item.span(),
                            "Invalid attribute, expected `#[xcq::call_def]` or `#[xcq::entrypoint]`",
                        ));
                    }
                }
            }
        }

        let entrypoint = entrypoint.ok_or_else(|| Error::new(mod_span, "No entrypoint function found"))?;
        let def = Def { calls, entrypoint };

        Ok(def)
    }
}

/// List of additional token to be used for parsing.
mod keyword {
    syn::custom_keyword!(xcq);
    syn::custom_keyword!(call_def);
    syn::custom_keyword!(extension_id);
    syn::custom_keyword!(extern_types);
    syn::custom_keyword!(entrypoint);
}
#[derive(Debug, Clone)]
pub struct ExternTypesAttr {
    pub types: Vec<syn::Type>,
    pub span: proc_macro2::Span,
}

impl syn::parse::Parse for ExternTypesAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        let extern_types = content.parse_terminated(syn::Type::parse, syn::Token![,])?;
        Ok(ExternTypesAttr {
            types: extern_types.into_iter().collect(),
            span: content.span(),
        })
    }
}
