use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut, signer)]
    pub creator: AccountInfo<'info>,

    #[account(
        mut,
        has_one = creator,
        seeds = [b"content", creator.key.as_ref(), &content_record.content_hash],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
}

pub fn handler(
    ctx: Context<UpdateMetadata>,
    new_title: Option<String>,
    new_category: Option<String>,
    new_uri: Option<String>,
) -> Result<()> {
    let record = &mut ctx.accounts.content_record;

    if let Some(t) = new_title {
        record.title = t;
    }
    if let Some(c) = new_category {
        record.category = c;
    }
    if let Some(u) = new_uri {
        record.uri = u;
    }

    msg!("✅ Metadata updated by {:?}", record.creator);
    Ok(())
}
