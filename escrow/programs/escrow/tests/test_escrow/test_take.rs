use super::*;

#[test]
fn test_take_success() {
    let mut ctx = setup_svm();

    let (make_res, escrow_pda, _vault_pda) =
        execute_make(&mut ctx, TEST_SEED, RECEIVE_AMOUNT, DEPOSIT_AMOUNT);
    assert!(make_res.is_ok(), "Не удалось подготовить эскроу через make");

    let taker_b_before = get_token_balance(&ctx, &ctx.taker_ata_b);
    let maker_b_before = get_token_balance(&ctx, &ctx.maker_ata_b);

    let take_res = execute_take(&mut ctx, &escrow_pda);
    assert!(
        take_res.is_ok(),
        "Исполнение инструкции take завершилось ошибкой"
    );

    // 1. Проверяем получение токенов Taker-ом
    let taker_a_after = get_token_balance(&ctx, &ctx.taker_ata_a);
    assert_eq!(
        taker_a_after, DEPOSIT_AMOUNT,
        "Taker не получил токены Mint A"
    );

    // 2. Проверяем списание токенов у Taker-а
    let taker_b_after = get_token_balance(&ctx, &ctx.taker_ata_b);
    assert_eq!(
        taker_b_after,
        taker_b_before - RECEIVE_AMOUNT,
        "С Taker списалась неверная сумма Mint B"
    );

    // 3. Проверяем получение токенов Maker-ом
    let maker_b_after = get_token_balance(&ctx, &ctx.maker_ata_b);
    assert_eq!(
        maker_b_after,
        maker_b_before + RECEIVE_AMOUNT,
        "Maker не получил токены Mint B"
    );

    // 4. Проверяем закрытие аккаунта Escrow
    let closed_escrow = ctx.svm.get_account(&escrow_pda);
    assert!(
        closed_escrow.is_none(),
        "Аккаунт Escrow не был закрыт после take"
    );
}
