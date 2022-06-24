use crate::whitelist_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateWhitelistConfig<'info> {
    #[account(init, 
        space = 8 + std::mem::size_of::<WhitelistConfig>(),
        seeds = [b"whitelist-config".as_ref(), magic_hat_creator.key().as_ref()],
        bump,
        payer = magic_hat_creator
    )]
    whitelist_config: Account<'info, WhitelistConfig>,
    #[account(mut)]
    magic_hat_creator: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateWhitelistConfig>, whitelist_schedule: WhitelistSchedule) -> Result<()> {
    msg!(
        "whitelist_schedule{:?}",
        whitelist_schedule,
    );
    
    let whitelist_config = &mut ctx.accounts.whitelist_config;
    whitelist_schedule.verify_schedule_invariants();
    whitelist_config.whitelist_schedule = whitelist_schedule;
    whitelist_config.magic_hat_creator = ctx.accounts.magic_hat_creator.key();
    whitelist_config.bump = *ctx.bumps.get("whitelist_config").unwrap();
    Ok(())
}