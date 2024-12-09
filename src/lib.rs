use anchor_lang::prelude::*;
use instructions::*;

declare_id!("");

pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

#[program]
mod gempad_solana_lock {
    use super::*;

    pub fn lock_token(
        ctx: Context<InitializeLockPda>,
        input: String,
        lock_amount: u64,
        lock_time: u64,
        lock_name: String,
        extra_data: String,
        is_nft: bool,
        project_token_mint: Pubkey,
        wsol_mint: Pubkey,
    ) -> Result<()> {
        return instructions::lock_token::handler(
            ctx,
            input,
            lock_amount,
            lock_time,
            lock_name,
            extra_data,
            is_nft,
            project_token_mint,
            wsol_mint,
        );
    }

    pub fn unlock_token(ctx: Context<UnlockPda>, input: String) -> Result<()> {
        return instructions::unlock_token::handler(ctx, input);
    }

    pub fn create_lock_pda(
        ctx: Context<LockTest>,
        input: String,
        lock_amount: u64,
        lock_time: u64,
        lock_name: String,
        extra_data: String,
        dex_name: String,
        token_mint_a: Pubkey,
        token_mint_b: Pubkey,
    ) -> Result<()> {
        return instructions::create_lock_pda::handler(
            ctx,
            input,
            lock_amount,
            lock_time,
            lock_name,
            extra_data,
            dex_name,
            token_mint_a,
            token_mint_b,
        );
    }

    pub fn unlock_lp(ctx: Context<UnlockLp>, input: String) -> Result<()> {
        return instructions::unlock_lp::handler(ctx, input);
    }

    pub fn lock_vesting(
        ctx: Context<LockVesting>,
        input: String,
        lock_time: u64,
        lock_name: String,
        extra_data: String,
        first_release: f64,
        vesting_period: u64,
        amount_per_vesting: f64,
        user_list: Vec<Pubkey>,
        user_amount: Vec<u64>,
    ) -> Result<()> {
        return instructions::lock_vesting::handler(
            ctx,
            input,
            lock_time,
            lock_name,
            extra_data,
            first_release,
            vesting_period,
            amount_per_vesting,
            user_list,
            user_amount,
        );
    }

    pub fn unlock_vesting(ctx: Context<UnlockVesting>, input: String) -> Result<()> {
        return instructions::unlock_vesting::handler(ctx, input);
    }

    pub fn extend_lock_time(ctx: Context<ExtendLockTime>, lock_time: u64) -> Result<()> {
        return instructions::extend_lock_time::handler(ctx, lock_time);
    }

}
