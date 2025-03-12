#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloc::vec::Vec;
use polkavm::ModuleConfig;
pub use polkavm::{Caller, Config, Engine, Linker, Module, ProgramBlob};

pub trait PvqExecutorContext {
    type UserData;
    type UserError;
    fn register_host_functions(&mut self, linker: &mut Linker<Self::UserData, Self::UserError>);
    fn data(&mut self) -> &mut Self::UserData;
}

pub struct PvqExecutor<Ctx: PvqExecutorContext> {
    engine: Engine,
    linker: Linker<Ctx::UserData, Ctx::UserError>,
    context: Ctx,
}

#[derive(Debug)]
pub enum PvqExecutorError<UserError> {
    MemoryAllocationError,
    MemoryAccessError(polkavm::MemoryAccessError),
    CallError(polkavm::CallError<UserError>),
    OtherPVMError(polkavm::Error),
}

impl<UserError> From<polkavm::MemoryAccessError> for PvqExecutorError<UserError> {
    fn from(err: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(err)
    }
}

impl<UserError> From<polkavm::Error> for PvqExecutorError<UserError> {
    fn from(err: polkavm::Error) -> Self {
        Self::OtherPVMError(err)
    }
}

impl<UserError> From<polkavm::CallError<UserError>> for PvqExecutorError<UserError> {
    fn from(err: polkavm::CallError<UserError>) -> Self {
        Self::CallError(err)
    }
}

impl<Ctx: PvqExecutorContext> PvqExecutor<Ctx> {
    pub fn new(config: Config, mut context: Ctx) -> Self {
        let engine = Engine::new(&config).unwrap();
        let mut linker = Linker::<Ctx::UserData, Ctx::UserError>::new();
        // Register user-defined host functions
        context.register_host_functions(&mut linker);
        Self {
            engine,
            linker,
            context,
        }
    }

    pub fn execute(
        &mut self,
        program: &[u8],
        args: &[u8],
        _gas_limit: u64,
    ) -> Result<Vec<u8>, PvqExecutorError<Ctx::UserError>> {
        let blob = ProgramBlob::parse(program.into()).map_err(polkavm::Error::from)?;

        // TODO: make this configurable
        let mut module_config = ModuleConfig::new();
        module_config.set_aux_data_size(args.len() as u32);

        let module = Module::from_blob(&self.engine, &module_config, blob)?;
        let instance_pre = self.linker.instantiate_pre(&module)?;
        let mut instance = instance_pre.instantiate()?;

        instance.write_memory(module.memory_map().aux_data_address(), args)?;

        let res = instance.call_typed_and_get_result::<u64, (u32, u32)>(
            self.context.data(),
            "main",
            (module.memory_map().aux_data_address(), args.len() as u32),
        )?;
        let res_size = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as u32;
        let result = instance.read_memory(res_ptr, res_size)?;
        Ok(result)
    }
}
