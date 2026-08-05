use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid Maker")]
    InvalidMaker,
    #[msg("Invalid Mint A")]
    InvalidMintA,
    #[msg("Invalid Mint B")]
    InvalidMintB,
    #[msg("Invalid Amount")]
    InvalidAmount,
}
