#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
static GLOBAL: polkavm_derive::LeakingAllocator = polkavm_derive::LeakingAllocator;
const _: () = {
    #[rustc_std_internal_symbol]
    unsafe fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc(
            &GLOBAL,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) -> () {
        ::core::alloc::GlobalAlloc::dealloc(
            &GLOBAL,
            ptr,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_realloc(
        ptr: *mut u8,
        size: usize,
        align: usize,
        new_size: usize,
    ) -> *mut u8 {
        ::core::alloc::GlobalAlloc::realloc(
            &GLOBAL,
            ptr,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
            new_size,
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc_zeroed(
            &GLOBAL,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
};
use alloc::vec::Vec;
extern crate alloc;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        asm!("unimp", options(noreturn));
    }
}
#[cfg(all(any(target_arch = "riscv32", target_arch = "riscv64"), target_feature = "e"))]
#[link_section = ".text.polkavm_import"]
unsafe fn host_call(extension_id: u64, call_ptr: u32, call_len: u32) -> u64 {
    const _: () = {
        if !(<<(
            <(
                <(
                    (),
                    <u64 as ::polkavm_derive::default_abi::IntoHost>::Regs,
                ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out,
                <u32 as ::polkavm_derive::default_abi::IntoHost>::Regs,
            ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out,
            <u32 as ::polkavm_derive::default_abi::IntoHost>::Regs,
        ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out as ::polkavm_derive::default_abi::private::CountTuple>::COUNT
            <= ::polkavm_derive::default_abi::private::MAXIMUM_INPUT_REGS)
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "too many registers required by the arguments to the imported function \'host_call\'",
                    ),
                );
            }
        }
    };
    let (extension_id, _destructor) = ::polkavm_derive::default_abi::IntoHost::into_host(
        extension_id,
    );
    let (call_ptr, _destructor) = ::polkavm_derive::default_abi::IntoHost::into_host(
        call_ptr,
    );
    let (call_len, _destructor) = ::polkavm_derive::default_abi::IntoHost::into_host(
        call_len,
    );
    let regs = ();
    let regs = ::polkavm_derive::default_abi::private::JoinTuple::join_tuple((
        regs,
        extension_id,
    ));
    let regs = ::polkavm_derive::default_abi::private::JoinTuple::join_tuple((
        regs,
        call_ptr,
    ));
    let regs = ::polkavm_derive::default_abi::private::JoinTuple::join_tuple((
        regs,
        call_len,
    ));
    #[link_section = ".polkavm_metadata"]
    static METADATA_SYMBOL: &[u8] = b"host_call";
    #[link_section = ".polkavm_metadata"]
    static METADATA: ::polkavm_derive::default_abi::private::ExternMetadata = ::polkavm_derive::default_abi::private::ExternMetadata {
        version: 1,
        flags: 0,
        symbol_length: METADATA_SYMBOL.len() as u32,
        symbol: ::polkavm_derive::default_abi::private::MetadataPointer(
            METADATA_SYMBOL.as_ptr(),
        ),
        input_regs: <<(
            <(
                <(
                    (),
                    <u64 as ::polkavm_derive::default_abi::IntoHost>::Regs,
                ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out,
                <u32 as ::polkavm_derive::default_abi::IntoHost>::Regs,
            ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out,
            <u32 as ::polkavm_derive::default_abi::IntoHost>::Regs,
        ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out as ::polkavm_derive::default_abi::private::CountTuple>::COUNT,
        output_regs: <<u64 as ::polkavm_derive::default_abi::FromHost>::Regs as ::polkavm_derive::default_abi::private::CountTuple>::COUNT,
    };
    struct Sym;
    #[cfg(target_arch = "riscv32")]
    impl ::polkavm_derive::default_abi::private::ImportSymbol for Sym {
        extern fn trampoline(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
            unsafe {
                asm!(
                    ".insn r 0xb, 0, 0, zero, zero, zero\n\n.4byte {6}\n\nret\n", in
                    ("a0") a0, in ("a1") a1, in ("a2") a2, in ("a3") a3, in ("a4") a4, in
                    ("a5") a5, sym METADATA, options(noreturn)
                );
            }
        }
    }
    let result = ::polkavm_derive::default_abi::private::CallImport::call_import::<
        Sym,
    >(regs);
    let result = ::polkavm_derive::default_abi::private::IntoTuple::into_tuple(
        result.0,
        result.1,
    );
    ::polkavm_derive::default_abi::FromHost::from_host(result)
}
fn sum_balance(calls: Vec<BalanceCall>) -> u64 {
    let mut sum = 0;
    for call in calls {
        sum += call.call();
    }
    sum
}
struct BalanceCall {
    pub extension_id: u64,
    pub call_ptr: u32,
    pub call_size: u32,
}
impl BalanceCall {
    pub fn call(&self) -> u64 {
        let res = unsafe { host_call(self.extension_id, self.call_ptr, self.call_size) };
        let res_len = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as *const u8;
        let res_bytes = unsafe {
            core::slice::from_raw_parts(res_ptr, res_len as usize)
        };
        let (int_bytes, _) = res_bytes.split_at(core::mem::size_of::<u64>());
        u64::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
struct TotalSupplyCall {
    pub extension_id: u64,
    pub call_ptr: u32,
    pub call_size: u32,
}
impl TotalSupplyCall {
    pub fn call(&self) -> u64 {
        let res = unsafe { host_call(self.extension_id, self.call_ptr, self.call_size) };
        let res_len = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as *const u8;
        let res_bytes = unsafe {
            core::slice::from_raw_parts(res_ptr, res_len as usize)
        };
        let (int_bytes, _) = res_bytes.split_at(core::mem::size_of::<u64>());
        u64::from_le_bytes(int_bytes.try_into().unwrap())
    }
}
fn main(ptr: u32, size: u32) -> u64 {
    #[cfg(
        all(any(target_arch = "riscv32", target_arch = "riscv64"), target_feature = "e")
    )]
    #[doc(hidden)]
    mod __polkavm_export {
        use ::polkavm_derive::default_abi::private::Reg;
        #[link_section = ".text.polkavm_export.main"]
        extern fn trampoline(
            a0: Reg,
            a1: Reg,
            a2: Reg,
            a3: Reg,
            a4: Reg,
            a5: Reg,
        ) -> ::polkavm_derive::default_abi::private::ReturnTy {
            let result = {
                let regs = (a0, a1, a2, a3, a4, a5);
                let (a, regs) = ::polkavm_derive::default_abi::private::SplitTuple::<
                    <u32 as ::polkavm_derive::default_abi::FromHost>::Regs,
                >::split_tuple(regs);
                let (b, regs) = ::polkavm_derive::default_abi::private::SplitTuple::<
                    <u32 as ::polkavm_derive::default_abi::FromHost>::Regs,
                >::split_tuple(regs);
                let _ = regs;
                let a = ::polkavm_derive::default_abi::FromHost::from_host(a);
                let b = ::polkavm_derive::default_abi::FromHost::from_host(b);
                let result = { super::main(a, b) };
                let (result, destructor) = ::polkavm_derive::default_abi::IntoHost::into_host(
                    result,
                );
                #[allow(forgetting_copy_types)] core::mem::forget(destructor);
                result
            };
            ::polkavm_derive::default_abi::private::PackReturnTy::pack_return_ty(result)
        }
        #[link_section = ".polkavm_metadata"]
        static METADATA_SYMBOL: &str = "main";
        #[link_section = ".polkavm_metadata"]
        static METADATA: ::polkavm_derive::default_abi::private::ExternMetadata = ::polkavm_derive::default_abi::private::ExternMetadata {
            version: 1,
            flags: 0,
            symbol_length: METADATA_SYMBOL.len() as u32,
            symbol: ::polkavm_derive::default_abi::private::MetadataPointer(
                METADATA_SYMBOL.as_ptr(),
            ),
            input_regs: <<(
                <(
                    (),
                    <u32 as ::polkavm_derive::default_abi::FromHost>::Regs,
                ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out,
                <u32 as ::polkavm_derive::default_abi::FromHost>::Regs,
            ) as ::polkavm_derive::default_abi::private::JoinTuple>::Out as ::polkavm_derive::default_abi::private::CountTuple>::COUNT,
            output_regs: <<u64 as ::polkavm_derive::default_abi::IntoHost>::Regs as ::polkavm_derive::default_abi::private::CountTuple>::COUNT,
        };
    }
    let arg_bytes = unsafe {
        core::slice::from_raw_parts(ptr as *const u8, size as usize)
    };
    let arg_bytes = arg_bytes.to_vec();
    let mut arg_ptr = arg_bytes.as_ptr() as u32;
    let mut calls_0: alloc::vec::Vec<BalanceCall> = alloc::vec::Vec::new();
    let extension_id = unsafe { core::ptr::read_volatile((arg_ptr) as *const u64) };
    let call_num = unsafe { core::ptr::read_volatile((arg_ptr + 8) as *const u8) };
    let call_size = unsafe { core::ptr::read_volatile((arg_ptr + 9) as *const u8) };
    for i in 0..call_num {
        calls_0
            .push(BalanceCall {
                extension_id: extension_id,
                call_ptr: arg_ptr + 10 + (i as u32) * (size as u32),
                call_size: call_size as u32,
            });
    }
    arg_ptr += 10 + (call_num as u32) * (call_size as u32);
    let res = sum_balance(calls_0);
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
