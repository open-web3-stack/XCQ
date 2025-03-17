use super::Def;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn generate_preludes(def: &Def) -> TokenStream2 {
    let extern_crate = quote! {
        extern crate alloc;
    };

    let global_allocator = quote! {
        #[global_allocator]
        static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
    };

    let panic_fn = quote! {
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            unsafe {
                core::arch::asm!("unimp", options(noreturn));
            }
        }
    };

    let polkavm_derive = &def.polkavm_derive;

    let host_call_fn = quote! {
        #[#polkavm_derive::polkavm_import]
        extern "C" {
            fn host_call(extension_id:u64, call_ptr:u32, call_len: u32) -> u64;
        }
    };

    quote! {

        #extern_crate

        #global_allocator

        #panic_fn

        #host_call_fn
    }
}
