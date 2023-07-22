use anchor_lang::prelude::*;

declare_id!("AnWA7pY1LvXqorCTmZHTA2trn71XG1JWLF1WkAXh7oWU");

#[program]
pub mod contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
