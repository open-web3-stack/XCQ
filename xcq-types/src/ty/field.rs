use crate::vec::Vec;
use crate::XcqType;
use parity_scale_codec::Encode;
// A Named or Unnamed field in a composite type
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct Field {
    pub ident: Vec<u8>,
    pub ty: XcqType,
}
