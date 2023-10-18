use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount}
};

declare_id!("YDmDZLkm7q8L3cBNRuAYqL4XvLQ2FQPHgLE5MzJQwhb");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn destake(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::VAULT_SEED],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = token_vault_account,
    )]
    pub token_vault_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::STAKE_INFO_SEED, signer.key.as_ref()],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = stake_info_account
    )]
    pub stake_info_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        [constants::TOKEN_SEED, signer.key.as_ref()],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = stake_account

    )]
    pub stake_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct StakeInfo {
    pub stake_at_slot: u64,
    pub is_staked: bool,

}