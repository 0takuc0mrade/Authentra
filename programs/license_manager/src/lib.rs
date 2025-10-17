#![allow(unexpected_cfgs)]

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("46768WTgs123tGwaS1XJ4dVd8eSUheh3rSvzkuBVM4y5");

#[program]
pub mod license_manager {
    use super::*;

    pub fn initialize_license_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::initialize_config::initialize_license_config_handler(ctx)
    }

    // Dont forget to use cargo build-sbf to build instead of just cargo build
    pub fn purchase_license(ctx: Context<PurchaseLicense>) -> Result<()> {
        instructions::purchase_license::purchase_license_handler(ctx)
    }

    pub fn verify_license(ctx: Context<VerifyLicense>) -> Result<()> {
        instructions::verify_license::verify_license_handler(ctx)
    }
}
