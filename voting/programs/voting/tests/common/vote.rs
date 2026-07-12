use super::*;

pub fn make_vote_ix(
    program_id: Pubkey,
    payer: Pubkey,
    poll_id: u64,
    candidate_name: &str,
) -> Instruction {
    let (poll_pda, _) =
        Pubkey::find_program_address(&[POLL_SEED, &poll_id.to_le_bytes()], &program_id);
    let (candidate_pda, _) = Pubkey::find_program_address(
        &[&poll_id.to_le_bytes(), candidate_name.as_bytes()],
        &program_id,
    );

    let mut args = Vec::new();
    args.extend_from_slice(&poll_id.to_le_bytes());
    args.extend_from_slice(&(candidate_name.len() as u32).to_le_bytes());
    args.extend_from_slice(candidate_name.as_bytes());

    let mut data = anchor_discriminator("vote").to_vec();
    data.extend(args);

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new(candidate_pda, false),
        ],
        data,
    }
}
