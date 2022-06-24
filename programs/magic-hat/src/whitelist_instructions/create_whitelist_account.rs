use anchor_lang::prelude::*;
use crate::wallet_whitelist::*;
use crate::whitelist_config::*;
use crate::whitelist_errors::WhitelistErrorCode;

#[derive(Accounts)]
#[instruction( whitelist_type: String)]
pub struct CreateWhitelistAccount<'info> {
    #[account(init_if_needed, 
        payer = magic_hat_creator, 
        space = 8 + std::mem::size_of::<WalletWhitelist>(),
        seeds = [b"wallet-whitelist".as_ref(), whitelist_type.to_string().as_bytes(), whitelisted_address.key().as_ref()], 
        bump
    )]
    pub wallet_whitelist: Account<'info, WalletWhitelist>,
    #[account(mut, 
        has_one = magic_hat_creator,
    //     seeds = [b"whitelist-config".as_ref(), magic_hat_creator.key().as_ref()],
    //     bump = bump_config,
    )]
    whitelist_config: Account<'info, WhitelistConfig>,
    /// CHECK:
    pub whitelisted_address: AccountInfo<'info>,
    #[account(mut, address = whitelist_config.magic_hat_creator)]
    pub magic_hat_creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateWhitelistAccount>, whitelist_type: String) -> Result<()>{
    msg!(
        "whitelist_type{:?}",
        whitelist_type,
    );
    let wallet_whitelist = &mut ctx.accounts.wallet_whitelist;
    let whitelist_config = &mut ctx.accounts.whitelist_config;
    wallet_whitelist.whitelisted_address = ctx.accounts.whitelisted_address.key();
    wallet_whitelist.magic_hat_creator = ctx.accounts.magic_hat_creator.key();
    match whitelist_type.as_str() {
        "One" => {
            wallet_whitelist.whitelist_type = WLType::One;
            wallet_whitelist.number_of_whitelist_spots = 1;
            wallet_whitelist.special_discounted_price = whitelist_config.whitelist_schedule.wl_start_time_1.mint_price;
        }
        "Two" => {
            wallet_whitelist.whitelist_type = WLType::Two;
            wallet_whitelist.number_of_whitelist_spots = 2;
            wallet_whitelist.special_discounted_price = whitelist_config.whitelist_schedule.wl_start_time_2.mint_price;
        }
        "Three" => {
            wallet_whitelist.whitelist_type = WLType::Three;
            wallet_whitelist.number_of_whitelist_spots = 3;
            wallet_whitelist.special_discounted_price = whitelist_config.whitelist_schedule.wl_start_time_3.mint_price;
        }
    _ => return Err(error!(WhitelistErrorCode::InvalidWLType)),
    }
    wallet_whitelist.bump = *ctx.bumps.get("wallet_whitelist_account").unwrap();
    Ok(())
}
