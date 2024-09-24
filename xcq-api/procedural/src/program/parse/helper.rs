pub trait MutItemAttrs {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}
/// Take the first item attribute (e.g. attribute like `#[xcq..]`) and decode it to `Attr`
pub(crate) fn take_first_xcq_attr(item: &mut impl MutItemAttrs) -> syn::Result<Option<syn::Attribute>> {
    let Some(attrs) = item.mut_item_attrs() else {
        return Ok(None);
    };

    let Some(index) = attrs.iter().position(|attr| {
        attr.path()
            .segments
            .first()
            .map_or(false, |segment| segment.ident == "xcq")
    }) else {
        return Ok(None);
    };

    let xcq_attr = attrs.remove(index);
    Ok(Some(xcq_attr))
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
