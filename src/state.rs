use anchor_lang::prelude::*;

#[account]
pub struct LockPda {
    pub bump: u8, //1

    pub seed: String, //4+30
    pub lock_id: u32, //4
    pub authority: Pubkey, //32
    pub spl_mint: Pubkey, //32
    pub spl_mint_metadata_pda: Pubkey, //32
    pub lock_amount: u64, //8
    pub start_time: u64, //8
    pub end_time: u64, //8
    pub lock_name: String, //4+100 string length is 100
    pub extra_data: String, //4+100 string length is 100
    pub dex_name: String, //4+100 string length is 100
    pub token_mint_a: Pubkey, //32
    pub token_mint_b: Pubkey, //32
    pub position_mint: Pubkey, //32

    pub first_release: f64, //8
    pub vesting_period: u64, //8
    pub amount_per_vesting: f64, //8
    pub user_list: Vec<Pubkey>, //4+(32*100)
    pub user_amount: Vec<u64>, //4+(8*100)
    pub released_status: Vec<u8>, //4+(1*100)
    pub pre_unlocked_time: Vec<u64>, //4+(8*100)
    pub claimed_token_percent: u8,
}

impl LockPda {
    pub const LEN: usize =
        1 +
        (4 + 30) +
        4 +
        32 +
        32 +
        32 +
        8 +
        8 +
        8 +
        (4 + 100) +
        (4 + 100) +
        (4 + 100) +
        32 +
        32 +
        32 +
        8 +
        8 +
        8 +
        4 +
        32 * 100 +
        4 +
        8 * 100 +
        4 +
        1 * 100 +
        4 +
        8 * 100;
}
