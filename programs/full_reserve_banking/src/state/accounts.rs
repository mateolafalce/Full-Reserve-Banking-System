use anchor_lang::prelude::*;

#[account]
pub struct FullReserveBankData {
    pub official_name_of_the_bank: String,  // 4 + 32
    pub bump_original: u8,                  // 1
    pub authority: Pubkey,                  // 32
    pub active_users: u64,                  // 8
    pub is_private: bool,                   // 1
    pub capital: u64,                       // 8
    pub current_loans: u16,                 // 2
    pub historical_loans: u64,              // 8
    pub credits_waiting: Vec<Pubkey>        // 4 + (32 * 100)
}

impl FullReserveBankData {
    pub const SIZE: usize = 4 + 32 + 1 + 32 + 8 + 1 + 8 + 2 + 8 + 4 + (32 * 100);
}

#[account]
pub struct UserData {
    pub bump_original: u8,                              // 1
    pub requested_credits: u32,                         // 4
    pub money_returned: u32,                            // 4
    pub money_amount_requested_from_banks: u64,         // 8
    pub money_amount_given_to_banks: u64,               // 8
}

impl UserData {
    pub const SIZE: usize = 1 + 4 + 4 + 8 + 8;
}

#[account]
pub struct CreditData {
    pub bump_original: u8,                              // 1
    pub capital: u64,                                   // 8
    pub borrower: Pubkey,                               // 32
    pub interest_rate: u8,                              // 1
    pub period_divided_into: u8,                        // 1
    pub average_term_to_return_the_capital: i64,        // 8
    pub historical_loans: u64,                          // 8
    pub approved: bool,                                 // 1
}

impl CreditData {
    pub const SIZE: usize = 1 + 32 + 1 + 1 + 8 + 1;
}
