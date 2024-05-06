#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;

#[global_allocator]
static ALLOCATOR: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}

#[polkavm_derive::polkavm_import]
extern "C" {
    fn host_call(ptr: u32) -> u32;
}

// return value is u64 instead of (u32, u32) due to https://github.com/koute/polkavm/issues/116
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32) -> u64 {
    // ready first byte from ptr
    let byte = unsafe { core::ptr::read_volatile(ptr as *const u8) };
    match byte {
        0 => {
            let val = b"test";
            let val = Box::new(*val);
            // leak val
            let val = Box::into_raw(val);
            (val as u32 as u64) << 32 | 4
        }
        1 => {
            let val = unsafe { core::ptr::read_volatile((ptr + 1) as *const u8) };
            let val = Box::new(val);
            let ptr = Box::into_raw(val);
            let res = unsafe { host_call(ptr as u32) };
            let ret = res + 1;
            let ptr = Box::into_raw(Box::new(ret));
            (ptr as u32 as u64) << 32 | 1
        }
        _ => 0,
    }
}
