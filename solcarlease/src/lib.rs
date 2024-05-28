pub mod errors;
pub mod types;

use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use crate::errors::ContractError;
use crate::types::{Car, Lease};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];

    match instruction {
        0 => add_car(program_id, accounts, &instruction_data[1..]),
        1 => initiate_lease(program_id, accounts, &instruction_data[1..]),
        2 => make_payment(program_id, accounts, &instruction_data[1..]),
        3 => terminate_lease(program_id, accounts, &instruction_data[1..]),
        4 => return_car(program_id, accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn add_car(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;

    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let car_id = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    let model_length = instruction_data[8] as usize;
    let model = String::from_utf8(instruction_data[9..9 + model_length].to_vec()).unwrap();

    if car_id == 0 {
        return Err(ContractError::InvalidCarId.into());
    }

    let car = Car {
        car_id,
        model,
        owner: *owner_account.key,
        is_leased: false,
    };

    car.pack_into_slice(&mut owner_account.data.borrow_mut());
    Ok(())
}

fn initiate_lease(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lessee_account = next_account_info(account_info_iter)?;
    let car_account = next_account_info(account_info_iter)?;

    if !lessee_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let lease_id = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    let car_id = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());
    let start_date = u64::from_le_bytes(instruction_data[16..24].try_into().unwrap());
    let end_date = u64::from_le_bytes(instruction_data[24..32].try_into().unwrap());

    let mut car = Car::unpack_from_slice(&car_account.data.borrow())?;
    if car.is_leased {
        return Err(ContractError::CarAlreadyLeased.into());
    }

    let lease = Lease {
        lease_id,
        car_id,
        lessee: *lessee_account.key,
        start_date,
        end_date,
        payment_status: false,
    };

    lease.pack_into_slice(&mut car_account.data.borrow_mut());
    car.is_leased = true;
    car.pack_into_slice(&mut car_account.data.borrow_mut());
    Ok(())
}

fn make_payment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lessee_account = next_account_info(account_info_iter)?;
    let lease_account = next_account_info(account_info_iter)?;

    if !lessee_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let lease_id = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    let amount = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());

    let mut lease = Lease::unpack_from_slice(&lease_account.data.borrow())?;
    if lease.lease_id != lease_id {
        return Err(ContractError::InvalidLeaseId.into());
    }

    lease.payment_status = true;
    lease.pack_into_slice(&mut lease_account.data.borrow_mut());
    Ok(())
}

fn terminate_lease(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let car_account = next_account_info(account_info_iter)?;
    let lease_account = next_account_info(account_info_iter)?;

    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let lease_id = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());

    let lease = Lease::unpack_from_slice(&lease_account.data.borrow())?;
    if lease.lease_id != lease_id {
        return Err(ContractError::InvalidLeaseId.into());
    }

    let mut car = Car::unpack_from_slice(&car_account.data.borrow())?;
    car.is_leased = false;
    car.pack_into_slice(&mut car_account.data.borrow_mut());

    Ok(())
}

fn return_car(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lessee_account = next_account_info(account_info_iter)?;
    let car_account = next_account_info(account_info_iter)?;
    let lease_account = next_account_info(account_info_iter)?;

    if !lessee_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let lease_id = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    let condition_report_length = instruction_data[8] as usize;
    let condition_report = String::from_utf8(instruction_data[9..9 + condition_report_length].to_vec()).unwrap();

    let mut lease = Lease::unpack_from_slice(&lease_account.data.borrow())?;
    if lease.lease_id != lease_id {
        return Err(ContractError::InvalidLeaseId.into());
    }

    let mut car = Car::unpack_from_slice(&car_account.data.borrow())?;
    car.is_leased = false;
    car.pack_into_slice(&mut car_account.data.borrow_mut());

    Ok(())
}
