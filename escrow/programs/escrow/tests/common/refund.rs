use super::*;

#[allow(clippy::result_large_err)]
pub fn execute_refund(ctx: &mut TestContext, escrow_pda: &Pubkey) -> TransactionResult {
    let vault_pda = anchor_spl::associated_token::get_associated_token_address_with_program_id(
        escrow_pda,
        &ctx.mint_a,
        &anchor_spl::token::ID,
    );

    let instruction_data = anchor_discriminator("refund").to_vec();

    let accounts = vec![
        AccountMeta::new(ctx.maker.pubkey(), true),
        AccountMeta::new(*escrow_pda, false),
        AccountMeta::new_readonly(ctx.mint_a, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(ctx.maker_ata_a, false),
        AccountMeta::new_readonly(anchor_lang::solana_program::system_program::ID, false),
        AccountMeta::new_readonly(anchor_spl::associated_token::ID, false),
        AccountMeta::new_readonly(anchor_spl::token::ID, false),
    ];

    let instruction = Instruction {
        program_id: ctx.program_id,
        accounts,
        data: instruction_data,
    };

    send_tx_result(&mut ctx.svm, &[instruction], &ctx.maker)
}
