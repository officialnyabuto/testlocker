use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface },
};

use crate::state::*;
use crate::error::*;
use crate::event::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(input:String)]
pub struct UnlockVesting<'info> {
    #[account(
        mut,
        seeds = [input.as_ref(),spl_mint.key().as_ref(), lock_pda.authority.as_ref()],
        bump,
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    pub spl_mint: Box<InterfaceAccount<'info, Mint>>,

    ///CHECK:safe
    pub spl_mint_metadata_pda: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        token::mint = spl_mint,
        token::authority = lock_pda
    )]
    pub lock_pda_spl_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = spl_mint,
        associated_token::authority = authority
    )]
    pub authority_spl_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<UnlockVesting>, input: String) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    let clock = Clock::get();
    let block_time = clock.unwrap().unix_timestamp as u64;

    //check unlock_time for first release.
    if block_time * 1000 < lock_pda.end_time {
        return Err(LockError::NotUnlockTime.into());
    }

    let mut is_valid = 0;
    let mut index = 0;
    let unlock_amount: u64;

    for i in 0..lock_pda.user_list.len() {
        if ctx.accounts.authority.key() == lock_pda.user_list[i] {
            is_valid = 1;
            index = i;
        }
    }

    //check if the user is listed on the list
    if is_valid == 0 {
        return Err(LockError::AuthorizationErr.into());
    }

    let claim_percent;

    let rest_time = block_time - lock_pda.end_time / 1000;
    let is_first = ((rest_time as i64) - (lock_pda.vesting_period as i64) * 24 * 3600) as i64;

    let second_vesting_num = (
        (100.0 - (lock_pda.first_release as f64)) /
        (lock_pda.amount_per_vesting as f64)
    ).ceil() as u8;

    if is_first > 0 {
        let mut curr_num = (
            (is_first as f64) /
            ((lock_pda.vesting_period as f64) * 24.0 * 3600.0)
        ).floor() as u8;

        if curr_num > second_vesting_num {
            curr_num = second_vesting_num;
        }

        let mut curr_percent = (lock_pda.first_release +
            lock_pda.amount_per_vesting * (curr_num as f64)) as u8;

        if curr_percent > 100 {
            curr_percent = 100;
        }

        if curr_percent > lock_pda.claimed_token_percent {
            claim_percent = (curr_percent - lock_pda.claimed_token_percent) as f64;
            lock_pda.claimed_token_percent += claim_percent as u8;
            unlock_amount = (((lock_pda.user_amount[index] as f64) * claim_percent) / 100.0) as u64;
        } else {
            return Err(LockError::NotPerVestingUnlockTime.into());
        }
    } else {
        //first claim
        if lock_pda.claimed_token_percent > 0 {
            return Err(LockError::AlreadyDidFirstClaim.into());
        }
        claim_percent = lock_pda.first_release as f64;
        lock_pda.claimed_token_percent += claim_percent as u8;
        unlock_amount = (((lock_pda.user_amount[index] as f64) * claim_percent) / 100.0) as u64;
    }

    require_keys_eq!(ctx.accounts.spl_mint.key(), lock_pda.spl_mint, LockError::SplMintError);

    let authority = lock_pda.authority;
    let spl_mint = lock_pda.spl_mint;
    let lock_bump = lock_pda.bump;
    let seeds = &[input.as_ref(), spl_mint.as_ref(), authority.as_ref(), &[lock_bump]];
    let signer = &[&seeds[..]];

    transfer_from_pool_vault_to_user(
        lock_pda.to_account_info(),
        ctx.accounts.lock_pda_spl_ata.to_account_info(),
        ctx.accounts.authority_spl_ata.to_account_info(),
        ctx.accounts.spl_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        unlock_amount,
        ctx.accounts.spl_mint.decimals,
        signer
    )?;

    let total_unlocked_amount = (((lock_pda.user_amount[index] as f64) *
        (lock_pda.claimed_token_percent as f64)) /
        100.0) as u64;

    emit!(UnlockVestingEvent {
        event_name: "unlock_vesting".to_string(),
        lock_pda: lock_pda.key(),
        seed: lock_pda.seed.clone(),
        lock_id: lock_pda.lock_id,
        authority: lock_pda.authority,
        spl_mint: lock_pda.spl_mint,
        spl_mint_metadata_pda: lock_pda.spl_mint_metadata_pda,
        unlock_amount: total_unlocked_amount,
        start_time: lock_pda.start_time,
        end_time: lock_pda.end_time,
        lock_name: lock_pda.lock_name.clone(),
        extra_data: lock_pda.extra_data.clone(),
        first_release: lock_pda.first_release,
        vesting_period: lock_pda.vesting_period,
        amount_per_vesting: lock_pda.amount_per_vesting,
        user_list: lock_pda.user_list.clone(),
        user_amount: lock_pda.user_amount.clone(),
        tge_bps: 0,
        cycle: 0,
        cycle_bps: 0,
        unlocker: ctx.accounts.authority.key(),
    });

    Ok(())
}
