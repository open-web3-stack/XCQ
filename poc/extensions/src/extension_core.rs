use crate::{DispatchError, Dispatchable};
use crate::{ExtensionId, ExtensionIdTy};
use parity_scale_codec::{Decode, Encode};

pub trait ExtensionCore {
    type Config: Config;
    fn some_host_function(
        args: <Self::Config as Config>::ArgsOfSomeHostFunction,
    ) -> <Self::Config as Config>::ResultOfSomeHostFunction;
}

pub trait Config {
    type ArgsOfSomeHostFunction: Decode;
    type ResultOfSomeHostFunction: Encode;
}

// #[extension(ExtensionCore)]
// type Call;

mod generated_by_extension_decl {
    use super::*;

    #[derive(Decode)]
    pub enum ExtensionCoreCall<Impl: ExtensionCore> {
        SomeHostFunction {
            args: <Impl::Config as Config>::ArgsOfSomeHostFunction,
        },
    }

    impl<Impl: ExtensionCore> Dispatchable for ExtensionCoreCall<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Self::SomeHostFunction { args } => Ok(Impl::some_host_function(args).encode()),
            }
        }
    }

    impl<Impl: ExtensionCore> ExtensionId for ExtensionCoreCall<Impl> {
        const EXTENSION_ID: ExtensionIdTy = 0u64;
    }

    // TODO: remove this when formalized
    #[allow(dead_code)]
    pub type Call<Impl> = ExtensionCoreCall<Impl>;
}

#[allow(unused_imports)]
pub use generated_by_extension_decl::*;
