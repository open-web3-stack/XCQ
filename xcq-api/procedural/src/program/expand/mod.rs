use super::{Def, EntrypointDef};
use inflector::Inflector;
use proc_macro::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Error, FnArg, ItemFn, Result};
pub fn expand(def: Def) -> TokenStream2 {}

// At guest side, we only need call_ptr and size to perform call,
// the actual function signature is used at host side to construct the call data
fn generate_call(item: &ItemFn) -> TokenStream2 {
    let camel_case_ident = syn::Ident::new(&item.sig.ident.to_string().to_camel_case(), item.sig.ident.span());
    let call_name = format_ident!("{}Call", camel_case_ident);
    quote! {
        struct #call_name {
            pub extension_id: u64,
            pub call_ptr: u32,
            pub size: u32,
        }
        impl #call_name {
            pub fn call(&self) {
                unsafe {
                    call(self.extension_id, self.call_ptr, self.size);
                }
            }
        }
    }
}

fn generate_main(entrypoint: &EntrypointDef) -> Result<TokenStream2> {
    for (index, arg_type) in entrypoint.arg_types.iter().enumerate() {
        let ty = arg_type.ty;
        if arg_type.multi {
            tokens.extend({
                quote! {
                    let extension_id = unsafe {core::ptr::read_volatile((ptr) as *const u64)};
                    let num = unsafe {core::ptr::read_volatile((ptr+8) as *const u8)};
                    let size = unsafe {core::ptr::read_volatile((ptr+9) as *const u8)};
                    for index in 0..num {
                        calls.push(#ty {
                            extension_id: extension_id,
                            call_ptr: ptr+9+index*size,
                            size: size
                        })
                    }
                }
                .into_iter()
            })
        } else {
            tokens.extend({
                quote! {
                    let extension_id = unsafe{core::ptr::read_volatile((ptr) as *const u64)};
                    let size = unsafe {core::ptr::read_volatile((ptr+8) as *const u8)};
                    #ty {
                        extension_id: extension_id,
                        call_ptr: ptr+9,
                        size:size
                    }
                }
                .into_iter()
            })
        }
    }
    Ok(quote! {
        #[polkavm_derive::polkavm_export]
        extern "C" fn main(ptr: u32, size:u32) -> u64 {
            for i in 0..num_calls {

            }
            entrypoint(#(#call_names),*);
        }
    })
}

fn generate_preludes() -> TokenStream2 {
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
            fn call(extension_id:u64, call_ptr:u32, call_len: u32) -> u64;
        }
    };

    quote! {
        #![no_std]
        #![no_main]

        #panic_fn

        #host_call_fn
    }
}
