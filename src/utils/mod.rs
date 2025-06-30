use crate::models::{Account, InstructionResponse};
use base58::ToBase58;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

pub const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111112";
pub const SPL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub fn generate_token_instruction(
    program_id: &str,
    accounts: Vec<Account>,
    instruction_data: Vec<u8>,
) -> InstructionResponse {
    InstructionResponse {
        program_id: program_id.to_string(),
        accounts,
        data: general_purpose::STANDARD.encode(instruction_data),
    }
}

pub fn generate_random_address() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    
    bytes.to_vec().to_base58()
}

pub fn create_account(pubkey: &str, is_signer: bool, is_writable: bool) -> Account {
    Account {
        pubkey: pubkey.to_string(),
        is_signer,
        is_writable,
    }
}

pub fn generate_token_create_instruction_data(decimals: u8) -> Vec<u8> {
    let mut data = vec![0];
    data.push(decimals);
    
    let mut rng = rand::thread_rng();
    for _ in 0..8 {
        let random_byte: u8 = rng.gen_range(0..=255);
        data.push(random_byte);
    }
    
    data
}

pub fn generate_token_mint_instruction_data(amount: u64) -> Vec<u8> {
    let mut data = vec![7]; // Instruction index 7 for token minting
    
    data.extend_from_slice(&amount.to_le_bytes());
    
    data
}

pub fn generate_token_transfer_instruction_data(amount: u64) -> Vec<u8> {
    let mut data = vec![3]; // Instruction index 3 for token transfer
    
    data.extend_from_slice(&amount.to_le_bytes());
    
    data
}

pub fn generate_sol_transfer_instruction_data(lamports: u64) -> Vec<u8> {
    let mut data = vec![2]; // Instruction index 2 for system transfer
    
    data.extend_from_slice(&lamports.to_le_bytes());
    
    data
} 