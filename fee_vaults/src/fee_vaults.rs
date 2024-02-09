use scrypto::prelude::*;

use crate::events::*;

#[blueprint]
#[events(
    SetTreasuryPercentageEvent,
    SetBurnPercentageEvent,
    SetSwapAmountEvent,
    SetMaxEpochsEvent,
    TreasuryWithdrawEvent,
    ReserveWithdrawEvent,
    TreasuryDepositEvent,
    SwapVaultDepositEvent,
    ReserveDepositEvent,
    BurnEvent,
    SwapEvent,
)]
#[types(
    ResourceAddress,
    Vault,
    Epoch,
)]
mod fee_vaults {
    enable_method_auth! {
        roles {
            user => updatable_by: [OWNER];
            treasury_manager => updatable_by: [OWNER];
            reserve_manager => updatable_by: [OWNER];
        },
        methods {
            set_treasury_percentage => restrict_to: [OWNER];
            set_burn_percentage => restrict_to: [OWNER];
            set_swap_amount => restrict_to: [OWNER];
            set_max_epochs => restrict_to: [OWNER];
            treasury_withdraw => restrict_to: [treasury_manager];
            reserve_withdraw => restrict_to: [reserve_manager];
            swap => restrict_to: [user];
            get_treasury_percentage => PUBLIC;
            get_burn_percentage => PUBLIC;
            get_swap_amount => PUBLIC;
            get_max_epochs => PUBLIC;
            get_last_swapped_epoch => PUBLIC;
            get_swap_vault_amount => PUBLIC;
            get_reserve_amount => PUBLIC;
            get_treasury_vault_amount => PUBLIC;
            get_swap_price => PUBLIC;
            treasury_deposit => PUBLIC;
            swap_vault_deposit => PUBLIC;
            reserve_deposit => PUBLIC;
            deposit => PUBLIC;
            deposit_batch => PUBLIC;
        }
    }

    struct FeeVaults {
        /// The percentage of swap tokens that are burned (1 = 100%, 0 = 0%).
        burn_percentage: Decimal,
        /// The percentage of deposited tokens that go to the treasury (1 = 100%, 0 = 0%).
        treasury_percentage: Decimal,
        /// The base amount of tokens needed to swap.
        swap_amount: Decimal,
        /// The max epochs before the price of swapping is zero.
        max_epochs: u64,
        /// The treasury vaults.
        treasury_vaults: KeyValueStore<ResourceAddress, Vault>,
        /// The vaults that can be swapped for.
        swap_vaults: KeyValueStore<ResourceAddress, Vault>,
        /// The last epoch that the swap vault was swapped.
        last_swapped_epoch: KeyValueStore<ResourceAddress, Epoch>,
        /// Swap tokens vault.
        reserve_vault: Vault,
    }

    impl FeeVaults {
        /// Instantiate and globalize a new fee vaults owned by admin badge.
        ///
        /// # Arguments
        ///
        /// * `admin_badge_address` - The resource address of the admin badge to set as owner.
        /// * `swap_token_address` - The resource address of the token that is used for swapping.
        /// * `swap_amount` - The amount of tokens that are need to swap.
        ///
        /// # Returns
        ///
        /// * `Global<FeeVaults>` - The new fee vaults.
        ///
        /// # Requires
        ///
        /// * `swap_token_address` - The tokens is burnable.
        ///
        /// # Panics
        ///
        /// * If the swap amount is not greater than zero.
        ///
        /// # Access Rules
        ///
        /// * `set_treasury_percentage` - Owner required.
        /// * `set_burn_percentage` - Owner required.
        /// * `set_swap_amount` - Owner required.
        /// * `set_max_epochs` - Owner required.
        /// * `treasury_withdraw` - Treasury manager required.
        /// * `reserve_withdraw` - Reserve manager required.
        /// * `swap` - User required.
        /// * `get_treasury_percentage` - Public.
        /// * `get_burn_percentage` - Public.
        /// * `get_swap_amount` - Public.
        /// * `get_max_epochs` - Public.
        /// * `get_last_swapped_epoch` - Public.
        /// * `get_swap_vault_amount` - Public.
        /// * `get_reserve_amount` - Public.
        /// * `get_treasury_vault_amount` - Public.
        /// * `get_swap_price` - Public.
        /// * `treasury_deposit` - Public.
        /// * `swap_vault_deposit` - Public.
        /// * `reserve_deposit` - Public.
        /// * `deposit` - Public.
        /// * `deposit_batch` - Public.
        ///
        pub fn new(
            admin_badge_address: ResourceAddress,
            swap_token_address: ResourceAddress,
            swap_amount: Decimal,
        ) -> Global<FeeVaults> {
            // Instantiate and globalize with access rules
            Self::new_local(swap_token_address, swap_amount)
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
                .roles(roles!(
                    reserve_manager => rule!(require(admin_badge_address));
                    treasury_manager => rule!(require(admin_badge_address));
                    user => rule!(allow_all);
                ))
                .globalize()
        }

