use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistErrorCode {
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Can't decerease as the count is more than number of available spots.")]
    InvalidNumberofWL,
    #[msg("WLType is invalid.")]
    InvalidWLType,
}
