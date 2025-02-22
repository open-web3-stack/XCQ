use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};
mod expand;
mod parse;
pub use parse::{CallDef, Def, EntrypointDef};

pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemMod);
    match parse::Def::try_from(item) {
        Ok(def) => expand::expand(def).unwrap_or_else(|e| e.to_compile_error()).into(),
        Err(e) => e.to_compile_error().into(),
    }
}
