use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface:: TokenInterface ,
};

use crate::state::*;
use crate::event::*;
use crate::error::*;


// Accounts
#[derive(Accounts)]
#[instruction()]
pub struct ExtendLockTime<'info> {
    #[account(
        mut,
        seeds = [lock_pda.seed.as_ref(), lock_pda.spl_mint.as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn handler(
    ctx: Context<ExtendLockTime>,
    lock_time: u64,
) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;
    if lock_time <lock_pda.end_time{
        return Err(LockError::NotBiggerThanEndTime.into());
    }

    lock_pda.end_time = lock_time;

    emit!(ExtendLockTimeEvent {
        event_name: "extend_lock_time_event".to_string(),
        seed: lock_pda.seed.clone(),
        lock_pda: lock_pda.key(),
        end_time: lock_pda.end_time,
    });

    Ok(())
}
