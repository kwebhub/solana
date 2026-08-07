use {
    anchor_lang::{
        prelude::Pubkey, solana_program::instruction::Instruction, AccountDeserialize,
        InstructionData, ToAccountMetas,
    },
    litesvm::{types::TransactionResult, LiteSVM},
    solana_keypair::Keypair,
    solana_message::{v0, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

pub fn setup_svm() -> (LiteSVM, Pubkey, Keypair) {
    let program_id = counter::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/counter.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    (svm, program_id, payer)
}

pub fn create_ix<D: InstructionData, A: ToAccountMetas>(
    program_id: Pubkey,
    data: &D,
    accounts: &A,
) -> Instruction {
    Instruction::new_with_bytes(program_id, &data.data(), accounts.to_account_metas(None))
}

#[allow(clippy::result_large_err)]
pub fn send_tx(
    svm: &mut LiteSVM,
    instruction: &[Instruction],
    payer: &Keypair,
) -> TransactionResult {
    let msg = v0::Message::try_compile(&payer.pubkey(), instruction, &[], svm.latest_blockhash())
        .expect("Failed to compile v0 message");
    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[&payer])
        .expect("Failed to sign transaction");

    svm.send_transaction(tx)
}

pub fn get_state<T: AccountDeserialize>(svm: &LiteSVM, address: Pubkey) -> T {
    let account = svm.get_account(&address).unwrap();
    let mut data: &[u8] = &account.data;
    T::try_deserialize(&mut data).unwrap()
}
