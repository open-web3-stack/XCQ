#![cfg_attr(not(feature = "std"), no_std)]

mod extension_core {
    use parity_scale_codec::{Codec, Decode, Encode};
    use pvq_extension::{DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};

    pub trait ExtensionCore {
        type ExtensionId: Codec;
        fn has_extension(id: Self::ExtensionId) -> bool;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
    #[allow(non_camel_case_types)]
    pub enum ExtensionCoreFunctions<Impl: ExtensionCore> {
        has_extension(Impl::ExtensionId),
        __marker(core::marker::PhantomData<Impl>),
    }

    impl<Impl: ExtensionCore> Dispatchable for ExtensionCoreFunctions<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                ExtensionCoreFunctions::has_extension(id) => Ok(Impl::has_extension(id).encode()),
                ExtensionCoreFunctions::__marker(_) => Err(DispatchError::PhantomData),
            }
        }
    }

    impl<Impl: ExtensionCore> ExtensionId for ExtensionCoreFunctions<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 0u64;
    }
}

mod extension_fungibles {
    use parity_scale_codec::{Codec, Decode, Encode};
    use pvq_extension::{DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};

    pub trait ExtensionFungibles {
        type AssetId: Codec;
        type AccountId: Codec;
        type Balance: Codec;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
    #[allow(non_camel_case_types)]
    pub enum ExtensionFungiblesFunctions<Impl: ExtensionFungibles> {
        total_supply(Impl::AssetId),
        balance(Impl::AssetId, Impl::AccountId),
        __marker(core::marker::PhantomData<Impl>),
    }

    impl<Impl: ExtensionFungibles> Dispatchable for ExtensionFungiblesFunctions<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                ExtensionFungiblesFunctions::total_supply(asset) => Ok(Impl::total_supply(asset).encode()),
                ExtensionFungiblesFunctions::balance(asset, who) => Ok(Impl::balance(asset, who).encode()),
                ExtensionFungiblesFunctions::__marker(_) => Err(DispatchError::PhantomData),
            }
        }
    }

    impl<Impl: ExtensionFungibles> ExtensionId for ExtensionFungiblesFunctions<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 1u64;
    }
}

mod extensions_impl {
    pub struct Extensions;
    use super::*;

    impl extension_core::ExtensionCore for Extensions {
        type ExtensionId = u64;
        fn has_extension(id: u64) -> bool {
            matches!(id, 0 | 1)
        }
    }

    impl extension_fungibles::ExtensionFungibles for Extensions {
        type AssetId = u32;
        type AccountId = [u8; 32];
        type Balance = u64;
        fn total_supply(asset: u32) -> u64 {
            200
        }
        fn balance(asset: u32, who: [u8; 32]) -> u64 {
            100
        }
    }
}

mod tests {
    use super::*;
    use parity_scale_codec::Encode;
    use pvq_extension::{ExtensionsExecutor, InvokeSource};
    use tracing_subscriber::prelude::*;

    type Extensions = (
        extension_core::ExtensionCoreFunctions<extensions_impl::Extensions>,
        extension_fungibles::ExtensionFungiblesFunctions<extensions_impl::Extensions>,
    );

    #[test]
    fn test_runtime_executor() {
        let registry = tracing_subscriber::registry();

        let filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing::Level::DEBUG.into())
            .from_env_lossy();

        registry
            .with(tracing_subscriber::fmt::layer().with_filter(filter))
            .try_init()
            .expect("Failed to initialize tracing");

        let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::Runtime);
        let program_blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm").to_vec();
        let mut args = vec![];
        args.extend(1u64.to_le_bytes());
        extension_fungibles::ExtensionFungiblesFunctions::<extensions_impl::Extensions>::total_supply(1u32)
            .encode_to(&mut args);
        let res = executor.execute_method(&program_blob, &args, 0);
        println!("res: {:?}", res);
        assert_eq!(res, Ok(200u64.to_le_bytes().to_vec()));
    }
}
