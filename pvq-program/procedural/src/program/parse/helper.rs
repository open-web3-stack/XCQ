use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
pub trait MutItemAttrs {
    fn mut_item_attrs(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}
/// Take the first item attribute (e.g. attribute like `#[pvq..]`) and decode it to `Attr`
pub(crate) fn take_first_program_attr(item: &mut impl MutItemAttrs) -> syn::Result<Option<syn::Attribute>> {
    let Some(attrs) = item.mut_item_attrs() else {
        return Ok(None);
    };

    let Some(index) = attrs.iter().position(|attr| {
        attr.path()
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "program")
    }) else {
        return Ok(None);
    };

    let pvq_attr = attrs.remove(index);
    Ok(Some(pvq_attr))
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

pub fn generate_crate_access(def_crate: &str) -> syn::Result<syn::Path> {
    let ident = match crate_name(def_crate) {
        Ok(FoundCrate::Itself) => {
            let name = def_crate.replace('-', "_");
            Ok(syn::Ident::new(&name, Span::call_site()))
        }
        Ok(FoundCrate::Name(name)) => Ok(syn::Ident::new(&name, Span::call_site())),
        Err(e) => Err(syn::Error::new(Span::call_site(), e)),
    }?;
    Ok(syn::Path::from(ident))
}
