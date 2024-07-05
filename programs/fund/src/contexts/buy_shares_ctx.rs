use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::FundAccount;

#[derive(Accounts)]
pub struct BuyShares<'info> {
    #[account(
        mut,
        seeds = [b"fund".as_ref()],
        bump,
    )]
    pub fund : Account<'info, FundAccount>,

    #[account(
        seeds = [b"mint".as_ref()],
        bump,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"fund_vault".as_ref()],
        bump,
    )]
    pub fund_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_fund_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

}