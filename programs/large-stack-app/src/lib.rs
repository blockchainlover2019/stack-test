use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("HHTUQDcC2Ht1CPhRJzkRLhr1qZhbkQ9TFgMZ4hg9gbT7");

#[program]
pub mod large_stack_app {
    use super::*;
    pub fn use_large_stack(_context: Context<UseLargeStack>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseLargeStack<'info> {
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

    pub token_program: Program<'info, Token>,
}
