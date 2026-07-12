pub mod candidate;
pub mod poll;
pub mod vote;

pub use anchor_lang::prelude::{system_program, AccountMeta, Pubkey};
pub use anchor_lang::AccountDeserialize;
pub use candidate::{fetch_candidate_account, make_candidate_ix};
pub use litesvm::{types::TransactionResult, LiteSVM};
pub use poll::{fetch_poll_account, make_poll_ix};
pub use solana_message::Instruction;
pub use vote::make_vote_ix;
pub use voting::constants::POLL_SEED;
pub use voting::{Candidate, Poll};

use anchor_lang::solana_program::clock::Clock;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static DEPLOY_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if path.to_string_lossy().contains("programs") {
        path.pop();
        path.pop();
    }
    path.push("target/deploy");
    path
});

pub fn get_program_id() -> Pubkey {
    let keypair_path = DEPLOY_DIR.join("voting-keypair.json");
    let program_keypair = solana_keypair::read_keypair_file(&keypair_path)
        .unwrap_or_else(|_| panic!("Не удалось найти файл ключей, {:?}", keypair_path));
    Pubkey::from(program_keypair.pubkey().to_bytes())
}

/// Вычисляет 8-байтовый дискриминатор Anchor для имени метода
pub fn anchor_discriminator(name: &str) -> [u8; 8] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", name).as_bytes());
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

/// Инициализирует окружение LiteSVM, загружает программу и пополняет баланс плательщика
pub fn setup_svm() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = get_program_id();
    let so_path = DEPLOY_DIR.join("voting.so");

    svm.add_program_from_file(program_id, &so_path)
        .unwrap_or_else(|_| panic!("Не удалось найти файл программы по пути: {:?}", so_path));

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    (svm, program_id, payer)
}

/// Перематывает время в LiteSVM вперед на заданное количество секунд через слоты
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
    assert!(res.is_ok(), "Transaction execution failed: {:?}", res.err());
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