        /// Instantiate a new fee vaults.
        ///
        /// # Arguments
        ///
        /// * `swap_token_address` - The resource address of the token that is used for swapping.
        /// * `swap_amount` - The amount of tokens that are swapped needed to swap.
        ///
        /// # Returns
        ///
        /// * `Owned<FeeVaults>` - The new fee vaults.
        ///
        /// # Requires
        ///
        /// * `swap_token_address` - The token is burnable.
        ///
        /// # Panics
        ///
        /// * If the swap amount is not greater than zero.
        /// * If the swap token is not maximum divisible.
        /// * If the swap token is not burnable or can be made not burnable.
        ///
        pub fn new_local(
            swap_token_address: ResourceAddress,
            swap_amount: Decimal,
        ) -> Owned<FeeVaults> {
            assert!(
                swap_amount > Decimal::ZERO,
                "Swap amount must be greater than zero."
            );

            // Get the token manager
            let token_manager = ResourceManager::from(swap_token_address);

            // Check swap token divisibility
            assert_eq!(
                token_manager.resource_type().divisibility().unwrap(), DIVISIBILITY_MAXIMUM,
                "Swap token must be maximum divisible.",
            );

            // Check swap token burnable
            let burner_role = token_manager.get_role(BURNER_ROLE);
            let burner_updater_role = token_manager.get_role(BURNER_UPDATER_ROLE);
            assert!(
                burner_role.is_some() && burner_role.unwrap() == AccessRule::AllowAll &&
                (burner_updater_role.is_none() || burner_updater_role.unwrap() == AccessRule::DenyAll),
                "Swap token must be burnable.",
            );

            // Instantiate
            Self {
                swap_amount,
                burn_percentage: Decimal::ONE,
                treasury_percentage: Decimal::ZERO,
                treasury_vaults: KeyValueStore::new_with_registered_type(),
                swap_vaults: KeyValueStore::new_with_registered_type(),
                last_swapped_epoch: KeyValueStore::new_with_registered_type(),
                reserve_vault: Vault::new(swap_token_address),
                max_epochs: 10000,
            }
            .instantiate()
        }

        /// OWNER: Set the percentage of fees that go to the treasury.
        ///
        /// # Arguments
        ///
        /// * `treasury_percentage` - The percentage of fees that go to the treasury.
        ///
        /// # Panics
        ///
        /// * If the percentage is not between 0 and 1.
        /// 
        /// # Events
        /// 
        /// * `SetTreasuryPercentageEvent` - Event emitted when the percentage of fees that go to the treasury is set.
        ///
        pub fn set_treasury_percentage(&mut self, treasury_percentage: Decimal) {
            // Assert valid parameters
            assert!(
                treasury_percentage <= Decimal::ONE && treasury_percentage >= Decimal::ZERO,
                "Percentage of fees for treasury must be between 0 and 1."
            );

            // Set the treasury percentage
            self.treasury_percentage = treasury_percentage;

            // Emit set treasury percentage event
            Runtime::emit_event(SetTreasuryPercentageEvent {
                treasury_percentage,
            });
        }

        /// OWNER: Set the percentage of swap tokens that are burned.
        ///
        /// # Arguments
        /// 
        /// * `burn_percentage` - The percentage of swap tokens that are burned.
        /// 
        /// # Panics
        /// 
        /// * If the percentage is not between 0 and 1.
        /// 
        /// # Events
        /// 
        /// * `SetBurnPercentageEvent` - Event emitted when the percentage of swap tokens that are burned is set.
        /// 
        pub fn set_burn_percentage(&mut self, burn_percentage: Decimal) {
            // Assert valid parameters
            assert!(
                burn_percentage <= Decimal::ONE && burn_percentage >= Decimal::ZERO,
                "Percentage of swap tokens for burning must be between 0 and 1."
            );

            // Set the burn percentage
            self.burn_percentage = burn_percentage;

            // Emit set burn percentage event
            Runtime::emit_event(SetBurnPercentageEvent { 
                burn_percentage 
            });
        }

