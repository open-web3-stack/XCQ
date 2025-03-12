use quote::ToTokens;
pub trait MutItemAttrs {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}

/// Take the first extension_decl attribute (e.g. attribute like `#[extension_decl..]`) and decode it to `Attr`
pub(crate) fn take_first_item_extension_decl_attr<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Option<Attr>>
where
    Attr: syn::parse::Parse,
{
    let Some(attrs) = item.mut_item_attrs() else {
        return Ok(None);
    };

    let Some(index) = attrs.iter().position(|attr| {
        attr.path()
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "extension_decl")
    }) else {
        return Ok(None);
    };

    let extension_attr = attrs.remove(index);
    Ok(Some(syn::parse2(extension_attr.into_token_stream())?))
}

/// Take all the extension_decl attributes (e.g. attribute like `#[extension_decl..]`) and decode them to `Attr`
pub(crate) fn take_item_extension_decl_attrs<Attr>(item: &mut impl MutItemAttrs) -> syn::Result<Vec<Attr>>
where
    Attr: syn::parse::Parse,
{
    let mut extension_attrs = Vec::new();

    while let Some(attr) = take_first_item_extension_decl_attr(item)? {
        extension_attrs.push(attr)
    }

    Ok(extension_attrs)
}

impl MutItemAttrs for syn::Item {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        match self {
            Self::Const(item) => Some(item.attrs.as_mut()),
            Self::Enum(item) => Some(item.attrs.as_mut()),
            Self::ExternCrate(item) => Some(item.attrs.as_mut()),
            Self::Fn(item) => Some(item.attrs.as_mut()),
            Self::ForeignMod(item) => Some(item.attrs.as_mut()),
            Self::Impl(item) => Some(item.attrs.as_mut()),
            Self::Macro(item) => Some(item.attrs.as_mut()),
            Self::Mod(item) => Some(item.attrs.as_mut()),
            Self::Static(item) => Some(item.attrs.as_mut()),
            Self::Struct(item) => Some(item.attrs.as_mut()),
            Self::Trait(item) => Some(item.attrs.as_mut()),
            Self::TraitAlias(item) => Some(item.attrs.as_mut()),
            Self::Type(item) => Some(item.attrs.as_mut()),
            Self::Union(item) => Some(item.attrs.as_mut()),
            Self::Use(item) => Some(item.attrs.as_mut()),
            _ => None,
        }
    }
}

impl MutItemAttrs for syn::TraitItem {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        match self {
            Self::Const(item) => Some(item.attrs.as_mut()),
            Self::Fn(item) => Some(item.attrs.as_mut()),
            Self::Type(item) => Some(item.attrs.as_mut()),
            Self::Macro(item) => Some(item.attrs.as_mut()),
            _ => None,
        }
    }
}

impl MutItemAttrs for Vec<syn::Attribute> {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        Some(self)
    }
}

impl MutItemAttrs for syn::ItemMod {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        Some(&mut self.attrs)
    }
}

impl MutItemAttrs for syn::ImplItemFn {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        Some(&mut self.attrs)
    }
}

impl MutItemAttrs for syn::ItemType {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        Some(&mut self.attrs)
    }
}
