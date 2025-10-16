use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod utils;

use instructions::*;

declare_id!("HtmwUFTvDsLbwbSRDPTJwJqvMuxuChK9L1TJM1tEmM6T");

#[program]
pub mod registery {
    use super::*;

    pub fn register_content(
        ctx: Context<RegisterContent>,
        content_hash: [u8; 32],
        uri: String,
        title: String,
        category: String,
    ) -> Result<()> {
        register_content::handler(ctx, content_hash, uri, title, category)
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        new_title: Option<String>,
        new_category: Option<String>,
        new_uri: Option<String>,
    ) -> Result<()> {
        update_metadata::handler(ctx, new_title, new_category, new_uri)
    }

    pub fn verify_ownership(ctx: Context<VerifyOwnership>) -> Result<()> {
        verify_ownership::handler(ctx)
    }
}


