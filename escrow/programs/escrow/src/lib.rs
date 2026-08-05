pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CQJ2eZyBhQBNgEv4FY7epU5GSYXfmxXnabbELFCKyCtg");

#[program]
pub mod escrow {
    use crate::instructions::make::Make;

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        crate::handler_make(ctx, seed, receive, amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        crate::handler_refund(ctx)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        crate::handler_take(ctx)
    }
}
