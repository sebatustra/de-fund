use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::states::FundAccount;

#[derive(Accounts)]
pub struct InitializeFund<'info> {
    #[account(
        init,
        payer = manager,
        space = FundAccount::get_len(),
        seeds = [b"fund".as_ref()],
        bump
    )]
    pub fund : Account<'info, FundAccount>,

    #[account(
        init,
        seeds = [b"mint".as_ref()],
        bump,
        payer = manager,
        mint::decimals = 0,
        mint::authority = fund
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = manager,
        seeds = [b"fund_vault".as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = fund,
    )]
    pub fund_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub manager: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}