use proc_macro::TokenStream;
mod decl_extensions;
mod impl_extensions;
mod runtime_metadata;
mod utils;

#[proc_macro]
pub fn decl_extensions(input: TokenStream) -> TokenStream {
    decl_extensions::decl_extensions_impl(input)
}

#[proc_macro]
pub fn impl_extensions(input: TokenStream) -> TokenStream {
    impl_extensions::impl_extensions_impl(input)
}
