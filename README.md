# Blank

## counter

- state.rs: Counter (count, authority)
- error.rs: ErrorCode (Unauthorized, CounterOverflow)
- constants.rs: COUNTER_SEED, HELLO_WORLD_LAMPORTS, MAX_COUNT

- instructions/initialize.rs:
  - Initialize (payer, counter, system_program)
  - handle_initialize
    - count = 0, authority = payer.key()
    - cpi: system_program::Transfer (from: payer, to: counter)
    - system_program::transfer(cpi_ctx, HELLO_WORLD_LAMPORTS)?;

- instructions/increment.rs:
  - Increment (counter, authority)
  - handle_increment
    - require_keys_eq: counter.authority, authority.key(), Unauthorized
    - require!: counter.count < MAX_COUNT, CounterOverflow
    - counter.count += 1

- tests/test_initialize.rs:
  - setup_svm
  - counter address
  - init_data
  - init_accs
  - inc_data
  - inc_accs
  - create_ix
    - initialize instruction
    - increment instruction
  - send_tx
    - initialize instruction
    - increment instruction
  - get_state
    - initialize instruction
    - increment instruction
