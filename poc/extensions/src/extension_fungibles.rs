use crate::{DispatchError, Dispatchable};
use crate::{ExtensionId, ExtensionIdTy};
use core::marker::PhantomData;
use parity_scale_codec::{Decode, Encode};

pub trait ExtensionFungibles {
    fn free_balance_of(who: [u8; 32]) -> u32;
    fn reserved_balance_of(who: [u8; 32]) -> u32;
}

// #[extension(ExtensionFungibles)]
// type Call;

mod generated_by_extension_decl {

    use super::*;
    #[derive(Decode)]
    pub enum ExtensionFungiblesCall<Impl: ExtensionFungibles> {
        FreeBalanceOf { who: [u8; 32] },
        ReservedBalanceOf { who: [u8; 32] },
        _Marker(PhantomData<Impl>),
    }

    impl<Impl: ExtensionFungibles> Dispatchable for ExtensionFungiblesCall<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Self::FreeBalanceOf { who } => Ok(Impl::free_balance_of(who).encode()),
                Self::ReservedBalanceOf { who } => Ok(Impl::reserved_balance_of(who).encode()),
                Self::_Marker(_) => Err(DispatchError::PhantomData),
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
