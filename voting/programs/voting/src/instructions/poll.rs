use super::*;

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = PollAcc::DISCRIMINATOR.len() + PollAcc::INIT_SPACE,
        seeds = [POLL_SEED, &poll_id.to_le_bytes()],
        bump
    )]
    pub poll_account: Account<'info, PollAcc>,
    pub system_program: Program<'info, System>,
}

pub fn handler_poll(
    ctx: Context<InitPoll>,
    _poll_id: u64,
    name: String,
    desc: String,
    start_time: u64,
    end_time: u64,
) -> Result<()> {
    ctx.accounts.poll_account.poll_name = name;
    ctx.accounts.poll_account.poll_desc = desc;
    ctx.accounts.poll_account.poll_voting_start = start_time;
    ctx.accounts.poll_account.poll_voting_end = end_time;
    Ok(())
}
