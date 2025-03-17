mod preludes;
use super::{Def, ExtensionFn};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
pub fn expand(mut def: Def) -> TokenStream2 {
    let preludes = preludes::generate_preludes(&def);

    let expanded_extension_fns = def
        .extension_fns
        .iter_mut()
        .map(|extension_fn| expand_extension_fn(extension_fn, &def.parity_scale_codec))
        .collect::<Vec<_>>();

    let main_fn = expand_main(&def);

    let new_items = quote! {
        #preludes
        #(#expanded_extension_fns)*
        #main_fn
    };

    def.item
        .content
        .as_mut()
        .expect("This is checked by parsing")
        .1
        .push(syn::Item::Verbatim(new_items));
    def.item.into_token_stream()
}

fn expand_extension_fn(extension_fn: &mut ExtensionFn, parity_scale_codec: &syn::Path) -> TokenStream2 {
    let extension_id = extension_fn.extension_id;
    let fn_index = extension_fn.fn_index;
    let fn_name = &extension_fn.item_fn.sig.ident;
    let args = &extension_fn.item_fn.sig.inputs;
    let enum_name = format_ident!("{}Call", fn_name);
    let expanded_enum = quote! (
        #[allow(non_camel_case_types)]
        #[derive(#parity_scale_codec::Encode, #parity_scale_codec::Decode)]
        enum #enum_name {
            #[codec(index = #fn_index)]
            #fn_name {
                #args
            }
        }
    );
    let arg_names = args
        .iter()
        .map(|arg| {
            let syn::FnArg::Typed(pat_type) = arg else {
                unreachable!("Checked in parse stage")
            };
            &pat_type.pat
        })
        .collect::<Vec<_>>();

    let fn_name_str = fn_name.to_string();
    extension_fn.item_fn.block = Box::new(syn::parse_quote!(
        {
            let encoded_call = #parity_scale_codec::Encode::encode(&#enum_name::#fn_name {
                #(#arg_names),*
            });
            let res = unsafe {
                host_call(#extension_id, encoded_call.as_ptr() as u32, encoded_call.len() as u32)
            };
            let res_ptr = res as u32 as *const u8;
            let res_len = (res >> 32) as usize;
            let mut res_bytes = unsafe { core::slice::from_raw_parts(res_ptr, res_len) };
            #parity_scale_codec::Decode::decode(&mut res_bytes).expect(concat!("Failed to decode result of ", #fn_name_str))
        }
    ));
    let modified_extension_fn = &extension_fn.item_fn;
    quote!(
        #expanded_enum
        #modified_extension_fn
    )
}

fn expand_main(def: &Def) -> TokenStream2 {
    let parity_scale_codec = &def.parity_scale_codec;

    // Get `ident: Type`s
    let arg_pats = def.entrypoint.item_fn.sig.inputs.iter().collect::<Vec<_>>();
    // Get `ident`s
    let arg_identifiers = arg_pats
        .iter()
        .map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                pat_type.pat.to_token_stream()
            } else {
                unreachable!("Checked in parse stage")
            }
        })
        .collect::<Vec<_>>();
    let arg_identifiers_str = arg_identifiers.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();

    let decode_args = quote! {
        #(let #arg_pats = #parity_scale_codec::Decode::decode(&mut arg_bytes).expect(concat!("Failed to decode ", #arg_identifiers_str));)*
    };

    let entrypoint_ident = &def.entrypoint.item_fn.sig.ident;
    let call_entrypoint = quote! {
        let res = #entrypoint_ident(#(#arg_identifiers),*);
    };

    quote! {
        #[polkavm_derive::polkavm_export]
        extern "C" fn pvq(arg_ptr: u32, size: u32) -> u64 {
        let mut arg_bytes = unsafe { core::slice::from_raw_parts(arg_ptr as *const u8, size as usize) };

        #decode_args

        #call_entrypoint

        let encoded_res = #parity_scale_codec::Encode::encode(&res);
        (encoded_res.len() as u64) << 32 | (encoded_res.as_ptr() as u64)

        }
    }
}
