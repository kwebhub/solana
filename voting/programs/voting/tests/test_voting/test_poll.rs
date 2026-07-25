use super::*;

#[test]
fn test_initialize_poll() {
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
}
