mod extension;
mod helper;

use crate::extension_decl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

pub fn expand(mut def: Def) -> TokenStream2 {
    let extension_expanded = extension::expand_extension(&mut def);

    let new_items = quote::quote! {
        #extension_expanded
    };

    def.item
        .content
        .as_mut()
        .expect("This is checked by parsing")
        .1
        .push(syn::Item::Verbatim(new_items));
    def.item.into_token_stream()
}
