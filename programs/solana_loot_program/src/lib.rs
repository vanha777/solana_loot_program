use anchor_lang::prelude::*;

declare_id!("B1WrZv1rwvX38iCXKsW2Eh98G2oySWzfBssG9XjVhN1V");

#[program]
pub mod solana_loot_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
