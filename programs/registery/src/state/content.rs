use anchor_lang::prelude::*;

#[account]
pub struct ContentRecord {
    pub owner: Pubkey,
    pub metadata_uri: String,
    pub timestamp: i64,
    pub verified: bool,
}

impl ContentRecord {
    pub const LEN: usize = 32 + 4 + 200 + 8 + 1;
}
