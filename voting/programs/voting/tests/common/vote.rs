use super::*;

pub fn create_vote_ix(
    program_id: Pubkey,
    voter: &Keypair,
    poll_pda: Pubkey,
    candidate_pda: Pubkey,
    vote_data: voting::instruction::Vote,
) -> anchor_lang::solana_program::instruction::Instruction {
    let vote_accs = voting::accounts::InitVote {
        payer: voter.pubkey(),
        poll_account: poll_pda,
        candidate_account: candidate_pda,
    };

    create_ix(program_id, &vote_data, &vote_accs)
}
