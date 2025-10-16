use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct VerifyOwnership<'info> {
    #[account(
        seeds = [b"content", content_record.creator.as_ref(), &content_record.content_hash],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
}

pub fn handler(ctx: Context<VerifyOwnership>) -> Result<()> {
    let record = &ctx.accounts.content_record;

    msg!("📄 Content verified:");
    msg!("Creator: {:?}", record.creator);
    msg!("URI: {}", record.uri);
    msg!("Registered at: {}", record.registered_at);

    Ok(())
}
