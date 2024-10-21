# RFC-0000: XCQ(Cross Consensus Query)

|                 |                                                                                             |
| --------------- | ------------------------------------------------------------------------------------------- |
| **Start Date**  | Date of initial proposal                                                                    |
| **Description** | Introduce XCQ (a cross consensus query system)                                                          |
| **Authors**     ||

## Summary

## Motivation

> Longer motivation behind the content of the RFC, presented as a combination of both problems and requirements for the solution.

XCM enables mutable interactions across different consensus systems.
But we still need another subsystem to query information across different consensus systems, which abstracts away the concrete implementations in these systems.

Such a subsystem will benefit the tools and UI developers.
For example, for a substrate-based chain, an account can have different value-bearing assets in different pallets (i.e. balances pallet, DEX pallet, liquid staking pallet). Conventionally, wallet developers have no easy way but examine all the pallets to scrape all the information they need. Some operations require reading storage directly, which is also subject to breaking changes.

The proposal of XCQ will serve as a layer between specific chain implementations and tools/UI developers.

There are some use cases collected:

- Improve XCM bridge UI
  - Use XCQ to query asset balances
  - Query XCM weight and fee from hop and dest chain
- Wallet
  - Use XCQ to query asset balances
  - Use XCQ to query weights and fees for related operations from different chains
- Universal dApp that supports all the parachains
  - Use XCQ to perform feature discovery
  - Use XCQ to query pallet specific feature
  - Use XCQ to help construct extrinsic by querying pallet index, call index, etc

## Stakeholders

> A brief catalogue of the primary stakeholder sets of this RFC, with some description of previous socialization of the proposal

- Runtime Developers
- Wallet/dApps Developers

## Explanation

> Detail-heavy explanation of the RFC, suitable for explanation to an implementer of the changeset. This should address corner cases in detail and provide justification behind decisions, and provide rationale for how the design meets the solution requirements.
is overall query pattern of the XCQ is three folds:

- Runtime: view-functions across different pallets are amalgamated through an extension-based system
- XCQ query: custom computations over the view-function results can be encapsulated via compiling them as a PolkaVM program.
- XCQ query arguments: input variables like accounts to be queried can be specified as the query arguments

We design the following components to support the aforementioned pattern.

### XCQ runtime API

We define the runtime api for off-chain query usage, like:

The runtime api `XcqApi` includes two methods:

- `execute_query`: execute the query and return result
- `metadata`: return the metadata of the supported extensions of the chain, which serves as the feature discovery functionality

```rust
decl_runtime_apis! {
    pub trait XcqApi {
        // query: The query program as PolkaVM binary
        // input: The arguments to be fed into the query program
        fn execute_query(query: Vec<u8>, input: Vec<u8>) -> XcqResult;
        // SCALE-encoded metadata of all supported xcq extensions.
        fn metadata() -> Vec<u8>;
    }
}
type XcqResult =  Result<XcqResponse, XcqError>;
type XcqResponse = Vec<u8>;
enum XcqError {
    Custom(String),
}
```

**Example metadata**:

```rust
Metadata {
    extensions: vec![
        ExtensionMetadata {
            name: "ExtensionCore",
            methods: vec![MethodMetadata {
                name: "has_extension",
                inputs: vec![MethodParamMetadata {
                    name: "id",
                    ty: XcqType::Primitive(PrimitiveType::U64)
                }],
                output: XcqType::Primitive(PrimitiveType::Bool)
            }]
        },
        ExtensionMetadata {
            name: "ExtensionFungibles",
            methods: vec![
                MethodMetadata {
                    name: "total_supply",
                    inputs: vec![MethodParamMetadata {
                        name: "asset",
                        ty: XcqType::Primitive(PrimitiveType::U32)
                    }],
                    output: XcqType::Primitive(PrimitiveType::U64)
                },
                MethodMetadata {
                    name: "balance",
                    inputs: vec![
                        MethodParamMetadata {
                            name: "asset",
                            ty: XcqType::Primitive(PrimitiveType::U32)
                        },
                        MethodParamMetadata {
                            name: "who",
                            ty: XcqType::Primitive(PrimitiveType::H256)
                        }
                    ],
                    output: XcqType::Primitive(PrimitiveType::U64)
                }
            ]
        }
    ]
}
```

