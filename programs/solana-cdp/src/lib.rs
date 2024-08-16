use anchor_lang::prelude::*;

declare_id!("2oH36vZv8NbKeuWfBG1NCeLyJYHZhvfbqogKCxhTUbmr");

#[program]
pub mod solana_cdp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
