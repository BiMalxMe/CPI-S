use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let data_account = next_account_info(&mut account_info_iter)?;
    let program_account = next_account_info(&mut account_info_iter)?;

    let instruction_to_invoke = Instruction {
        program_id: *program_account.key,
        accounts: vec![
            AccountMeta {
                pubkey: *data_account.key,
                is_signer: true,
                is_writable: true,
            },
        ],
        data: vec![],
    };

    invoke(
        &instruction_to_invoke,
        &[
            data_account.clone(),
        ],
    )?;

    Ok(())
}