use anchor_lang::{
    prelude::*,
    solana_program::{
        pubkey::Pubkey,
        program::invoke,
        system_instruction
    }
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn new_user_in_a_bank(
    ctx: Context<NewUserInABank>,
    credit_balance_to_account: u64
) -> Result<()> {
    require!(
        ctx.accounts.new_user_account_pda.key() == ctx.accounts.new_user_account.key(),
        ErrorCode::PubkeyError
    );
    invoke(
        &system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &ctx.accounts.new_user_account.key(),
            credit_balance_to_account
        ),
        &[
        ctx.accounts.signer.to_account_info(),
        ctx.accounts.new_user_account_pda.to_account_info().clone()
        ],
    ).expect("Error");
    let (_new_user_account_pda, new_user_account_bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        ctx.accounts.signer.key().to_bytes().as_ref(),
        ctx.accounts.full_reserve_bank.key().to_bytes().as_ref()
        ], ctx.program_id);
    let new_user_account: &mut Account<UserData> = &mut ctx.accounts.new_user_account;
    let full_reserve_bank: &mut Account<FullReserveBankData> = &mut ctx.accounts.full_reserve_bank;
    new_user_account.bump_original = new_user_account_bump;
    full_reserve_bank.active_users += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct NewUserInABank<'info> {
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
            signer.key().to_bytes().as_ref(),
            full_reserve_bank.key().to_bytes().as_ref(),
        ],
        bump,
        payer = signer,
        space = UserData::SIZE + 8
    )]
    pub new_user_account: Account<'info, UserData>,
    /// CHECK: This the signer
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub new_user_account_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
