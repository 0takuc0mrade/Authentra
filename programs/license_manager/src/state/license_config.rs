use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LicenseConfig {
    pub authority: Pubkey,
    pub default_fee: u64,
    pub platform_fee_bps: u16,
    pub bump: u8,
}

impl LicenseConfig {
    pub const SEED_PREFIX: &'static [u8] = b"license-config";

    pub const SIZE: usize = 8 + 32 + 8 + 2 + 1;

    pub fn init(&mut self, authority: Pubkey, bump: u8) -> Result<()> {
        self.authority = authority;
        self.default_fee = 1_000_000;
        self.platform_fee_bps = 300;
        self.bump = bump;
        Ok(())
    }
}
