use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RegisterContent<'info> {
    #[account(init, payer = user, space = 8 + ContentRecord::LEN)]
    pub content_account: Account<'info, ContentRecord>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterContent>, metadata_uri: String) -> Result<()> {
    let content = &mut ctx.accounts.content_account;
    content.owner = ctx.accounts.user.key();
    content.metadata_uri = metadata_uri;
    content.timestamp = Clock::get()?.unix_timestamp;
    content.verified = false;

    msg!("✅ Content registered by {:?}", content.owner);
    Ok(())
}
