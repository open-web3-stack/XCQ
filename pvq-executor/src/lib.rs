#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloc::vec::Vec;
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

    pub fn execute(&mut self, raw_blob: &[u8], input: &[u8]) -> Result<Vec<u8>, PvqExecutorError<Ctx::UserError>> {
        let blob = ProgramBlob::parse(raw_blob.into()).map_err(polkavm::Error::from)?;
        let module = Module::from_blob(&self.engine, &Default::default(), blob)?;
        let instance_pre = self.linker.instantiate_pre(&module)?;
        let mut instance = instance_pre.instantiate()?;

        let input_ptr = if !input.is_empty() {
            // First sbrk call to get the start of the heap
            let start_ptr = instance
                .sbrk(0)
                .expect("should not fail because we don't allocate")
                .expect("should not fail because we don't allocate");
            // Second sbrk call to check the allocation doesn't exceed the heap limit
            instance
                .sbrk(input.len() as u32)
                .map_err(|_| PvqExecutorError::MemoryAllocationError)?
                .ok_or(PvqExecutorError::MemoryAllocationError)?;
            // Args are passed via guest's heap
            instance.write_memory(start_ptr, input)?;
            start_ptr
        } else {
            0
        };
        tracing::info!("(passing args): input_ptr: {}, input_len: {:?}", input_ptr, input.len());

        let res = instance.call_typed_and_get_result::<u64, (u32, u32)>(
            self.context.data(),
            "main",
            (input_ptr, input.len() as u32),
        )?;
        let res_size = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as u32;
        let result = instance.read_memory(res_ptr, res_size)?;
        Ok(result)
    }
}
