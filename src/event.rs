use anchor_lang::prelude::*;

#[event]
pub struct CreateLockEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub lock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
    pub dex_name: String,
}

#[event]
pub struct UnlockEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub unlock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
}

#[event]
pub struct CreateDexLockEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub lock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
    pub dex_name: String,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub position_mint: Pubkey,
}

#[event]
pub struct UnlockLpEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub lock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
    pub dex_name: String,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub position_mint: Pubkey,
}

#[event]
pub struct LockVestingEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub lock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
    pub dex_name: String,
    pub first_release: f64,
    pub vesting_period: u64,
    pub amount_per_vesting: f64,
    pub user_list: Vec<Pubkey>,
    pub user_amount: Vec<u64>,
}

#[event]
pub struct UnlockVestingEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub lock_id: u32,
    pub authority: Pubkey,
    pub spl_mint: Pubkey,
    pub spl_mint_metadata_pda: Pubkey,
    pub unlock_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub lock_name: String,
    pub extra_data: String,
    pub tge_bps: u8,
    pub cycle: u64,
    pub cycle_bps: u8,
    pub first_release: f64,
    pub vesting_period: u64,
    pub amount_per_vesting: f64,
    pub user_list: Vec<Pubkey>,
    pub user_amount: Vec<u64>,
    pub unlocker: Pubkey,
}

#[event]
pub struct ExtendLockTimeEvent {
    pub event_name: String,
    pub seed: String,
    pub lock_pda: Pubkey,
    pub end_time: u64,
}
