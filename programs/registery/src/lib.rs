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

      pub fn register_content(ctx: Context<RegisterContent>, metadata_uri: String) -> Result<()> {
        register_content::handler(ctx, metadata_uri)
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, new_uri: String) -> Result<()> {
        update_metadata::handler(ctx, new_uri)
    }

    pub fn verify_ownership(ctx: Context<VerifyOwnership>) -> Result<()> {
        verify_ownership::handler(ctx)
    }
}

