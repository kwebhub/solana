use super::*;

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitVote<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [POLL_SEED, &poll_id.to_le_bytes()], bump)]
    pub poll_account: Account<'info, PollAcc>,
    #[account(mut, seeds = [&poll_id.to_le_bytes(), candidate.as_bytes()], bump)]
    pub candidate_account: Account<'info, CandidateAcc>,
}

pub fn handler_vote(ctx: Context<InitVote>, _poll_id: u64, _candidate: String) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time > ctx.accounts.poll_account.poll_voting_end {
        return Err(ErrorCode::VotingEnded.into());
    }
    if current_time < ctx.accounts.poll_account.poll_voting_start {
        return Err(ErrorCode::VotingNotStarted.into());
    }
    ctx.accounts.candidate_account.candidate_votes += 1;
    Ok(())
}
