use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("9itEZxf4oDGCmYFf1ngQbXsUzhPUCvopGVtMdGHKkHz6");

#[program]
pub mod full_reserve_banking {
    use super::*;
    pub fn create_a_full_reserve_bank(
        ctx: Context<CreateAFullReserveBank>,
        official_name_of_the_bank: String,
        is_a_private_bank: bool,
        day_the_shares_go_on_sale: i64,
        day_the_shares_close: i64,
    ) -> Result<()> {
        instructions::create_a_full_reserve_bank::create_a_full_reserve_bank(
            ctx,
            official_name_of_the_bank,
            is_a_private_bank,
            day_the_shares_go_on_sale,
            day_the_shares_close,
        )
    }
    pub fn new_user_in_a_bank(
        ctx: Context<NewUserInABank>,
        credit_balance_to_account: u64
    ) -> Result<()> {
        instructions::new_user_in_a_bank::new_user_in_a_bank(
            ctx,
            credit_balance_to_account
        )
    }
    pub fn request_a_credit(
        ctx: Context<RequestACredit>,
        interest_rate: u8,
        period_divided_into: u8,
        average_term_to_return_the_capital: i64,
    ) -> Result<()> {
        instructions::request_a_credit::request_a_credit(
            ctx,
            interest_rate,
            period_divided_into,
            average_term_to_return_the_capital,
        )
    }
}
