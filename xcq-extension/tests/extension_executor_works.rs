use parity_scale_codec::{Decode, Encode};
use xcq_extension::{extension, ExtensionsExecutor, Guest, Input, InvokeSource, Method};

mod extension_core {
    use super::*;
    #[extension]
    pub trait ExtensionCore {
        type Config: Config;
        fn has_extension(id: <Self::Config as Config>::ExtensionId) -> bool;
        // crypto functions
        // fn blake2_64(data: Vec<u8>) -> [u8; 8];
        // fn blake2_128(data: Vec<u8>) -> [u8; 16];
        // fn blake2_256(data: Vec<u8>) -> [u8; 32];
        // fn twox_64(data: Vec<u8>) -> [u8; 8];
        // fn read_storage(key: Vec<u8>) -> Option<Vec<u8>>;
    }
    pub trait Config {
        type ExtensionId: Decode;
    }
}

mod extension_fungibles {
    use super::*;
    pub type AccountIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AccountId;
    pub type BalanceFor<T> = <<T as ExtensionFungibles>::Config as Config>::Balance;
    pub type AssetIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AssetId;
    #[extension]
    pub trait ExtensionFungibles {
        type Config: Config;
        fn total_supply(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
        fn balance(asset: AssetIdFor<Self>, who: AccountIdFor<Self>) -> BalanceFor<Self>;
    }
    pub trait Config {
        type AccountId: Decode;
        type AssetId: Decode;
        type Balance: Encode;
    }
}

// extension_core impls
pub struct ExtensionCoreImpl;

pub struct ExtensionCoreConfigImpl;
impl extension_core::Config for ExtensionCoreConfigImpl {
    type ExtensionId = u64;
}

impl extension_core::ExtensionCore for ExtensionCoreImpl {
    type Config = ExtensionCoreConfigImpl;
    fn has_extension(id: <Self::Config as extension_core::Config>::ExtensionId) -> bool {
        matches!(id, 0 | 1)
    }
}

// extension_fungibles impls
pub struct ExtensionFungiblesImpl;
pub struct ExtensionFungiblesConfigImpl;

impl extension_fungibles::Config for ExtensionFungiblesConfigImpl {
    type AccountId = [u8; 32];
    type Balance = u32;
    type AssetId = u64;
}

impl extension_fungibles::ExtensionFungibles for ExtensionFungiblesImpl {
    type Config = ExtensionFungiblesConfigImpl;
    fn balance(
        _asset: extension_fungibles::AssetIdFor<Self>,
        _who: extension_fungibles::AccountIdFor<Self>,
    ) -> extension_fungibles::BalanceFor<Self> {
        0
    }
    fn total_supply(_asset: extension_fungibles::AssetIdFor<Self>) -> extension_fungibles::BalanceFor<Self> {
        100
    }
}

type Extensions = (
    extension_core::Call<ExtensionCoreImpl>,
    extension_fungibles::Call<ExtensionFungiblesImpl>,
);

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
    TotalSupply { asset: u64 },
    Balance { asset: u64, who: [u8; 32] },
}
#[test]
fn call_core_works() {
    let blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm");
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let guest = GuestImpl { program: blob.to_vec() };
    let method = CoreMethod::HasExtension { id: 0 };
    let mut input_data = 0u64.encode();
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
    let method = FungiblesMethod::TotalSupply { asset: 1u64 };
    let mut input_data = 1u64.encode();
    input_data.extend_from_slice(&method.encode());
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();
    assert_eq!(res, vec![100u8, 0u8, 0u8, 0u8]);
}
