use std::u8;

use solana_program::{account_info::next_account_info,entrypoint, entrypoint::{ ProgramResult, __AccountInfo}, pubkey::Pubkey};
use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize)]
struct OnchainData{
    count : u32
}

entrypoint!(process_instruction);
fn process_instruction(
    publickey : &Pubkey,
    accounts : &[__AccountInfo],
    instruction : &[u8]
) -> ProgramResult{
    let mut iter =accounts.iter();
    let acc = next_account_info(&mut iter)?;

    let mut counter = OnchainData::try_from_slice(&acc.data.borrow())?;
    if counter.count == 0 {
        counter.count +=  1
    }else{
        counter.count *= 2;
    };

    //writin back into the accounts
    counter.serialize(&mut *acc.data.borrow_mut());

    Ok(())
}