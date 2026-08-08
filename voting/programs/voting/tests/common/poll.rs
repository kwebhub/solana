use super::*;

pub fn create_poll_ix(
    program_id: Pubkey,
    payer: &Keypair,
    poll_pda: Pubkey,
    poll_data: voting::instruction::Poll,
) -> Instruction {
    let poll_accs = voting::accounts::InitPoll {
        payer: payer.pubkey(),
        poll_account: poll_pda,
        system_program: system_program::ID,
    };
    create_ix(program_id, &poll_data, &poll_accs)
}
