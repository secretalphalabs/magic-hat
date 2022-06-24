use anchor_lang::prelude::*;
use crate::wallet_whitelist::*;
use crate::state::MagicHat;
use crate::whitelist_errors::WhitelistErrorCode;

#[derive(Accounts)]
#[instruction( whitelist_type: String)]
pub struct CreateWhitelistAccount<'info> {
    #[account(init_if_needed, 
        payer = authority, 
        space = 8 + std::mem::size_of::<WalletWhitelist>(),
        seeds = [b"wallet-whitelist".as_ref(), whitelist_type.to_string().as_bytes(), whitelisted_address.key().as_ref(), magic_hat.key().as_ref()], 
        bump
    )]
    pub wallet_whitelist_account: Account<'info, WalletWhitelist>,
    #[account(mut, has_one = authority, constraint = magic_hat.to_account_info().owner == program_id)]
    pub magic_hat: Box<Account<'info, MagicHat>>,
    /// CHECK:
    pub whitelisted_address: AccountInfo<'info>,
    #[account(mut, address = magic_hat.authority)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateWhitelistAccount>, whitelist_type: String ) -> Result<()>{
    let wallet_whitelist_account = &mut ctx.accounts.wallet_whitelist_account;
    wallet_whitelist_account.magic_hat = ctx.accounts.magic_hat.key();
    wallet_whitelist_account.whitelisted_address = ctx.accounts.whitelisted_address.key();
    match whitelist_type.as_str() {
        "One" => {
            wallet_whitelist_account.whitelist_type = WLType::One;
            wallet_whitelist_account.number_of_whitelist_spots = 1;
        }
        "Two" => {
            wallet_whitelist_account.whitelist_type = WLType::Two;
            wallet_whitelist_account.number_of_whitelist_spots = 2;
        }
        "Three" => {
            wallet_whitelist_account.whitelist_type = WLType::Three;
            wallet_whitelist_account.number_of_whitelist_spots = 3;
        }
        "Four" => {
            wallet_whitelist_account.whitelist_type = WLType::Four;
            wallet_whitelist_account.number_of_whitelist_spots = 4;
        }
    _ => return Err(error!(WhitelistErrorCode::InvalidWLType)),
    }
    wallet_whitelist_account.bump = *ctx.bumps.get("wallet_whitelist_account").unwrap();
    Ok(())
}
