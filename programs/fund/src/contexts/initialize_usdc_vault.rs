use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::states::FundAccount;

#[derive(Accounts)]
pub struct InitializeUSDCVault<'info> {
    #[account(
        mut,
        seeds = [b"fund".as_ref()],
        bump,
        constraint = fund.manager == *manager.key
    )]
    pub fund : Box<Account<'info, FundAccount>>,

    pub usdc_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = manager,
        seeds = [b"fund_usdc_vault".as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = manager
    )]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub manager: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}