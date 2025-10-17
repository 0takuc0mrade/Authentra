use crate::state::{ActiveLicense, LicenseConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PurchaseLicense<'info> {
    #[account(mut)]
    pub licensee: Signer<'info>,

    #[account(
        seeds = [LicenseConfig::SEED_PREFIX],
        bump = license_config.bump,
    )]
    pub license_config: Account<'info, LicenseConfig>,

    /// We just need to verify this MintManager exists and get its authority
    ///  this is supposed to be a proper account from creator-standard
    pub mint_manager: AccountInfo<'info>,

    #[account(
        init,
        payer = licensee,
        space = ActiveLicense::SIZE,
        seeds = [
            ActiveLicense::SEED_PREFIX,
            mint_manager.key().as_ref(),
            licensee.key().as_ref()
        ],
        bump
    )]
    pub active_license: Account<'info, ActiveLicense>,

    pub system_program: Program<'info, System>,
}

pub fn purchase_license_handler(ctx: Context<PurchaseLicense>) -> Result<()> {
    let license_config = &ctx.accounts.license_config;
    let active_license = &mut ctx.accounts.active_license;

    // Calculate fees
    let total_fee = license_config.default_fee;
    let platform_fee = total_fee * license_config.platform_fee_bps as u64 / 10000;
    let creator_fee = total_fee - platform_fee;

    // Simplified creator account for testing purposes
    let creator = ctx.accounts.mint_manager.key();

    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.licensee.key(),
        &creator,
        total_fee,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.licensee.to_account_info(),
            ctx.accounts.mint_manager.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    active_license.init(
        ctx.accounts.mint_manager.key(),
        ctx.accounts.licensee.key(),
        total_fee,
        ctx.bumps.active_license,
    )?;

    emit!(LicensePurchased {
        mint_manager: ctx.accounts.mint_manager.key(),
        licensee: ctx.accounts.licensee.key(),
        purchase_amount: total_fee,
        platform_fee,
        creator_fee,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("License purchased successfully");
    msg!("Mint Manager: {}", ctx.accounts.mint_manager.key());
    msg!("Licensee: {}", ctx.accounts.licensee.key());
    msg!("Total Fee: {} lamports", total_fee);
    msg!("Platform Fee: {} lamports", platform_fee);
    msg!("Creator Fee: {} lamports", creator_fee);

    Ok(())
}

#[event]
pub struct LicensePurchased {
    pub mint_manager: Pubkey,
    pub licensee: Pubkey,
    pub purchase_amount: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub timestamp: i64,
}
