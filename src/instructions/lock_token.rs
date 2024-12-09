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
pub struct InitializeLockPda<'info> {
    #[account(
        init,
        space = 8 + LockPda::LEN,
        seeds = [input.as_ref(), spl_mint.key().as_ref(), owner.key().as_ref()],
        bump,
        payer = authority
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    pub spl_mint: Box<InterfaceAccount<'info, Mint>>,

    ///CHECK:safe
    pub spl_mint_metadata_pda: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    ///CHECK:safe
    pub owner: AccountInfo<'info>,

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
    ctx: Context<InitializeLockPda>,
    input: String,
    lock_amount: u64,
    lock_time: u64,
    lock_name: String,
    extra_data: String,
    is_nft: bool,
    project_token_mint: Pubkey,
    wsol_mint: Pubkey
) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    // if lock_amount == 0 {
    //     return Err(LockError::AmountZero.into());
    // }

    let real_lock_amount;

    if is_nft {
        real_lock_amount = ctx.accounts.authority_spl_ata.amount;
    } else {
        real_lock_amount = lock_amount;
    }

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
    lock_pda.authority = ctx.accounts.owner.key();
    lock_pda.spl_mint_metadata_pda = ctx.accounts.spl_mint_metadata_pda.key();
    lock_pda.lock_amount = real_lock_amount;
    lock_pda.start_time = block_time;
    lock_pda.lock_name = lock_name;
    lock_pda.extra_data = extra_data;

    if is_nft {
        //for the manual lp lock
        lock_pda.spl_mint = project_token_mint;
        lock_pda.dex_name = "manual lp".to_string();
        if project_token_mint > wsol_mint {
            lock_pda.token_mint_a = wsol_mint;
            lock_pda.token_mint_b = project_token_mint;
        }else{
            lock_pda.token_mint_a = project_token_mint;
            lock_pda.token_mint_b = wsol_mint; 
        }
        lock_pda.end_time = lock_time/1000;
        lock_pda.position_mint = ctx.accounts.spl_mint.key();
    } else {
        //for the normal token lock
        lock_pda.end_time = lock_time;
        lock_pda.spl_mint = ctx.accounts.spl_mint.key();
        lock_pda.dex_name = "manual".to_string();
    }

    transfer_from_user_to_pool_vault(
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority_spl_ata.to_account_info(),
        ctx.accounts.lock_pda_spl_ata.to_account_info(),
        ctx.accounts.spl_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        lock_pda.lock_amount,
        ctx.accounts.spl_mint.decimals
    )?;

    lock_pda.lock_id = block_time as u32;

    if is_nft {
        //for the manual lp lock
        emit!(CreateDexLockEvent {
            event_name: "create_dex_lock".to_string(),
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
            tge_bps: 0,
            cycle: 0,
            cycle_bps: 0,
            dex_name: lock_pda.dex_name.clone(),
            token_mint_a: lock_pda.token_mint_a,
            token_mint_b: lock_pda.token_mint_b,
            position_mint: lock_pda.position_mint,
        });
    } else {
        //for the normal token lock
        emit!(CreateLockEvent {
            event_name: "create_lock".to_string(),
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
            tge_bps: 0,
            cycle: 0,
            cycle_bps: 0,
            dex_name: lock_pda.dex_name.clone(),
        });
    }

    Ok(())
}
