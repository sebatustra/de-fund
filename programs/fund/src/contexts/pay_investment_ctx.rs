use anchor_lang::prelude::*;
use crate::states::{FundAccount, InvestmentAccount};

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct PayInvestment<'info> {
    #[account(
        mut,
        seeds = [b"fund".as_ref()],
        bump,
        constraint = fund.manager == *manager.key
    )]
    pub fund : Box<Account<'info, FundAccount>>,

    #[account(
        mut,
        seeds = [b"investment".as_ref(), identifier.as_bytes()],
        bump,
    )]
    pub investment: Box<Account<'info, InvestmentAccount>>,

    #[account(mut)]
    pub manager: Signer<'info>,

    pub system_program: Program<'info, System>,
}