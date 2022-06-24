use anchor_lang::prelude::*;

use crate::{
    constants::COLLECTIONS_FEATURE_INDEX, is_feature_active, MagicHat, MagicHatData, MagicHatError,
};

/// Update the magic hat state.
#[derive(Accounts)]
pub struct UpdateMagicHat<'info> {
    #[account(
    mut,
    has_one = authority
    )]
    magic_hat: Account<'info, MagicHat>,
    authority: Signer<'info>,
    /// CHECK: wallet can be any account and is not written to or read
    wallet: UncheckedAccount<'info>,
}

pub fn handle_update_authority(
    ctx: Context<UpdateMagicHat>,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    let magic_hat = &mut ctx.accounts.magic_hat;

    if let Some(new_auth) = new_authority {
        magic_hat.authority = new_auth;
    }

    Ok(())
}

// updates without modifying UUID
pub fn handle_update_magic_hat(ctx: Context<UpdateMagicHat>, data: MagicHatData) -> Result<()> {
    let magic_hat = &mut ctx.accounts.magic_hat;

    if data.items_available != magic_hat.data.items_available && data.hidden_settings.is_none() {
        return err!(MagicHatError::CannotChangeNumberOfLines);
    }

    if magic_hat.data.items_available > 0
        && magic_hat.data.hidden_settings.is_none()
        && data.hidden_settings.is_some()
    {
        return err!(MagicHatError::CannotSwitchToHiddenSettings);
    }

    let old_uuid = magic_hat.data.uuid.clone();
    magic_hat.wallet = ctx.accounts.wallet.key();
    if is_feature_active(&old_uuid, COLLECTIONS_FEATURE_INDEX) && !data.retain_authority {
        return err!(MagicHatError::MagicHatCollectionRequiresRetainAuthority);
    }
    magic_hat.data = data;
    magic_hat.data.uuid = old_uuid;

    if !ctx.remaining_accounts.is_empty() {
        magic_hat.token_mint = Some(ctx.remaining_accounts[0].key())
    } else {
        magic_hat.token_mint = None;
    }
    Ok(())
}
