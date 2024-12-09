use anchor_lang::prelude::*;
use anchor_spl::{ token::{ self, Mint, Token, TokenAccount }, associated_token::AssociatedToken };

use crate::state::*;
use crate::error::*;
use crate::event::*;

// Accounts
#[derive(Accounts)]
#[instruction(input:String)]
pub struct LockTest<'info> {
    #[account(
        init,
        space = 8 + LockPda::LEN,
        seeds = [input.as_ref(), spl_mint.key().as_ref(), lock_pda_authority.key().as_ref()],
        bump,
        payer = lock_pda_authority
    )]
    pub lock_pda: Box<Account<'info, LockPda>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    
    ///CHECK:safe
    #[account(mut)]
    pub lock_pda_authority: Signer<'info>,

    pub spl_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,

    pub position_mint: Account<'info, Mint>,

    ///CHECK:safe
    pub spl_mint_metadata_pda: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = lock_pda_authority,
        associated_token::mint = position_mint,
        associated_token::authority = lock_pda
    )]
    pub lock_pda_spl_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub authority_spl_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub token_2022_program: Interface<'info, anchor_spl::token_interface:: TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<LockTest>,
    input: String,
    lock_amount: u64,
    lock_time: u64,
    lock_name: String,
    extra_data: String,
    dex_name: String,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey
) -> Result<()> {
    let lock_pda = &mut ctx.accounts.lock_pda;

    if lock_amount == 0 {
        return Err(LockError::AmountZero.into());
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
    lock_pda.seed = input.clone();
    lock_pda.authority = ctx.accounts.authority.key();
    lock_pda.spl_mint = token_mint_a;
    lock_pda.spl_mint_metadata_pda = ctx.accounts.spl_mint_metadata_pda.key();
    lock_pda.lock_amount = lock_amount;
    lock_pda.start_time = block_time;
    lock_pda.end_time = lock_time;
    lock_pda.lock_name = lock_name;
    lock_pda.extra_data = extra_data;
    lock_pda.dex_name = dex_name;
    lock_pda.token_mint_a = token_mint_a;
    lock_pda.token_mint_b = token_mint_b;
    lock_pda.position_mint = ctx.accounts.position_mint.key();

    // transfer the tokens
    let token_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            authority: ctx.accounts.authority.to_account_info(),
            from: ctx.accounts.authority_spl_ata.to_account_info(),
            to: ctx.accounts.lock_pda_spl_ata.to_account_info(),
        }
    );
    token::transfer(token_ctx, lock_amount)?;

    lock_pda.lock_id = block_time as u32;

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

    Ok(())
}
