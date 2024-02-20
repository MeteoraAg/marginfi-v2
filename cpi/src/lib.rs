pub mod constants;
pub mod errors;
pub mod instructions;
pub mod macros;
pub mod prelude;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;
use prelude::*;

declare_id!("MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA");

#[program]
pub mod marginfi {
    use super::*;

    #[allow(unused_variables)]
    pub fn marginfi_account_initialize(ctx: Context<MarginfiAccountInitialize>) -> MarginfiResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn lending_account_deposit(
        ctx: Context<LendingAccountDeposit>,
        amount: u64,
    ) -> MarginfiResult {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn lending_account_withdraw(
        ctx: Context<LendingAccountWithdraw>,
        amount: u64,
        withdraw_all: Option<bool>,
    ) -> MarginfiResult {
        Ok(())
    }

    #[allow(unused_variables)]
    // Operational instructions
    pub fn lending_pool_accrue_bank_interest(
        ctx: Context<LendingPoolAccrueBankInterest>,
    ) -> MarginfiResult {
        Ok(())
    }
}
