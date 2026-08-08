use super::*;

pub fn create_cand_ix(
    program_id: Pubkey,
    payer: &Keypair,
    poll_pda: Pubkey,
    candidate_pda: Pubkey,
    cand_data: voting::instruction::Candidate,
) -> Instruction {
    let cand_accs = voting::accounts::InitCandidate {
        payer: payer.pubkey(),
        poll_account: poll_pda,
        candidate_account: candidate_pda,
        system_program: system_program::ID,
    };
    create_ix(program_id, &cand_data, &cand_accs)
}
