pub trait Dispatchable {
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

pub type ExtensionTypeId = u64;

pub enum DispatchError {
    InvalidMethod,
}
