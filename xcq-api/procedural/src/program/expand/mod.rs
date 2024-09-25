use super::{CallDef, Def, EntrypointDef};
use inflector::Inflector;
use parity_scale_codec::Encode;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{ItemFn, PathArguments, Result};
pub fn expand(def: Def) -> Result<TokenStream2> {
    let preludes = generate_preludes();
    // eprintln!("def{:?}", def.calls);
    let host_calls = def
        .calls
        .iter()
        .map(|call_def| generate_call(&call_def.item_fn))
        .collect::<Result<Vec<_>>>()?;
    let entrypoint_def = generate_entrypoint(&def.entrypoint)?;
    let main_fn = generate_main(&def.calls, &def.entrypoint)?;
    Ok(quote! {
        #preludes
        #entrypoint_def
        #(#host_calls)*
        #main_fn
    })
}

// Generate a callable that holds the call data and a method to perform the call
// At compile time: extension_id and call_index are specified
// and they can be used to construct the runtime call data by front-end.
// At run time: we only forward call_data(including call_ptr and size) to host,
// and then we got the return bytes and convert to concrete numeric type
fn generate_call(item: &ItemFn) -> Result<TokenStream2> {
    let camel_case_ident = syn::Ident::new(&item.sig.ident.to_string().to_pascal_case(), item.sig.ident.span());
    let call_name = format_ident!("{}Call", camel_case_ident);
    // This return_ty is a concrete unsigned integer type
    let return_ty = match &item.sig.output {
        syn::ReturnType::Type(_, return_ty) => return_ty,
        _ => {
            return Err(syn::Error::new_spanned(
                item.sig.fn_token,
                "expected function to have a return type",
            ))
        }
    };
    let expand = quote! {
        struct #call_name {
            pub extension_id: u64,
            pub call_ptr: u32,
            pub call_size: u32,
        }
        impl #call_name {
            pub fn call(&self) ->  #return_ty   {
                // TODO: use xcq-types to represent the return type
                let res = unsafe {
                    host_call(self.extension_id, self.call_ptr, self.call_size)
                };
                let res_len = (res >> 32) as u32;
                let res_ptr = (res & 0xffffffff) as *const u8;
                let res_bytes = unsafe {
                    core::slice::from_raw_parts(res_ptr, res_len as usize)
                };
                let (int_bytes, _) = res_bytes.split_at(core::mem::size_of::<#return_ty>());
                #return_ty::from_le_bytes(int_bytes.try_into().unwrap())
            }
        }
    };
    Ok(expand)
}

// Modify the calculation parts in the entrypoint function
// use type assertion to get the return type at runtime
fn generate_entrypoint(entrypoint: &EntrypointDef) -> Result<TokenStream2> {
    Ok(entrypoint.item_fn.to_token_stream())
}

fn pass_byte_to_host() -> TokenStream2 {
    // TODO check res type to determine the appropriate serializing method
    quote! {
        let res_bytes = res.to_le_bytes();
        let res_ptr = polkavm_derive::sbrk(0);
        let end_ptr = polkavm_derive::sbrk(res_bytes.len());
        if end_ptr.is_null(){
            return 0;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(res_bytes.as_ptr(),res_ptr,res_bytes.len());
        }
        (res_bytes.len() as u64) << 32 | (res_ptr as u64)
    }
}

