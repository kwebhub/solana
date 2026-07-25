pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Dw1kZDi1UCjPrvt2NL49D7GtyQyLRapxhuPmW6Le7v5i");

#[program]
pub mod voting {
    use super::*;

    pub fn poll(
        ctx: Context<InitPoll>,
        poll_id: u64,
        name: String,
        desc: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        crate::handler_poll(ctx, poll_id, name, desc, start_time, end_time)
    }

    pub fn candidate(ctx: Context<InitCandidate>, poll_id: u64, candidate: String) -> Result<()> {
        crate::handler_cand(ctx, poll_id, candidate)
    }

    pub fn vote(ctx: Context<InitVote>, poll_id: u64, candidate: String) -> Result<()> {
        crate::handler_vote(ctx, poll_id, candidate)
    }
}
