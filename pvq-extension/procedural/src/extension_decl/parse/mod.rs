pub mod extension;
mod helper;

use crate::utils::generate_crate_access;
use syn::spanned::Spanned;

mod keyword {
    syn::custom_keyword!(extension_decl);
    syn::custom_keyword!(extension);
}
pub struct Def {
    pub item: syn::ItemMod,
    pub extension: extension::Extension,
    pub pvq_extension: syn::Path,
}

impl Def {
    pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
        let pvq_extension = generate_crate_access("pvq-extension")?;
        let item_span = item.span();
        let mod_name = &item.ident;
        let items = &mut item
            .content
            .as_mut()
            .ok_or_else(|| {
                let msg = "Invalid extension_decl definition, expected mod to be inline.";
                syn::Error::new(item_span, msg)
            })?
            .1;
        let mut extension = None;
        for item in items.iter_mut() {
            let extension_attr: Option<ExtensionDeclAttr> = helper::take_first_item_extension_decl_attr(item)?;

            match extension_attr {
                Some(ExtensionDeclAttr::Extension(_)) if extension.is_none() => {
                    extension = Some(extension::Extension::try_from(&mod_name.to_string(), item)?);
                }
                Some(attr) => {
                    let msg = "Invalid duplicated attribute";
                    return Err(syn::Error::new(attr.span(), msg));
                }
                None => (),
            }
        }

        Ok(Self {
            item,
            extension: extension.ok_or_else(|| syn::Error::new(item_span, "Missing `#[extension_decl::extension]`"))?,
            pvq_extension,
        })
    }
}

/// Parse attributes for item in extension module
/// syntax must be `extension_decl::` (e.g. `#[extension_decl::config]`)
enum ExtensionDeclAttr {
    Extension(proc_macro2::Span),
}

impl syn::parse::Parse for ExtensionDeclAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        content.parse::<keyword::extension_decl>()?;
        content.parse::<syn::Token![::]>()?;

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::extension) {
            let span = content.parse::<keyword::extension>()?.span();
            Ok(Self::Extension(span))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ExtensionDeclAttr {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Extension(span) => *span,
        }
    }
}
