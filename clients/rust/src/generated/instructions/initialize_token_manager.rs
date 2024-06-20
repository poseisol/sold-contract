//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct InitializeTokenManager {
    pub token_manager: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub metadata: solana_program::pubkey::Pubkey,

    pub mint: solana_program::pubkey::Pubkey,

    pub quote_mint: solana_program::pubkey::Pubkey,

    pub owner: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub token_metadata_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl InitializeTokenManager {
    pub fn instruction(
        &self,
        args: InitializeTokenManagerInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeTokenManagerInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_manager,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.quote_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_metadata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeTokenManagerInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SOLD_ISSUANCE_ID,
            accounts,
            data,
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
pub struct InitializeTokenManagerInstructionData {
    discriminator: [u8; 8],
}

impl InitializeTokenManagerInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [67, 249, 6, 71, 87, 19, 139, 58],
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeTokenManagerInstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub exchange_rate: u64,
    pub emergency_fund_basis_points: u16,
    pub merkle_root: [u8; 32],
    pub admin: Pubkey,
    pub minter: Pubkey,
    pub gate_keepers: Vec<Pubkey>,
    pub mint_limit_per_slot: u64,
    pub redemption_limit_per_slot: u64,
    pub withdraw_time_lock: i64,
    pub withdraw_execution_window: i64,
}

/// Instruction builder for `InitializeTokenManager`.
///
/// ### Accounts:
///
///   0. `[writable]` token_manager
///   1. `[writable]` vault
///   2. `[writable]` metadata
///   3. `[writable]` mint
///   4. `[]` quote_mint
///   5. `[writable, signer]` owner
///   6. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[optional]` token_metadata_program (default to `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`)
///   10. `[]` associated_token_program
#[derive(Default)]
pub struct InitializeTokenManagerBuilder {
    token_manager: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    metadata: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    quote_mint: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_metadata_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    decimals: Option<u8>,
    exchange_rate: Option<u64>,
    emergency_fund_basis_points: Option<u16>,
    merkle_root: Option<[u8; 32]>,
    admin: Option<Pubkey>,
    minter: Option<Pubkey>,
    gate_keepers: Option<Vec<Pubkey>>,
    mint_limit_per_slot: Option<u64>,
    redemption_limit_per_slot: Option<u64>,
    withdraw_time_lock: Option<i64>,
    withdraw_execution_window: Option<i64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeTokenManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn token_manager(&mut self, token_manager: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_manager = Some(token_manager);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn quote_mint(&mut self, quote_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.quote_mint = Some(quote_mint);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s']`
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.decimals = Some(decimals);
        self
    }
    #[inline(always)]
    pub fn exchange_rate(&mut self, exchange_rate: u64) -> &mut Self {
        self.exchange_rate = Some(exchange_rate);
        self
    }
    #[inline(always)]
    pub fn emergency_fund_basis_points(&mut self, emergency_fund_basis_points: u16) -> &mut Self {
        self.emergency_fund_basis_points = Some(emergency_fund_basis_points);
        self
    }
    #[inline(always)]
    pub fn merkle_root(&mut self, merkle_root: [u8; 32]) -> &mut Self {
        self.merkle_root = Some(merkle_root);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn minter(&mut self, minter: Pubkey) -> &mut Self {
        self.minter = Some(minter);
        self
    }
    #[inline(always)]
    pub fn gate_keepers(&mut self, gate_keepers: Vec<Pubkey>) -> &mut Self {
        self.gate_keepers = Some(gate_keepers);
        self
    }
    #[inline(always)]
    pub fn mint_limit_per_slot(&mut self, mint_limit_per_slot: u64) -> &mut Self {
        self.mint_limit_per_slot = Some(mint_limit_per_slot);
        self
    }
    #[inline(always)]
    pub fn redemption_limit_per_slot(&mut self, redemption_limit_per_slot: u64) -> &mut Self {
        self.redemption_limit_per_slot = Some(redemption_limit_per_slot);
        self
    }
    #[inline(always)]
    pub fn withdraw_time_lock(&mut self, withdraw_time_lock: i64) -> &mut Self {
        self.withdraw_time_lock = Some(withdraw_time_lock);
        self
    }
    #[inline(always)]
    pub fn withdraw_execution_window(&mut self, withdraw_execution_window: i64) -> &mut Self {
        self.withdraw_execution_window = Some(withdraw_execution_window);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts =
            InitializeTokenManager {
                token_manager: self.token_manager.expect("token_manager is not set"),
                vault: self.vault.expect("vault is not set"),
                metadata: self.metadata.expect("metadata is not set"),
                mint: self.mint.expect("mint is not set"),
                quote_mint: self.quote_mint.expect("quote_mint is not set"),
                owner: self.owner.expect("owner is not set"),
                rent: self.rent.unwrap_or(solana_program::pubkey!(
                    "SysvarRent111111111111111111111111111111111"
                )),
                system_program: self
                    .system_program
                    .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                )),
                token_metadata_program: self.token_metadata_program.unwrap_or(
                    solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                ),
                associated_token_program: self
                    .associated_token_program
                    .expect("associated_token_program is not set"),
            };
        let args = InitializeTokenManagerInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            decimals: self.decimals.clone().expect("decimals is not set"),
            exchange_rate: self
                .exchange_rate
                .clone()
                .expect("exchange_rate is not set"),
            emergency_fund_basis_points: self
                .emergency_fund_basis_points
                .clone()
                .expect("emergency_fund_basis_points is not set"),
            merkle_root: self.merkle_root.clone().expect("merkle_root is not set"),
            admin: self.admin.clone().expect("admin is not set"),
            minter: self.minter.clone().expect("minter is not set"),
            gate_keepers: self.gate_keepers.clone().expect("gate_keepers is not set"),
            mint_limit_per_slot: self
                .mint_limit_per_slot
                .clone()
                .expect("mint_limit_per_slot is not set"),
            redemption_limit_per_slot: self
                .redemption_limit_per_slot
                .clone()
                .expect("redemption_limit_per_slot is not set"),
            withdraw_time_lock: self
                .withdraw_time_lock
                .clone()
                .expect("withdraw_time_lock is not set"),
            withdraw_execution_window: self
                .withdraw_execution_window
                .clone()
                .expect("withdraw_execution_window is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_token_manager` CPI accounts.
pub struct InitializeTokenManagerCpiAccounts<'a, 'b> {
    pub token_manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub quote_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_token_manager` CPI instruction.
pub struct InitializeTokenManagerCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub quote_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeTokenManagerInstructionArgs,
}

impl<'a, 'b> InitializeTokenManagerCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeTokenManagerCpiAccounts<'a, 'b>,
        args: InitializeTokenManagerInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            token_manager: accounts.token_manager,
            vault: accounts.vault,
            metadata: accounts.metadata,
            mint: accounts.mint,
            quote_mint: accounts.quote_mint,
            owner: accounts.owner,
            rent: accounts.rent,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            token_metadata_program: accounts.token_metadata_program,
            associated_token_program: accounts.associated_token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_manager.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.quote_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_metadata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeTokenManagerInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLD_ISSUANCE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.token_manager.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.quote_mint.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_metadata_program.clone());
        account_infos.push(self.associated_token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeTokenManager` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` token_manager
///   1. `[writable]` vault
///   2. `[writable]` metadata
///   3. `[writable]` mint
///   4. `[]` quote_mint
///   5. `[writable, signer]` owner
///   6. `[]` rent
///   7. `[]` system_program
///   8. `[]` token_program
///   9. `[]` token_metadata_program
///   10. `[]` associated_token_program
pub struct InitializeTokenManagerCpiBuilder<'a, 'b> {
    instruction: Box<InitializeTokenManagerCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeTokenManagerCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeTokenManagerCpiBuilderInstruction {
            __program: program,
            token_manager: None,
            vault: None,
            metadata: None,
            mint: None,
            quote_mint: None,
            owner: None,
            rent: None,
            system_program: None,
            token_program: None,
            token_metadata_program: None,
            associated_token_program: None,
            name: None,
            symbol: None,
            uri: None,
            decimals: None,
            exchange_rate: None,
            emergency_fund_basis_points: None,
            merkle_root: None,
            admin: None,
            minter: None,
            gate_keepers: None,
            mint_limit_per_slot: None,
            redemption_limit_per_slot: None,
            withdraw_time_lock: None,
            withdraw_execution_window: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn token_manager(
        &mut self,
        token_manager: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_manager = Some(token_manager);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn quote_mint(
        &mut self,
        quote_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.quote_mint = Some(quote_mint);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.instruction.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.instruction.decimals = Some(decimals);
        self
    }
    #[inline(always)]
    pub fn exchange_rate(&mut self, exchange_rate: u64) -> &mut Self {
        self.instruction.exchange_rate = Some(exchange_rate);
        self
    }
    #[inline(always)]
    pub fn emergency_fund_basis_points(&mut self, emergency_fund_basis_points: u16) -> &mut Self {
        self.instruction.emergency_fund_basis_points = Some(emergency_fund_basis_points);
        self
    }
    #[inline(always)]
    pub fn merkle_root(&mut self, merkle_root: [u8; 32]) -> &mut Self {
        self.instruction.merkle_root = Some(merkle_root);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: Pubkey) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn minter(&mut self, minter: Pubkey) -> &mut Self {
        self.instruction.minter = Some(minter);
        self
    }
    #[inline(always)]
    pub fn gate_keepers(&mut self, gate_keepers: Vec<Pubkey>) -> &mut Self {
        self.instruction.gate_keepers = Some(gate_keepers);
        self
    }
    #[inline(always)]
    pub fn mint_limit_per_slot(&mut self, mint_limit_per_slot: u64) -> &mut Self {
        self.instruction.mint_limit_per_slot = Some(mint_limit_per_slot);
        self
    }
    #[inline(always)]
    pub fn redemption_limit_per_slot(&mut self, redemption_limit_per_slot: u64) -> &mut Self {
        self.instruction.redemption_limit_per_slot = Some(redemption_limit_per_slot);
        self
    }
    #[inline(always)]
    pub fn withdraw_time_lock(&mut self, withdraw_time_lock: i64) -> &mut Self {
        self.instruction.withdraw_time_lock = Some(withdraw_time_lock);
        self
    }
    #[inline(always)]
    pub fn withdraw_execution_window(&mut self, withdraw_execution_window: i64) -> &mut Self {
        self.instruction.withdraw_execution_window = Some(withdraw_execution_window);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeTokenManagerInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            symbol: self.instruction.symbol.clone().expect("symbol is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            decimals: self
                .instruction
                .decimals
                .clone()
                .expect("decimals is not set"),
            exchange_rate: self
                .instruction
                .exchange_rate
                .clone()
                .expect("exchange_rate is not set"),
            emergency_fund_basis_points: self
                .instruction
                .emergency_fund_basis_points
                .clone()
                .expect("emergency_fund_basis_points is not set"),
            merkle_root: self
                .instruction
                .merkle_root
                .clone()
                .expect("merkle_root is not set"),
            admin: self.instruction.admin.clone().expect("admin is not set"),
            minter: self.instruction.minter.clone().expect("minter is not set"),
            gate_keepers: self
                .instruction
                .gate_keepers
                .clone()
                .expect("gate_keepers is not set"),
            mint_limit_per_slot: self
                .instruction
                .mint_limit_per_slot
                .clone()
                .expect("mint_limit_per_slot is not set"),
            redemption_limit_per_slot: self
                .instruction
                .redemption_limit_per_slot
                .clone()
                .expect("redemption_limit_per_slot is not set"),
            withdraw_time_lock: self
                .instruction
                .withdraw_time_lock
                .clone()
                .expect("withdraw_time_lock is not set"),
            withdraw_execution_window: self
                .instruction
                .withdraw_execution_window
                .clone()
                .expect("withdraw_execution_window is not set"),
        };
        let instruction = InitializeTokenManagerCpi {
            __program: self.instruction.__program,

            token_manager: self
                .instruction
                .token_manager
                .expect("token_manager is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            metadata: self.instruction.metadata.expect("metadata is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            quote_mint: self.instruction.quote_mint.expect("quote_mint is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_metadata_program: self
                .instruction
                .token_metadata_program
                .expect("token_metadata_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct InitializeTokenManagerCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    token_manager: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    quote_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_metadata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    decimals: Option<u8>,
    exchange_rate: Option<u64>,
    emergency_fund_basis_points: Option<u16>,
    merkle_root: Option<[u8; 32]>,
    admin: Option<Pubkey>,
    minter: Option<Pubkey>,
    gate_keepers: Option<Vec<Pubkey>>,
    mint_limit_per_slot: Option<u64>,
    redemption_limit_per_slot: Option<u64>,
    withdraw_time_lock: Option<i64>,
    withdraw_execution_window: Option<i64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
