
// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint::{self, ProgramResult },
//     msg,
//     pubkey::Pubkey,
// } ;

// entrypoint!(process_instruction);

// #[derive(BorshDeserialize,BorshSerialize)]
// enum MethodType{
//     Increment(u32),
//     Decrement(u32)
// }

// struct  Counter{
//     count : u32
// }

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let acc = next_account_info(&mut accounts.iter())?;

//     let data = Method_type::try_from_slice(instruction_data)?;

//     let mut counter = Counter::try_from_slice(&acc.data.borrow());

//     match Method_type {
//         Method_type::Increment(amount) =>{
//             counter.count += amount;
//         }
//         Method_type::Decrement(amount) =>{
//             counter.count -= amount;
//         }
//     }

//     counter.deserialize(&mut  *acc.data.borrow_mut())
// }
use solana_program::entrypoint;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{ ProgramResult},
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
enum MethodType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let acc = next_account_info(accounts_iter)?;

    let method = MethodType::try_from_slice(instruction_data)?;

    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;

    match method {
        MethodType::Increment(amount) => {
            msg!("Incrementing by {}", amount);
            counter.count += amount;
        }
        MethodType::Decrement(amount) => {
            msg!("Decrementing by {}", amount);
            counter.count = counter.count.saturating_sub(amount);
        }
    }

    counter.serialize(&mut &mut acc.data.borrow_mut()[..])?;

    Ok(())
}

