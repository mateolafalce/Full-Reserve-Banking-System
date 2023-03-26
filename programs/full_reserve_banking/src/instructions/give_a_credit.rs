use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::{
    state::accounts::*,
    errors::ErrorCode
};

pub fn give_a_credit(
    ctx: Context<GiveACredit>,
) -> Result<()> {
    let credit_account: &mut Account<CreditData> = &mut ctx.accounts.credit;
    let user_data: &mut Account<UserData> = &mut ctx.accounts.user_data;
    require!(ctx.accounts.signer.key() == ctx.accounts.full_reserve_bank.authority.key(), ErrorCode::PubkeyError);
    require!(credit_account.approved == false, ErrorCode::ApprovedError);
    **ctx.accounts.full_reserve_bank.to_account_info().try_borrow_mut_lamports()? -= credit_account.capital;
    **ctx.accounts.borrower.to_account_info().try_borrow_mut_lamports()? += credit_account.capital;
    credit_account.approved = true;
    user_data.requested_credits += 1;
    user_data.money_amount_requested_from_banks = credit_account.capital;
    Ok(())
}

#[derive(Accounts)]
pub struct GiveACredit<'info> {
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub borrower: AccountInfo<'info>,
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
    )]
    pub credit: Account<'info, CreditData>,
    #[account(
        mut,
        seeds = [
            signer.key().to_bytes().as_ref(),
            full_reserve_bank.key().to_bytes().as_ref(),
        ],
        bump = user_data.bump_original,
    )]
    pub user_data: Account<'info, UserData>,
    /// CHECK: This the signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
