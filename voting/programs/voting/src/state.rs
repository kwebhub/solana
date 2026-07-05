use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Poll {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}
/*
* структура аккаунта голосования:
* вычислить размер
* определить поля:
* pub poll_name: String,
* pub poll_description: String,
* pub poll_voting_start: u64,
* pub poll_voting_end: u64,
* pub poll_option_index: u64,
*/

/*
* структура аккаунта кандидата:
* вычислить размер
* определить поля:
* pub candidate_name: String,
* pub candidate_votes: u64,
*/
