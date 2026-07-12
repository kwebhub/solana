use super::*;

/// Читает и десериализует аккаунт структуры Poll напрямую из LiteSVM
pub fn fetch_poll_account(svm: &LiteSVM, poll_pda: Pubkey) -> Poll {
    let account = svm.get_account(&poll_pda).expect("Аккаунт Poll не найден");
    let mut data_ref = &account.data[..];
    Poll::try_deserialize(&mut data_ref).expect("Не удалось десериализовать Poll")
}

pub fn make_poll_ix(
    program_id: Pubkey,
    payer: Pubkey,
    poll_id: u64,
    name: &str,
    desc: &str,
    start_time: u64,
    end_time: u64,
) -> Instruction {
    let (poll_pda, _) =
        Pubkey::find_program_address(&[POLL_SEED, &poll_id.to_le_bytes()], &program_id);

    let mut args = Vec::new();
    args.extend_from_slice(&poll_id.to_le_bytes());
    args.extend_from_slice(&(name.len() as u32).to_le_bytes());
    args.extend_from_slice(name.as_bytes());
    args.extend_from_slice(&(desc.len() as u32).to_le_bytes());
    args.extend_from_slice(desc.as_bytes());
    args.extend_from_slice(&start_time.to_le_bytes());
    args.extend_from_slice(&end_time.to_le_bytes());

    let mut data = anchor_discriminator("poll").to_vec();
    data.extend(args);

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(poll_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data,
    }
}
