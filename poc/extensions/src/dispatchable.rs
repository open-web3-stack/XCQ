pub trait Dispatchable {
    // TODO: check if origin is specified as associated type
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

pub enum DispatchError {
    UnsupportedContext,
}