        /// OWNER: Set the base amount of tokens needed to swap.
        ///
        /// # Arguments
        ///
        /// * `swap_amount` - The base amount of tokens needed to swap.
        ///
        /// # Panics
        ///
        /// * If the swap amount is not greater than zero.
        /// 
        /// # Events
        /// 
        /// * `SetSwapAmountEvent` - Event emitted when the base amount of tokens needed to swap is set.
        ///
        pub fn set_swap_amount(&mut self, swap_amount: Decimal) {
            // Assert valid parameters
            assert!(
                swap_amount > Decimal::ZERO,
                "Swap amount must be greater than zero."
            );

            // Set the swap amount
            self.swap_amount = swap_amount;

            // Emit set swap amount event
            Runtime::emit_event(SetSwapAmountEvent { 
                swap_amount 
            });
        }

        /// OWNER: Set the max epochs before the price of swapping is zero.
        ///
        /// # Arguments
        ///
        /// * `max_epochs` - The max epochs.
        /// 
        /// # Events
        /// 
        /// * `SetMaxEpochsEvent` - Event emitted when max epochs is set.
        ///
        pub fn set_max_epochs(&mut self, max_epochs: u64) {
            // Set the max epochs
            self.max_epochs = max_epochs;

            // Emit set max epochs event
            Runtime::emit_event(SetMaxEpochsEvent { 
                max_epochs 
            });
        }

        /// TREASURY MANAGER: Withdraw tokens from the treasury.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - The resource address of the tokens to withdraw.
        ///
        /// # Returns
        ///
        /// * `Bucket` - Contains the tokens withdrawn.
        ///
        /// # Panics
        ///
        /// * If the vault does not exist.
        /// 
        /// # Events
        /// 
        /// * `TreasuryWithdrawEvent` - Event emitted when tokens are withdrawn from the treasury.
        ///
        pub fn treasury_withdraw(&mut self, resource_address: ResourceAddress) -> Bucket {
            // Withdraw tokens from the treasury vault
            let tokens = self.treasury_vaults
                .get_mut(&resource_address)
                .expect("Vault not found.")
                .take_all();

            // Emit treasury withdraw event
            Runtime::emit_event(TreasuryWithdrawEvent {
                resource_address,
                amount: tokens.amount(),
            });

            // Return the tokens
            tokens
        }

        /// RESERVE MANAGER: Withdraw tokens from the reserve.
        /// 
        /// # Arguments
        /// 
        /// * `amount` - The amount of tokens to withdraw.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains the tokens withdrawn.
        /// 
        /// # Events
        /// 
        /// * `ReserveWithdrawEvent` - Event emitted when tokens are withdrawn from the reserve.
        /// 
        pub fn reserve_withdraw(&mut self, amount: Decimal) -> Bucket {
            // Assert amount is positive
            assert!(
                amount >= Decimal::ZERO, 
                "Amount must be positive."
            );

            // Withdraw tokens from the reserve vault
            let amount = amount.min(self.reserve_vault.amount());
            let tokens = self.reserve_vault.take(amount);

            // Emit reserve withdraw event
            Runtime::emit_event(ReserveWithdrawEvent { 
                amount,
                new_balance: self.reserve_vault.amount(),
            });

            // Return the tokens
            tokens
        }

        /// Get the percentage of fees that go to the treasury.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The percentage of fees that go to the treasury.
        ///
        pub fn get_treasury_percentage(&self) -> Decimal {
            self.treasury_percentage
        }

        /// Get the percentage of swap tokens that are burned.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - The percentage of swap tokens that are burned.
        /// 
        pub fn get_burn_percentage(&self) -> Decimal {
            self.burn_percentage
        }

        /// Get the swap amount
        ///
        /// # Returns
        ///
        /// * `Decimal` - The swap amount.
        ///
        pub fn get_swap_amount(&self) -> Decimal {
            self.swap_amount
        }

        /// Get the max epochs before the price of swapping is zero.
        ///
        /// # Returns
        ///
        /// * `u64` - The max epochs.
        ///
        pub fn get_max_epochs(&self) -> u64 {
            self.max_epochs
        }

