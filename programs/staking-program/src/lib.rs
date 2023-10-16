use anchor_lang::prelude::*;

declare_id!("YDmDZLkm7q8L3cBNRuAYqL4XvLQ2FQPHgLE5MzJQwhb");

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
