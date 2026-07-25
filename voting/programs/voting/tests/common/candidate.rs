use super::*;
use voting::Candidate;

pub fn fetch_cand_account(svm: &LiteSVM, candidate_pda: Pubkey) -> Candidate {
    let account = svm
        .get_account(&candidate_pda)
        .expect("Аккаунт Candidate не найден");
    let mut data_ref = &account.data[..];
    Candidate::try_deserialize(&mut data_ref).expect("Не удалось десериализовать Candidate аккаунт")
}

pub fn make_cand_ix(
    program_id: Pubkey,
    payer: Pubkey,
    poll_id: u64,
    candidate: &str,
) -> Instruction {
    let (poll_pda, _) =
        Pubkey::find_program_address(&[POLL_SEED, &poll_id.to_le_bytes()], &program_id);
    let (candidate_pda, _) =
        Pubkey::find_program_address(&[&poll_id.to_le_bytes(), candidate.as_bytes()], &program_id);
    let mut args = Vec::new();
    args.extend_from_slice(&poll_id.to_le_bytes());
    args.extend_from_slice(&(candidate.len() as u32).to_le_bytes());
    args.extend_from_slice(candidate.as_bytes());

    let mut data = anchor_discriminator("candidate").to_vec();
    data.extend(args);

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new(candidate_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    }
}
