use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
struct OnchainData {
    count: u32,
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let acc = next_account_info(&mut iter)?;

    let mut counter = OnchainData::try_from_slice(&acc.data.borrow())?;
    if counter.count == 0 {
        counter.count += 1
    } else {
        counter.count *= 2;
    };

    counter.serialize(&mut *acc.data.borrow_mut())?;

    Ok(())
}