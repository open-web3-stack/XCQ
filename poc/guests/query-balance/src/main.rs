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
    fn query_balance(variant: u32, account_id_ptr: u32, account_id_size: u32) -> u64;
}
// return value is u64 instead of (u32, u32) due to https://github.com/koute/polkavm/issues/116
// higher 32bits are address, lower 32bits are size
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, size: u32) -> u64 {
    // ready first byte from ptr
    let mut sum = 0u64;
    let variant = unsafe { core::ptr::read_volatile(ptr as *const u8) };
    // hardcode since we know account_id_num
    let account_id_size = (size - 1) / 2;
    for i in 0..2 {
        sum += unsafe { query_balance(variant as u32, ptr + 1 + (account_id_size * i), account_id_size) };
    }
    let ptr = polkavm_derive::sbrk(core::mem::size_of_val(&sum));
    if ptr.is_null() {
        return 0;
    }
    unsafe {
        core::ptr::write_volatile(ptr as *mut u64, sum);
    }
    (ptr as u64) << 32 | (core::mem::size_of::<u64>() as u64)
}
