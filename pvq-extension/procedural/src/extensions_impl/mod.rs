mod expand;
mod parse;
pub use parse::Def;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned;

pub fn extensions_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        let msg = "Invalid extensions_impl macro call: unexpected attribute. Macro call must be bare, such as `#[extensions_impl]`.";
        let span = TokenStream2::from(attr).span();
        return syn::Error::new(span, msg).to_compile_error().into();
    }

    let item = syn::parse_macro_input!(item as syn::ItemMod);
    match parse::Def::try_from(item) {
        Ok(def) => expand::expand(def).into(),
        Err(e) => e.to_compile_error().into(),
    }
}
