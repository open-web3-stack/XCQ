use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};
mod expand;
mod parse;
pub(crate) use parse::{Def, ExtensionFn};

pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemMod);
    match parse::Def::try_from(item) {
        Ok(def) => expand::expand(def).into(),
        Err(e) => e.to_compile_error().into(),
    }
}
