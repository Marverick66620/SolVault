use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::solana_program::rent::Rent;

declare_id!("Gzgzwp95t1sZPRhLT2qax1YjZrh1oopoS2bfZHNYR1rE");


#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        let signer_seeds = &[b"vault", ctx.accounts.signer.key.as_ref(), &[ctx.bumps.vault]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[signer_seeds],
            ),
            ctx.accounts.vault.lamports(),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    /// CHECK: This is the PDA vault account holding lamports.
    /// The PDA is derived using seeds ["vault", signer.key()] and bump,
    /// ensuring it is unique and controlled by this program.
    #[account(mut, seeds = [b"vault", signer.key().as_ref()], bump)]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already has funds.")]
    VaultAlreadyExists,

    #[msg("Invalid deposit amount.")]
    InvalidAmount,
}
