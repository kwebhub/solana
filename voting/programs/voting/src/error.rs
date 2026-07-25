use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Голосование окончено")]
    VotingEnded,
    #[msg("Голосование не начато")]
    VotingNotStarted,
}
