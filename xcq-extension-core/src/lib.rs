use parity_scale_codec::{Decode, Encode};
use xcq_extension::extension;

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
