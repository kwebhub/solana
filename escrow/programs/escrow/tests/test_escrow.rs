pub mod common;

use anchor_lang::{prelude::Pubkey, AccountDeserialize};
use escrow::Escrow;
use litesvm_token::spl_token::state::Account as TokenAccount;
use solana_signer::Signer;

use common::{
    make::execute_make, refund::execute_refund, setup_svm, take::execute_take, TestContext,
};

#[path = "test_escrow/test_make.rs"]
mod test_make;

#[path = "test_escrow/test_take.rs"]
mod test_take;

#[path = "test_escrow/test_refund.rs"]
mod test_refund;

pub const TEST_SEED: u64 = 42;
pub const RECEIVE_AMOUNT: u64 = 200_000_000;
pub const DEPOSIT_AMOUNT: u64 = 500_000_000;

pub fn get_token_balance(ctx: &TestContext, ata: &Pubkey) -> u64 {
    let account_raw = ctx
        .svm
        .get_account(ata)
        .expect("Токенный аккаунт не найден")
        .data;
    let token_account =
        <TokenAccount as anchor_lang::solana_program::program_pack::Pack>::unpack_from_slice(
            &account_raw,
        )
        .expect("Ошибка распаковки токенного аккаунта");
    token_account.amount
}
