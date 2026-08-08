use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PollAcc {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_desc: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAcc {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}
