use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct RegisterContent<'info> {
    #[account(mut, signer)]
    pub creator: AccountInfo<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + ContentRecord::MAX_SIZE,
        seeds = [b"content", creator.key.as_ref(), &content_hash],
        bump
    )]
    pub content_record: Account<'info, ContentRecord>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterContent>,
    content_hash: [u8; 32],
    uri: String,
    title: String,
    category: String,
) -> Result<()> {
    let record = &mut ctx.accounts.content_record;

    record.creator = *ctx.accounts.creator.key;
    record.content_hash = content_hash;
    record.uri = uri;
    record.title = title;
    record.category = category;
    record.registered_at = Clock::get()?.unix_timestamp;
    record.bump = *ctx.bumps.get("content_record").unwrap();

    msg!("✅ Content registered by {:?}", record.creator);
    Ok(())
}
