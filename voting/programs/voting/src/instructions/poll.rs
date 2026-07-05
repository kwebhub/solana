use anchor_lang::prelude::*;

use crate::{constants::*, state::Poll};

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [POLL_SEED, poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, Poll>,
    system_program: Program<'info, System>,
}

pub fn handler_poll(
    ctx: Context<InitPoll>,
    _poll_id: u64,
    name: String,
    description: String,
    start_time: u64,
    end_time: u64,
) -> Result<()> {
    ctx.accounts.poll_account.poll_name = name;
    ctx.accounts.poll_account.poll_description = description;
    ctx.accounts.poll_account.poll_voting_start = start_time;
    ctx.accounts.poll_account.poll_voting_end = end_time;
    Ok(())
}

/*
* обработка аккаунтов с использованием poll_id
* структура контекста создания голосования, время жизни ссылок на данные:
*
* плательщик: подписант, ячейка памяти для плательщика,
* изменяемая для сохранения изменений баланса подписанта
*
* голосование: ячейка памяти для сохранения кода создания голосования
* команда создать если нужно, кто плательщик, сколько места,
* PDA с константой POLL_SEED и аргументом poll_id
* десериализация массива байтов аккаунта Poll из памяти блок-чейна в структуру Rust
* получить доступ к system_program
*/

// создание голосования с контекстом InitPoll
// _poll_id: u64, name: String, description: String, start_time: u64, end_time: u64,
// присвоить значение аргументов полям аккаунта Poll
