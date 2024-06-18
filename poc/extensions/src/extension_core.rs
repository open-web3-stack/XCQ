use crate::Vec;
use crate::{DispatchError, Dispatchable};
use crate::{ExtensionId, ExtensionIdTy};
use parity_scale_codec::{Decode, Encode};

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

// #[extension(ExtensionCore)]
// type Call;

mod generated_by_extension_decl {
    use super::*;

    type ExtensionIdOf<T> = <<T as ExtensionCore>::Config as Config>::ExtensionId;
    #[derive(Decode)]
    pub enum ExtensionCoreCall<Impl: ExtensionCore> {
        HasExtension { id: ExtensionIdOf<Impl> },
    }

    impl<Impl: ExtensionCore> Dispatchable for ExtensionCoreCall<Impl> {
        fn dispatch(self) -> Result<Vec<u8>, DispatchError> {
            match self {
                Self::HasExtension { id } => Ok(Impl::has_extension(id).encode()),
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
