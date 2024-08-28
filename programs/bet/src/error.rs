use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("Cannot enter")]
    CannotEnter,
    #[msg("Cannot claim")]
    CannotClaim,
    #[msg("Cannot close")]
    CannotClose,
    #[msg("Pyth account key does not match")]
    InvalidPythKey,
    #[msg("Invalid Pyth account")]
    InvalidPythAccount,
    #[msg("Price is too big toparse to u32")]
    PriceTooBig,
}