Note: `ty` is represented by a meta-type system called `xcq-types`

#### xcq-types

`xcq-types` is a meta-type system similar to `scale-info` but much simpler. A meta-type system is required to make different chains with different type definitions work via a common operation. The front-end codes will know how to construct call data to XCQ programs according to the metadata provided by different chains.

**Limitations**

- No generics support yet
- No type registry to compress type info and represent self-referential types

### XCQ Extension

*Since the XCQ usage may vary between the different use cases, we adopt an extension-based design, which has the following features*:

- More extensible and flexible
- New functionalities can be added in a permission-less manner without upgrading the core part of the XCQ.
- Ensure the core part is in a minimal scope.

We don't adopt a versioned design. Instead, every extension is identified via an extension id, which is a hash based on the extension name and method sets. An update of an extension is viewed as a brand-new extension.

We provide the following components to facilitate the development, including some macros to declare and implement extensions as well as useful structs connecting the executor.

- `decl_extensions` macro: defines an extension as a Rust trait with optional associated types.

Example:

```rust
use xcq_extension::decl_extensions;

pub trait Config {
    type AssetId: Codec;
    type AccountId: Codec;
    type Balance: Codec;
}
decl_extensions! {
    pub trait ExtensionFungibles {
        type Config: Config;
        fn total_supply(asset: <Self::Config as Config>::AssetId) -> <Self::Config as Config>::Balance;
        fn balance(asset: <Self::Config as Config>::AssetId, who: <Self::Config as Config>::AccountId) -> <Self::Config as Config>::Balance;
    }
}
```

- `impl_extensions` macro: generates extension implementations and extension-level metadata.

**Example**:

```rust
// ExtensionImpl is an aggregate struct to impl different extensions
impl extension_fungibles::Config for ExtensionImpl {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
}
impl_extensions! {
    impl extension_core::ExtensionCore for ExtensionImpl {
        type Config = ExtensionImpl;
        fn has_extension(id: <Self::Config as extension_core::Config>::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    impl extension_fungibles::ExtensionFungibles for ExtensionImpl {
        type Config = ExtensionImpl;
        #[allow(unused_variables)]
        fn total_supply(asset: <Self::Config as extension_fungibles::Config>::AssetId) -> <Self::Config as extension_fungibles::Config>::Balance {
            200
        }
        #[allow(unused_variables)]
        fn balance(asset: <Self::Config as extension_fungibles::Config>::AssetId, who: <Self::Config as extension_fungibles::Config>::AccountId) -> <Self::Config as extension_fungibles::Config>::Balance {
            100
        }
    }
}
```

- `ExtensionExecutor`: connects extension implementations and xcq-executor. Host functions are aggregated under a unified `host_call` entry. Guest call requests are dispatched to corresponding extensions.
- `PermController`: filters guest XCQ program calling requests. That is useful for host chains to disable some queries by filtering invoking sources.

### XCQ Program API

Since the PolkaVM program API only supports several numeric types when crossing the guest/host boundaries, we need to pass pointers to support custom data types. However, pointer operations like moving and reading the correct size of bytes are error-prone. Therefore, we provide some macros to allow the developers to define their custom data types.
Additionally, we also cover some boilerplates in the macro such as defining panic handlers and checking the call definition in the query program matches the definition of the extensions exposed.

**Example**:
The following XCQ program sums up the balances of several accounts and calculates the percent of the total supply that the sum accounts for.

```rust
#[xcq_api::program]
mod sum_balance {
    #[xcq::call_def(extension_id = 0x92F353DB95824F9Du64, call_index = 1)]
    fn balance(asset: u32, who: [u8; 32]) -> u64 {}
    #[xcq::call_def(extension_id = 0x92F353DB95824F9Du64, call_index = 0)]
    fn total_supply(asset: u32) -> u64 {}

    #[xcq::entrypoint]
    fn sum_balance(balances: Vec<BalanceCall>, total_supply: TotalSupplyCall) -> u64 {
        let mut sum_balance = 0;
        for call in balances {
            sum_balance += call.call();
        }
        sum_balance * 100 / total_supply.call()
    }
}

```

