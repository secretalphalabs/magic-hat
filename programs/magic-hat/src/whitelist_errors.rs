use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistErrorCode {
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Can't decerease as the count is more than number of available spots.")]
    InvalidNumberofWL,
    #[msg("WLType is invalid.")]
    InvalidWLType,
    #[msg("WL1 not scheduled.")]
    WL1NotScheduled,
    #[msg("WL2 not scheduled.")]
    WL2NotScheduled,
    #[msg("WL3 not scheduled.")]
    WL3NotScheduled,
    #[msg("WL4 not scheduled.")]
    WL4NotScheduled,
}
