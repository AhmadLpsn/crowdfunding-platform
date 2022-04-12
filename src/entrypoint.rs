use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint,
    entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct CampaignAccount {
    pub wner: Pubkey,
    pub amounts: u64,
    pub descriptions: String,
    pub fulfilled: u64,
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_byte, all_other_bytes) = instruction_data.split_first().unwrap();
    let itertable_accounts = &mut accounts.iter();

    if *instruction_byte == 0 {
        // create campaign

        let campaign_account = next_account_info(itertable_accounts).unwrap();
        let amount = all_other_bytes
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .unwrap();
        let description = String::from_utf8(all_other_bytes[9..].to_vec()).unwrap();

        let mut campaign_account_data =
            CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        campaign_account_data.amounts = amount;
        campaign_account_data.descriptions = description;
        campaign_account_data.fulfilled = 0;

        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    } else if *instruction_byte == 1 {
        // fund a campaign
    } else if *instruction_byte == 2 {
        // get how much funds are left to reach the requested amount
        let campaign_account = next_account_info(itertable_accounts).unwrap();
        let campaign_account_data =
            CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        msg!(
            "{}",
            campaign_account_data.amounts - campaign_account_data.fulfilled
        )
    } else if *instruction_byte == 3 {

        // withdraw all collected funds and close campaign
    }
    Ok(())
}