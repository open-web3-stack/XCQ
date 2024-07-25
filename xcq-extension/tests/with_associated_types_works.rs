use parity_scale_codec::{Decode, Encode};
use xcq_extension::{
    decl_extensions, impl_extensions, ExtensionId, ExtensionsExecutor, Guest, Input, InvokeSource, Method,
};

mod extension_core {
    use super::*;
    pub trait Config {
        type ExtensionId: Encode + Decode;
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
        type AssetId: Encode + Decode;
        type AccountId: Encode + Decode;
        type Balance: Encode + Decode;
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
            100
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
    type Balance = u32;
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
fn call_fungibles_works() {
    let blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let method = FungiblesMethod::TotalSupply { asset: 1u32 };
    let mut input_data = extension_fungibles::EXTENSION_ID.encode();
    input_data.extend_from_slice(&method.encode());
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![100u8, 0u8, 0u8, 0u8]);
}
