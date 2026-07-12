use super::*;
use crate::common::{make_poll_ix, send_tx, setup_svm};

#[test]
fn test_initialize_poll() {
    let (mut svm, program_id, payer) = setup_svm();

    let poll_ix = make_poll_ix(
        program_id,
        payer.pubkey(),
        POLL_ID,
        POLL_NAME,
        POLL_DESC,
        0,
        1893456000,
    );

    send_tx(&mut svm, &[poll_ix], &payer);
}