        /// Get the last swapped epoch for the given resource address type.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - The resource address of the tokens to get the last swapped epoch of.
        ///
        /// # Returns
        ///
        /// * `Epoch` - The last swapped epoch for the given resource address type.
        ///
        /// # Panics
        ///
        /// * If the vault does not exist.
        ///
        pub fn get_last_swapped_epoch(&self, resource_address: ResourceAddress) -> Epoch {
            *self.last_swapped_epoch.get(&resource_address).unwrap()
        }

        /// Get the amount of tokens in the treasury of the given resource address type.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - The resource address of the tokens to get the amount of.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The amount of tokens in the treasury of the given resource address type.
        ///
        /// # Panics
        ///
        /// * If the vault does not exist.
        ///
        pub fn get_treasury_vault_amount(&self, resource_address: ResourceAddress) -> Decimal {
            self.treasury_vaults
                .get(&resource_address)
                .unwrap()
                .amount()
        }

        /// Get the amount of tokens in the treasury of the given resource address type.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - The resource address of the tokens to get the amount of.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The amount of tokens in the treasury of the given resource address type.
        ///
        /// # Panics
        ///
        /// * If the vault does not exist.
        ///
        pub fn get_swap_vault_amount(&self, resource_address: ResourceAddress) -> Decimal {
            self.swap_vaults.get(&resource_address).unwrap().amount()
        }

        /// Get the amount of tokens in the reserve.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - The amount of tokens in the reserve.
        /// 
        pub fn get_reserve_amount(&self) -> Decimal {
            self.reserve_vault.amount()
        }

        /// Get the amount of tokens needed to swap for to the token in a swap vault.
        /// Gets cheaper the longer it has been since the last swap.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - The resource address of the token to swap for.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The amount of tokens needed to swap for to the token in a swap vault.
        ///
        /// # Panics
        ///
        /// * If the vault does not exist.
        ///
        pub fn get_swap_price(&self, resource_address: ResourceAddress) -> Decimal {
            // Retrieve the last swapped epoch for the given resource
            let last_swapped_epoch = *self.last_swapped_epoch.get(&resource_address).expect("Swap vault not found.");

            // Calculate the time since the last swap
            let epoch_diff: u64 = Runtime::current_epoch().number() - last_swapped_epoch.number();

            // Calculate the swap price
            if epoch_diff > self.max_epochs {
                Decimal::ZERO
            } else {
                let decimal_diff = Decimal::from(epoch_diff);
                let decimal_max_epochs = Decimal::from(self.max_epochs);

                self.swap_amount * (Decimal::ONE - (decimal_diff / decimal_max_epochs))
            }
        }

        /// Deposit tokens into the treasury.
        ///
        /// # Arguments
        ///
        /// * `tokens` - The tokens to deposit.
        /// 
        /// # Events
        /// 
        /// * `TreasuryDepositEvent` - Event emitted when tokens are deposited into the treasury vault.
        ///
        pub fn treasury_deposit(&mut self, tokens: Bucket) {
            // Get the token resource address
            let resource_address = tokens.resource_address();
            
            // Deposit the tokens into treasury vault
            let amount = tokens.amount();
            let new_balance = if self.treasury_vaults.get(&resource_address).is_none() {
                self.treasury_vaults.insert(resource_address, Vault::with_bucket(tokens));
                amount
            } else {
                let mut vault = self.treasury_vaults.get_mut(&resource_address).unwrap();
                vault.put(tokens);
                vault.amount()
            };

            // Emit treasury deposit event
            Runtime::emit_event(TreasuryDepositEvent {
                resource_address,
                amount,
                new_balance,
            });
        }

        /// Deposit tokens into the swap vault.
        /// 
        /// # Arguments
        /// 
        /// * `tokens` - The tokens to deposit.
        /// 
        /// # Events
        /// 
        /// * `SwapVaultDepositEvent` - Event emitted when tokens are deposited into the swap vault.
        /// 
        pub fn swap_vault_deposit(&mut self, tokens: Bucket) {
            // Get the token resource address
            let resource_address = tokens.resource_address();

            // Deposit the tokens into swap vault
            let amount = tokens.amount();
            let new_balance = if self.swap_vaults.get(&resource_address).is_none() {
                self.swap_vaults.insert(resource_address, Vault::with_bucket(tokens));
                self.last_swapped_epoch.insert(resource_address, Runtime::current_epoch());
                amount
            } else {
                let mut vault = self.swap_vaults.get_mut(&resource_address).unwrap();
                vault.put(tokens);
                vault.amount()
            };

            // Emit swap vault deposit event
            Runtime::emit_event(SwapVaultDepositEvent {
                resource_address,
                amount,
                new_balance,
            });
        }

