# XCQ

Cross-Consensus Query Language for Polkadot

## Getting Started

### Prerequites

-   Install Rust toolchain targeting RISC-V RV32E: https://github.com/paritytech/rustc-rv32e-toolchain
-   Install bun (or npm or yarn) to run Chopsticks: https://bun.sh
-   Install jq: https://stedolan.github.io/jq/

### Run PoC

1.  Install polkatool(for relinking to .polkavm blob from a standard RV32E ELF) and chain-spec-builder(for building chainspec from a wasm): `make tools`
2.  Build a PolkaVM guest program: `make poc-guest`
3.  Two options:
    -   Run a simple host program which executes guest program (with trace turned on): `make poc-host`
    -   Run a runtime with `execute_query` api which executes guest program bytes via [chopsticks](https://github.com/AcalaNetwork/chopsticks): `make run`

## Explainations

-   (TODO)How guest program communicate with host?
-   (TODO)How to pass bytes from host to guest and vice versa?
-   (TODO)How to pass non-primitive data types between guest and host?

## References

[PolkaVm](https://github.com/koute/polkavm) is a general purpose user-level RISC-V based virtual machine.

For more details, please refer to [PolkaVM Discussion on Polkadot Forum](https://forum.polkadot.network/t/announcing-polkavm-a-new-risc-v-based-vm-for-smart-contracts-and-possibly-more)
