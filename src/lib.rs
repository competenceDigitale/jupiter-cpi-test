use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey, instruction::AccountMeta,
};
use solana_program::program::*;
use solana_program::instruction::Instruction;
use jupiter_cpi::*;
use anchor_lang::prelude::*;
use jupiter_cpi::jupiter_override::{Swap, SwapLeg};


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let program = next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;
    let user_transfer_authority=next_account_info(accounts_iter)?;
    let destination_token_account=next_account_info(accounts_iter)?;
    
    let accounts = vec![
        AccountMeta::new(*token_program.key, false),
        AccountMeta::new(*user_transfer_authority.key, false),
        AccountMeta::new(*destination_token_account.key, false),
    ];
    //let swap_leg= jupiter_cpi::jupiter_override::SwapLeg::Swap::Lifinity;
// Define an array of SwapLegs to chain together
let swap_legs = vec![
    //SwapLeg::Swap(Swap::Sencha),
    //SwapLeg::Swap(Swap::Mercurial),
];

// Chain together the SwapLegs into a single SwapLeg using the Chain variant
let swap_leg = SwapLeg::Chain { swap_legs };
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
    Ok(())
}