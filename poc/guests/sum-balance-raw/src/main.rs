#![no_std]
#![no_main]
#[global_allocator]
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
use alloc::vec::Vec;
extern crate alloc;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp", options(noreturn));
    }
}
#[polkavm_derive::polkavm_import]
extern "C" {
    fn host_call(extension_id: u64, call_ptr: u32, size: u32) -> u64;
}

struct BalanceCall {
    pub extension_id: u64,
    pub call_ptr: u32,
    pub size: u32,
}
impl BalanceCall {
    pub fn call(&self) -> u64 {
        let res = unsafe { host_call(self.extension_id, self.call_ptr, self.size) };
        let res_len = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as *const u8;
        let res_bytes = unsafe { core::slice::from_raw_parts(res_ptr, res_len as usize) };
        let (int_bytes, _) = res_bytes.split_at(core::mem::size_of::<u64>());
        u64::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
struct TotalSupplyCall {
    pub extension_id: u64,
    pub call_ptr: u32,
    pub size: u32,
}
impl TotalSupplyCall {
    pub fn call(&self) -> u64 {
        let res = unsafe { host_call(self.extension_id, self.call_ptr, self.size) };
        let res_len = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as *const u8;
        let res_bytes = unsafe { core::slice::from_raw_parts(res_ptr, res_len as usize) };
        let (int_bytes, _) = res_bytes.split_at(core::mem::size_of::<u64>());
        u64::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, size: u32) -> u64 {
    let mut calls_0: alloc::vec::Vec<BalanceCall> = alloc::vec::Vec::new();
    let extension_id = unsafe { core::ptr::read_volatile((ptr) as *const u64) };
    let num = unsafe { core::ptr::read_volatile((ptr + 8) as *const u8) };
    let size = unsafe { core::ptr::read_volatile((ptr + 9) as *const u8) };
    let mut res = 0u64;
    unsafe {
        host_call(extension_id, ptr + 10, size as u32);
        host_call(extension_id, ptr + 10 + (size as u32), size as u32);
    }
    for i in 0..num {
        calls_0.push(BalanceCall {
            extension_id: extension_id,
            call_ptr: ptr + 10 + (i as u32) * (size as u32),
            size: size as u32,
        });
    }
    let mut res = 0u64;
    for call in calls_0 {
        res += call.call();
    }
    let res_bytes = res.to_le_bytes();
    let ptr = polkavm_derive::sbrk(res_bytes.len());
    if ptr.is_null() {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(res_bytes.as_ptr(), ptr, res_bytes.len());
    }
    (res_bytes.len() as u64) << 32 | (ptr as u64)
}
