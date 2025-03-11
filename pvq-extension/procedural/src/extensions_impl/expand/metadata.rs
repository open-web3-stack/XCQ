use crate::extensions_impl::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// generate the `metadata` function in the #[extensions_impl] module
pub fn expand_metadata(def: &Def) -> TokenStream2 {
    let pvq_extension = &def.pvq_extension;
    let mut extension_metadata_call_list = Vec::new();

    for impl_ in &def.extension_impls {
        let mut trait_path = impl_.trait_path.clone();
        trait_path.segments.pop();

        // Replace trait_path with a call to the metadata function with the impl struct as generic parameter
        let impl_struct_ident = &def.impl_struct.ident;

        // Create a method call expression instead of a path
        let method_call = quote!(
            #trait_path metadata::<#impl_struct_ident>()
        );

        extension_metadata_call_list.push(method_call);
    }

    // let query_metadata_by_extension_id = quote! {
    //     impl #pvq_extension::ExtensionImplMetadata for #extension_impl_name {
    //         fn extension_metadata(extension_id: #pvq_extension::ExtensionIdTy) -> #pvq_extension::metadata::ExtensionMetadata {
    //             let extension_metadata = match extension_id {
    //                 #(#extension_ids => #extension_metadata_list,)*
    //                 _ => panic!("Unknown extension id"),
    //             };
    //             extension_metadata
    //         }
    //     }
    // };

    let metadata = quote! {
        pub fn metadata() -> #pvq_extension::metadata::Metadata {
            #pvq_extension::metadata::Metadata::new(
                scale_info::prelude::vec![ #( #extension_metadata_call_list, )* ],
            )
        }
    };
    metadata
}
