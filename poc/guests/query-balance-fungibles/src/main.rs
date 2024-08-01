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
    fn call(extension_id: u64, call_ptr: u32, call_len: u32) -> u64;
}

#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, size: u32) -> u64 {
    // no variant for this input, since the return type is same for total_supply/balance
    let extension_id = unsafe { core::ptr::read_volatile(ptr as *const u64) };
    let num_query = unsafe { core::ptr::read_volatile((ptr + 8) as *const u8) };
    let query_size = (size - 9) / num_query as u32;
    let mut sum = 0u64;
    // in this settings, the return type is same for total_supply/balance
    // otherwise, we need to recognize return type through input data
    for i in 0..num_query {
        let res = unsafe { call(extension_id, ptr + 9 + query_size * i as u32, query_size) };
        let res_len = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as *const u8;
        let res_bytes = unsafe { core::slice::from_raw_parts(res_ptr, res_len as usize) };
        sum += u64::from_le_bytes(res_bytes.try_into().unwrap());
    }
    let sum_bytes = sum.to_le_bytes();
    let ptr = polkavm_derive::sbrk(sum_bytes.len());
    if ptr.is_null() {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(sum_bytes.as_ptr(), ptr, sum_bytes.len());
    }
    (sum_bytes.len() as u64) << 32 | (ptr as u64)
}
