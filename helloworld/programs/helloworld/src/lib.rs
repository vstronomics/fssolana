use anchor_lang::prelude::*;

declare_id!("69tPWhtyBoHrDZnwaQwSHR43NizSNtFBXKrkXnXga3T1");

#[program]
mod helloworld {
    use super::*;
    // RPC Handler
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }
    
    // RPC Handler
    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// Request object defining expected parameters
#[derive(Accounts)]
pub struct Create<'info> {
    // #[account(...)] defines constraints and instructions; if these do not hold, instructions will execute
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Request object defining expected parameters
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a Transaction Instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}
