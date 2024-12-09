use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface },
};

use crate::state::*;
use crate::error::*;
use crate::event::*;
use crate::utils::*;

// Accounts
#[derive(Accounts)]
#[instruction(input:String)]
pub struct LockVesting<'info> {
    #[account(
        init,
        space = 8 + LockPda::LEN,
        seeds = [input.as_ref(), spl_mint.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    pub spl_mint: Box<InterfaceAccount<'info, Mint>>,

    ///CHECK:safe
    pub spl_mint_metadata_pda: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = spl_mint,
        associated_token::authority = lock_pda
    )]
    pub lock_pda_spl_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub authority_spl_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<LockVesting>,
    input: String,
    lock_time: u64,
    lock_name: String,
    extra_data: String,
    first_release: f64,
    vesting_period: u64,
    amount_per_vesting: f64,
    user_list: Vec<Pubkey>,
    user_amount: Vec<u64>
) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    if lock_time == 0 {
        return Err(LockError::TimeZero.into());
    }

    if lock_pda.lock_amount > 0 {
        return Err(LockError::AlreadyLocked.into());
    }

    //check spl_mint fllows metaplex standard
    // let expected = Metadata::find_pda(&ctx.accounts.spl_mint.key()).0;

    // let metadata_pda = &ctx.accounts.spl_mint_metadata_pda;
    // if metadata_pda.key() != expected {
    //     return Err(LockError::NotValidToken.into());
    // }

    // if metadata_pda.data_len() == 0 {
    //     return Err(LockError::NotValidToken.into());
    // }

    // if metadata_pda.data_len() != 0 {
    //     if metadata_pda.owner != mpl_token_metadata::ID {
    //         return Err(VendingMachineError::NotValidToken.into());
    //     }
    // }

    let clock = Clock::get();
    let block_time = clock.unwrap().unix_timestamp as u64;

    if lock_time <= block_time {
        return Err(LockError::BeforeNow.into());
    }

    lock_pda.bump = ctx.bumps.lock_pda;
    lock_pda.seed = input;
    lock_pda.authority = ctx.accounts.authority.key();
    lock_pda.spl_mint = ctx.accounts.spl_mint.key();
    lock_pda.spl_mint_metadata_pda = ctx.accounts.spl_mint_metadata_pda.key();
    lock_pda.start_time = block_time;
    lock_pda.end_time = lock_time;
    lock_pda.lock_name = lock_name;
    lock_pda.extra_data = extra_data;
    lock_pda.dex_name = "manual".to_string();
    lock_pda.first_release = first_release;
    lock_pda.vesting_period = vesting_period;
    lock_pda.amount_per_vesting = amount_per_vesting;
    lock_pda.user_list = user_list;
    lock_pda.user_amount = user_amount;

    let mut lock_amount=0;

    for i in 0..lock_pda.user_amount.len() {
        lock_amount += lock_pda.user_amount[i];
    }

    lock_pda.released_status =
    vec![0u8; lock_pda.user_amount.len()];
    lock_pda.pre_unlocked_time =
    vec![0u64; lock_pda.user_amount.len()];

    if lock_amount == 0 {
        return Err(LockError::AmountZero.into());
    }

    lock_pda.lock_amount = lock_amount;

    transfer_from_user_to_pool_vault(
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority_spl_ata.to_account_info(),
        ctx.accounts.lock_pda_spl_ata.to_account_info(),
        ctx.accounts.spl_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        lock_amount,
        ctx.accounts.spl_mint.decimals
    )?;

    emit!(LockVestingEvent {
        event_name: "lock_vesting".to_string(),
        lock_pda: lock_pda.key(),
        seed: lock_pda.seed.clone(),
        lock_id: lock_pda.lock_id,
        authority: lock_pda.authority,
        spl_mint: lock_pda.spl_mint,
        spl_mint_metadata_pda: lock_pda.spl_mint_metadata_pda,
        lock_amount: lock_pda.lock_amount,
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
        dex_name: lock_pda.dex_name.clone(),
    });

    Ok(())
}
