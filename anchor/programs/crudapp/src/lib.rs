#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8QJXi7Jgd512foSuyfV7hReNPTWNqMpxgk1YHCjaJYQb");

#[program]
pub mod crudapp {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateJournalEntry>, title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.owner = *ctx.accounts.owner.key;
      journal_entry.title = title;
      journal_entry.message = message;
      Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateJournalEntry>, _title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.message = message;

      Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteJournalEntry>, _title: String) -> Result<()> {
      Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  pub owner: Pubkey,
  #[max_len(50)]
  pub title: String,
  #[max_len(100)]
  pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    init,
    payer = owner,
    space = 8 + JournalEntryState::INIT_SPACE,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
  #[account(mut)]
  owner: Signer<'info>,

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true,
    bump
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteJournalEntry<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    close = owner,
    bump
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  pub system_program: Program<'info, System>
}