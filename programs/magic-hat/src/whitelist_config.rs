use crate::wallet_whitelist::WLType;
use anchor_lang::prelude::*;

#[proc_macros::assert_size(128)]
#[repr(C)]
#[account]
pub struct WhitelistConfig {
    pub whitelist_schedule: WhitelistSchedule,
    pub magic_hat_creator: Pubkey, //32
}

#[proc_macros::assert_size(32)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub struct WhitelistTierConfig {
    pub whitelist_type: WLType,
    pub number_of_whitelist_spots_total: u64,
    pub mint_price: u64,
    pub start_time: u64,
}

#[proc_macros::assert_size(96)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub struct WhitelistSchedule {
    pub wl_start_time_3: WhitelistTierConfig,
    pub wl_start_time_2: WhitelistTierConfig,
    pub wl_start_time_1: WhitelistTierConfig,
}
impl WhitelistSchedule {
    pub fn verify_schedule_invariants(&self) {
        assert!(self.wl_start_time_1.start_time >= self.wl_start_time_2.start_time);
        assert!(self.wl_start_time_2.start_time >= self.wl_start_time_3.start_time);
    }
    // pub fn verify_schedule_invariants(&self) {
    //     let public_mint_start_time = self.public_mint_start_time.start_time;
    //     if let Some(wl1) = self.wl_start_time_1 {
    //         if let Some(wl2) = self.wl_start_time_2 {
    //             if let Some(wl3) = self.wl_start_time_3 {
    //                 if let Some(wl4) = self.wl_start_time_4 {
    //                     assert!(public_mint_start_time >= wl1.start_time);
    //                     assert!(wl1.start_time >= wl2.start_time);
    //                     assert!(wl2.start_time >= wl3.start_time);
    //                     assert!(wl3.start_time >= wl4.start_time);
    //                 } else {
    //                     //4 np
    //                     assert!(public_mint_start_time >= wl1.start_time);
    //                     assert!(wl1.start_time >= wl2.start_time);
    //                     assert!(wl2.start_time >= wl3.start_time);
    //                 }
    //             } else if let Some(wl4) = self.wl_start_time_4 {
    //                 // 3 np
    //                 assert!(public_mint_start_time >= wl1.start_time);
    //                 assert!(wl1.start_time >= wl2.start_time);
    //                 assert!(wl2.start_time >= wl4.start_time);
    //             } else {
    //                 //3,4 np
    //                 assert!(public_mint_start_time >= wl1.start_time);
    //                 assert!(wl1.start_time >= wl2.start_time);
    //             }
    //         } else if let Some(wl3) = self.wl_start_time_3 {
    //             //2 np
    //             if let Some(wl4) = self.wl_start_time_4 {
    //                 assert!(public_mint_start_time >= wl1.start_time);
    //                 assert!(wl1.start_time >= wl3.start_time);
    //                 assert!(wl3.start_time >= wl4.start_time);
    //             } else {
    //                 //2,4 np
    //                 assert!(public_mint_start_time >= wl1.start_time);
    //                 assert!(wl1.start_time >= wl3.start_time);
    //             }
    //         } else if let Some(wl4) = self.wl_start_time_4 {
    //             //2,3 np
    //             assert!(public_mint_start_time >= wl1.start_time);
    //             assert!(wl1.start_time >= wl4.start_time);
    //         } else {
    //             //2,3,4 np
    //             assert!(public_mint_start_time >= wl1.start_time);
    //         }
    //     } else if let Some(wl2) = self.wl_start_time_2 {
    //         //1np
    //         if let Some(wl3) = self.wl_start_time_3 {
    //             if let Some(wl4) = self.wl_start_time_4 {
    //                 assert!(public_mint_start_time >= wl2.start_time);
    //                 assert!(wl2.start_time >= wl3.start_time);
    //                 assert!(wl3.start_time >= wl4.start_time);
    //             } else {
    //                 //1,4 np
    //                 assert!(public_mint_start_time >= wl2.start_time);
    //                 assert!(wl2.start_time >= wl3.start_time);
    //             }
    //         } else if let Some(wl4) = self.wl_start_time_4 {
    //             //1,3 np
    //             assert!(public_mint_start_time >= wl2.start_time);
    //             assert!(wl2.start_time >= wl4.start_time);
    //         } else {
    //             //1,3,4 np
    //             assert!(public_mint_start_time >= wl2.start_time);
    //         }
    //     } else if let Some(wl3) = self.wl_start_time_3 {
    //         //1,2 np
    //         if let Some(wl4) = self.wl_start_time_4 {
    //             assert!(public_mint_start_time >= wl3.start_time);
    //             assert!(wl3.start_time >= wl4.start_time);
    //         } else {
    //             //1,2,4 np
    //             assert!(public_mint_start_time >= wl3.start_time);
    //         }
    //     } else if let Some(wl4) = self.wl_start_time_4 {
    //         //1,2,3 np
    //         assert!(public_mint_start_time >= wl4.start_time);
    //     }
    // }
}
