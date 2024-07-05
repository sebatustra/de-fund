use anchor_lang::prelude::*;

use crate::states::{FundAccount, InvestmentAccount};

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct AddInvestment<'info> {
    #[account(
        mut,
        seeds = [b"fund".as_ref()],
        bump,
        constraint = fund.manager == *manager.key
    )]
    pub fund : Account<'info, FundAccount>,

    #[account(
        init,
        seeds = [b"investment".as_ref(), identifier.as_bytes()],
        bump,
        payer = manager,
        space = InvestmentAccount::get_len(&identifier)
    )]
    pub investment: Account<'info, InvestmentAccount>,

    #[account(mut)]
    pub manager: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}
