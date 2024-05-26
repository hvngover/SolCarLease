use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;
use thiserror::Error;

entrypoint!(process_instruction);

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("Invalid Input")]
    InvalidInput,
    #[error("Unauthorized Access")]
    Unauthorized,
    #[error("Insufficient Funds")]
    InsufficientFunds,
    #[error("Overflow Error")]
    OverflowError,
}

impl From<ContractError> for ProgramError {
    fn from(e: ContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CarListing {
    pub id: u64,
    pub price: u64,
    pub owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LeaseAgreement {
    pub car_id: u64,
    pub lessee: Pubkey,
    pub lease_start: u64,
    pub lease_end: u64,
    pub paid: bool,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let admin_account = next_account_info(accounts_iter)?;

    if !admin_account.is_signer {
        return Err(ContractError::Unauthorized.into());
    }

    match instruction_data[0] {
        0 => add_car_listing(accounts_iter, instruction_data),
        1 => create_lease_agreement(accounts_iter, instruction_data),
        _ => Err(ContractError::InvalidInput.into()),
    }
}

fn add_car_listing(
    accounts: &mut std::slice::Iter<AccountInfo>,
    instruction_data: &[u8],
) -> ProgramResult {
    let car_id_bytes: [u8; 8] = instruction_data[1..9]
        .try_into()
        .map_err(|_| ContractError::InvalidInput)?;
    let price_bytes: [u8; 8] = instruction_data[9..17]
        .try_into()
        .map_err(|_| ContractError::InvalidInput)?;

    let car_id = u64::from_le_bytes(car_id_bytes);
    let price = u64::from_le_bytes(price_bytes);

    if car_id == 0 || price == 0 {
        return Err(ContractError::InvalidInput.into());
    }

    let car_listing = CarListing {
        id: car_id,
        price,
        owner: *accounts.next().unwrap().key,
    };

    msg!("Car listing added: {:?}", car_listing);
    Ok(())
}

fn create_lease_agreement(
    accounts: &mut std::slice::Iter<AccountInfo>,
    instruction_data: &[u8],
) -> ProgramResult {
    let car_id_bytes: [u8; 8] = instruction_data[1..9]
        .try_into()
        .map_err(|_| ContractError::InvalidInput)?;
    let lease_start_bytes: [u8; 8] = instruction_data[9..17]
        .try_into()
        .map_err(|_| ContractError::InvalidInput)?;
    let lease_end_bytes: [u8; 8] = instruction_data[17..25]
        .try_into()
        .map_err(|_| ContractError::InvalidInput)?;

    let car_id = u64::from_le_bytes(car_id_bytes);
    let lease_start = u64::from_le_bytes(lease_start_bytes);
    let lease_end = u64::from_le_bytes(lease_end_bytes);

    if lease_start >= lease_end {
        return Err(ContractError::InvalidInput.into());
    }

    let lease_agreement = LeaseAgreement {
        car_id,
        lessee: *accounts.next().unwrap().key,
        lease_start,
        lease_end,
        paid: false,
    };

    msg!("Lease agreement created: {:?}", lease_agreement);
    Ok(())
}
