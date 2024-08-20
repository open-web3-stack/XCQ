use super::{Def, EntrypointDef};
use inflector::Inflector;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{ItemFn, Result};
pub fn expand(def: Def) -> Result<TokenStream2> {
    let preludes = generate_preludes();
    // eprintln!("def{:?}", def.calls);
    let host_calls = def
        .calls
        .iter()
        .map(|call_def| generate_call(&call_def.item_fn))
        .collect::<Result<Vec<_>>>()?;
    let entrypoint_def = &def.entrypoint.item_fn;
    let main_fn = generate_main(&def.entrypoint)?;
    Ok(quote! {
        #preludes
        #entrypoint_def
        #(#host_calls)*
        #main_fn
    })
}

// At guest side, we only need call_ptr and size to perform call,
// the actual function signature is used at host side to construct the call data
fn generate_call(item: &ItemFn) -> Result<TokenStream2> {
    let camel_case_ident = syn::Ident::new(&item.sig.ident.to_string().to_pascal_case(), item.sig.ident.span());
    let call_name = format_ident!("{}Call", camel_case_ident);
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
            pub size: u32,
        }
        impl #call_name {
            pub fn call(&self) -> #return_ty {
                let res = unsafe {
                    host_call(self.extension_id, self.call_ptr, self.size)
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
fn pass_byte_to_host() -> TokenStream2 {
    // TODO check res type to determine the appropriate serializing method
    quote! {
        let res_bytes = res.to_le_bytes();
        let ptr = polkavm_derive::sbrk(res_bytes.len());
        if ptr.is_null(){
            return 0;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(res_bytes.as_ptr(),ptr,res_bytes.len());
        }
        (res_bytes.len() as u64) << 32 | (ptr as u64)
    }
}

fn generate_main(entrypoint: &EntrypointDef) -> Result<TokenStream2> {
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
                    let extension_id = unsafe {core::ptr::read_volatile((ptr) as *const u64)};
                    // for multi calls, we assume the number of calls are given in the call data
                    let num = unsafe {core::ptr::read_volatile((ptr+8) as *const u8)};
                    let size = unsafe {core::ptr::read_volatile((ptr+9) as *const u8)};
                    for i in 0..num {
                        #calls_ident.push(#ty {
                            extension_id: extension_id,
                            call_ptr: ptr+10+(i as u32)*(size as u32),
                            size: size as u32
                        });
                    }
                }
                .into_iter()
            })
        } else {
            get_call_data.extend({
                quote! {
                    let extension_id = unsafe{core::ptr::read_volatile((ptr) as *const u64)};
                    let size = unsafe {core::ptr::read_volatile((ptr+8) as *const u8)};
                    let #calls_ident = #ty {
                        extension_id: extension_id,
                        call_ptr: ptr+9,
                        size:size as u32
                    };
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
        extern "C" fn main(ptr: u32, size:u32) -> u64 {
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

    quote! {

        #extern_crate

        #panic_fn

        #host_call_fn
    }
}
