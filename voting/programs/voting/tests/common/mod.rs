pub mod candidate;
pub mod poll;
pub mod vote;
pub use candidate::*;
pub use poll::*;
pub use vote::*;

use {
    crate::{CANDIDATE, POLL_ID},
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{clock::Clock, instruction::Instruction},
        system_program, AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::{types::TransactionResult, LiteSVM},
    solana_keypair::Keypair,
    solana_message::{v0, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    voting::POLL_SEED,
};

pub fn setup_svm() -> (LiteSVM, Pubkey, Pubkey, Pubkey, Keypair) {
    let program_id = voting::id();
    let payer = Keypair::new();
    let poll_pda =
        Pubkey::find_program_address(&[POLL_SEED, &POLL_ID.to_le_bytes()], &program_id).0;
    let candidate_pda =
        Pubkey::find_program_address(&[&POLL_ID.to_le_bytes(), CANDIDATE.as_bytes()], &program_id)
            .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/voting.so"));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, program_id, poll_pda, candidate_pda, payer)
}
pub fn warp_forward_seconds(svm: &mut LiteSVM, seconds: u64) {
    let clock: Clock = svm.get_sysvar();
    let slots_to_add = (seconds as f64 / 0.4) as u64;
    let target_slot = clock.slot + slots_to_add;
    svm.warp_to_slot(target_slot);
}

pub fn create_ix<D: InstructionData, A: ToAccountMetas>(
    program_id: Pubkey,
    data: &D,
    accounts: &A,
) -> Instruction {
    Instruction::new_with_bytes(program_id, &data.data(), accounts.to_account_metas(None))
}

pub fn send_tx(svm: &mut LiteSVM, instructions: &[Instruction], payer: &Keypair) {
    let msg = v0::Message::try_compile(&payer.pubkey(), instructions, &[], svm.latest_blockhash())
        .expect("Failed to compile v0 message");
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[&payer])
        .expect("Failed to sign transaction");
    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "Трансакция отменена: ,{:?}", res.is_err());
}

#[allow(clippy::result_large_err)]
pub fn send_tx_result(
    svm: &mut LiteSVM,
    instructions: &[Instruction],
    payer: &Keypair,
) -> TransactionResult {
    let msg = v0::Message::try_compile(&payer.pubkey(), instructions, &[], svm.latest_blockhash())
        .expect("Failed to compile v0 message");
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[&payer])
        .expect("Failed to sign transaction");
    svm.send_transaction(tx)
}

pub fn get_state<T: AccountDeserialize>(svm: &LiteSVM, address: Pubkey) -> T {
    let account = svm.get_account(&address).expect("Аккаунт не найден");
    let mut data: &[u8] = &account.data;
    T::try_deserialize(&mut data).expect("Не удалось десериализовать аккаунт")
}
