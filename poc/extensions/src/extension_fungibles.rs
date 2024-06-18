use crate::Vec;
use crate::{DispatchError, Dispatchable};
use crate::{ExtensionId, ExtensionIdTy};
use parity_scale_codec::{Decode, Encode};

pub type AccountIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AccountId;
pub type BalanceFor<T> = <<T as ExtensionFungibles>::Config as Config>::Balance;
pub type AssetIdFor<T> = <<T as ExtensionFungibles>::Config as Config>::AssetId;

pub trait ExtensionFungibles {
    type Config: Config;
    // fungibles::Inspect (not extensive)
    // fn total_inssuance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    // fn minimum_balance(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    fn total_supply(asset: AssetIdFor<Self>) -> BalanceFor<Self>;
    fn balance(asset: AssetIdFor<Self>, who: AccountIdFor<Self>) -> BalanceFor<Self>;
    // fungibles::InspectEnumerable
    // fn asset_ids() -> Vec<AccountIdFor<Self>>;
    // fn account_balances(who: AccountIdFor<Self>) -> Vec<(AssetIdFor<Self>, BalanceFor<Self>)>;
}

pub trait Config {
    type AccountId: Decode;
    type AssetId: Decode;
    type Balance: Encode;
}

// #[extension(ExtensionFungibles)]
// type Call;

mod generated_by_extension_decl {

    use super::*;

    #[derive(Decode)]
    pub enum ExtensionFungiblesCall<Impl: ExtensionFungibles> {
        // TODO: not extensive
        Balance {
            asset: AssetIdFor<Impl>,
            who: AccountIdFor<Impl>,
        },
        TotalSupply {
            asset: AssetIdFor<Impl>,
        },
    }

    impl<Impl: ExtensionFungibles> Dispatchable for ExtensionFungiblesCall<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Self::Balance { asset, who } => Ok(Impl::balance(asset, who).encode()),
                Self::TotalSupply { asset } => Ok(Impl::total_supply(asset).encode()),
            }
        }
    }

    impl<Impl: ExtensionFungibles> ExtensionId for ExtensionFungiblesCall<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 1u64;
    }

    // TODO: remove this when formalized
    #[allow(dead_code)]
    pub type Call<Impl> = ExtensionFungiblesCall<Impl>;
}

#[allow(unused_imports)]
pub use generated_by_extension_decl::*;
