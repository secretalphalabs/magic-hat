use anchor_lang::prelude::*;
use std::fmt::Debug;

//#[proc_macros::assert_size(4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub enum WLType {
    Null,
    One,
    Two,
    Three,
}

//#[proc_macros::assert_size(88)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct WalletWhitelist {
    pub magic_hat_creator: Pubkey,      //32
    pub whitelisted_address: Pubkey,    //32
    pub whitelist_type: WLType,         //4
    pub number_of_whitelist_spots: u64, //8
    pub special_discounted_price: u64,  //8
}
