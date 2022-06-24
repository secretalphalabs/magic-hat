use anchor_lang::prelude::*;
use mpl_token_metadata::{
    assertions::collection::assert_master_edition, instruction::approve_collection_authority,
    state::Metadata, utils::create_or_allocate_account_raw,
};
use solana_program::program::invoke;

use crate::{
    cmp_pubkeys,
    constants::{COLLECTIONS_FEATURE_INDEX, COLLECTION_PDA_SIZE},
    set_feature_flag, CollectionPDA, MagicHat, MagicHatError,
};

/// Set the collection PDA for the magic hat
#[derive(Accounts)]
pub struct SetCollection<'info> {
    #[account(mut, has_one = authority)]
    magic_hat: Account<'info, MagicHat>,
    authority: Signer<'info>,
    /// CHECK: account constraints checked in account trait
    #[account(mut, seeds = [b"collection".as_ref(), magic_hat.to_account_info().key.as_ref()], bump)]
    collection_pda: UncheckedAccount<'info>,
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,

    /// CHECK: account checked in CPI
    metadata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    mint: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    edition: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    collection_authority_record: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
}

pub fn handle_set_collection(ctx: Context<SetCollection>) -> Result<()> {
    let mint = ctx.accounts.mint.to_account_info();
    let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.metadata.to_account_info())?;
    if !cmp_pubkeys(&metadata.update_authority, &ctx.accounts.authority.key()) {
        return err!(MagicHatError::IncorrectCollectionAuthority);
    };
    if !cmp_pubkeys(&metadata.mint, &mint.key()) {
        return err!(MagicHatError::MintMismatch);
    }
    let edition = ctx.accounts.edition.to_account_info();
    let authority_record = ctx.accounts.collection_authority_record.to_account_info();
    let magic_hat = &mut ctx.accounts.magic_hat;
    if magic_hat.items_redeemed > 0 {
        return err!(MagicHatError::NoChangingCollectionDuringMint);
    }
    if !magic_hat.data.retain_authority {
        return err!(MagicHatError::MagicHatCollectionRequiresRetainAuthority);
    }
    assert_master_edition(&metadata, &edition)?;
    if authority_record.data_is_empty() {
        let approve_collection_infos = vec![
            authority_record.clone(),
            ctx.accounts.collection_pda.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            mint.clone(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!(
            "About to approve collection authority for {} with new authority {}.",
            ctx.accounts.metadata.key(),
            ctx.accounts.collection_pda.key
        );
        invoke(
            &approve_collection_authority(
                ctx.accounts.token_metadata_program.key(),
                authority_record.key(),
                ctx.accounts.collection_pda.to_account_info().key(),
                ctx.accounts.authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.metadata.key(),
                *mint.key,
            ),
            approve_collection_infos.as_slice(),
        )?;
        msg!(
            "Successfully approved collection authority. Now setting PDA mint to {}.",
            mint.key()
        );
    }
    if ctx.accounts.collection_pda.data_is_empty() {
        create_or_allocate_account_raw(
            crate::id(),
            &ctx.accounts.collection_pda.to_account_info(),
            &ctx.accounts.rent.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            COLLECTION_PDA_SIZE,
            &[
                b"collection".as_ref(),
                magic_hat.key().as_ref(),
                &[*ctx.bumps.get("collection_pda").unwrap()],
            ],
        )?;
    }
    let mut data_ref: &mut [u8] = &mut ctx.accounts.collection_pda.try_borrow_mut_data()?;
    let mut collection_pda_object: CollectionPDA = AnchorDeserialize::deserialize(&mut &*data_ref)?;
    collection_pda_object.mint = mint.key();
    collection_pda_object.magic_hat = magic_hat.key();
    collection_pda_object.try_serialize(&mut data_ref)?;
    set_feature_flag(&mut magic_hat.data.uuid, COLLECTIONS_FEATURE_INDEX);
    Ok(())
}
