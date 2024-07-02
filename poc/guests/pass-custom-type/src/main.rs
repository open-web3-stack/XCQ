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
    fn host_call_impl(arg_ptr: u32, out_ptr: u32);
}

#[derive(Clone, Copy)]
#[repr(C)]
struct GuestArgs {
    arg0: u32,
    arg1: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct GuestReturn {
    data0: u64,
    data1: u64,
}

fn host_call<Args: Copy, Return: Copy>(input: Args, out: &mut Return) {
    let arg_ptr = &input as *const Args;
    let out_ptr = out as *mut Return;
    // since args and return type should be ABI compatible with the host
    // there is no need to specify the size for args and returned value
    // note: assume the function is infallible for now
    unsafe { host_call_impl(arg_ptr as u32, out_ptr as u32) };
}
// A poc guest function that shows the following usage:
// 1. Host passes data that pre-allocated on the heap (via sbrk in host function) to the guest function
// 2. Guest calls host function, passing the address of args and return values on the stack
// 3. Guest returns the value which located on the guest's stack, and then have the host to read it.
// return value is u64 instead of (u32, u32) due to https://github.com/koute/polkavm/issues/116
// higher 32bits are address, lower 32bits are size
#[polkavm_derive::polkavm_export]
extern "C" fn main(ptr: u32, _size: u32) -> u64 {
    // ready first byte from ptr
    let byte = unsafe { core::ptr::read_volatile(ptr as *const u8) };
    match byte {
        0 => {
            let val = b"test";
            let size = core::mem::size_of_val(val);
            let val_ptr = val.as_ptr();
            (size as u64) << 32 | val_ptr as u64
        }
        1 => {
            let val = unsafe { core::ptr::read_volatile((ptr + 1) as *const u8) };
            let guest_args = GuestArgs {
                arg0: val as u32,
                arg1: 1,
            };
            let mut ret: GuestReturn = unsafe { core::mem::zeroed() };
            host_call(guest_args, &mut ret);
            let res = ret.data0 as u32 + 1;
            let ptr = polkavm_derive::sbrk(core::mem::size_of_val(&res));
            if ptr.is_null() {
                return 0;
            }
            unsafe { core::ptr::write_volatile(ptr as *mut u32, res) };
            let size = core::mem::size_of::<u32>();
            (size as u64) << 32 | ptr as u64
        }
        _ => 0,
    }
}
