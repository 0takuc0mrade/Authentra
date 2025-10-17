use crate::state::LicenseConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = LicenseConfig::SIZE,
        seeds = [LicenseConfig::SEED_PREFIX],
        bump
    )]
    pub license_config: Account<'info, LicenseConfig>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_license_config_handler(ctx: Context<InitializeConfig>) -> Result<()> {
    let license_config = &mut ctx.accounts.license_config;

    license_config.init(ctx.accounts.authority.key(), ctx.bumps.license_config)?;

    emit!(LicenseConfigInitialized {
        authority: ctx.accounts.authority.key(),
        default_fee: license_config.default_fee,
        platform_fee_bps: license_config.platform_fee_bps,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("License config initialized successfully");
    msg!("Authority: {}", ctx.accounts.authority.key());
    msg!("Default Fee: {} lamports", license_config.default_fee);
    msg!("Platform Fee: {} bps", license_config.platform_fee_bps);
    msg!("PDA Bump: {}", license_config.bump);

    Ok(())
}

#[event]
pub struct LicenseConfigInitialized {
    pub authority: Pubkey,
    pub default_fee: u64,
    pub platform_fee_bps: u16,
    pub timestamp: i64,
}
