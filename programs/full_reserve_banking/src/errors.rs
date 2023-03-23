use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The name of a banking institution based on a 100% reserve must be less than or equal to 32 characters")]
    LenghtError,
    #[msg("The two pubkeys compared are not the same")]
    PubkeyError,
    #[msg("There was an error in the timestamp. The action is not valid. Check the start and end dates of the offer")]
    TimestampError,
}
