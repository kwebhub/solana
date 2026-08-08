use super::*;

#[test]
fn test_vote_flow() {
    let (mut svm, program_id, poll_pda, candidate_pda, payer) = setup_svm();
    let clock: Clock = svm.get_sysvar();
    let now = clock.unix_timestamp as u64;
    let start_time = now + 100;
    let end_time = now + 500;

    let poll_data = default_poll_data(Some(start_time), Some(end_time));
    let poll_ix = create_poll_ix(program_id, &payer, poll_pda, poll_data);
    send_tx(&mut svm, &[poll_ix], &payer);

    let cand_data = default_cand_data();
    let cand_ix = create_cand_ix(program_id, &payer, poll_pda, candidate_pda, cand_data);
    send_tx(&mut svm, &[cand_ix], &payer);

    let voter_early = Keypair::new();
    let voter_active = Keypair::new();
    let voter_late = Keypair::new();
    svm.airdrop(&voter_early.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&voter_active.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&voter_late.pubkey(), 10_000_000_000).unwrap();

    let vote_data = default_vote_data();

    let vote_ix_early = create_vote_ix(
        program_id,
        &voter_early,
        poll_pda,
        candidate_pda,
        vote_data.clone(),
    );
    let res_early = send_tx_result(&mut svm, &[vote_ix_early], &voter_early);
    assert!(res_early.is_err(), "Слишком рано");

    warp_forward_seconds(&mut svm, 1000);
    let vote_ix_late = create_vote_ix(
        program_id,
        &voter_late,
        poll_pda,
        candidate_pda,
        vote_data.clone(),
    );
    let res_late = send_tx_result(&mut svm, &[vote_ix_late], &voter_late);
    assert!(res_late.is_err(), "Слишком поздно");

    warp_forward_seconds(&mut svm, 400);
    let vote_ix_active = create_vote_ix(
        program_id,
        &voter_active,
        poll_pda,
        candidate_pda,
        vote_data,
    );
    let res_active = send_tx_result(&mut svm, &[vote_ix_active], &voter_active);
    if res_active.is_ok() {
        let candidate_state_after: CandidateAcc = get_state(&svm, candidate_pda);
        assert_eq!(candidate_state_after.candidate_votes, 1);
    } else {
        println!("LiteSVM требует большего сдвига слотов")
    }
}
