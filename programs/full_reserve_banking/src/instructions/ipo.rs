use anchor_lang::{
    prelude::*,
    solana_program::{
        pubkey::Pubkey,
        clock::Clock
    }
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn ipo(
    ctx: Context<Ipo>,
    capital: u64,
) -> Result<()> {
    require!(Clock::get().unwrap().unix_timestamp >= ctx.accounts.ipo_account.day_the_shares_go_on_sale, ErrorCode::TimestampError);
    require!(Clock::get().unwrap().unix_timestamp <= ctx.accounts.ipo_account.day_the_shares_close, ErrorCode::TimestampError);

    Ok(())
}

#[derive(Accounts)]
pub struct Ipo<'info> {
    #[account(
        mut,
        seeds = [
            full_reserve_bank.official_name_of_the_bank.as_bytes().as_ref()
        ],
        bump = full_reserve_bank.bump_original
    )]
    pub full_reserve_bank: Account<'info, FullReserveBankData>,
    #[account(
        mut,
        seeds = [
            full_reserve_bank.key().to_bytes().as_ref(),
        ],
        bump = ipo_account.bump_original,
    )]
    pub ipo_account: Account<'info, IPOData>,
    /// CHECK: This the signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
