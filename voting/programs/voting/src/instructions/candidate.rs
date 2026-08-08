use super::*;

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [POLL_SEED, &poll_id.to_le_bytes()], bump)]
    pub poll_account: Account<'info, PollAcc>,
    #[account(init, payer = payer, space = CandidateAcc::DISCRIMINATOR.len() + CandidateAcc::INIT_SPACE, seeds = [&poll_id.to_le_bytes(), candidate.as_bytes()], bump)]
    pub candidate_account: Account<'info, CandidateAcc>,
    pub system_program: Program<'info, System>,
}

pub fn handler_cand(ctx: Context<InitCandidate>, _poll_id: u64, candidate: String) -> Result<()> {
    ctx.accounts.candidate_account.candidate_name = candidate;
    ctx.accounts.poll_account.poll_option_index += 1;
    Ok(())
}
