pub mod common;
use common::*;

use crate::common::{
    candidate::{fetch_cand_account, make_cand_ix},
    poll::{fetch_poll_account, make_poll_ix},
    vote::make_vote_ix,
};
use anchor_lang::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;
use voting::POLL_SEED;

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
