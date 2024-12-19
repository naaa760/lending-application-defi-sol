use anchor_lang::prelude::*;

declare_id!("4Haj6HwbeWLTi3PFBMQG1Gf1DCJsf8FaVySUPXL6fvYv");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
