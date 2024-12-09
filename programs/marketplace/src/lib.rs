use anchor_lang::prelude::*;

declare_id!("6ajpuB3vzhC8kUWfysmiTe8bqN2DnAYiwz6rUMWoFPjR");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
