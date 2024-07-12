use anchor_lang::prelude::*;
use anchor_spl::token;

mod states;
mod contexts;

use contexts::*;
use states::*;

declare_id!("6vUjvBGWETdE4duVqQBeu4WLCC3XgDkCmzhx4aCC7V4g");

#[program]
pub mod fund {

    use super::*;

    pub fn initialize_fund(
        ctx: Context<InitializeFund>,
        initial_investment: u64,
        initial_shares: u64
    ) -> Result<()> {
        let fund_account = &mut ctx.accounts.fund;
        fund_account.set_inner(FundAccount {
            manager: *ctx.accounts.manager.key,
            fund_token_mint: ctx.accounts.fund_token_mint.key(),
            fund_shares_vault: ctx.accounts.fund_shares_vault.key(),
            usdc_vault: None,
            total_value: initial_investment,
            total_shares: initial_shares
        });

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                token::MintTo {
                    mint: ctx.accounts.fund_token_mint.to_account_info(),
                    to: ctx.accounts.fund_shares_vault.to_account_info(),
                    authority: ctx.accounts.fund.to_account_info()
                }, 
                &[&[
                    "fund".as_bytes(),
                    &[ctx.bumps.fund]
                ]]
            ),
            initial_shares
        )?;

        Ok(())
    }

    pub fn initialize_usdc_vault(
        ctx: Context<InitializeUSDCVault>
    ) -> Result<()> {

        let fund_account = &mut ctx.accounts.fund;
        fund_account.usdc_vault = Some(ctx.accounts.usdc_vault.key());

        Ok(())
    }

    pub fn add_investment(
        ctx: Context<AddInvestment>,
        identifier: String,
        investment_amount: i64,
        maturity_date: i64
    ) -> Result<()> {

        let investment = &mut ctx.accounts.investment;
        investment.set_inner(InvestmentAccount {
            investment_amount,
            payment_amount: None,
            maturity_date,
            payment_date: None,
            is_active: true,
            identifier
        });

        let fund_account = &mut ctx.accounts.fund;
        fund_account.total_value = fund_account.total_value.checked_add(investment_amount.try_into().unwrap()).unwrap();

        Ok(())
    }

    pub fn pay_investment(
        ctx: Context<PayInvestment>,
        _identifier: String,
        payment_amount: i64,
        payment_date: i64
    ) -> Result<()> {

        let investment = &mut ctx.accounts.investment;
        investment.payment_date = Some(payment_date);
        investment.payment_amount = Some(payment_amount);
        investment.is_active = false;

        let final_balance = payment_amount.checked_sub(investment.investment_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let fund_account = &mut ctx.accounts.fund;

        if final_balance >= 0 {
            let final_balance = final_balance as u64;
            fund_account.total_value = fund_account.total_value
                .checked_add(final_balance)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        } else {
            let balance_abs_u64 = final_balance.abs() as u64;
            fund_account.total_value = fund_account.total_value
                .checked_sub(balance_abs_u64)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }

        Ok(())
    }

    pub fn buy_shares(
        ctx: Context<BuyShares>,
        usdc_amount: u64
    ) -> Result<()> {

        let fund_account = &mut ctx.accounts.fund;

        let current_share_value = 
            fund_account.total_value / fund_account.total_shares;

        let shares_to_issue = 
            usdc_amount / current_share_value;

        fund_account.total_shares = fund_account.total_shares
            .checked_add(shares_to_issue)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        fund_account.total_value = fund_account.total_value
            .checked_add(usdc_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                token::Transfer {
                    from: ctx.accounts.buyer_usdc_token_account.to_account_info(),
                    to: ctx.accounts.fund_usdc_vault.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info()
                }
            ), 
            usdc_amount
        )?;

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.fund_token_mint.to_account_info(),
                    to: ctx.accounts.buyer_fund_token_account.to_account_info(),
                    authority: ctx.accounts.fund.to_account_info()
                }, 
                &[&[
                    "fund".as_bytes(),
                    &[ctx.bumps.fund]
                ]]
            ),
            shares_to_issue
        )?;

        Ok(())
    }
}
