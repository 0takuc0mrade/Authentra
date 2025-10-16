use anchor_lang::prelude::*;

#[account]
pub struct ContentRecord {
    pub creator: Pubkey,        // wallet of creator
    pub content_hash: [u8; 32], // hash of content file
    pub uri: String,            // IPFS/Arweave URL
    pub title: String,          // short title
    pub category: String,       // e.g. music, art, doc
    pub registered_at: i64,     // unix timestamp
    pub bump: u8,               // pda bump
}

impl ContentRecord {
    pub const MAX_SIZE: usize =
        32 +      // creator
        32 +      // hash
        4 + 200 + // uri (max 200 bytes)
        4 + 100 + // title
        4 + 50 +  // category
        8 +       // timestamp
        1;        // bump
}
