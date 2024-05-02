#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}

#[polkavm_derive::polkavm_import]
extern "C" {
    fn host_call() -> u32;
}

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u32 {
    42 + unsafe { host_call() }
}
