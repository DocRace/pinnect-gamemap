use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    program_error::ProgramError, program_pack::{IsInitialized, Pack, Sealed}, 
    program::{invoke, invoke_signed},
};

use serde_derive::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};

// Define data structures
#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq)]


8888
    pub owner: Pubkey,
    pub map_id: Pubkey,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub tags: String,
    pub image: String,
    pub is_on_chain: bool,
}

// Instruction data
pub enum MagiPopMapInstruction {
    CreateMap { name: String, description: String, image: String },
    CreateLocation { 
        owner: Pubkey, 
        map_id: Pubkey, 
        name: String, 
        x: f64, 
        y: f64, 
        tags: String, 
        image: String 
    },
}

impl MagiPopMapInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (name, rest) = unpack_string(rest)?;
                let (description, image) = unpack_string(rest)?;
                Self::CreateMap { name, description, image }
            }
            1 => {
                let (owner, rest) = unpack_pubkey(rest)?;
                let (map_id, rest) = unpack_pubkey(rest)?;
                let (name, rest) = unpack_string(rest)?;
                let (x, rest) = unpack_f64(rest)?;
                let (y, rest) = unpack_f64(rest)?;
                let (tags, image) = unpack_string(rest)?;
                Self::CreateLocation { owner, map_id, name, x, y, tags, image }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
    fn unpack_string(input: &[u8]) -> Result<(String, &[u8]), ProgramError> {
        let length = input[0] as usize;
        let string_bytes = &input[1..=length];
        let string = String::from_utf8(string_bytes.to_vec())
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok((string, &input[length + 1..]))
    }
    
    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        let pubkey_bytes = &input[..32];
        let pubkey = Pubkey::new(pubkey_bytes);
        Ok((pubkey, &input[32..]))
    }
    
    fn unpack_f64(input: &[u8]) -> Result<(f64, &[u8]), ProgramError> {
        let f64_bytes = &input[..8];
        let f64_bits = u64::from_le_bytes(f64_bytes.try_into().unwrap());
        let float = f64::from_bits(f64_bits);
        Ok((float, &input[8..]))
    }
    
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = MagiPopMapInstruction::unpack(instruction_data)?;

    match instruction {
        MagiPopMapInstruction::CreateMap { name, description, image } => {
            process_create_map(accounts, name, description, image)
        }
        MagiPopMapInstruction::CreateLocation { 
            owner, 
            map_id, 
            name, 
            x, 
            y, 
            tags, 
            image 
        } => {
            process_create_location(accounts, owner, map_id, name, x, y, tags, image)
        }
    }
}
fn process_create_map(
    accounts: &[AccountInfo], 
    name: String, 
    description: String, 
    image: String,
) -> ProgramResult {
    let map_account = &mut accounts[0].try_borrow_mut_data()?;
    let map_info = Map {
        name,
        description,
        image,
    };
    map_info.serialize(&mut &mut map_account[..])?;
    Ok(())
}

fn process_create_location(
    accounts: &[AccountInfo], 
    owner: Pubkey, 
    map_id: Pubkey, 
    name: String, 
    x: f64, 
    y: f64, 
    tags: String, 
    image: String
) -> ProgramResult {
    let location_account = &mut accounts[0].try_borrow_mut_data()?;
    let location_info = Location {
        owner,
        map_id,
        name,
        x,
        y,
        tags,
        image,
        is_on_chain: false,
    };
    location_info.serialize(&mut &mut location_account[..])?;
    Ok(())
}