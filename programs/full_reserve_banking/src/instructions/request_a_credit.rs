use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn request_a_credit(
    ctx: Context<RequestACredit>,
    interest_rate: u8,
    period_divided_into: u8,
    average_term_to_return_the_capital: i64,
) -> Result<()> {
    let (_credit_pda, credit_bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        &ctx.accounts.full_reserve_bank.key().to_bytes(),
        &ctx.accounts.full_reserve_bank.historical_loans.to_le_bytes()
        ], ctx.program_id);
    require!(ctx.accounts.signer.key() == ctx.accounts.full_reserve_bank.authority.key(), ErrorCode::PubkeyError);
    require!(average_term_to_return_the_capital >= 0, ErrorCode::LenghtError);
    let full_reserve_bank: &mut Account<FullReserveBankData> = &mut ctx.accounts.full_reserve_bank;
    full_reserve_bank.credits_waiting.push(ctx.accounts.new_credit.key());
    full_reserve_bank.historical_loans += 1;
    let credit: &mut Account<CreditData> = &mut ctx.accounts.new_credit;
    credit.bump_original = credit_bump;
    credit.borrower = ctx.accounts.signer.key();
    credit.interest_rate = interest_rate;
    credit.period_divided_into = period_divided_into;
    credit.average_term_to_return_the_capital = average_term_to_return_the_capital;
    Ok(())
}

#[derive(Accounts)]
pub struct RequestACredit<'info> {
    #[account(
        mut,
        seeds = [
            full_reserve_bank.official_name_of_the_bank.as_bytes().as_ref()
        ],
        bump = full_reserve_bank.bump_original
    )]
    pub full_reserve_bank: Account<'info, FullReserveBankData>,
    #[account(
        init,
        seeds = [
            full_reserve_bank.key().to_bytes().as_ref(),
            full_reserve_bank.historical_loans.to_be_bytes().as_ref(),
        ],
        bump,
        payer = signer,
        space = CreditData::SIZE + 8
    )]
    pub new_credit: Account<'info, CreditData>,
    #[account(
        mut,
        seeds = [
            signer.key().to_bytes().as_ref(),
            full_reserve_bank.key().to_bytes().as_ref(),
        ],
        bump = user_account.bump_original,
    )]
    pub user_account: Account<'info, IPOData>,
    /// CHECK: This the signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
