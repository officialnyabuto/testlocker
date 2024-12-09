use anchor_lang::prelude::*;
use anchor_spl::{ token::{ self, Mint, Token, TokenAccount }, associated_token::AssociatedToken };

use crate::state::*;
use crate::error::*;
use crate::event::*;

#[derive(Accounts)]
#[instruction(input:String)]
pub struct UnlockLp<'info> {
    #[account(
        mut,
        seeds = [input.as_ref(), spl_mint.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub spl_mint: Account<'info, Mint>,

    pub position_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = position_mint,
        associated_token::authority = lock_pda
    )]
    pub lock_pda_spl_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = position_mint,
        associated_token::authority = authority
    )]
    pub authority_spl_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<UnlockLp>, input: String) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    let clock = Clock::get();
    let block_time = clock.unwrap().unix_timestamp as u64;

    if block_time < lock_pda.end_time {
        return Err(LockError::NotUnlockTime.into());
    }

    if lock_pda.lock_amount == 0 {
        return Err(LockError::AlreadyUnlocked.into());
    }

    // require_keys_eq!(
    //     ctx.accounts.authority.key(),
    //     lock_pda.authority,
    //     LockError::AuthorizationErr
    // );
   
    require_keys_eq!(ctx.accounts.position_mint.key(), lock_pda.position_mint, LockError::SplMintError);

    let authority = ctx.accounts.authority.key();
    let spl_mint = ctx.accounts.spl_mint.key();
    let lock_bump = lock_pda.bump;
    let seeds = &[input.as_ref(), spl_mint.as_ref(), authority.as_ref(), &[lock_bump]];
    let signer = &[&seeds[..]];

    let transfer_token_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            authority: lock_pda.to_account_info(),
            from: ctx.accounts.lock_pda_spl_ata.to_account_info(),
            to: ctx.accounts.authority_spl_ata.to_account_info(),
        },
        signer
    );
    token::transfer(transfer_token_ctx, lock_pda.lock_amount)?;

    lock_pda.lock_amount = 0;

    emit!(UnlockLpEvent {
        event_name: "unlock_lp_event".to_string(),
        lock_pda: lock_pda.key(),
        seed: lock_pda.seed.clone(),
        lock_id: lock_pda.lock_id,
        authority: ctx.accounts.authority.key(),
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

    Ok(())
}
