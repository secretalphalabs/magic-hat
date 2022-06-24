pub mod constants;
pub mod errors;
pub mod processor;
pub mod state;
pub mod utils;
pub mod wallet_whitelist;
pub mod whitelist_config;
pub mod whitelist_config_instructions;
pub mod whitelist_errors;
pub mod whitelist_instructions;

use anchor_lang::prelude::*;
pub use errors::MagicHatError;
pub use processor::*;
pub use state::*;
pub use utils::*;
pub use whitelist_instructions::*;
declare_id!("WUFFcLvSqCsof9z2R6EEcAbKCVonrGN7j8eASVVofQ3");

#[program]
pub mod magic_hat {
    use super::*;

    pub fn initialize_magic_hat(
        ctx: Context<InitializeMagicHat>,
        data: MagicHatData,
    ) -> Result<()> {
        handle_initialize_magic_hat(ctx, data)
    }

    pub fn update_magic_hat(ctx: Context<UpdateMagicHat>, data: MagicHatData) -> Result<()> {
        handle_update_magic_hat(ctx, data)
    }

    pub fn update_authority(
        ctx: Context<UpdateMagicHat>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        handle_update_authority(ctx, new_authority)
    }

    pub fn add_config_lines(
        ctx: Context<AddConfigLines>,
        index: u32,
        config_lines: Vec<ConfigLine>,
    ) -> Result<()> {
        handle_add_config_lines(ctx, index, config_lines)
    }

    pub fn set_collection(ctx: Context<SetCollection>) -> Result<()> {
        handle_set_collection(ctx)
    }

    pub fn remove_collection(ctx: Context<RemoveCollection>) -> Result<()> {
        handle_remove_collection(ctx)
    }

    pub fn mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
        creator_bump: u8,
    ) -> Result<()> {
        handle_mint_nft(ctx, creator_bump)
    }

    pub fn set_collection_during_mint(ctx: Context<SetCollectionDuringMint>) -> Result<()> {
        handle_set_collection_during_mint(ctx)
    }

    pub fn withdraw_funds<'info>(ctx: Context<WithdrawFunds<'info>>) -> Result<()> {
        handle_withdraw_funds(ctx)
    }

    pub fn create_whitelist_account(
        ctx: Context<CreateWhitelistAccount>,
        whitelist_type: String,
    ) -> Result<()> {
        whitelist_instructions::create_whitelist_account::handler(ctx, whitelist_type)
    }

    // pub fn decrease_whitelist_count(ctx: Context<DecreaseWhitelistSpots>, count: u8) -> Result<()> {
    //     whitelist_instructions::decrease_whitelist_count::handler(ctx, count)
    // }
}
