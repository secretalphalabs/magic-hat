use crate::whitelist_config::*;
use anchor_lang::prelude::*;
use crate::wallet_whitelist::*;

#[derive(Accounts)]
pub struct CreateWhitelistConfig<'info> {
    #[account(init, 
        payer = magic_hat_creator,
        space = 8 + std::mem::size_of::<WhitelistConfig>(),
        seeds = [b"whitelist-config".as_ref(), magic_hat_creator.key().as_ref()],
        bump,
    )]
    whitelist_config: Account<'info, WhitelistConfig>,
    #[account(mut)]
    magic_hat_creator: Signer<'info>,
    system_program: Program<'info, System>,
}

//pub fn handler(ctx: Context<CreateWhitelistConfig>, whitelist_schedule: WhitelistSchedule) -> Result<()> {
pub fn handler_create_whitelist_config(ctx: Context<CreateWhitelistConfig>, 
            wl_start_time_3_wl_spots: u64, 
            wl_start_time_3_wl_mp: u64, 
            wl_start_time_3_wl_st: u64,  
            wl_start_time_2_wl_spots: u64, 
            wl_start_time_2_wl_mp: u64, 
            wl_start_time_2_wl_st: u64,  
            wl_start_time_1_wl_spots: u64, 
            wl_start_time_1_wl_mp: u64, 
            wl_start_time_1_wl_st: u64, ) -> Result<()> {
    msg!("handler_create_whitelist_config");
    let whitelist_schedule =  WhitelistSchedule{
        wl_start_time_3: WhitelistTierConfig{
            whitelist_type: WLType::Three,
            number_of_whitelist_spots_total: wl_start_time_3_wl_spots,
            mint_price: wl_start_time_3_wl_mp,
            start_time: wl_start_time_3_wl_st,
        }, 
        wl_start_time_2: WhitelistTierConfig{
            whitelist_type: WLType::Two,
            number_of_whitelist_spots_total: wl_start_time_2_wl_spots,
            mint_price: wl_start_time_2_wl_mp,
            start_time: wl_start_time_2_wl_st,
        }, 
        wl_start_time_1: WhitelistTierConfig{
            whitelist_type: WLType::One,
            number_of_whitelist_spots_total: wl_start_time_1_wl_spots,
            mint_price: wl_start_time_1_wl_mp,
            start_time: wl_start_time_1_wl_st,
        },  
    };
    msg!(
        "whitelist_schedule{:?}",
        whitelist_schedule,
    );
    
    let whitelist_config = &mut ctx.accounts.whitelist_config;
    whitelist_schedule.verify_schedule_invariants();
    whitelist_config.whitelist_schedule = whitelist_schedule;
    whitelist_config.magic_hat_creator = ctx.accounts.magic_hat_creator.key();
    msg!("ctx.accounts.magic_hat_creator.key(){}", ctx.accounts.magic_hat_creator.key());
    Ok(())
}