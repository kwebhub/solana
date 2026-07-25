pub mod candidate;
pub mod poll;
pub mod vote;

use anchor_lang::prelude::*;
use litesvm::{types::TransactionResult, LiteSVM};
use sha2::{Digest, Sha256};
use solana_keypair::{read_keypair_file, Keypair};
use solana_message::{Instruction, Message};
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::{path::PathBuf, sync::LazyLock};
use voting::POLL_SEED;

static DEPLOY_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if path.to_string_lossy().contains("programs") {
        path.pop();
        path.pop();
    }
    path.push("target/deploy");
    path
});

fn get_program_id() -> Pubkey {
    let keypair = DEPLOY_DIR.join("voting-keypair.json");
    let program_keypair = read_keypair_file(&keypair)
        .unwrap_or_else(|_| panic!("Файл ключей не найден: ,{:?}", keypair));
    Pubkey::from(program_keypair.pubkey().to_bytes())
}

pub fn setup_svm() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = get_program_id();
    let payer = Keypair::new();
    let so_path = DEPLOY_DIR.join("voting.so");
    svm.add_program_from_file(program_id, &so_path)
        .unwrap_or_else(|_| panic!("Файл программы не найден"));
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, program_id, payer)
}

pub fn anchor_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", name));
    let res = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&res[..8]);
    discriminator
}

pub fn warp_forward_seconds(svm: &mut LiteSVM, seconds: u64) {
    let clock: Clock = svm.get_sysvar();
    let slots_to_add = (seconds as f64 / 0.4) as u64;
    let target_slot = clock.slot + slots_to_add;
    svm.warp_to_slot(target_slot);
}

pub fn send_tx(svm: &mut LiteSVM, instructions: &[Instruction], payer: &Keypair) {
    let msg = Message::new(instructions, Some(&payer.pubkey()));
    let tx = Transaction::new(&[payer], msg, svm.latest_blockhash());
    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "Трансакция отменена: ,{:?}", res.is_err());
}

#[allow(clippy::result_large_err)]
pub fn send_tx_result(
    svm: &mut LiteSVM,
    instructions: &[Instruction],
    payer: &Keypair,
) -> TransactionResult {
    let msg = Message::new(instructions, Some(&payer.pubkey()));
    let tx = Transaction::new(&[payer], msg, svm.latest_blockhash());
    svm.send_transaction(tx)
}
