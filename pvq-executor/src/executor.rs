use alloc::vec::Vec;
use polkavm::{Config, Engine, Linker, Module, ModuleConfig, ProgramBlob};

use crate::context::PvqExecutorContext;
use crate::error::PvqExecutorError;

type PvqExecutorResult<UserError> = Result<Vec<u8>, PvqExecutorError<UserError>>;
type GasLimit = Option<i64>;

pub struct PvqExecutor<Ctx: PvqExecutorContext> {
    engine: Engine,
    linker: Linker<Ctx::UserData, Ctx::UserError>,
    context: Ctx,
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
        gas_limit: GasLimit,
    ) -> (PvqExecutorResult<Ctx::UserError>, GasLimit) {
        let blob = match ProgramBlob::parse(program.into()) {
            Ok(blob) => blob,
            Err(_) => return (Err(PvqExecutorError::InvalidProgramFormat), gas_limit),
        };

        // TODO: make this configurable
        let mut module_config = ModuleConfig::new();
        module_config.set_aux_data_size(args.len() as u32);
        if gas_limit.is_some() {
            module_config.set_gas_metering(Some(polkavm::GasMeteringKind::Sync));
        }

        let module = match Module::from_blob(&self.engine, &module_config, blob) {
            Ok(module) => module,
            Err(err) => return (Err(err.into()), gas_limit),
        };

        let instance_pre = match self.linker.instantiate_pre(&module) {
            Ok(instance_pre) => instance_pre,
            Err(err) => return (Err(err.into()), gas_limit),
        };

        let mut instance = match instance_pre.instantiate() {
            Ok(instance) => instance,
            Err(err) => return (Err(err.into()), gas_limit),
        };

        if let Some(gas_limit) = gas_limit {
            instance.set_gas(gas_limit);
        }

        // From this point on, we include instance.gas() in the return value
        let result = (|| {
            instance.write_memory(module.memory_map().aux_data_address(), args)?;

            tracing::info!("Calling entrypoint with args: {:?}", args);
            let res = instance.call_typed_and_get_result::<u64, (u32, u32)>(
                self.context.data(),
                "pvq",
                (module.memory_map().aux_data_address(), args.len() as u32),
            )?;

            let res_size = (res >> 32) as u32;
            let res_ptr = (res & 0xffffffff) as u32;

            let result = instance.read_memory(res_ptr, res_size)?;

            tracing::info!("Result: {:?}", result);
            Ok(result)
        })();

        if gas_limit.is_some() {
            (result, Some(instance.gas()))
        } else {
            (result, None)
        }
    }
}
