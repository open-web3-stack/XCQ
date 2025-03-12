#![cfg_attr(not(feature = "std"), no_std)]

mod extension_core {
    use parity_scale_codec::{Codec, Decode, Encode};
    use pvq_extension::{DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};

    pub trait ExtensionCore {
        type ExtensionId: Codec + scale_info::TypeInfo + 'static;
        fn has_extension(id: Self::ExtensionId) -> bool;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
    #[allow(non_camel_case_types)]
    pub enum Functions<Impl: ExtensionCore> {
        has_extension {
            id: Impl::ExtensionId,
        },
        #[doc(hidden)]
        __marker(core::marker::PhantomData<Impl>),
    }

    impl<Impl: ExtensionCore> Dispatchable for Functions<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Functions::has_extension { id } => Ok(Impl::has_extension(id).encode()),
                Functions::__marker(_) => Err(DispatchError::PhantomData),
            }
        }
    }

    impl<Impl: ExtensionCore> ExtensionId for Functions<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 0u64;
    }

    pub fn metadata<Impl: ExtensionCore>() -> pvq_extension::metadata::ExtensionMetadata {
        pvq_extension::metadata::ExtensionMetadata {
            name: "ExtensionCore",
            functions: vec![pvq_extension::metadata::FunctionMetadata {
                name: "has_extension",
                inputs: vec![pvq_extension::metadata::FunctionParamMetadata {
                    name: "id",
                    ty: scale_info::meta_type::<Impl::ExtensionId>(),
                }],
                output: scale_info::meta_type::<bool>(),
            }],
        }
    }
}

mod extension_fungibles {
    use parity_scale_codec::{Codec, Decode, Encode};
    use pvq_extension::{DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};

    pub trait ExtensionFungibles {
        type AssetId: Codec + scale_info::TypeInfo + 'static;
        type AccountId: Codec + scale_info::TypeInfo + 'static;
        type Balance: Codec + scale_info::TypeInfo + 'static;
        fn total_supply(asset: Self::AssetId) -> Self::Balance;
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
    #[allow(non_camel_case_types)]
    pub enum Functions<Impl: ExtensionFungibles> {
        total_supply {
            asset: Impl::AssetId,
        },
        balance {
            asset: Impl::AssetId,
            who: Impl::AccountId,
        },
        #[doc(hidden)]
        __marker(core::marker::PhantomData<Impl>),
    }

    impl<Impl: ExtensionFungibles> Dispatchable for Functions<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Functions::total_supply { asset } => Ok(Impl::total_supply(asset).encode()),
                Functions::balance { asset, who } => Ok(Impl::balance(asset, who).encode()),
                Functions::__marker(_) => Err(DispatchError::PhantomData),
            }
        }
    }

    impl<Impl: ExtensionFungibles> ExtensionId for Functions<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 1u64;
    }

    pub fn metadata<Impl: ExtensionFungibles>() -> pvq_extension::metadata::ExtensionMetadata {
        pvq_extension::metadata::ExtensionMetadata {
            name: "ExtensionFungibles",
            functions: vec![
                pvq_extension::metadata::FunctionMetadata {
                    name: "total_supply",
                    inputs: vec![pvq_extension::metadata::FunctionParamMetadata {
                        name: "asset",
                        ty: scale_info::meta_type::<Impl::AssetId>(),
                    }],
                    output: scale_info::meta_type::<Impl::Balance>(),
                },
                pvq_extension::metadata::FunctionMetadata {
                    name: "balance",
                    inputs: vec![
                        pvq_extension::metadata::FunctionParamMetadata {
                            name: "asset",
                            ty: scale_info::meta_type::<Impl::AssetId>(),
                        },
                        pvq_extension::metadata::FunctionParamMetadata {
                            name: "who",
                            ty: scale_info::meta_type::<Impl::AccountId>(),
                        },
                    ],
                    output: scale_info::meta_type::<Impl::Balance>(),
                },
            ],
        }
    }
}

mod extensions_impl {
    pub struct ExtensionsImpl;
    use super::*;

    impl extension_core::ExtensionCore for ExtensionsImpl {
        type ExtensionId = u64;
        fn has_extension(id: u64) -> bool {
            matches!(id, 0 | 1)
        }
    }

    impl extension_fungibles::ExtensionFungibles for ExtensionsImpl {
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

    pub type Extensions = (
        extension_core::Functions<ExtensionsImpl>,
        extension_fungibles::Functions<ExtensionsImpl>,
    );

    pub fn metadata() -> pvq_extension::metadata::Metadata {
        pvq_extension::metadata::Metadata::new(vec![
            extension_core::metadata::<ExtensionsImpl>(),
            extension_fungibles::metadata::<ExtensionsImpl>(),
        ])
    }
}

mod tests {
    use super::*;
    use parity_scale_codec::Encode;
    use pvq_extension::{ExtensionsExecutor, InvokeSource};
    use tracing_subscriber::prelude::*;

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

        let mut executor = ExtensionsExecutor::<extensions_impl::Extensions, ()>::new(InvokeSource::Runtime);
        let program_blob = include_bytes!("../../output/poc-guest-transparent-call.polkavm").to_vec();
        let mut args = vec![];
        args.extend(1u64.to_le_bytes());
        extension_fungibles::Functions::<extensions_impl::ExtensionsImpl>::total_supply { asset: 1u32 }
            .encode_to(&mut args);
        let res = executor.execute_method(&program_blob, &args, 0);
        println!("res: {:?}", res);
        assert_eq!(res, Ok(200u64.to_le_bytes().to_vec()));
    }
}
