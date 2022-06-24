use anchor_lang::prelude::*;
use mpl_token_metadata::{instruction::revoke_collection_authority, state::Metadata};
use solana_program::program::invoke;

use crate::{
    cmp_pubkeys, constants::COLLECTIONS_FEATURE_INDEX, remove_feature_flag, CollectionPDA,
    MagicHat, MagicHatError,
};

/// Set the collection PDA for the magic hat
#[derive(Accounts)]
pub struct RemoveCollection<'info> {
    #[account(mut, has_one = authority)]
    magic_hat: Account<'info, MagicHat>,
    authority: Signer<'info>,
    #[account(mut, seeds = [b"collection".as_ref(), magic_hat.to_account_info().key.as_ref()], bump, close=authority)]
    collection_pda: Account<'info, CollectionPDA>,
    /// CHECK: account checked in CPI
    metadata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    mint: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    collection_authority_record: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
}

pub fn handle_remove_collection(ctx: Context<RemoveCollection>) -> Result<()> {
    let mint = ctx.accounts.mint.to_account_info();
    let magic_hat = &mut ctx.accounts.magic_hat;
    if magic_hat.items_redeemed > 0 {
        return err!(MagicHatError::NoChangingCollectionDuringMint);
    }
    let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.metadata.to_account_info())?;
    if !cmp_pubkeys(&metadata.update_authority, &ctx.accounts.authority.key()) {
        return err!(MagicHatError::IncorrectCollectionAuthority);
    };
    if !cmp_pubkeys(&metadata.mint, &mint.key()) {
        return err!(MagicHatError::MintMismatch);
    }
    let authority_record = ctx.accounts.collection_authority_record.to_account_info();
    let revoke_collection_infos = vec![
        authority_record.clone(),
        ctx.accounts.collection_pda.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        mint.clone(),
    ];
    msg!(
        "About to revoke collection authority for {}.",
        ctx.accounts.metadata.key()
    );
    invoke(
        &revoke_collection_authority(
            ctx.accounts.token_metadata_program.key(),
            authority_record.key(),
            ctx.accounts.collection_pda.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.metadata.key(),
            mint.key(),
        ),
        revoke_collection_infos.as_slice(),
    )?;
    remove_feature_flag(&mut magic_hat.data.uuid, COLLECTIONS_FEATURE_INDEX);
    Ok(())
}
