use polkavm::Linker;

pub trait PvqExecutorContext {
    type UserData;
    type UserError;
    fn register_host_functions(&mut self, linker: &mut Linker<Self::UserData, Self::UserError>);
    fn data(&mut self) -> &mut Self::UserData;
}
