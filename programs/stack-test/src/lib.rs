use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use large_stack_app;
use large_stack_app::cpi::accounts::UseLargeStack;
use large_stack_app::cpi::use_large_stack;
use large_stack_app::program::LargeStackApp;

declare_id!("G94eEM6q11o6XKuRV83aTbzmr8B56W5RRiC3vgVciwJB");

#[program]
pub mod stack_test {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn test_stack(ctx: Context<TestStack>) -> Result<()> {
        let cpi_program = ctx.accounts.large_stack_program.to_account_info();
        let cpi_accounts = UseLargeStack {
            tester: ctx.accounts.tester.to_account_info().clone(),
            account0: ctx.accounts.account0.to_account_info().clone(),
            account1: ctx.accounts.account1.to_account_info().clone(),
            account2: ctx.accounts.account2.to_account_info().clone(),
            account3: ctx.accounts.account3.to_account_info().clone(),
            account4: ctx.accounts.account4.to_account_info().clone(),
            token_program: ctx.accounts.token_program.to_account_info().clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        use_large_stack(cpi_context)?;
        Ok(())
    }

    pub fn test_stack_no_box(ctx: Context<TestStackNoBox>) -> Result<()> {
        let cpi_program = ctx.accounts.large_stack_program.to_account_info();
        let cpi_accounts = UseLargeStack {
            tester: ctx.accounts.tester.to_account_info().clone(),
            account0: ctx.accounts.account0.to_account_info().clone(),
            account1: ctx.accounts.account1.to_account_info().clone(),
            account2: ctx.accounts.account2.to_account_info().clone(),
            account3: ctx.accounts.account3.to_account_info().clone(),
            account4: ctx.accounts.account4.to_account_info().clone(),
            token_program: ctx.accounts.token_program.to_account_info().clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        use_large_stack(cpi_context)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
#[derive(Accounts)]
pub struct TestStack<'info> {
    #[account(mut)]
    pub tester: Signer<'info>,
    #[account(mut)]
    pub account0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account3: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account4: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub account5: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub account6: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account7: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account8: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account9: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account10: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub large_stack_program: Program<'info, LargeStackApp>,
}

#[derive(Accounts)]
pub struct TestStackNoBox<'info> {
    #[account(mut)]
    pub tester: Signer<'info>,
    #[account(mut)]
    pub account0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account3: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account4: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account5: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(mut)]
    pub account6: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account7: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account8: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account9: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub account10: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub large_stack_program: Program<'info, LargeStackApp>,
}
