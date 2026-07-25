use super::*;

#[test]
fn test_initialize_cand() {
    let (mut svm, program_id, payer) = setup_svm();
    let poll_ix = make_poll_ix(
        program_id,
        payer.pubkey(),
        POLL_ID,
        POLL_NAME,
        POLL_DESC,
        START_TIME,
        END_TIME,
    );
    send_tx(&mut svm, &[poll_ix], &payer);
    let cand_ix = make_cand_ix(program_id, payer.pubkey(), POLL_ID, CANDIDATE);
    send_tx(&mut svm, &[cand_ix], &payer);

    let (poll_pda, _) =
        Pubkey::find_program_address(&[POLL_SEED, &POLL_ID.to_le_bytes()], &program_id);
    let poll_options_after = fetch_poll_account(&svm, poll_pda);
    assert_eq!(poll_options_after.poll_option_index, 1);
}
