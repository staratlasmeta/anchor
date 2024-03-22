use std::ops::Deref;
use anchor_lang::solana_program::account_info::AccountInfo;

use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
pub use spl_token_2022;
pub use spl_token_2022::ID;
#[deprecated(
since = "0.28.0",
note = "please use `transfer_checked` or `transfer_checked_with_fee` instead"
)]
pub fn transfer<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, Transfer<'info>>,
  amount: u64,
) -> Result<()> {
  #[allow(deprecated)]
    let ix = spl_token_2022::instruction::transfer(
    ctx.program.key,
    ctx.accounts.from.key,
    ctx.accounts.to.key,
    ctx.accounts.authority.key,
    &[],
    amount,
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[ctx.accounts.from, ctx.accounts.to, ctx.accounts.authority],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn transfer_checked<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, TransferChecked<'info>>,
  amount: u64,
  decimals: u8,
) -> Result<()> {
  let ix = spl_token_2022::instruction::transfer_checked(
    ctx.program.key,
    ctx.accounts.from.key,
    ctx.accounts.mint.key,
    ctx.accounts.to.key,
    ctx.accounts.authority.key,
    &[],
    amount,
    decimals,
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[
      ctx.accounts.from,
      ctx.accounts.mint,
      ctx.accounts.to,
      ctx.accounts.authority,
    ],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn mint_to<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, MintTo<'info>>,
  amount: u64,
) -> Result<()> {
  let ix = spl_token_2022::instruction::mint_to(
    ctx.program.key,
    ctx.accounts.mint.key,
    ctx.accounts.to.key,
    ctx.accounts.authority.key,
    &[],
    amount,
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[ctx.accounts.to, ctx.accounts.mint, ctx.accounts.authority],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn burn<'info>(ctx: CpiContext<'_, '_, '_, 'info, Burn<'info>>, amount: u64) -> Result<()> {
  let ix = spl_token_2022::instruction::burn(
    ctx.program.key,
    ctx.accounts.from.key,
    ctx.accounts.mint.key,
    ctx.accounts.authority.key,
    &[],
    amount,
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[ctx.accounts.from, ctx.accounts.mint, ctx.accounts.authority],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn approve<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, Approve<'info>>,
  amount: u64,
) -> Result<()> {
  let ix = spl_token_2022::instruction::approve(
    ctx.program.key,
    ctx.accounts.to.key,
    ctx.accounts.delegate.key,
    ctx.accounts.authority.key,
    &[],
    amount,
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[
      ctx.accounts.to,
      ctx.accounts.delegate,
      ctx.accounts.authority,
    ],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn revoke<'info>(ctx: CpiContext<'_, '_, '_, 'info, Revoke<'info>>) -> Result<()> {
  let ix = spl_token_2022::instruction::revoke(
    ctx.program.key,
    ctx.accounts.source.key,
    ctx.accounts.authority.key,
    &[],
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[ctx.accounts.source, ctx.accounts.authority],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn initialize_account<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeAccount<'info>>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_account(
    ctx.program.key,
    ctx.accounts.account.key,
    ctx.accounts.mint.key,
    ctx.accounts.authority.key,
  )?;
  solana_program::program::invoke(
    &ix,
    &[
      ctx.accounts.account,
      ctx.accounts.mint,
      ctx.accounts.authority,
      ctx.accounts.rent,
    ],
  )
    .map_err(Into::into)
}
pub fn initialize_account3<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeAccount3<'info>>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_account3(
    ctx.program.key,
    ctx.accounts.account.key,
    ctx.accounts.mint.key,
    ctx.accounts.authority.key,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.account, ctx.accounts.mint])
    .map_err(Into::into)
}
pub fn close_account<'info>(ctx: CpiContext<'_, '_, '_, 'info, CloseAccount<'info>>) -> Result<()> {
  let ix = spl_token_2022::instruction::close_account(
    ctx.program.key,
    ctx.accounts.account.key,
    ctx.accounts.destination.key,
    ctx.accounts.authority.key,
    &[], // TODO: support multisig
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[
      ctx.accounts.account,
      ctx.accounts.destination,
      ctx.accounts.authority,
    ],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn freeze_account<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::freeze_account(
    ctx.program.key,
    ctx.accounts.account.key,
    ctx.accounts.mint.key,
    ctx.accounts.authority.key,
    &[], // TODO: Support multisig signers.
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[
      ctx.accounts.account,
      ctx.accounts.mint,
      ctx.accounts.authority,
    ],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn thaw_account<'info>(ctx: CpiContext<'_, '_, '_, 'info, ThawAccount<'info>>) -> Result<()> {
  let ix = spl_token_2022::instruction::thaw_account(
    ctx.program.key,
    ctx.accounts.account.key,
    ctx.accounts.mint.key,
    ctx.accounts.authority.key,
    &[], // TODO: Support multisig signers.
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[
      ctx.accounts.account,
      ctx.accounts.mint,
      ctx.accounts.authority,
    ],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn initialize_mint<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeMint<'info>>,
  decimals: u8,
  authority: &Pubkey,
  freeze_authority: Option<&Pubkey>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_mint(
    ctx.program.key,
    ctx.accounts.mint.key,
    authority,
    freeze_authority,
    decimals,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.mint, ctx.accounts.rent])
    .map_err(Into::into)
}
pub fn initialize_mint2<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeMint2<'info>>,
  decimals: u8,
  authority: &Pubkey,
  freeze_authority: Option<&Pubkey>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_mint2(
    ctx.program.key,
    ctx.accounts.mint.key,
    authority,
    freeze_authority,
    decimals,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}
pub fn set_authority<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>,
  authority_type: spl_token_2022::instruction::AuthorityType,
  new_authority: Option<Pubkey>,
) -> Result<()> {
  let mut spl_new_authority: Option<&Pubkey> = None;
  if new_authority.is_some() {
    spl_new_authority = new_authority.as_ref()
  }
  let ix = spl_token_2022::instruction::set_authority(
    ctx.program.key,
    ctx.accounts.account_or_mint.key,
    spl_new_authority,
    authority_type,
    ctx.accounts.current_authority.key,
    &[], // TODO: Support multisig signers.
  )?;
  solana_program::program::invoke_signed(
    &ix,
    &[ctx.accounts.account_or_mint, ctx.accounts.current_authority],
    ctx.signer_seeds,
  )
    .map_err(Into::into)
}
pub fn sync_native<'info>(ctx: CpiContext<'_, '_, '_, 'info, SyncNative<'info>>) -> Result<()> {
  let ix = spl_token_2022::instruction::sync_native(ctx.program.key, ctx.accounts.account.key)?;
  solana_program::program::invoke(&ix, &[ctx.accounts.account]).map_err(Into::into)
}
pub fn get_account_data_size<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, GetAccountDataSize<'info>>,
  extension_types: &[spl_token_2022::extension::ExtensionType],
) -> Result<u64> {
  let ix = spl_token_2022::instruction::get_account_data_size(
    ctx.program.key,
    ctx.accounts.mint.key,
    extension_types,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.mint])?;
  solana_program::program::get_return_data()
    .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)
    .and_then(|(key, data)| {
      if key != *ctx.program.key {
        Err(solana_program::program_error::ProgramError::IncorrectProgramId)
      } else {
        data.try_into().map(u64::from_le_bytes).map_err(|_| {
          solana_program::program_error::ProgramError::InvalidInstructionData
        })
      }
    })
    .map_err(Into::into)
}
pub fn initialize_mint_close_authority<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeMintCloseAuthority<'info>>,
  close_authority: Option<&Pubkey>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_mint_close_authority(
    ctx.program.key,
    ctx.accounts.mint.key,
    close_authority,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.mint]).map_err(Into::into)
}
pub fn initialize_immutable_owner<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, InitializeImmutableOwner<'info>>,
) -> Result<()> {
  let ix = spl_token_2022::instruction::initialize_immutable_owner(
    ctx.program.key,
    ctx.accounts.account.key,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.account]).map_err(Into::into)
}
pub fn amount_to_ui_amount<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, AmountToUiAmount<'info>>,
  amount: u64,
) -> Result<String> {
  let ix = spl_token_2022::instruction::amount_to_ui_amount(
    ctx.program.key,
    ctx.accounts.account.key,
    amount,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.account])?;
  solana_program::program::get_return_data()
    .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)
    .and_then(|(key, data)| {
      if key != *ctx.program.key {
        Err(solana_program::program_error::ProgramError::IncorrectProgramId)
      } else {
        String::from_utf8(data).map_err(|_| {
          solana_program::program_error::ProgramError::InvalidInstructionData
        })
      }
    })
    .map_err(Into::into)
}
pub fn ui_amount_to_amount<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, UiAmountToAmount<'info>>,
  ui_amount: &str,
) -> Result<u64> {
  let ix = spl_token_2022::instruction::ui_amount_to_amount(
    ctx.program.key,
    ctx.accounts.account.key,
    ui_amount,
  )?;
  solana_program::program::invoke(&ix, &[ctx.accounts.account])?;
  solana_program::program::get_return_data()
    .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)
    .and_then(|(key, data)| {
      if key != *ctx.program.key {
        Err(solana_program::program_error::ProgramError::IncorrectProgramId)
      } else {
        data.try_into().map(u64::from_le_bytes).map_err(|_| {
          solana_program::program_error::ProgramError::InvalidInstructionData
        })
      }
    })
    .map_err(Into::into)
}
#[derive(Accounts)]
pub struct Transfer<'info> {
  pub from: AccountInfo<'info>,
  pub to: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct TransferChecked<'info> {
  pub from: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub to: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct MintTo<'info> {
  pub mint: AccountInfo<'info>,
  pub to: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Burn<'info> {
  pub mint: AccountInfo<'info>,
  pub from: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Approve<'info> {
  pub to: AccountInfo<'info>,
  pub delegate: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Revoke<'info> {
  pub source: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeAccount<'info> {
  pub account: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeAccount3<'info> {
  pub account: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct CloseAccount<'info> {
  pub account: AccountInfo<'info>,
  pub destination: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct FreezeAccount<'info> {
  pub account: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct ThawAccount<'info> {
  pub account: AccountInfo<'info>,
  pub mint: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeMint<'info> {
  pub mint: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeMint2<'info> {
  pub mint: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetAuthority<'info> {
  pub current_authority: AccountInfo<'info>,
  pub account_or_mint: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SyncNative<'info> {
  pub account: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct GetAccountDataSize<'info> {
  pub mint: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeMintCloseAuthority<'info> {
  pub mint: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeImmutableOwner<'info> {
  pub account: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct AmountToUiAmount<'info> {
  pub account: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct UiAmountToAmount<'info> {
  pub account: AccountInfo<'info>,
}
#[derive(Clone)]
pub struct Token2022;
impl anchor_lang::Id for Token2022 {
  fn id() -> Pubkey {
    ID
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Token2022Account(spl_token_2022::state::Account);

impl anchor_lang::AccountDeserialize for Token2022Account {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Account>::unpack(
      buf,
    )
      .map(|t| Token2022Account(t.base))
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for Token2022Account {}

impl anchor_lang::Owner for Token2022Account {
  fn owner() -> Pubkey {
    spl_token_2022::ID
  }
}

impl Deref for Token2022Account {
  type Target = spl_token_2022::state::Account;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(feature = "idl-build")]
impl anchor_lang::IdlBuild for Token2022Account {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mint2022(spl_token_2022::state::Mint);

impl anchor_lang::AccountDeserialize for Mint2022 {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(buf)
      .map(|t| Mint2022(t.base))
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for Mint2022 {}

impl anchor_lang::Owner for Mint2022 {
  fn owner() -> Pubkey {
    spl_token_2022::ID
  }
}

impl Deref for Mint2022 {
  type Target = spl_token_2022::state::Mint;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// Field parsers to save compute. All account validation is assumed to be done
// outside of these methods.
pub use crate::token::accessor;