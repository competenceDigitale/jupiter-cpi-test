use anchor_lang::InstructionData;
use jupiter_cpi::*;
use solana_program::instruction::Instruction;
use solana_program::program::*;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::AccountMeta,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let authority_ppu_info = next_account_info(accounts_iter)?;
    let create_program_ppu_info = next_account_info(accounts_iter)?;

    let (first_byte, remaining_bytes) = instruction_data.split_first().unwrap();
    let nonce_ppu = *first_byte as u8;

    let mut jupiter_metas = Vec::new();
    let (first_accounts, remaining_accounts) = accounts.split_at(2);

    let expected_allocated_key = Pubkey::create_program_address(
        &[&create_program_ppu_info.key.to_bytes()[..32], &[nonce_ppu]],
        program_id,
    )?;
    // Allocated key does not match the derived address
    if *authority_ppu_info.key != expected_allocated_key {
        return Err(ProgramError::InvalidArgument);
    }
    let authority_signature_seeds = [&create_program_ppu_info.key.to_bytes()[..32], &[nonce_ppu]];
    let signers = &[&authority_signature_seeds[..]];

    for account in remaining_accounts {
        let account_meta = to_account_meta(&account, authority_ppu_info.key);
        jupiter_metas.push(account_meta);
    }
    msg!("accounts metas size {:?} accounts infos size {:?} data size {:?} nonce {:?} ",jupiter_metas.len(),accounts.len(),remaining_bytes.len(),nonce_ppu);
    msg!("jupiter account 1 {:?}",jupiter_metas[0]);
    
    let swap_ix = Instruction {
        program_id: jupiter_cpi::ID,
        accounts: jupiter_metas,
        data: remaining_bytes.to_vec(),
    };
    invoke_signed(&swap_ix, &remaining_accounts, signers)?;
    Ok(())
}

fn to_account_meta(info: &AccountInfo, authority_key: &Pubkey) -> AccountMeta {
    let mut is_signer = info.is_signer;
    if *authority_key == *info.key {
        is_signer = true
    }
    AccountMeta {
        pubkey: *info.key,
        is_signer: is_signer,
        is_writable: info.is_writable,
    }
}
