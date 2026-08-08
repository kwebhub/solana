pub mod common;
use anchor_lang::solana_program::clock::Clock;
use common::*;
use solana_keypair::Keypair;
use solana_signer::Signer;
use voting::{CandidateAcc, PollAcc};

#[path = "test_voting/test_poll.rs"]
mod test_poll;

#[path = "test_voting/test_candidate.rs"]
mod test_candidate;

#[path = "test_voting/test_vote.rs"]
mod test_vote;

const POLL_ID: u64 = 1;
const POLL_NAME: &str = "Best Framework";
const POLL_DESC: &str = "Choose your favorite";
const CANDIDATE: &str = "Candidate 1";
const START_TIME: u64 = 0;
const END_TIME: u64 = 1893456000;

pub(crate) fn default_poll_data(
    start_time: Option<u64>,
    end_time: Option<u64>,
) -> voting::instruction::Poll {
    voting::instruction::Poll {
        poll_id: POLL_ID,
        name: POLL_NAME.to_string(),
        desc: POLL_DESC.to_string(),
        start_time: start_time.unwrap_or(START_TIME),
        end_time: end_time.unwrap_or(END_TIME),
    }
}
pub(crate) fn default_cand_data() -> voting::instruction::Candidate {
    voting::instruction::Candidate {
        poll_id: POLL_ID,
        candidate: CANDIDATE.to_string(),
    }
}
pub(crate) fn default_vote_data() -> voting::instruction::Vote {
    voting::instruction::Vote {
        poll_id: POLL_ID,
        candidate: CANDIDATE.to_string(),
    }
}
