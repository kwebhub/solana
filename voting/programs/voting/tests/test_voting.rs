pub mod common;
#[path = "test_voting/poll.rs"]
mod poll;

#[path = "test_voting/candidate.rs"]
mod candidate;

#[path = "test_voting/vote.rs"]
mod vote;
pub use solana_signer::Signer;

pub const POLL_ID: u64 = 1;
pub const POLL_NAME: &str = "Best Framework";
pub const POLL_DESC: &str = "Choose your favorite";
pub const CANDIDATE_NAME: &str = "Candidate 1";
