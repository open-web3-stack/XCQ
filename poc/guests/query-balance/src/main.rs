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
    fn query_balances(variant: u32, accounts_ptr: u32, accounts_len: u32) -> u64;
}
// return value is u64 instead of (u32, u32) due to https://github.com/koute/polkavm/issues/116
// higher 32bits are address, lower 32bits are size
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, len: u32) -> u64 {
    // ready first byte from ptr
    let byte_ptr = ptr as *const u8;
    let variant = unsafe { core::ptr::read_volatile(byte_ptr) };
    // TODO: need to figure out which encoding/decoding mechanism is appropriate, self-describing or just specify size or not  when not.
    // Specifying type may bloat the code, should be researched.
    // some principles: make host functions api more standardized
    // like query_balance(variant, single_account_ptr);
    unsafe { query_balances(variant as u32, byte_ptr.offset(1) as u32, len - 1) }
}
