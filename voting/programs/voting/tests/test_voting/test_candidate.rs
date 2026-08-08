use super::*;

#[test]
fn test_initialize_cand() {
    let (mut svm, program_id, poll_pda, candidate_pda, payer) = setup_svm();

    let poll_data = default_poll_data(None, None);
    let poll_ix = create_poll_ix(program_id, &payer, poll_pda, poll_data);
    send_tx(&mut svm, &[poll_ix], &payer);

    let cand_data = default_cand_data();
    let cand_ix = create_cand_ix(program_id, &payer, poll_pda, candidate_pda, cand_data);
    send_tx(&mut svm, &[cand_ix], &payer);

    let poll_options_after: PollAcc = get_state(&svm, poll_pda);
    assert_eq!(poll_options_after.poll_option_index, 1);

    let candidate_state: CandidateAcc = get_state(&svm, candidate_pda);
    assert_eq!(candidate_state.candidate_name, CANDIDATE);
    assert_eq!(candidate_state.candidate_votes, 0);
}
