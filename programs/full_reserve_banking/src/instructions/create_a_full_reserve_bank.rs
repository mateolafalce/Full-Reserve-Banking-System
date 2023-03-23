use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn create_a_full_reserve_bank(
    ctx: Context<CreateAFullReserveBank>,
    official_name_of_the_bank: String,
    is_a_private_bank: bool,
    day_the_shares_go_on_sale: i64,
    day_the_shares_close: i64,
) -> Result<()> {
    require!(official_name_of_the_bank.len() <= 32, ErrorCode::LenghtError);
    let full_reserve_bank: &mut Account<FullReserveBankData> = &mut ctx.accounts.new_full_reserve_bank;
    let ipo_bank_data: &mut Account<IPOData> = &mut ctx.accounts.ipo_data;
    let (full_reserve_bank_pda, full_reserve_bank_bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        official_name_of_the_bank.as_bytes().as_ref()
        ], ctx.program_id);
    let (_ipo_pda, ipo_bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        full_reserve_bank_pda.key().to_bytes().as_ref()
        ], ctx.program_id);
    full_reserve_bank.bump_original = full_reserve_bank_bump;
    full_reserve_bank.authority = ctx.accounts.signer.key();
    full_reserve_bank.credits_waiting = [].to_vec();
    full_reserve_bank.is_private = is_a_private_bank;
    full_reserve_bank.active_users = 0;
    ipo_bank_data.day_the_shares_go_on_sale = day_the_shares_go_on_sale;
    ipo_bank_data.day_the_shares_close = day_the_shares_close;
    ipo_bank_data.bump_original = ipo_bump;
    ipo_bank_data.bank_shareholders = [].to_vec();
    Ok(())
}

#[derive(Accounts)]
#[instruction(official_name_of_the_bank: String)]
pub struct CreateAFullReserveBank<'info> {
    #[account(
        init,
        seeds = [
            official_name_of_the_bank.as_bytes().as_ref()
        ],
        bump,
        payer = signer,
        space = FullReserveBankData::SIZE + 8
    )]
    pub new_full_reserve_bank: Account<'info, FullReserveBankData>,
    #[account(
        init,
        seeds = [
            new_full_reserve_bank.key().to_bytes().as_ref(),
        ],
        bump,
        payer = signer,
        space = IPOData::SIZE + 8
    )]
    pub ipo_data: Account<'info, IPOData>,
    /// CHECK: This the signer
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