fn generate_return_ty_assertion(call_def: &CallDef) -> Result<TokenStream2> {
    let call_ty = &call_def.item_fn.sig.output;
    // TODO: bytes representation is to be decided
    let expected_ty_bytes = match call_ty {
        syn::ReturnType::Type(_, return_ty) => match return_ty.as_ref() {
            syn::Type::Path(path) => {
                let last_segment = path
                    .path
                    .segments
                    .last()
                    .ok_or_else(|| syn::Error::new_spanned(path, "expected function return type to be a path"))?;
                match last_segment.ident.to_string().as_str() {
                    "u8" => {
                        let encoded_ty_bytes = xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U8).encode();
                        quote! {
                            &[#(#encoded_ty_bytes),*]
                        }
                    }
                    "u16" => {
                        let encoded_ty_bytes = xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U16).encode();
                        quote! {
                            &[#(#encoded_ty_bytes),*]
                        }
                    }
                    "u32" => {
                        let encoded_ty_bytes = xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U32).encode();
                        quote! {
                            &[#(#encoded_ty_bytes),*]
                        }
                    }
                    "u64" => {
                        let encoded_ty_bytes = xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U64).encode();
                        quote! {
                            &[#(#encoded_ty_bytes),*]
                        }
                    }
                    "u128" => {
                        let encoded_ty_bytes = xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U128).encode();
                        quote! {
                            &[#(#encoded_ty_bytes),*]
                        }
                    }
                    "Vec" => {
                        if let PathArguments::AngleBracketed(generic_args) = &last_segment.arguments {
                            if generic_args.args.len() == 1 {
                                match generic_args.args.first() {
                                    Some(syn::GenericArgument::Type(syn::Type::Path(path)))
                                        if path.path.is_ident("u8") =>
                                    {
                                        let encoded_ty_bytes = xcq_types::XcqType::Sequence(Box::new(
                                            xcq_types::XcqType::Primitive(xcq_types::PrimitiveType::U8),
                                        ))
                                        .encode();
                                        quote! {
                                            &[#(#encoded_ty_bytes),*]
                                        }
                                    }
                                    _ => quote! { &[0u8] },
                                }
                            } else {
                                quote! { &[0u8] }
                            }
                        } else {
                            quote! {&[0u8]}
                        }
                    }
                    _ => quote! { &[0u8] },
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    call_ty,
                    "expected function return type to be a path",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                call_ty,
                "expected function return type to be a path",
            ))
        }
    };
    let extension_id = call_def.extension_id;
    let call_index = call_def.call_index;
    let item_fn_ident_string = &call_def.item_fn.sig.ident.to_string();
    let expanded = quote! {
        if !assert_return_ty(#expected_ty_bytes, #extension_id, #call_index) {
            panic!("function {} (extension {} call {}) return type mismatch", #item_fn_ident_string, #extension_id, #call_index);
        }
    };
    Ok(expanded)
}

fn generate_main(call_defs: &[CallDef], entrypoint: &EntrypointDef) -> Result<TokenStream2> {
    let assertions = call_defs
        .iter()
        .map(generate_return_ty_assertion)
        .collect::<Result<Vec<_>>>()?;
    let assert_program_types_match = quote! {
        #(#assertions)*
    };
    // Construct call_data
    let mut get_call_data = TokenStream2::new();
    for (arg_type_index, arg_type) in entrypoint.arg_types.iter().enumerate() {
        let ty = &arg_type.ty;
        let calls_ident = format_ident!("calls_{}", arg_type_index);
        if arg_type.multi {
            get_call_data.extend(
                quote! {
                    let mut #calls_ident:alloc::vec::Vec<#ty> = alloc::vec::Vec::new();
                }
                .into_iter(),
            );
            get_call_data.extend({
                quote! {
                    // TODO: extension_id can be eliminated since we have call_def indicating it
                    let extension_id = unsafe {core::ptr::read_volatile((arg_ptr) as *const u64)};
                    // for multi calls, we assume the number of calls are given in the call data
                    let call_num = unsafe {core::ptr::read_volatile((arg_ptr+8) as *const u8)};
                    let call_size = unsafe {core::ptr::read_volatile((arg_ptr+9) as *const u8)};
                    for i in 0..call_num {
                        #calls_ident.push(#ty {
                            extension_id: extension_id,
                            call_ptr: arg_ptr+10+(i as u32)*(call_size as u32),
                            call_size: call_size as u32
                        });
                    }
                    arg_ptr += 10 + (call_num as u32)*(call_size as u32);
                }
                .into_iter()
            })
        } else {
            get_call_data.extend({
                quote! {
                    let extension_id = unsafe {core::ptr::read_volatile((arg_ptr) as *const u64)};
                    let call_size = unsafe {core::ptr::read_volatile((arg_ptr+8) as *const u8)};
                    let #calls_ident = #ty {
                        extension_id: extension_id,
                        call_ptr: arg_ptr+9,
                        call_size: call_size as u32
                    };
                    arg_ptr += 9 + call_size as u32;
                }
                .into_iter()
            })
        }
    }
    // call entrypoint
    let entrypoint_call_args = (0..entrypoint.arg_types.len())
        .map(|arg_type_index| {
            let calls_ident = format_ident!("calls_{}", arg_type_index);
            quote! {
                #calls_ident
            }
        })
        .collect::<Vec<_>>();
    let fn_ident = &entrypoint.item_fn.sig.ident;
    let call_entrypoint = quote! {
        let res = #fn_ident(#(#entrypoint_call_args),*);
    };
    // pass bytes back to host
    let pass_bytes_back = pass_byte_to_host();

    let main = quote! {
        #[polkavm_derive::polkavm_export]
        extern "C" fn main(mut arg_ptr: u32, size:u32) -> u64 {
            #assert_program_types_match
            #get_call_data
            #call_entrypoint
            #pass_bytes_back
        }
    };
    Ok(main)
}

fn generate_preludes() -> TokenStream2 {
    let extern_crate = quote! {
        extern crate alloc;
    };
    let panic_fn = quote! {
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            unsafe {
                core::arch::asm!("unimp", options(noreturn));
            }
        }
    };

    let host_call_fn = quote! {
        #[polkavm_derive::polkavm_import]
        extern "C" {
            fn host_call(extension_id:u64, call_ptr:u32, call_len: u32) -> u64;
        }
    };

    let host_return_ty_fn = quote! {
        #[polkavm_derive::polkavm_import]
        extern "C" {
            fn return_ty(extension_id:u64, call_index:u32) -> u64;
        }
    };

    let assert_return_ty_fn = quote! {
        fn assert_return_ty(expected_ty_bytes: &[u8],extension_id:u64, call_index:u32) -> bool {
            let return_ty = unsafe {return_ty(extension_id, call_index)};
            let ty_len = (return_ty >> 32) as u32;
            let ty_ptr = (return_ty & 0xffffffff) as *const u8;
            let ty_bytes = unsafe {
                core::slice::from_raw_parts(ty_ptr, ty_len as usize)
            };
            expected_ty_bytes == ty_bytes
        }
    };
    quote! {

        #extern_crate

        #panic_fn

        #host_call_fn

        #host_return_ty_fn

        #assert_return_ty_fn
    }
}
