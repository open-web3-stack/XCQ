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
    fn host_call(extension_id: u64, call_ptr: u32, call_len: u32) -> u64;
}

#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, size: u32) -> u64 {
    let extension_id = unsafe { core::ptr::read_volatile(ptr as *const u64) };
    unsafe { host_call(extension_id, ptr + 8, size - 8) }
}
