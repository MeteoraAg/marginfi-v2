use crate::{prelude::*, state::marginfi_account::MarginfiAccount};
use anchor_lang::prelude::*;
use solana_program::sysvar::Sysvar;

#[derive(Accounts)]
pub struct MarginfiAccountInitialize<'info> {
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,

    #[account(
        init,
        payer = fee_payer,
        space = 8 + std::mem::size_of::<MarginfiAccount>()
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
