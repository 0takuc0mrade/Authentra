use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ActiveLicense {
    pub mint_manager: Pubkey,
    pub licensee: Pubkey,
    pub purchase_amount: u64,
    pub purchase_date: i64,
    pub is_active: bool,
    pub bump: u8,
}

impl ActiveLicense {
    pub const SEED_PREFIX: &'static [u8] = b"active-license";

    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1;

    pub fn init(
        &mut self,
        mint_manager: Pubkey,
        licensee: Pubkey,
        purchase_amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.mint_manager = mint_manager;
        self.licensee = licensee;
        self.purchase_amount = purchase_amount;
        self.purchase_date = Clock::get()?.unix_timestamp;
        self.is_active = true;
        self.bump = bump;
        Ok(())
    }
}
