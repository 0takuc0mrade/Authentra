use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct VerifyOwnership<'info> {
    #[account(mut)]
    pub content_account: Account<'info, ContentRecord>,
    pub verifier: Signer<'info>,
}

pub fn handler(ctx: Context<VerifyOwnership>) -> Result<()> {
    let content = &mut ctx.accounts.content_account;
    content.verified = true;
    msg!("🔒 Ownership verified for {:?}", content.owner);
    Ok(())
}
