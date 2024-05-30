use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer_checked, Burn, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{error::SoldStakingError, StakePool};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, seeds = [b"stake-pool"], bump)]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::decimals = stake_pool.base_mint_decimals,
        address = stake_pool.base_mint,
    )]
    // Stable Mint
    pub base_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = base_mint,
        associated_token::authority = payer,
    )]
    pub payer_base_mint_ata: Account<'info, TokenAccount>,

    //  Quote Mint
    #[account(
        address = stake_pool.x_mint @ SoldStakingError::InvalidXMintAddress
    )]
    pub x_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = x_mint,
        associated_token::authority = payer,
    )]
    pub payer_x_mint_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = base_mint,
        associated_token::authority = payer,
    )]
    pub vault: Account<'info, TokenAccount>,

    // Other
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<Unstake>, quantity: u64) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let base_mint = &ctx.accounts.base_mint;

    // Burning
    let bump = stake_pool.bump; // Corrected to be a slice of a slice of a byte slice
    let signer_seeds: &[&[&[u8]]] = &[&[b"stake-pool", &[bump]]];

    // Todo calculate
    let x_amount = quantity
        .checked_mul(10u64.pow(base_mint.decimals.into()))
        .ok_or(SoldStakingError::CalculationOverflow)?;

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                authority: ctx.accounts.payer.to_account_info(),
                from: ctx.accounts.payer_x_mint_ata.to_account_info(),
                mint: ctx.accounts.base_mint.to_account_info(),
            },
        ),
        x_amount,
    )?;

    let base_amount = quantity
        .checked_mul(10u64.pow(base_mint.decimals.into()))
        .ok_or(SoldStakingError::CalculationOverflow)?;
    // .checked_mul(token_manager.exchange_rate)
    // .ok_or(SoldIssuanceError::CalculationOverflow)?;

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.payer_base_mint_ata.to_account_info(),
                mint: ctx.accounts.base_mint.to_account_info(),
                authority: stake_pool.to_account_info(),
            },
            signer_seeds,
        ),
        base_amount,
        stake_pool.base_mint_decimals,
    )?;

    // Update token_manager
    stake_pool.x_supply -= x_amount;
    stake_pool.base_balance -= base_amount;

    Ok(())
}
