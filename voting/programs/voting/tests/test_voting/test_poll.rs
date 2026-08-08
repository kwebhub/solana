use super::*;

#[test]
fn test_initialize_poll() {
    let (mut svm, program_id, poll_pda, _candidate_pda, payer) = setup_svm();
    let poll_data = default_poll_data(None, None);
    let poll_ix = create_poll_ix(program_id, &payer, poll_pda, poll_data);

    send_tx(&mut svm, &[poll_ix], &payer);
}
