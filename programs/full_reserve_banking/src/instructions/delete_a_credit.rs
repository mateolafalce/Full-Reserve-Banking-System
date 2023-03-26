use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn delete_a_credit(
    ctx: Context<DeleteACredit>,
) -> Result<()> {
    let lamport: u64 = ctx.accounts.credit.to_account_info().lamports() - 890880;
    require!(ctx.accounts.signer.key() == ctx.accounts.credit.borrower.key(), ErrorCode::PubkeyError);
    require!(ctx.accounts.credit.approved == false, ErrorCode::ApprovedError);
    **ctx.accounts.credit.to_account_info().try_borrow_mut_lamports()? -= lamport;
    **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += lamport;
    let full_reserve_bank: &mut Account<FullReserveBankData> = &mut ctx.accounts.full_reserve_bank;
    for i in 0..full_reserve_bank.credits_waiting.len() {
        if full_reserve_bank.credits_waiting[i] == ctx.accounts.credit.key() {
            full_reserve_bank.credits_waiting.remove(i);
            break;
        }
    }
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteACredit<'info> {
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
            credit.historical_loans.to_be_bytes().as_ref(),
        ],
        bump = credit.bump_original,
        close = signer
    )]
    pub credit: Account<'info, CreditData>,
    /// CHECK: This the signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
