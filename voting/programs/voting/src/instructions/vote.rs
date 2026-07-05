use crate::{
    constants::*,
    error::ErrorCode,
    state::{Candidate, Poll},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitVote<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [POLL_SEED, poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, Candidate>,
}

pub fn handler_vote(ctx: Context<InitVote>, _poll_id: u64, _candidate: String) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    if current_time > (ctx.accounts.poll_account.poll_voting_end as i64) {
        return Err(ErrorCode::VotingEnded.into());
    }
    if current_time < (ctx.accounts.poll_account.poll_voting_start as i64) {
        return Err(ErrorCode::VotingNotStarted.into());
    }

    ctx.accounts.candidate_account.candidate_votes += 1;
    Ok(())
}
/*
* обработка аккаунта
* использовать аргументы poll_id и candidate
* структура контекста проведения голосования:
*
* плательщик: изменяемая ячейка памяти для информации о плательщике
*
* голосование: изменяемый PDA с константой POLL_SEED и аргументом poll_id
* десериализация аккаунта Poll
*
* кандидат: изменяемый PDA с аргументами poll_id и candidate
* десериализация аккаунта Candidate
*/

/*
* создание процесса голосования с контекстом InitVote, _poll_id: u64, _candidate: String
* переменная current_time для текущего времени в unix_timestamp
* если текущее время больше времени окончания голосования, то ошибка VotingEnded
* если текущее время меньше времени начала голосования, то ошибка VotingNotStarted
* увеличить количество голосов кандидата на еденицу
*/
