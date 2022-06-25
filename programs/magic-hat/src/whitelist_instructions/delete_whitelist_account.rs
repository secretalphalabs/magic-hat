use crate::wallet_whitelist::*;
use anchor_lang::prelude::*;
use common::close_account;

#[derive(Accounts)]
pub struct DeleteWhitelistAccount<'info> {
    #[account(mut, has_one = magic_hat_creator)]
    wallet_whitelist: Account<'info, WalletWhitelist>,
    #[account(mut)]
    magic_hat_creator: Signer<'info>,
}

pub fn handle_delete_whitelist_account(ctx: Context<DeleteWhitelistAccount>) -> Result<()> {
    close_account(
        &mut ctx.accounts.wallet_whitelist.to_account_info(),
        &mut ctx.accounts.magic_hat_creator.to_account_info(),
    )?;
    Ok(())
}
