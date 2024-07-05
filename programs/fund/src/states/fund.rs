use anchor_lang::prelude::*;

#[account]
pub struct FundAccount {
    pub manager: Pubkey,
    pub token_mint: Pubkey,
    pub fund_vault: Pubkey,
    pub total_shares: u64,
    pub total_value: i64,
}

impl FundAccount {
    pub fn get_len() -> usize {
        return 8
            + 32
            + 32
            + 32
            + 8
            + 8
    }
}