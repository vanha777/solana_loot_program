use anchor_lang::prelude::*;

declare_id!("31AsjBvWTL93SSjVEHM8p3JqvuwiRuTh8P9grM7Tzs8L");

#[program]
pub mod nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
