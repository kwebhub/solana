use super::*;

#[derive(AnchorSerialize)]
struct MakeArgs {
    seed: u64,
    receive: u64,
    amount: u64,
}

pub fn execute_make(
    ctx: &mut TestContext,
    seed: u64,
    receive_amount: u64,
    deposit_amount: u64,
) -> (TransactionResult, Pubkey, Pubkey) {
    let (escrow_pda, _) = Pubkey::find_program_address(
        &[
            ESCROW_SEED,
            ctx.maker.pubkey().as_ref(),
            &seed.to_le_bytes(),
        ],
        &ctx.program_id,
    );

    let vault_pda = anchor_spl::associated_token::get_associated_token_address_with_program_id(
        &escrow_pda,
        &ctx.mint_a,
        &anchor_spl::token::ID,
    );

    let mut instruction_data = anchor_discriminator("make").to_vec();
    let args = MakeArgs {
        seed,
        receive: receive_amount,
        amount: deposit_amount,
    };
    args.serialize(&mut instruction_data).unwrap();

    let accounts = vec![
        AccountMeta::new(ctx.maker.pubkey(), true),
        AccountMeta::new(escrow_pda, false),
        AccountMeta::new_readonly(ctx.mint_a, false),
        AccountMeta::new_readonly(ctx.mint_b, false),
        AccountMeta::new(ctx.maker_ata_a, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(anchor_lang::solana_program::system_program::ID, false),
        AccountMeta::new_readonly(anchor_spl::associated_token::ID, false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
    ];

    let instruction = Instruction {
        program_id: ctx.program_id,
        accounts,
        data: instruction_data,
    };

    let result = send_tx_result(&mut ctx.svm, &[instruction], &ctx.maker);
    (result, escrow_pda, vault_pda)
}
