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
    fn call_sbrk_indirectly(size: u32) -> u32;
    fn host_write(src: u32, size: u32, dst: u32);
    fn host_call(ptr: u32) -> u32;
}

// return value is u64 instead of (u32, u32) due to https://github.com/koute/polkavm/issues/116
// higher 32bits are address, lower 32bits are size
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32) -> u64 {
    // ready first byte from ptr
    let byte = unsafe { core::ptr::read_volatile(ptr as *const u8) };
    match byte {
        0 => {
            let val = b"test";
            let out = unsafe { call_sbrk_indirectly(core::mem::size_of_val(val) as u32) };
            if out == 0 {
                return 0;
            }
            unsafe { host_write(val.as_ptr() as u32, val.len() as u32, out) };
            (out as u64) << 32 | val.len() as u64
        }
        1 => {
            let res = unsafe { host_call(ptr + 1) };
            let ret = res + 1;
            let size = core::mem::size_of_val(&ret) as u32;
            let out = unsafe { call_sbrk_indirectly(size) };
            if out == 0 {
                return 0;
            }
            // not a real write to host, instead let the host read the memory from the guest's stack
            unsafe { host_write(&ret as *const u32 as u32, size, out) };
            (out as u64) << 32 | size as u64 / core::mem::size_of::<u32>() as u64
        }
        _ => 0,
    }
}
