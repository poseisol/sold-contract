use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        burn, mint_to, spl_token::instruction::mint_to_checked, transfer_checked, Burn, Mint,
        MintTo, Token, TokenAccount, TransferChecked,
    },
};

use crate::TokenManager;

#[derive(Accounts)]
pub struct RedeemTokens<'info> {
    #[account(mut, seeds = [b"token-manager"], bump)]
    pub token_manager: Account<'info, TokenManager>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_mint_ata: Account<'info, TokenAccount>,
    pub quote_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = quote_mint,
        associated_token::authority = payer,
    )]
    pub payer_quote_mint_ata: Account<'info, TokenAccount>,
    #[account(
        associated_token::mint = quote_mint,
        associated_token::authority = token_manager,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<RedeemTokens>, quantity: u64) -> Result<()> {
    let token_manager = &mut ctx.accounts.token_manager;

    let bump = token_manager.bump; // Corrected to be a slice of a slice of a byte slice
    let signer_seeds: &[&[&[u8]]] = &[&[b"token-manager", &[bump]]];

    let token_manager = &mut ctx.accounts.token_manager;

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                authority: ctx.accounts.payer.to_account_info(),
                from: ctx.accounts.payer_mint_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        quantity,
    )?;

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.payer_quote_mint_ata.to_account_info(),
                mint: ctx.accounts.quote_mint.to_account_info(),
                authority: token_manager.to_account_info(),
            },
            signer_seeds,
        ),
        quantity,
        6,
    )?;

    // Update token_manager
    token_manager.total_supply -= quantity;

    Ok(())
}