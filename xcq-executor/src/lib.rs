#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloc::vec::Vec;
pub use polkavm::{Caller, Config, Engine, Linker, Module, ProgramBlob};

pub trait XcqExecutorContext {
    fn register_host_functions<T>(&mut self, linker: &mut Linker<T>);
}

pub struct XcqExecutor<Ctx: XcqExecutorContext> {
    engine: Engine,
    linker: Linker<Ctx>,
    context: Ctx,
}

#[derive(Debug)]
pub enum XcqExecutorError {
    ProgramParseError(polkavm::ProgramParseError),
    PrepareError(polkavm::Error),
    ExecutionError(polkavm::ExecutionError<polkavm::Error>),
}

impl From<polkavm::ProgramParseError> for XcqExecutorError {
    fn from(err: polkavm::ProgramParseError) -> Self {
        Self::ProgramParseError(err)
    }
}

impl From<polkavm::Error> for XcqExecutorError {
    fn from(err: polkavm::Error) -> Self {
        Self::PrepareError(err)
    }
}

impl From<polkavm::ExecutionError<polkavm::Error>> for XcqExecutorError {
    fn from(err: polkavm::ExecutionError<polkavm::Error>) -> Self {
        Self::ExecutionError(err)
    }
}

impl<Ctx: XcqExecutorContext> XcqExecutor<Ctx> {
    pub fn new(config: Config, mut context: Ctx) -> Self {
        let engine = Engine::new(&config).unwrap();
        let mut linker = Linker::<Ctx>::new(&engine);
        context.register_host_functions(&mut linker);
        Self {
            engine,
            linker,
            context,
        }
    }

    pub fn execute(
        &mut self,
        raw_blob: &[u8],
        method: impl AsRef<[u8]>,
        input: &[u8],
    ) -> Result<Vec<u8>, XcqExecutorError> {
        let blob = ProgramBlob::parse(raw_blob.into())?;
        let module = Module::from_blob(&self.engine, &Default::default(), blob)?;
        let instance_pre = self.linker.instantiate_pre(&module)?;
        let instance = instance_pre.instantiate()?;

        let input_ptr = if !input.is_empty() {
            // First sbrk call to get the start of the heap
            let start_ptr = instance.sbrk(0)?.expect("should not fail because we don't allocate");
            // Second sbrk call to check the allocation doesn't exceed the heap limit
            if instance.sbrk(input.len() as u32)?.is_none() {
                return Err(XcqExecutorError::ExecutionError(polkavm::ExecutionError::Error(
                    polkavm::Error::from("cannot srk enough memory"),
                )));
            };
            // Args are passed via guest's heap
            instance
                .write_memory(start_ptr, input)
                .map_err(|e| XcqExecutorError::ExecutionError(polkavm::ExecutionError::Trap(e)))?;
            start_ptr
        } else {
            0
        };
        tracing::info!("(passing args): input_ptr: {}, input_len: {:?}", input_ptr, input.len());

        let res = instance.call_typed::<(u32, u32), u64>(&mut self.context, method, (input_ptr, input.len() as u32))?;
        let res_size = (res >> 32) as u32;
        let res_ptr = (res & 0xffffffff) as u32;
        let result = instance
            .read_memory_into_vec(res_ptr, res_size)
            .map_err(|e| XcqExecutorError::ExecutionError(polkavm::ExecutionError::Trap(e)))?;
        Ok(result)
    }
}
