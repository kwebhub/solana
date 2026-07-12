use super::*;
use crate::common::{make_candidate_ix, make_poll_ix, send_tx, setup_svm};

#[test]
fn test_initialize_candidate() {
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

    let cand_ix = make_candidate_ix(program_id, payer.pubkey(), POLL_ID, CANDIDATE_NAME);
    send_tx(&mut svm, &[cand_ix], &payer);
}
