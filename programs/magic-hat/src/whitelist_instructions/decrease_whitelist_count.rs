use crate::wallet_whitelist::*;
use crate::whitelist_errors::WhitelistErrorCode;
use anchor_lang::prelude::*;
use common::*;
#[derive(Accounts)]
pub struct DecreaseWhitelistSpots<'info> {
    #[account(mut,
        has_one = whitelisted_address,
        // seeds = [b"wallet-whitelist", wallet_whitelist_account.whitelist_type.to_string().as_bytes() , whitelisted_address.key().as_ref(), magic_hat_id.key().as_ref()], 
        // bump = wallet_whitelist_account.bump
    )]
    pub wallet_whitelist_account: Box<Account<'info, WalletWhitelist>>,
    /// CHECK:
    #[account(constraint = wallet_whitelist_account.whitelisted_address == whitelisted_address.key())]
    pub whitelisted_address: AccountInfo<'info>,
    // pub whitelisted_address: Signer<'info>,
}

pub fn handler(ctx: Context<DecreaseWhitelistSpots>, count: u64) -> Result<()> {
    let wallet_whitelist_account = &mut ctx.accounts.wallet_whitelist_account;
    if count > wallet_whitelist_account.number_of_whitelist_spots {
        return Err(WhitelistErrorCode::InvalidNumberofWL.into());
    }
    wallet_whitelist_account
        .number_of_whitelist_spots
        .try_sub_assign(count)?;
    Ok(())
}
