use anchor_lang::prelude::{system_program, Pubkey};
use counter::{constants::COUNTER_SEED, Counter};
use solana_signer::Signer;

mod common;
use common::{create_ix, get_state, send_tx, setup_svm};

#[test]
fn test_counter() {
    let (mut svm, program_id, payer) = setup_svm();
    let counter = Pubkey::find_program_address(&[COUNTER_SEED], &program_id).0;

    let init_data = counter::instruction::Initialize {};
    let init_accs = counter::accounts::Initialize {
        payer: payer.pubkey(),
        counter,
        system_program: system_program::ID,
    };

    let inc_data = counter::instruction::Increment {};
    let inc_accs = counter::accounts::Increment {
        counter,
        authority: payer.pubkey(),
    };

    let initialize = create_ix(program_id, &init_data, &init_accs);
    let increment = create_ix(program_id, &inc_data, &inc_accs);

    send_tx(&mut svm, &[initialize], &payer).unwrap();
    let counter_state: Counter = get_state(&svm, counter);
    assert_eq!(counter_state.count, 0);
    assert_eq!(counter_state.authority, payer.pubkey());

    send_tx(&mut svm, &[increment], &payer).unwrap();
    let counter_state: Counter = get_state(&svm, counter);
    assert_eq!(counter_state.count, 1);
    assert_eq!(counter_state.authority, payer.pubkey());
}