Every program is declared in a separate Rust mod.

- `[xcq::call_def]` declares a single call to be used in the query. `extension_id` and `call_index` should be specified.
- `[xcq::entrypoint]` declares an entrypoint function that executes the main logic. Every call type is named like `{CallDefMethodName}Call`. Multiple calls of the same type can be specified as `Vec<SomeCall>`

## Drawbacks

> Description of recognized drawbacks to the approach given in the RFC. Non-exhaustively, drawbacks relating to performance, ergonomics, user experience, security, or privacy.

### Performance issues

- XCQ Query Program Size: For on-chain usage, we need to store the pre-built query program on-chain or send it via XCMP/HRMP. However, the program size of a simple PoC to query and sum the balances is about 7KB, which is far from an ideal size. It's mainly because we include a global allocator in the program to support the usage of dynamic-size array like `Vec`. In future, the program size can be shrinked by splitting the raw PolkaVM program and we can just store/send the parts that are the main logic of the query, leaving other parts being hard-coded or prepared in advance.

### User experience issues

- Debugging: Currently, there is no fully fledged debuggers for PolkaVM programs. The only debugging approach is to set the PolkaVM backend in interpreter mode and then log the operations at the assembly level, which is too low-level to debug efficiently.
- Gas computation: According to [this issue](https://github.com/koute/polkavm/issues/17), the gas cost model of PolkaVM is not accurate for now.

## Testing, Security, and Privacy

> Describe the the impact of the proposal on these three high-importance areas - how implementations can be tested for adherence, effects that the proposal has on security and privacy per-se, as well as any possible implementation pitfalls which should be clearly avoided.

- Testing: Some representative use cases collected from the community will be validated to ensure the basic functionalities of the proposal.
- Security:
  - The query operation should be read-only. However we don't have a mechanism to ensure it, which depends on the correct implementation by parachain developers.
- Privacy:
  *Not sure yet*

## Performance, Ergonomics, and Compatibility

> Describe the impact of the proposal on the exposed functionality of Polkadot.

### Performance

> Is this an optimization or a necessary pessimization? What steps have been taken to minimize additional overhead?
It's a new functionality, which doesn't modify the existing implementations.

### Ergonomics

> If the proposal alters exposed interfaces to developers or end-users, which types of usage patterns have been optimized for?
The proposal facilitate the wallets and dApps developers. Not only They don't need to examine every concrete implementations for supporting conceptually the same operations in different chains, but also has a more modular development experience by encapsulating custom computations over the exposed apis in PolkaVM programs.

### Compatibility

> Does this proposal break compatibility with existing interfaces, older versions of implementations? Summarize necessary migrations or upgrade strategies, if any.
The proposal defines new apis, which doesn't break compatibility with existing interfaces.

## Prior Art and References

> Provide references to either prior art or other relevant research for the submitted design.
There are several discussions related to the proposal, including:

- <https://forum.polkadot.network/t/wasm-view-functions/1045> is the original discussion about having a mechanism to avoid code duplications between the runtime and front-ends/wallets. In the original design, the custom computations are compiled as a wasm function.
- <https://forum.polkadot.network/t/wasm-view-functions/1045> is the issue tracking the view functions implementation in runtime implementations
- <https://github.com/paritytech/polkadot-sdk/pull/4722> is the on-going `view function` pull request. It works at pallet level. If two chains use two different pallets to provide similar functionalities, like pallet-assets and pallet-erc20, we still need to have different codes to support. Therefore, it doesn't conflict with XCQ, and can be utilized by XCQ.

## Unresolved Questions

> Provide specific questions to discuss and address before the RFC is voted on by the Fellowship. This should include, for example, alternatives to aspects of the proposed design where the appropriate trade-off to make is unclear.

- *Subscribility*

## Future Directions and Related Material

> Describe future work which could be enabled by this RFC, if it were accepted, as well as related RFCs. This is a place to brain-dump and explore possibilities, which themselves may become their own RFCs.

Since XCQ are supported both in off-chain and on-chain, we have a related XCM-Format.
