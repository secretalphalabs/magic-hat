use anchor_lang::prelude::*;
use std::fmt;
use std::fmt::Debug;

#[proc_macros::assert_size(4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
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
impl Default for WLType {
    fn default() -> Self {
        WLType::Null
    }
}

#[proc_macros::assert_size(120)]
#[repr(C)]
#[account]
#[derive(Default, Debug)]
pub struct WalletWhitelist {
    pub magic_hat: Pubkey,              //32
    pub whitelisted_address: Pubkey,    //32
    pub whitelist_type: WLType,         //4
    pub whitelist_creator: Pubkey,      //32
    pub number_of_whitelist_spots: u64, //8
    pub bump: u8,                       //1
    _reserved: [u8; 3],                 //3
}
