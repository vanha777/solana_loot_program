use anchor_lang::prelude::*;

declare_id!("HfMV3T1CGFGTC4eJUhDfZUrigXQMWmVxFCxTdnSGemt7");

#[program]
pub mod game_sessions {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
