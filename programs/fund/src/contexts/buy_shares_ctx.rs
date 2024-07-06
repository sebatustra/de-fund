use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::FundAccount;

#[derive(Accounts)]
#[instruction(usdc_amount: u64)]
pub struct BuyShares<'info> {
    #[account(
        mut,
        seeds = [b"fund".as_ref()],
        bump,
    )]
    pub fund : Box<Account<'info, FundAccount>>,

    #[account(
        mut,
        seeds = [b"mint".as_ref()],
        bump,
    )]
    pub fund_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = fund_token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_fund_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"fund_usdc_vault".as_ref()],
        bump,
    )]
    pub fund_usdc_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = buyer_usdc_token_account.owner == *buyer.key,
        constraint = buyer_usdc_token_account.mint == usdc_mint.key(),
        constraint = buyer_usdc_token_account.amount >= usdc_amount
    )]
    pub buyer_usdc_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

}