pub trait Dispatchable {
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

pub enum DispatchError {
    InvalidMethod,
}
