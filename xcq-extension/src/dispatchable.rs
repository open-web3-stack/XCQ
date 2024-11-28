use scale_info::prelude::vec::Vec;
pub trait Dispatchable {
    fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

#[derive(Debug)]
pub enum DispatchError {
    PhantomData,
}
