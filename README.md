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
3.  Two options:
    -   Run a simple host program which executes guest program (with trace turned on): `make poc-host`
    -   Run a runtime with `execute_query` api which executes guest program bytes via [chopsticks](https://github.com/AcalaNetwork/chopsticks): `make run`

## Explainations

-   How guest program communicate with host?

    Accessing host functions is similar to what you'do for WASM, annotate with polkavm's proc-marco `polkavm_import`[^3]. Similarly, the exports is annotated with `polkavm_export`. You can have a look at [guest program example-hello-world in polkavm official repo](https://github.com/koute/polkavm/tree/master/guest-programs/example-hello-world/src/main.rs).

-   How to pass bytes from host to guest and vice versa?

    -   Pass bytes from host to guest:
        Host calls [`polkavm::Instance::sbrk`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.sbrk) and [`polkavm::Instance::write_memory`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.write_memory) to allocate and write memory in guest, then returned ptr is passed as an argument to guest functions via [`polkavm::Instance::call_typed`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.call_typed)

    -   Pass Bytes from guest to host:
        First guest should set a global allocator like [`polkavm_derive::LeakingAllocator`](https://docs.rs/polkavm-derive/latest/polkavm_derive/struct.LeakingAllocator.html), then use `alloc::box::Box` to create heap vals like normal, finally leak heap ptr as return value. Since currently `AbiTy` only support limited types like single integer value, we can construct a u64 with higher 32bits as ptr and lower 32bits as length. In host, we use [`polkavm::Instance::read_memory_into_vec`](https://docs.rs/polkavm/latest/polkavm/struct.Instance.html#method.read_memory_into_vec) to get result.

-   (TODO) How to pass non-primitive data types between guest and host?

## References

[PolkaVm](https://github.com/koute/polkavm) is a general purpose user-level RISC-V based virtual machine.

For more details, please refer to [PolkaVM Announcement on Polkadot Forum](https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more)

[^1]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#the-compilation-pipeline-7 "The compilation pipeline"
[^2]: https://github.com/paritytech/polkadot-sdk/tree/master/substrate/bin/utils/chain-spec-builder "chain-spec-builder"
[^3]: https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more/3811#wasm-like-import-export-model-6 "WASM-like import-export model"
