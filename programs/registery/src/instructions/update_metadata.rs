use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut, has_one = owner)]
    pub content_account: Account<'info, ContentRecord>,

    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateMetadata>, new_uri: String) -> Result<()> {
    let content = &mut ctx.accounts.content_account;
    content.metadata_uri = new_uri;
    msg!("📝 Metadata updated for {:?}", content.owner);
    Ok(())
}