        /// Deposit tokens into the reserve.
        /// 
        /// # Arguments
        /// 
        /// * `tokens` - The tokens to deposit.
        /// 
        /// # Panics
        /// 
        /// * If the tokens are not of the correct type.
        /// 
        /// # Events
        /// 
        /// * `ReserveDepositEvent` - Event emitted when tokens are deposited into the reserve vault.
        /// 
        pub fn reserve_deposit(&mut self, tokens: Bucket) {
            // Deposit the tokens into reserve vault
            let amount = tokens.amount();
            self.reserve_vault.put(tokens);

            // Emit reserve vault deposit event
            Runtime::emit_event(ReserveDepositEvent { 
                amount,
                new_balance: self.reserve_vault.amount(),
            });
        }

        /// Deposit tokens. Will be split between the treasury and the swap vaults.
        ///
        /// # Arguments
        ///
        /// * `tokens` - The tokens to deposit.
        ///
        pub fn deposit(&mut self, mut tokens: Bucket) {
            // Take tokens for the treasury
            if self.treasury_percentage > Decimal::ZERO {
                self.treasury_deposit(tokens.take_advanced(tokens.amount() * self.treasury_percentage, WithdrawStrategy::Rounded(RoundingMode::ToZero)));
            }

            // Take tokens for the swap vault
            self.swap_vault_deposit(tokens);
        }

        /// Deposit a vector of tokens. Will be split between the treasury and the swap vaults.
        ///
        /// # Arguments
        ///
        /// * `tokens` - The vector tokens to deposit.
        ///
        pub fn deposit_batch(&mut self, tokens: Vec<Bucket>) {
            for token in tokens {
                self.deposit(token);
            }
        }

        /// USER: Buy tokens in swap vault using swap tokens.
        ///
        /// # Arguments
        ///
        /// * `swap_tokens` - Swap tokens to use for buying the swap vault tokens.
        /// * `resource_address` - The resource address of the tokens to buy.
        ///
        /// # Returns
        ///
        /// * `Bucket` - Contains the tokens that were bought.
        /// * `Bucket` - Contains remaining swap tokens.
        ///
        /// # Panics
        ///
        /// * If the swap vault does not exist.
        /// * If the swap tokens are not of the correct type.
        /// * If there are not enough swap tokens.
        /// * If the swap tokens are not burnable.
        /// 
        /// # Events
        /// 
        /// * `SwapEvent` - Event emitted when tokens are swapped.
        ///
        pub fn swap(
            &mut self,
            mut swap_tokens: Bucket,
            resource_address: ResourceAddress,
        ) -> (Bucket, Bucket) {
            // Assert that the swap tokens are of the correct type
            assert!(
                swap_tokens.resource_address() == self.reserve_vault.resource_address(),
                "Invalid tokens for swapping."
            );

            // Assert that there is enough swap tokens
            let swap_price = self.get_swap_price(resource_address);
            assert!(
                swap_tokens.amount() >= swap_price, 
                "Not enough tokens."
            );

            // Calculate the amount of tokens to burn and deposit
            let burn_amount = swap_price * self.burn_percentage;
            let deposit_amount = swap_price - burn_amount;

            // Burn the swap tokens
            if burn_amount > Decimal::ZERO {
                swap_tokens.take(burn_amount).burn();
            }

            // Emit burn event
            Runtime::emit_event(BurnEvent {
                amount: burn_amount,
            });

            // Deposit swap tokens to the reserve
            self.reserve_deposit(swap_tokens.take(deposit_amount));

            // Get the swap vault
            let tokens = self
                .swap_vaults
                .get_mut(&resource_address)
                .expect("Vault not found.")
                .take_all();

            // Update the last swapped epoch
            self.last_swapped_epoch
                .insert(resource_address, Runtime::current_epoch());

            // Emit swap event
            Runtime::emit_event(SwapEvent {
                resource_address,
                swap_price,
                amount: tokens.amount(),
            });

            (tokens, swap_tokens)
        }
    }
}
