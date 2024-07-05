use anchor_lang::prelude::*;

#[account]
pub struct InvestmentAccount {
    pub investment_amount: i64,
    pub payment_amount: Option<i64>,
    pub maturity_date: i64,
    pub payment_date: Option<i64>,
    pub is_active: bool,
    pub identifier: String,
}

impl InvestmentAccount {
    pub fn get_len(identifier: &str) -> usize {
        return 8
            + 8
            + 1 + 8
            + 8
            + 1 + 8
            + 1
            + 4 + identifier.len()
    }
}