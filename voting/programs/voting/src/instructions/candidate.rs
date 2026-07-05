use anchor_lang::prelude::*;

use crate::{
    constants::*,
    state::{Candidate, Poll},
};

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [POLL_SEED, poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, Poll>,
    #[account(
        init,
        payer = payer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

pub fn handle_candidate(
    ctx: Context<InitCandidate>,
    _poll_id: u64,
    candidate: String,
) -> Result<()> {
    ctx.accounts.candidate_account.candidate_name = candidate;
    ctx.accounts.poll_account.poll_option_index += 1;
    Ok(())
}
/*
* обработка аккаунтов с использованием poll_id и candidate
* структура контекста создания кандидата, время жизни ссылок на данные:
*
* плательщик: подписант, ячейка памяти для плательщика,
* изменяемая для сохранения изменений баланса подписанта
*
* голосование: ячейка памяти - изменяемый PDA с константой POLL_SEED и аргументом poll_id
* для валидации по seeds соответствующего голосования
* десериализация массива байтов аккаунта Poll из памяти блок-чейна в структуру Rust
*
* кандидат: ячейка памяти для сохранения кода создания кандидата
* команда создать, кто плательщик, сколько места,
* PDA с аргументами poll_id и candidate для связи голосования и кандидата
* десериализация массива байтов аккаунта Candidate из памяти блок-чейна в структуру Rust
* получить доступ к system_program
*/

// создание кандидата с контекстом InitCandidate (_poll_id: u64, candidate: String)
// присвоить значение аргумента candidate полю candidate_name аккаунта Candidate
// Увеличить счетчик общего количества вариантов/кандидатов в аккаунте голосования на 1.
