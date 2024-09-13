use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;
use crate::{constant::*, error::*, states::*};

declare_id!("GSdLaosgZz3jkC3Jou5hNwEBZY5ZLBf6z16UdbRL6NqQ");

#[program]
pub mod solana_todo_app {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;

        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, _content: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill contents with argument
        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = _content;
        todo_account.marked = false;

        // Increase todo idx for PDA
        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

        // Increase total todo count
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
