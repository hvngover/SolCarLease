use solana_sdk::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ContractError {
    #[error("Invalid car ID")]
    InvalidCarId,
    #[error("Car is already leased")]
    CarAlreadyLeased,
    #[error("Invalid lease ID")]
    InvalidLeaseId,
    #[error("Unauthorized access")]
    UnauthorizedAccess,
    #[error("Invalid input")]
    InvalidInput,
}

impl From<ContractError> for ProgramError {
    fn from(e: ContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
