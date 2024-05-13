# XCQ

Cross-Consensus Query Language for Polkadot

## Getting Started

### Prerequites

-   Install [Rust toolchain targeting RISC-V RV32E](https://github.com/paritytech/rustc-rv32e-toolchain)
-   Install [bun](https://bun.sh) (or npm or yarn) to run [Chopsticks](https://github.com/AcalaNetwork/chopsticks)
-   Install [jq](https://stedolan.github.io/jq/)

### Run PoC

1.  Install polkatool[^1](for relinking to .polkavm blob from a standard RV32E ELF) and chain-spec-builder[^2](for building chainspec from a wasm): `make tools`
2.  Build a PolkaVM guest program[^1]: `make poc-guest`
3.  Run a PoC and expected runtime structure:
    -   Run a simple host program which executes guest program (with trace turned on): `make poc-host`
    -   Run a runtime with `execute_query` api which executes guest program bytes via [chopsticks](https://github.com/AcalaNetwork/chopsticks): `make run`

## Explainations

### How guest program communicate with host?

Polkavm adopts a similar approach for guest accessing host functions to WASM.[^3]
In guest program, the host functions declarations are annotated with polkavm's proc-marco [`polkavm_import`](https://docs.rs/polkavm-derive/latest/polkavm_derive/attr.polkavm_import.html).
The definitions of guest functions are annotated with [`polkavm_export`](https://docs.rs/polkavm-derive/latest/polkavm_derive/attr.polkavm_export.html).
In host program, we register host functions through [`linker.func_wrap`](https://docs.rs/polkavm/latest/polkavm/struct.Linker.html#method.func_wrap)
Due to the limit of ABI, the signature of the those functions are limited to some primitive numeric types like `u32`, `i32`, `u64`(represented by two `u32` register).

### How to pass bytes from host to guest and vice versa?

In general, we can pass bytes between host and guest via guest's stack or heap. [^4][^5] The stack size of a guest program is 64KB, and the heap size is less than 4GB.

-   If we need some space on the stack, it's easy for guest to define local variables on stack, and then pass pointer to host to have the host write data to it. However, it's not trivial to let host write data directly on the guest's stack without the guest's "guidance" because data written to an improper address might be overwritten later.

-   If we need some space on the heap, Polkavm provides a dynamic allocation function both in host and guest through [`polkavm::Instance::sbrk`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.sbrk) and [`polkavm_derive::sbrk`](https://docs.rs/polkavm-derive/latest/polkavm_derive/fn.sbrk.html) respectively.

    According to the PolkaVM's doc[^6], memory allocated through `sbrk` can only be freed once the program finishes execution and its whole memory is cleared.

    Note: Including a global allocator in guest will cause the guest program bloats, which is unacceptable because we need keep the guest program as small as possible to store it on chain compactly.

Specific Usages in Details:

-   Pass arguements (at the entrypoint of the host function):
    Currently we only support passing argumensts via heap memory.
    Before calling guest function, host calls `sbrk` and [`polkavm::Instance::write_memory`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.write_memory) to allocate and write memory, then pass ptr as argument to guest via [`polkavm::Instance::call_typed`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.call_typed).

-   Return value from guest to host (at the end of the host function):
    In this case, it's viable to put the returned value on stack or heap. We recommend put the data on stack to prevent unnecessary memory allocation. The guest will return a `u64` which has the higher 32 bits as ptr and lower 32 bits as size due the limit of the ABI, and then have the host [`read_memory_into_vec`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.read_memory_into_vec) to get the result.

-   Call host function from guest, pass some data and get back some data (during the execution of the host function):
    We construct arguments and returned values on the stack, then we pass the address of them to host to have the host read, process input and write output to the given address.

### How to pass non-primitive data types between guest and host?

Basically, if a data type contains no objects on the heap, then byte-to-byte copy is enough, and both guest and host should have the same layout of the type to interpret data correctly.

## References

[PolkaVm](https://github.com/koute/polkavm) is a general purpose user-level RISC-V based virtual machine.

For more details, please refer to [PolkaVM Announcement on Polkadot Forum](https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more)

[^1]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#the-compilation-pipeline-7 "The compilation pipeline"
[^2]: https://github.com/paritytech/polkadot-sdk/tree/master/substrate/bin/utils/chain-spec-builder "chain-spec-builder"
[^3]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#wasm-like-import-export-model-6 "WASM-like import-export model"
[^4]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#security-and-sandboxing-4 "Security and sandboxing"
[^5]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#guest-program-memory-map-13 "Guest program memory map"
[^6]: https://docs.rs/polkavm-derive/latest/polkavm_derive/fn.sbrk.html "polkavm_derive::sbrk"
