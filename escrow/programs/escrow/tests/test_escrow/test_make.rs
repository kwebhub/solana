use super::*;

#[test]
fn test_make_success() {
    let mut ctx = setup_svm();

    let initial_maker_balance = get_token_balance(&ctx, &ctx.maker_ata_a);

    let (result, escrow_pda, vault_pda) =
        execute_make(&mut ctx, TEST_SEED, RECEIVE_AMOUNT, DEPOSIT_AMOUNT);
    assert!(
        result.is_ok(),
        "Исполнение инструкции make завершилось ошибкой"
    );

    // 1. Проверяем баланс хранилища (vault)
    let vault_balance = get_token_balance(&ctx, &vault_pda);
    assert_eq!(vault_balance, DEPOSIT_AMOUNT, "Баланс vault некорректен");

    // 2. Проверяем баланс создателя (maker)
    let final_maker_balance = get_token_balance(&ctx, &ctx.maker_ata_a);
    assert_eq!(
        final_maker_balance,
        initial_maker_balance - DEPOSIT_AMOUNT,
        "Сумма не списалась с баланса maker"
    );

    // 3. Проверяем состояние созданного аккаунта Escrow
    let escrow_account_raw = ctx
        .svm
        .get_account(&escrow_pda)
        .expect("Аккаунт escrow не найден")
        .data;
    let escrow_data = Escrow::try_deserialize(&mut &escrow_account_raw[..])
        .expect("Ошибка десериализации Escrow");

    assert_eq!(escrow_data.seed, TEST_SEED);
    assert_eq!(escrow_data.maker, ctx.maker.pubkey());
    assert_eq!(escrow_data.mint_a, ctx.mint_a);
    assert_eq!(escrow_data.mint_b, ctx.mint_b);
    assert_eq!(escrow_data.receive, RECEIVE_AMOUNT);
}
