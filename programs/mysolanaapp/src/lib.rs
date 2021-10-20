use anchor_lang::prelude::*;

declare_id!("9WSTCnLnpovgyoyCEbhpzAfSLezPXmkS3F67rSUWAeam");

#[program]
mod mysolanaapp {
    use super::*;

    pub fn create(_ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(_ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}