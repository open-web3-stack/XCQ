use pvq_executor::PvqExecutor;
use pvq_primitives::{PvqError, PvqResult};

use crate::{
    perm_controller::{InvokeSource, PermissionController},
    CallDataTuple, Context,
};

/// Executor for extensions
///
/// This struct provides an executor for running extension code.
/// It wraps a PvqExecutor with a Context for extensions.
pub struct ExtensionsExecutor<C: CallDataTuple, P: PermissionController> {
    /// The underlying PVQ executor
    executor: PvqExecutor<Context<C, P>>,
}

impl<C: CallDataTuple, P: PermissionController> ExtensionsExecutor<C, P> {
    /// Create a new extensions executor
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the invocation
    pub fn new(source: InvokeSource) -> Self {
        let context = Context::<C, P>::new(source);
        let executor = PvqExecutor::new(Default::default(), context);
        Self { executor }
    }

    ///
    /// # Arguments
    ///
    /// * `program` - The program data
    /// * `args` - The input data
    ///
    /// # Returns
    ///
    /// The result of the execution or an error
    pub fn execute(&mut self, program: &[u8], args: &[u8], gas_limit: Option<i64>) -> (PvqResult, Option<i64>) {
        let (result, gas_remaining) = self.executor.execute(program, args, gas_limit);
        (result.map_err(PvqError::from), gas_remaining)
    }
}
