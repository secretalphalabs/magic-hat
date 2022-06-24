use anchor_lang::prelude::*;
use std::fmt;
use std::fmt::Debug;

#[proc_macros::assert_size(4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, PartialOrd)]
pub enum WLType {
    Four,
    Three,
    Two,
    One,
    Null,
}

impl fmt::Display for WLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[proc_macros::assert_size(96)]
#[account]
#[repr(C)]
#[derive(Debug)]
pub struct WalletWhitelist {
    pub magic_hat_creator: Pubkey,      //32
    pub whitelisted_address: Pubkey,    //32
    pub whitelist_type: WLType,         //4
    pub number_of_whitelist_spots: u64, //8
    pub special_discounted_price: u64,  //8
    pub bump: u8,                       //1
    _reserved: [u8; 3],                 //3
}
