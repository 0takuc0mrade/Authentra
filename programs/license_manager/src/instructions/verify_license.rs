use crate::{errors::LicenseError, state::ActiveLicense};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyLicense<'info> {
    pub licensee: Signer<'info>,

    pub mint_manager: AccountInfo<'info>,

    #[account(
        seeds = [
            ActiveLicense::SEED_PREFIX,
            mint_manager.key().as_ref(),
            licensee.key().as_ref()
        ],
        bump = active_license.bump,
        constraint = active_license.is_active @ LicenseError::LicenseInactive
    )]
    pub active_license: Account<'info, ActiveLicense>,
}

pub fn verify_license_handler(ctx: Context<VerifyLicense>) -> Result<()> {
    let active_license = &ctx.accounts.active_license;

    require!(
        active_license.licensee == ctx.accounts.licensee.key(),
        LicenseError::InvalidLicensee
    );

    require!(
        active_license.mint_manager == ctx.accounts.mint_manager.key(),
        LicenseError::VerificationFailed
    );

    emit!(LicenseVerified {
        mint_manager: ctx.accounts.mint_manager.key(),
        licensee: ctx.accounts.licensee.key(),
        verification_date: Clock::get()?.unix_timestamp,
        license_purchase_date: active_license.purchase_date,
    });

    msg!("License verification successful");
    msg!("Mint Manager: {}", ctx.accounts.mint_manager.key());
    msg!("Licensee: {}", ctx.accounts.licensee.key());
    msg!("Purchased: {}", active_license.purchase_date);
    msg!("Active: {}", active_license.is_active);

    Ok(())
}

#[event]
pub struct LicenseVerified {
    pub mint_manager: Pubkey,
    pub licensee: Pubkey,
    pub verification_date: i64,
    pub license_purchase_date: i64,
}
