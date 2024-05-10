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
    fn sbrk_indirectly_impl(size: u32) -> u32;
    fn host_write_impl(src: u32, size: u32, dst: u32);
    fn host_call(ptr: u32) -> u32;
}

#[polkavm_derive::polkavm_export]
extern "C" fn sbrk_indirectly(size: usize) -> *mut u8 {
    unsafe { sbrk_indirectly_impl(size as u32) as *mut u8 }
}

fn sbrk(size: usize) -> *mut u8 {
    polkavm_derive::sbrk(size)
}

unsafe fn host_write(dst: *mut u8, val: &[u8]) {
    let src = val.as_ptr();
    let size = val.len();
    unsafe { host_write_impl(src as u32, size as u32, dst as u32) }
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
            let size = core::mem::size_of_val(val);
            let out = sbrk(size);
            if out.is_null() {
                return 0;
            }
            unsafe { host_write(out, val) };
            (out as u64) << 32 | size as u64 / core::mem::size_of::<u8>() as u64
        }
        1 => {
            let res = unsafe { host_call(ptr + 1) };
            let ret = res + 1;
            let size = core::mem::size_of_val(&ret);
            let out = sbrk(size);
            if out.is_null() {
                return 0;
            }
            unsafe { host_write(out, &ret.to_le_bytes()) };
            (out as u64) << 32 | size as u64 / core::mem::size_of::<u32>() as u64
        }
        _ => 0,
    }
}
