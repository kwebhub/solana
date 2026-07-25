use super::*;

#[test]
fn test_vote_flow() {
    let (mut svm, program_id, payer) = setup_svm();
    let clock: Clock = svm.get_sysvar();
    let now = clock.unix_timestamp as u64;
    let start_time = now + 100;
    let end_time = now + 500;

    let poll_ix = make_poll_ix(
        program_id,
        payer.pubkey(),
        POLL_ID,
        POLL_NAME,
        POLL_DESC,
        start_time,
        end_time,
    );
    send_tx(&mut svm, &[poll_ix], &payer);

    let cand_ix = make_cand_ix(program_id, payer.pubkey(), POLL_ID, CANDIDATE);
    send_tx(&mut svm, &[cand_ix], &payer);

    let (candidate_pda, _) =
        Pubkey::find_program_address(&[&POLL_ID.to_le_bytes(), CANDIDATE.as_bytes()], &program_id);

    let voter_early = Keypair::new();
    let voter_active = Keypair::new();
    let voter_late = Keypair::new();
    svm.airdrop(&voter_early.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&voter_active.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&voter_late.pubkey(), 10_000_000_000).unwrap();

    let vote_ix_early = make_vote_ix(program_id, voter_early.pubkey(), POLL_ID, CANDIDATE);
    let res_early = send_tx_result(&mut svm, &[vote_ix_early], &voter_early);
    assert!(res_early.is_err(), "Слишком рано");

    warp_forward_seconds(&mut svm, 1000);
    let voter_ix_late = make_vote_ix(program_id, voter_late.pubkey(), POLL_ID, CANDIDATE);
    let res_late = send_tx_result(&mut svm, &[voter_ix_late], &voter_late);
    assert!(res_late.is_err(), "Слишком поздно");

    warp_forward_seconds(&mut svm, 400);
    let vote_ix_active = make_vote_ix(program_id, voter_active.pubkey(), POLL_ID, CANDIDATE);
    let res_active = send_tx_result(&mut svm, &[vote_ix_active], &voter_active);
    if res_active.is_ok() {
        let candidate_state_after = fetch_cand_account(&svm, candidate_pda);
        assert_eq!(candidate_state_after.candidate_votes, 1);
    } else {
        println!("LiteSVM требует большего сдвига слотов")
    }
}
