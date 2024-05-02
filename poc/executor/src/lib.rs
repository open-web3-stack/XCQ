#![cfg_attr(not(feature = "std"), no_std)]

use polkavm::{Config, Engine, Linker, Module, ProgramBlob};

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

    pub fn execute(&mut self, raw_blob: &[u8]) -> Result<u32, XcqExecutorError> {
        let blob = ProgramBlob::parse(&raw_blob[..])?;
        let module = Module::from_blob(&self.engine, &Default::default(), &blob)?;
        let instance_pre = self.linker.instantiate_pre(&module)?;
        let instance = instance_pre.instantiate()?;

        let result = instance.call_typed::<(), u32>(&mut self.context, "main", ())?;

        Ok(result)
    }
}
