use super::*;

#[test]
fn test_refund_success() {
    let mut ctx = setup_svm();

    let maker_ata_init = get_token_balance(&ctx, &ctx.maker_ata_a);

    let (make_res, escrow_pda, _vault_pda) =
        execute_make(&mut ctx, TEST_SEED, RECEIVE_AMOUNT, DEPOSIT_AMOUNT);
    assert!(
        make_res.is_ok(),
        "Не удалось открыть эскроу для теста refund"
    );

    let refund_res = execute_refund(&mut ctx, &escrow_pda);
    assert!(
        refund_res.is_ok(),
        "Исполнение инструкции refund завершилось ошибкой"
    );

    // 1. Проверяем возврат токенов
    let maker_ata_after = get_token_balance(&ctx, &ctx.maker_ata_a);
    assert_eq!(
        maker_ata_after, maker_ata_init,
        "Токены не вернулись на баланс Maker после отмены"
    );

    // 2. Проверяем уничтожение аккаунта Escrow
    let closed_escrow = ctx.svm.get_account(&escrow_pda);
    assert!(
        closed_escrow.is_none(),
        "Аккаунт Escrow не был уничтожен после refund"
    );
}
