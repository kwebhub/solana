use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}
// перечисление ошибок (VotingNotStarted, VotingEnded)
// и сообщений (Voting has not started yet, Voting has ended)
