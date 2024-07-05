use anchor_lang::prelude::*;
use anchor_spl::token;

mod states;
mod contexts;

use contexts::*;
use states::*;

declare_id!("FXvCybtDJeNSAKGAQhBujXQWp3ab61ReP84EFpyYFe4r");

#[program]
pub mod fund {

    use super::*;

    pub fn initialize_fund(
        ctx: Context<InitializeFund>,
        initial_investment: i64,
        initial_shares: u64
    ) -> Result<()> {
        let fund_account = &mut ctx.accounts.fund;
        fund_account.set_inner(FundAccount {
            manager: *ctx.accounts.manager.key,
            token_mint: ctx.accounts.token_mint.key(),
            fund_vault: ctx.accounts.fund_vault.key(),
            total_value: initial_investment,
            total_shares: initial_shares
        });

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                token::MintTo {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.fund_vault.to_account_info(),
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
        fund_account.total_value = fund_account.total_value.checked_add(investment_amount).unwrap();

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

        let final_balance = payment_amount.checked_sub(investment.investment_amount).unwrap();

        let fund_account = &mut ctx.accounts.fund;
        fund_account.total_value = fund_account.total_value.checked_add(final_balance).unwrap();

        Ok(())
    }

    pub fn buy_shares(
        ctx: Context<BuyShares>,
        USDC_invested: u64
    ) -> Result<()> {

        let fund_account = &mut ctx.accounts.fund;

        let share_value = fund_account.total_value.checked_div(fund_account.total_shares.try_into().unwrap()).unwrap();

        let new_shares = USDC_invested.checked_div(share_value.try_into().unwrap()).unwrap() ;

        fund_account.total_value = fund_account.total_value.checked_add(USDC_invested.try_into().unwrap()).unwrap();







        Ok(())
    }
}
