use anchor_lang::prelude::*;

declare_id!("YDmDZLkm7q8L3cBNRuAYqL4XvLQ2FQPHgLE5MzJQwhb");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn destake(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize {}
