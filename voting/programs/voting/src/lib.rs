pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FKD4Z7cBZpfTbLg47YSBrCw7DxVVivX4J4ouHRjryTz7");

#[program]
pub mod voting {
    use super::*;

    pub fn poll(
        ctx: Context<InitPoll>,
        poll_id: u64,
        name: String,
        description: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        crate::handler_poll(ctx, poll_id, name, description, start_time, end_time)
    }

    pub fn candidate(ctx: Context<InitCandidate>, poll_id: u64, candidate: String) -> Result<()> {
        crate::handle_candidate(ctx, poll_id, candidate)
    }

    pub fn vote(ctx: Context<InitVote>, poll_id: u64, candidate: String) -> Result<()> {
        crate::handler_vote(ctx, poll_id, candidate)
    }
}

/*
 * проброс аргументов для инструкции создания голосования
 * ctx: Context<InitPoll>, _poll_id: u64, name: String, description: String, start_time: u64, end_time: u64,
 */

/*
 * создать кандидата голосования: проброс агрументов для инструкции создания кандидата
 * ctx: Context<InitCandidate>, _poll_id: u64, candidate: String,
 */

/*
 * провести голосование: проброс аргументов для интсрукции проведения голосования
 * ctx: Context<InitVote>, poll_id: u64, candidate: String
 */
