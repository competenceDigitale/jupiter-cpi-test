
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::program::*;
//use jupiter_cpi::*;
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    /*let accounts_iter = &mut accounts.iter();
    let program = next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;
    let user_transfer_authority=next_account_info(accounts_iter)?;
    let destination_token_account=next_account_info(accounts_iter)?;

    let accounts = jupiter_cpi::accounts::Route{
        token_program: tokenProgram.clone(),
        user_transfer_authority:user_transfer_authority.clone(),
        destination_token_account:destination_token_account.clone(),
    };
    let swap_ix = Instruction {
        program_id: jupiter_cpi::ID,
        accounts,
        data: jupiter_override::Route {
            swap_leg,
            in_amount: 10,
            quoted_out_amount: 0,
            slippage_bps: 0,
            platform_fee_bps: 0,
        }
        .data(),
    };
    
    invoke(
        &ix,
        &[
            tokenProgram.clone(),
            user_transfer_authority.clone(),
            destination_token_account.clone(),
        ]
    )?;*/


    Ok(())
}
