use anchor_lang::prelude::*;

#[account]
pub struct FundAccount {
    pub manager: Pubkey,
    pub fund_token_mint: Pubkey,
    pub fund_shares_vault: Pubkey,
    pub usdc_vault: Option<Pubkey>,
    pub total_shares: u64,
    pub total_value: u64,
}

impl FundAccount {
    pub fn get_len() -> usize {
        return 8
            + 32
            + 32
            + 32
            + 1 + 32
            + 8
            + 8
    }
}