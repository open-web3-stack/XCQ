use crate::extensions_impl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

fn expand_extensions_tuple(def: &Def) -> TokenStream2 {
    let mut extensions = Vec::new();
    let impl_struct_ident = &def.impl_struct.ident;
    for impl_ in &def.extension_impls {
        let mut trait_path = impl_.trait_path.clone();

        // Replace the last segment of the trait path with Functions<Impl>
        if let Some(segment) = trait_path.segments.last_mut() {
            *segment = syn::parse_quote!(Functions<#impl_struct_ident>);
        }

        extensions.push(trait_path);
    }

    quote::quote! {
        pub type Extensions = (
            #(#extensions),*
        );
    }
}

pub fn expand(mut def: Def) -> TokenStream2 {
    let extensions_tuple_expanded = expand_extensions_tuple(&def);

    let new_items = quote::quote! {
        #extensions_tuple_expanded
    };

    def.item
        .content
        .as_mut()
        .expect("This is checked by parsing")
        .1
        .push(syn::Item::Verbatim(new_items));
    def.item.into_token_stream()
}
