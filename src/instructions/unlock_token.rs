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
pub struct UnlockPda<'info> {
    #[account(
        mut,
        seeds = [input.as_ref(),spl_mint.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    pub spl_mint:  Box<InterfaceAccount<'info, Mint>>,

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


pub fn handler(ctx: Context<UnlockPda>, input: String) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    let clock = Clock::get();
    let block_time = clock.unwrap().unix_timestamp as u64;

    if block_time*1000 < lock_pda.end_time {
        return Err(LockError::NotUnlockTime.into());
    }

    if lock_pda.lock_amount == 0 {
        return Err(LockError::AlreadyUnlocked.into());
    }

    require_keys_eq!(
        ctx.accounts.authority.key(),
        lock_pda.authority,
        LockError::AuthorizationErr
    );
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
        lock_pda.lock_amount,
        ctx.accounts.spl_mint.decimals,
        signer
    )?;

    lock_pda.lock_amount = 0;

    emit!(UnlockEvent {
        event_name: "unlock".to_string(),
        lock_pda: lock_pda.key(),
        seed: lock_pda.seed.clone(),
        lock_id: lock_pda.lock_id,
        authority: lock_pda.authority,
        spl_mint: lock_pda.spl_mint,
        spl_mint_metadata_pda: lock_pda.spl_mint_metadata_pda,
        unlock_amount: lock_pda.lock_amount,
        start_time: lock_pda.start_time,
        end_time: lock_pda.end_time,
        lock_name: lock_pda.lock_name.clone(),
        extra_data: lock_pda.extra_data.clone(),
        tge_bps: 0,
        cycle: 0,
        cycle_bps: 0,
    });

    Ok(())
}
