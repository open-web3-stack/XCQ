use crate::vec::Vec;
use crate::XcqType;
// A Named or Unnamed field in a composite type
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Field {
    pub ident: Vec<u8>,
    pub ty: XcqType,
}
