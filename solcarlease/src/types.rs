use solana_sdk::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Car {
    pub car_id: u64,
    pub model: String,
    pub owner: Pubkey,
    pub is_leased: bool,
}

impl Sealed for Car {}

impl IsInitialized for Car {
    fn is_initialized(&self) -> bool {
        self.car_id != 0
    }
}

impl Pack for Car {
    const LEN: usize = 73;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Car::LEN];
        let (car_id_dst, model_dst, owner_dst, is_leased_dst) = mut_array_refs![dst, 8, 32, 32, 1];

        *car_id_dst = self.car_id.to_le_bytes();
        let model_bytes = self.model.as_bytes();
        let model_len = model_bytes.len().min(32);
        model_dst[..model_len].copy_from_slice(&model_bytes[..model_len]);
        for i in model_len..32 {
            model_dst[i] = 0;
        }
        owner_dst.copy_from_slice(self.owner.as_ref());
        is_leased_dst[0] = self.is_leased as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Car::LEN];
        let (car_id_src, model_src, owner_src, is_leased_src) = array_refs![src, 8, 32, 32, 1];

        let car_id = u64::from_le_bytes(*car_id_src);
        let model = String::from_utf8(model_src.to_vec()).unwrap_or_default().trim_end_matches('\0').to_string();
        let owner = Pubkey::new_from_array(*owner_src);
        let is_leased = is_leased_src[0] != 0;

        Ok(Car {
            car_id,
            model,
            owner,
            is_leased,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lease {
    pub lease_id: u64,
    pub car_id: u64,
    pub lessee: Pubkey,
    pub start_date: u64,
    pub end_date: u64,
    pub payment_status: bool,
}

impl Sealed for Lease {}

impl IsInitialized for Lease {
    fn is_initialized(&self) -> bool {
        self.lease_id != 0
    }
}

impl Pack for Lease {
    const LEN: usize = 65;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Lease::LEN];
        let (lease_id_dst, car_id_dst, lessee_dst, start_date_dst, end_date_dst, payment_status_dst) = mut_array_refs![dst, 8, 8, 32, 8, 8, 1];

        *lease_id_dst = self.lease_id.to_le_bytes();
        *car_id_dst = self.car_id.to_le_bytes();
        lessee_dst.copy_from_slice(self.lessee.as_ref());
        *start_date_dst = self.start_date.to_le_bytes();
        *end_date_dst = self.end_date.to_le_bytes();
        payment_status_dst[0] = self.payment_status as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Lease::LEN];
        let (lease_id_src, car_id_src, lessee_src, start_date_src, end_date_src, payment_status_src) = array_refs![src, 8, 8, 32, 8, 8, 1];

        let lease_id = u64::from_le_bytes(*lease_id_src);
        let car_id = u64::from_le_bytes(*car_id_src);
        let lessee = Pubkey::new_from_array(*lessee_src);
        let start_date = u64::from_le_bytes(*start_date_src);
        let end_date = u64::from_le_bytes(*end_date_src);
        let payment_status = payment_status_src[0] != 0;

        Ok(Lease {
            lease_id,
            car_id,
            lessee,
            start_date,
            end_date,
            payment_status,
        })
    }
}
