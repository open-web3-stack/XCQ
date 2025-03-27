use pvq_primitives::PvqError;
#[derive(Debug)]
pub enum PvqExecutorError<UserError> {
    InvalidProgramFormat,
    MemoryAccessError(polkavm::MemoryAccessError),
    // Extract from the PVM CallError
    Trap,
    // Extract from the PVM CallError
    NotEnoughGas,
    // Usually a custom error type from the extension system definition
    User(UserError),
    // Other errors directly from the PVM
    OtherPvmError(polkavm::Error),
}

impl<UserError> From<polkavm::MemoryAccessError> for PvqExecutorError<UserError> {
    fn from(err: polkavm::MemoryAccessError) -> Self {
        Self::MemoryAccessError(err)
    }
}

impl<UserError> From<polkavm::Error> for PvqExecutorError<UserError> {
    fn from(err: polkavm::Error) -> Self {
        Self::OtherPvmError(err)
    }
}

impl<UserError> From<polkavm::CallError<UserError>> for PvqExecutorError<UserError> {
    fn from(err: polkavm::CallError<UserError>) -> Self {
        match err {
            polkavm::CallError::Trap => Self::Trap,
            polkavm::CallError::NotEnoughGas => Self::NotEnoughGas,
            polkavm::CallError::Error(e) => Self::OtherPvmError(e),
            polkavm::CallError::User(e) => Self::User(e),
        }
    }
}

impl<UserError> From<PvqExecutorError<UserError>> for PvqError {
    fn from(e: PvqExecutorError<UserError>) -> PvqError {
        match e {
            PvqExecutorError::InvalidProgramFormat => PvqError::InvalidPvqProgramFormat,
            PvqExecutorError::MemoryAccessError(_) => PvqError::MemoryAccessError,
            PvqExecutorError::Trap => PvqError::Trap,
            PvqExecutorError::NotEnoughGas => PvqError::QueryExceedsWeightLimit,
            PvqExecutorError::User(_) => PvqError::HostCallError,
            PvqExecutorError::OtherPvmError(_) => PvqError::Other,
        }
    }
}
