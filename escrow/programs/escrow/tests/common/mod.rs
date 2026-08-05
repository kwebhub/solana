pub mod make;
pub mod refund;
pub mod take;

use anchor_lang::{prelude::*, AnchorSerialize};
use escrow::ESCROW_SEED;
use litesvm::{types::TransactionResult, LiteSVM};
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo};
use sha2::{Digest, Sha256};
use solana_keypair::{read_keypair_file, Keypair};
use solana_message::{Instruction, Message};
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::{path::PathBuf, sync::LazyLock};

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
    let keypair = DEPLOY_DIR.join("escrow-keypair.json");
    let program_keypair = read_keypair_file(&keypair)
        .unwrap_or_else(|_| panic!("Файл ключей не найден: ,{:?}", keypair));
    Pubkey::from(program_keypair.pubkey().to_bytes())
}

pub fn anchor_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", name));
    let res = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&res[..8]);
    discriminator
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

pub struct TestContext {
    pub svm: LiteSVM,
    pub program_id: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub maker: Keypair,
    pub maker_ata_a: Pubkey,
    pub maker_ata_b: Pubkey,
    pub taker: Keypair,
    pub taker_ata_a: Pubkey,
    pub taker_ata_b: Pubkey,
}

pub fn setup_svm() -> TestContext {
    let mut svm = LiteSVM::new();
    let program_id = get_program_id();
    let so_path = DEPLOY_DIR.join("escrow.so");
    svm.add_program_from_file(program_id, &so_path)
        .unwrap_or_else(|_| panic!("Файл программы не найден"));

    let mint_authority = Keypair::new();
    svm.airdrop(&mint_authority.pubkey(), 10_000_000_000)
        .unwrap();

    let mint_a = CreateMint::new(&mut svm, &mint_authority)
        .authority(&mint_authority.pubkey())
        .decimals(9)
        .send()
        .unwrap();

    let mint_b = CreateMint::new(&mut svm, &mint_authority)
        .authority(&mint_authority.pubkey())
        .decimals(9)
        .send()
        .unwrap();

    let maker = Keypair::new();
    svm.airdrop(&maker.pubkey(), 10_000_000_000).unwrap();

    let maker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &maker, &mint_a)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    let maker_ata_b = CreateAssociatedTokenAccount::new(&mut svm, &maker, &mint_b)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, 1_000_000_000)
        .owner(&mint_authority)
        .send()
        .unwrap();

    let taker = Keypair::new();
    svm.airdrop(&taker.pubkey(), 10_000_000_000).unwrap();

    let taker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &taker, &mint_a)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    let taker_ata_b = CreateAssociatedTokenAccount::new(&mut svm, &taker, &mint_b)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    MintTo::new(&mut svm, &taker, &mint_b, &taker_ata_b, 500_000_000)
        .owner(&mint_authority)
        .send()
        .unwrap();

    TestContext {
        svm,
        program_id,
        mint_a,
        mint_b,
        maker,
        maker_ata_a,
        maker_ata_b,
        taker,
        taker_ata_a,
        taker_ata_b,
    }
}
