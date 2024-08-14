use syn::spanned::Spanned;
use syn::{Error, ItemMod, Result};
mod call;
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
                let msg = "Invalid pallet definition, expected mod to be inlined.";
                syn::Error::new(mod_span, msg)
            })?
            .1;

        let mut calls = Vec::new();
        let mut entrypoint = None;

        for (index, item) in items.iter_mut().enumerate() {
            let xcq_attr: Option<XcqAttr> = helper::take_first_item_xcq_attr(item)?;

            match xcq_attr {
                Some(XcqAttr::CallDef(span, extern_types)) => {
                    calls.push(call::CallDef::try_from(span, index, item, extern_types)?);
                }
                Some(XcqAttr::Entrypoint(span)) => {
                    if entrypoint.is_some() {
                        return Err(Error::new(span, "Only one entrypoint function is allowed"));
                    }
                    let e = entrypoint::EntrypointDef::try_from(span, index, item)?;
                    entrypoint = Some(e);
                }
                None => {
                    return Err(Error::new(
                        item.span(),
                        "Invalid attribute, expected `#[xcq::call_def]` or `#[xcq::entrypoint]`",
                    ));
                }
            }
        }
        let entrypoint = match entrypoint {
            Some(entrypoint) => entrypoint,
            None => {
                return Err(Error::new(mod_span, "No entrypoint function found"));
            }
        };
        let def = Def { calls, entrypoint };

        Ok(def)
    }
}

/// List of additional token to be used for parsing.
mod keyword {
    syn::custom_keyword!(xcq);
    syn::custom_keyword!(call_def);
    syn::custom_keyword!(extern_types);
    syn::custom_keyword!(entrypoint);
}
enum XcqAttr {
    CallDef(proc_macro2::Span, Option<ExternTypesAttr>),
    Entrypoint(proc_macro2::Span),
}

// Custom parsing for xcq attribute
impl syn::parse::Parse for XcqAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        content.parse::<keyword::xcq>()?;
        content.parse::<syn::Token![::]>()?;

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::call_def) {
            let span = content.parse::<keyword::call_def>().expect("peeked").span();
            let extern_types = match content.is_empty() {
                true => None,
                false => Some(ExternTypesAttr::parse(&content)?),
            };
            Ok(XcqAttr::CallDef(span, extern_types))
        } else if lookahead.peek(keyword::entrypoint) {
            Ok(XcqAttr::Entrypoint(content.parse::<keyword::entrypoint>()?.span()))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternTypesAttr {
    pub types: Vec<syn::Type>,
    pub span: proc_macro2::Span,
}

impl syn::parse::Parse for ExternTypesAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::extern_types) {
            let span = content.parse::<keyword::extern_types>().expect("peeked").span();
            content.parse::<syn::Token![=]>().expect("peeked");
            let list;
            syn::bracketed!(list in content);
            let types = list.parse_terminated(syn::Type::parse, syn::Token![,])?;
            Ok(ExternTypesAttr {
                types: types.into_iter().collect(),
                span,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
