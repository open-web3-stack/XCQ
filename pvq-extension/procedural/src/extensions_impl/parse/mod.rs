mod extension;
mod helper;
mod impl_struct;
use crate::utils::generate_crate_access;
use syn::spanned::Spanned;

mod keyword {
    syn::custom_keyword!(extensions_impl);
    syn::custom_keyword!(impl_struct);
    syn::custom_keyword!(extension);
}

pub struct Def {
    pub item: syn::ItemMod,
    pub impl_struct: impl_struct::ImplStruct,
    pub extension_impls: Vec<extension::ExtensionImpl>,
    pub pvq_extension: syn::Path,
}

impl Def {
    pub fn try_from(mut item: syn::ItemMod) -> syn::Result<Self> {
        let pvq_extension = generate_crate_access("pvq-extension")?;
        let item_span = item.span();
        let items = &mut item
            .content
            .as_mut()
            .ok_or_else(|| {
                let msg = "Invalid extensions_impl definition, expected mod to be inline.";
                syn::Error::new(item_span, msg)
            })?
            .1;
        let mut impl_struct = None;
        let mut extension_impls = Vec::new();
        for item in items.iter_mut() {
            let extensions_impl_attr: Option<ExtensionsImplAttr> = helper::take_first_item_extensions_impl_attr(item)?;
            match extensions_impl_attr {
                Some(ExtensionsImplAttr::ImplStruct(_)) if impl_struct.is_none() => {
                    impl_struct = Some(impl_struct::ImplStruct::try_from(item)?);
                }
                Some(ExtensionsImplAttr::Extension(_)) => {
                    extension_impls.push(extension::ExtensionImpl::try_from(item)?);
                }
                Some(attr) => {
                    let msg = "Invalid duplicated attribute";
                    return Err(syn::Error::new(attr.span(), msg));
                }
                None => (),
            }
        }

        if extension_impls.is_empty() {
            let msg = "At least one `#[extensions_impl::extension]` is required";
            return Err(syn::Error::new(item_span, msg));
        }
        Ok(Self {
            item,
            impl_struct: impl_struct
                .ok_or_else(|| syn::Error::new(item_span, "Missing `#[extensions_impl::impl_struct]`"))?,
            extension_impls,
            pvq_extension,
        })
    }
}

enum ExtensionsImplAttr {
    ImplStruct(proc_macro2::Span),
    Extension(proc_macro2::Span),
}

impl syn::parse::Parse for ExtensionsImplAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        content.parse::<keyword::extensions_impl>()?;
        content.parse::<syn::Token![::]>()?;

        let lookahead = content.lookahead1();
        if lookahead.peek(keyword::impl_struct) {
            let span = content.parse::<keyword::impl_struct>()?.span();
            Ok(Self::ImplStruct(span))
        } else if lookahead.peek(keyword::extension) {
            let span = content.parse::<keyword::extension>()?.span();
            Ok(Self::Extension(span))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ExtensionsImplAttr {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::ImplStruct(span) => *span,
            Self::Extension(span) => *span,
        }
    }
}
