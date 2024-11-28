use parity_scale_codec::{Codec, Decode, Encode};
use xcq_extension::metadata::Metadata;
use xcq_extension::{decl_extensions, impl_extensions, ExtensionsExecutor, Guest, Input, InvokeSource, Method};

mod extension_core {
    use super::*;
    pub trait Config {
        type ExtensionId: Codec;
    }
    decl_extensions! {
        pub trait ExtensionCore {
            type Config:Config;
            fn has_extension(id: <Self::Config as Config>::ExtensionId) -> bool;
            // crypto functions
            // fn blake2_64(data: Vec<u8>) -> [u8; 8];
            // fn blake2_128(data: Vec<u8>) -> [u8; 16];
            // fn blake2_256(data: Vec<u8>) -> [u8; 32];
            // fn twox_64(data: Vec<u8>) -> [u8; 8];
            // fn read_storage(key: Vec<u8>) -> Option<Vec<u8>>;
        }
    }
}

mod extension_fungibles {
    use super::*;
    pub trait Config {
        type AssetId: Codec;
        type AccountId: Codec;
        type Balance: Codec;
    }
    decl_extensions! {
        pub trait ExtensionFungibles {
            type Config:Config;
            fn total_supply(asset: <Self::Config as Config>::AssetId) -> <Self::Config as Config>::Balance;
            fn balance(asset: <Self::Config as Config>::AssetId, who: <Self::Config as Config>::AccountId) -> <Self::Config as Config>::Balance;
        }
    }
}

pub struct ExtensionImpl;

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
impl extension_core::Config for ExtensionImpl {
    type ExtensionId = u64;
}

impl extension_fungibles::Config for ExtensionImpl {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
}

// guest impls
pub struct GuestImpl {
    pub program: Vec<u8>,
}

impl Guest for GuestImpl {
    fn program(&self) -> &[u8] {
        &self.program
    }
}

pub struct InputImpl {
    pub method: Method,
    pub args: Vec<u8>,
}

impl Input for InputImpl {
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn args(&self) -> &[u8] {
        &self.args
    }
}

#[derive(Encode, Decode)]
enum CoreMethod {
    HasExtension { id: u64 },
}

#[derive(Encode, Decode)]
enum FungiblesMethod {
    TotalSupply { asset: u32 },
    Balance { asset: u32, who: [u8; 32] },
}
#[test]
fn call_core_works() {
    let blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let method = CoreMethod::HasExtension { id: 0 };
    let mut input_data = extension_core::EXTENSION_ID.encode();
    input_data.extend_from_slice(&method.encode());
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![1]);
}
#[test]
fn multi_calls_works() {
    let blob = include_bytes!("../../output/poc-guest-sum-balance-percent.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let mut input_data = extension_fungibles::EXTENSION_ID.encode();
    input_data.extend_from_slice(&[2u8]);
    let method1 = FungiblesMethod::Balance {
        asset: 1,
        who: [0u8; 32],
    };
    let method1_encoded = method1.encode();
    input_data.extend_from_slice(&[method1_encoded.len() as u8]);
    let method2 = FungiblesMethod::Balance {
        asset: 1,
        who: [1u8; 32],
    };
    input_data.extend_from_slice(&method1_encoded);
    input_data.extend_from_slice(&method2.encode());
    input_data.extend_from_slice(&extension_fungibles::EXTENSION_ID.encode());
    let method3 = FungiblesMethod::TotalSupply { asset: 1 };
    let method3_encoded = method3.encode();
    input_data.extend_from_slice(&[method3_encoded.len() as u8]);
    input_data.extend_from_slice(&method3_encoded);
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![100u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
}

#[test]
fn calls_vec_works() {
    let blob = include_bytes!("../../output/poc-guest-sum-balance.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let mut input_data = extension_fungibles::EXTENSION_ID.encode();
    input_data.extend_from_slice(&vec![2u8]);
    let method1 = FungiblesMethod::Balance {
        asset: 1,
        who: [0u8; 32],
    };
    let method1_encoded = method1.encode();
    input_data.extend_from_slice(&vec![method1_encoded.len() as u8]);
    let method2 = FungiblesMethod::Balance {
        asset: 2,
        who: [0u8; 32],
    };
    input_data.extend_from_slice(&method1_encoded);
    input_data.extend_from_slice(&method2.encode());
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![200u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
}

#[test]
fn single_call_works() {
    let blob = include_bytes!("../../output/poc-guest-total-supply.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let mut input_data = extension_fungibles::EXTENSION_ID.encode();
    let method1 = FungiblesMethod::TotalSupply { asset: 1 };
    let method1_encoded = method1.encode();
    input_data.extend_from_slice(&vec![method1_encoded.len() as u8]);
    input_data.extend_from_slice(&method1_encoded);
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![200u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
}

#[test]
fn metadata_works() {
    let metadata: Metadata = ExtensionImpl::metadata().into();
    let registry = metadata.types;
    let extension_metadata_list = metadata.extensions;
    // bool, u8, u32, u64, [u8;32]
    assert_eq!(registry.types.len(), 5);
    assert_eq!(extension_metadata_list.len(), 2);
    assert_eq!(extension_metadata_list[0].name, "ExtensionCore");
    assert_eq!(extension_metadata_list[1].name, "ExtensionFungibles");
}
