use syn::spanned::Spanned;
pub struct ImplStruct {
    pub ident: syn::Ident,
}
impl ImplStruct {
    pub fn try_from(item: &mut syn::Item) -> syn::Result<Self> {
        let syn::Item::Struct(item) = item else {
            let msg = "Invalid extensions_impl::impl_struct, expected struct definition";
            return Err(syn::Error::new(item.span(), msg));
        };

        Ok(Self {
            ident: item.ident.clone(),
        })
    }
}
