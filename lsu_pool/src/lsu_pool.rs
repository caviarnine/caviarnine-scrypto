use crate::consts::*;
use crate::credit_receipt::*;
use crate::events::*;
use scrypto::prelude::*;

#[blueprint]
#[events(
    ValuationChangeEvent,
    AddLiquidityEvent,
    RemoveLiquidityEvent,
    SwapEvent,
    SetTokenValidatorEvent,
    SetProtocolFeeEvent,
    SetLiquidityFeeEvent,
    SetReserveFeeEvent,
)]
#[types(
    ResourceAddress,
    ComponentAddress,
    Decimal,
    Vault,
    u32,
    CreditReceipt,
)]
mod lsu_pool_mod {
    // Import Fee Vaults to send fee tokens to.
    extern_blueprint!(
        "package_sim1p4nhxvep6a58e88tysfu0zkha3nlmmcp6j8y5gvvrhl5aw47jfsxlt",
        FeeVaults {
            fn deposit(&self, tokens: Bucket);
            fn deposit_batch(&self, tokens: Vec<Bucket>);
        }
    );

    // Set fee vaults component.
    const FEE_VAULTS: Global<FeeVaults> = global_component!(
        FeeVaults,
        "component_sim1cz48mjgelfrdujgsys5ns2waps3n0utpjgjsjkaxp02c89u0gxfl3v"
    );

    // Set access rules.
    enable_method_auth! {
        roles {
            user => updatable_by: [OWNER];
        },
        methods {
            // test_admin
            set_token_validator => restrict_to: [OWNER];
            set_protocol_fee => restrict_to: [OWNER];
            set_liquidity_fee => restrict_to: [OWNER];
            set_reserve_fee => restrict_to: [OWNER];
            take_from_reserve_vaults => restrict_to: [OWNER];
            set_validator_max_before_fee => restrict_to: [OWNER];

            // test_getters
            get_token_validator_address => PUBLIC;
            get_fee_vaults_address => PUBLIC;
            get_vault_balance => PUBLIC;
            get_reserve_vault_balance => PUBLIC;
            get_price_lsu_xrd_cached => PUBLIC;
            get_dex_valuation_xrd => PUBLIC;
            get_liquidity_token_resource_address => PUBLIC;
            get_liquidity_token_total_supply => PUBLIC;
            get_credit_receipt_resource_address => PUBLIC;
            get_protocol_fee => PUBLIC;
            get_liquidity_fee => PUBLIC;
            get_reserve_fee => PUBLIC;
            get_price => PUBLIC;
            get_nft_data => PUBLIC;

            // test_validators
            is_lsu_token => PUBLIC;
            is_validator => PUBLIC;
            get_validator_address => PUBLIC;
            get_validator_price_lsu_xrd => PUBLIC;
            get_validator_price_lsu_xrd_and_update_valuation => PUBLIC;
            get_validator_max_before_fee => PUBLIC;

            // test_update_prices
            get_validator_counter => PUBLIC;
            get_validator_pointer => PUBLIC;
            get_validator_address_map => PUBLIC;
            update_multiple_validator_prices => PUBLIC;

            // test_credit_tools
            get_id_resources_from_credit_proof => PUBLIC;
            merge_credit => PUBLIC;

            // individual tests
            deposit_reserve_fee => PUBLIC;
            add_liquidity => restrict_to: [user];
            remove_liquidity => PUBLIC;
            swap => restrict_to: [user];
        }
    }

    // LsuPool component is used to create a liquidity pool for the LSU tokens, the native staking tokens on RadixDLT. 
    // Users can add liquidity in ANY valid LSU token that comes from the native validator blueprint. 
    // All LSU tokens are valued against XRD using the validator staking issue price. 
    // The LSU pool is used to swap between LSU tokens and LP to earn a Liquidity Fee. 
    // A small Protocol Fee is taken and sent to the external fee vaults. 
    // A small Reserve Fee is taken and sent to the internal reserve vaults. 
    // The reserve fee is used to buy back LSU tokens to keep the pool from accumulating LSU from validators that have been switch off and earn no yield. 
    // Adding liquidity is done by depositing LSU tokens, the LP receives an LP token back + a soul-bound Credit receipt. 
    // When adding Liquidity, a user can provide proof they have a Credit receipt (from a previous add liquidity) and the meta data will be updated. 
    // This removes the need to create multiple Credit receipts. 
    // The Credit receipt holds the amount of LSU tokens deposited. 
    // Removing liquidity is done by depositing LP tokens and choosing an LSU token to be returned. 
    // Since adding then removing liquidity in different tokens results in a swap, removing liquidity is charged. 
    // Removing liquidity is discounted *IF* the user provides proof of their Credit receipt *AND* it contains a credit in that LSU token. 
    //
    // The prices are kept up to date by regularly cycling over the validators and updating the prices. 
    // This maintains that the dex valuation is up to date. 
    // Since the number of validators isn't large and the cost of updating is small, this isn't a problem. 
    //
    // There is a cap on the number of validators / lsu tokens that can be added to the pool before a fee is charged.
    // This is to prevent the pool from being flooded with validators that are not earning yield as an attack vector.
    // Maximum validators is set to 200 even though the Radix validator set is 100
    ///
    struct LsuPool {
        /// Vaults that hold the lsu tokens for the pool.
        vaults: KeyValueStore<ResourceAddress, Vault>,
        /// Separate vaults to hold the reserve tokens.
        /// Note: This list could be different length than vaults.
        reserve_vaults: KeyValueStore<ResourceAddress, Vault>,
        /// The last prices for the LSU to XRD taken from the validator.
        /// Note: This list could be longer than vaults.
        prices_lsu_xrd: KeyValueStore<ResourceAddress, Decimal>,
        /// LSU resource address to validator component address.
        /// Note: This list could be longer than vaults or prices.
        lsu_to_validator: KeyValueStore<ResourceAddress, ComponentAddress>,
        /// Total valuation in XRD.
        dex_valuation_xrd: Decimal,
        /// Liquidity token resource manager.
        liquidity_token_resource_manager: ResourceManager,
        /// Credit receipt resource manager.
        credit_receipt_resource_manager: ResourceManager,
        /// Protocol fee are paid to the fee vaults.
        protocol_fee: Decimal,
        /// Liquidity fee are paid to the liquidity providers.
        liquidity_fee: Decimal,
        /// Reserve fees are collected and used to buy back LSU tokens.
        reserve_fee: Decimal,
        /// Validator max count.
        validator_max_before_fee: u32,
        /// Validator counter.
        validator_counter: u32,
        // Validator pointer for updating price.
        validator_pointer: u32,
        // Validator address map.
        validator_address_map: KeyValueStore<u32, ResourceAddress>,
        // Token validator for validating LSU tokens.
        token_validator: Global<AnyComponent>,
    }

    impl LsuPool {
        /// Instantiate and globalize new lsu Pool with access rules.
        ///
        /// # Arguments
        ///
        /// * `admin_badge_resource_address` - The address of the admin badge to set as the owner.
        /// * `token_validator_component_address` - Token validator component address.
        ///
        /// # Returns
        ///
        /// * `Global<LsuPool>` - The globalized lsu Pool component
        ///
        /// # Access Rules
        ///
        /// * `set_token_validator` - Owner required.
        /// * 'set_protocol_fee' - Owner required.
        /// * 'set_liquidity_fee' - Owner required.
        /// * 'set_reserve_fee' - Owner required.
        /// * 'take_from_reserve_vaults' - Owner required.
        /// * 'set_validator_max_before_fee' - Owner required.
        /// * 'add_liquidity' - User role required.
        /// * 'swap' - User role required.
        /// * `get_token_validator_address` - Public.
        /// * 'get_fee_vaults_address' - Public.
        /// * 'get_vault_balance' - Public.
        /// * 'get_reserve_vault_balance' - Public.
        /// * 'get_price_lsu_xrd_cached' - Public.
        /// * 'get_dex_valuation_xrd' - Public.
        /// * 'get_liquidity_token_resource_address' - Public.
        /// * 'get_liquidity_token_total_supply' - Public.
        /// * 'get_credit_receipt_resource_address' - Public.
        /// * 'get_protocol_fee' - Public.
        /// * 'get_liquidity_fee' - Public.
        /// * 'get_reserve_fee' - Public.
        /// * 'get_price' - Public.
        /// * 'get_nft_data' - Public.
        /// * 'is_lsu_token' - Public.
        /// * 'is_validator' - Public.
        /// * 'get_validator_address' - Public.
        /// * 'get_validator_price_lsu_xrd' - Public.
        /// * 'get_validator_price_lsu_xrd_and_update_valuation' - Public.
        /// * 'get_validator_max_before_fee' - Public.
        /// * 'get_validator_counter' - Public.
        /// * 'get_validator_pointer' - Public.
        /// * 'get_validator_address_map' - Public.
        /// * 'update_multiple_validator_prices' - Public.
        /// * 'get_id_resources_from_credit_proof' - Public.
        /// * 'merge_credit' - Public.
        /// * 'deposit_reserve_fee' - Public.
        /// * 'remove_liquidity' - Public.
        ///
        pub fn new(
            admin_badge_resource_address: ResourceAddress,
            token_validator_component_address: ComponentAddress,
        ) -> Global<LsuPool> {
            // Get component address
            let (address_reservation, component_address) = Runtime::allocate_component_address(LsuPool::blueprint_id());

            // Create liquidity token resource manager
            let liquidity_token_receipt_manager: ResourceManager =ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(admin_badge_resource_address))))
                .metadata(metadata!(init {
                    "package" => GlobalAddress::from(Runtime::package_address()), locked;
                    "component" => component_address.to_hex(), locked;
                    "name" =>  "LSU Pool LP", updatable;
                    "symbol" =>  "LSULP", updatable;
                    "description" =>  "Liquidity token for the LSU Pool", updatable;
                    "tags" => Vec::<String>::from(["defi".to_owned(), "LSU".to_owned(), "dex".to_owned(), "amm".to_owned(), "LP token".to_owned()]), updatable;
                }))
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .divisibility(DIVISIBILITY_MAXIMUM)
                .create_with_no_initial_supply();

            // Create credit receipt resource manager
            let credit_receipt_resource_manager: ResourceManager = ResourceBuilder::new_ruid_non_fungible_with_registered_type::<CreditReceipt>(OwnerRole::Updatable(rule!(require(admin_badge_resource_address))))
            .metadata(metadata!(init {
                "package" => GlobalAddress::from(Runtime::package_address()), locked;
                "component" => component_address.to_hex(), locked;
                "name" =>  "LSU Pool Credit Receipt", updatable;
                "symbol"=>  "LSUCR", updatable;
                "description"=>  "Credit receipt for the LSU Pool", updatable;
                "tags" => Vec::<String>::from(["defi".to_owned(), "LSU".to_owned(), "dex".to_owned(), "amm".to_owned(), "credit".to_owned(), "receipt".to_owned()]), updatable;
            })).mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(deny_all);
            })
            .burn_roles(burn_roles! {
                burner => rule!(allow_all);
                burner_updater => rule!(deny_all);
            })
            .withdraw_roles(withdraw_roles! {
                withdrawer => rule!(require(global_caller(component_address)));
                withdrawer_updater => rule!(deny_all);
            })
            .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                non_fungible_data_updater => rule!(require(global_caller(component_address)));
                non_fungible_data_updater_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            // Create the lsu pool
            Self {
                vaults: KeyValueStore::new_with_registered_type(),
                reserve_vaults: KeyValueStore::new_with_registered_type(),
                lsu_to_validator: KeyValueStore::new_with_registered_type(),
                prices_lsu_xrd: KeyValueStore::new_with_registered_type(),
                dex_valuation_xrd: Decimal::ZERO,
                liquidity_token_resource_manager: liquidity_token_receipt_manager,
                credit_receipt_resource_manager,
                protocol_fee: dec!(0.0001), // 1 bps
                liquidity_fee: dec!(0.0003), // 3 bps
                reserve_fee: dec!(0.0001), // 1 bps
                validator_max_before_fee: 200,
                validator_counter: 0,
                validator_pointer: 0,
                validator_address_map: KeyValueStore::new_with_registered_type(),
                token_validator: Global::from(token_validator_component_address),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_resource_address))))
            .with_address(address_reservation)
            .roles(roles!(
                user => rule!(allow_all);
            ))
            .globalize()
        }

        /// OWNER: Set token validator component.
        /// 
        /// # Arguments
        /// 
        /// * `lsu_validator_component_address` - Token validator component address.
        /// 
        /// # Events
        /// 
        /// * `SetTokenValidatorEvent` - Event emitted when lsu validator is set.
        /// 
        pub fn set_token_validator(&mut self, token_validator_component_address: ComponentAddress) {
            // Set token validator component
            self.token_validator = Global::from(token_validator_component_address);

            // Emit set token validator event
            Runtime::emit_event(SetTokenValidatorEvent {
                token_validator_component_address,
            });
        }

        /// OWNER: Set protocol fee
        ///
        /// # Arguments
        ///
        /// * `new_protocol_fee` - Decimal.
        ///
        /// # Panics
        ///
        /// * If new_protocol_fee is greater than PROTOCOL_FEE_MAX.
        /// * If new_protocol_fee is less than Decimal::ZERO.
        ///
        /// # Events
        ///
        /// * `SetProtocolFeeEvent` - Event emitted when the protocol fee is set.
        ///
        pub fn set_protocol_fee(&mut self, new_protocol_fee: Decimal) {
            assert!(new_protocol_fee <= PROTOCOL_FEE_MAX);
            assert!(new_protocol_fee >= Decimal::ZERO);
            self.protocol_fee = new_protocol_fee;

            // emit event
            Runtime::emit_event(SetProtocolFeeEvent {
                protocol_fee: new_protocol_fee,
            });
        }

        /// OWNER: Set liquidity fee
        ///
        /// # Arguments
        ///
        /// * `new_liquidity_fee` - Decimal.
        ///
        /// # Panics
        ///
        /// * If new_liquidity_fee is greater than LIQUIDITY_FEE_MAX.
        /// * If new_liquidity_fee is less than Decimal::ZERO.
        ///
        /// # Events
        ///
        /// * `SetLiquidityFeeEvent` - Event emitted when the liquidity fee is set.
        ///
        pub fn set_liquidity_fee(&mut self, new_liquidity_fee: Decimal) {
            assert!(new_liquidity_fee <= LIQUIDITY_FEE_MAX);
            assert!(new_liquidity_fee >= Decimal::ZERO);
            self.liquidity_fee = new_liquidity_fee;

            // emit event
            Runtime::emit_event(SetLiquidityFeeEvent {
                liquidity_fee: new_liquidity_fee,
            });
        }

        /// OWNER: Set reserve fee
        ///
        /// # Arguments
        ///
        /// * `new_reserve_fee` - Decimal.
        ///
        /// # Panics
        ///
        /// * If new_reserve_fee is greater than RESERVE_FEE_MAX.
        /// * If new_reserve_fee is less than Decimal::ZERO.
        ///
        /// # Events
        ///
        /// * `SetReserveFeeEvent` - Event emitted when the reserve fee is set.
        ///
        pub fn set_reserve_fee(&mut self, new_reserve_fee: Decimal) {
            assert!(new_reserve_fee <= RESERVE_FEE_MAX);
            assert!(new_reserve_fee >= Decimal::ZERO);
            self.reserve_fee = new_reserve_fee;

            // emit event
            Runtime::emit_event(SetReserveFeeEvent {
                reserve_fee: new_reserve_fee,
            });
        }

        /// OWNER: Take tokens from the reserve vaults.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the reserve vault.
        ///
        /// # Returns
        ///
        /// * `Bucket` - Bucket of tokens taken from the reserve vault.
        ///
        pub fn take_from_reserve_vaults(&mut self, resource_address: ResourceAddress) -> Bucket {
            self.reserve_vaults
                .get_mut(&resource_address)
                .unwrap()
                .take_all()
        }

        /// OWNER: Set the maximum number of validators before the fee is applied.
        ///
        /// # Arguments
        ///
        /// * `validator_max_before_fee` - Maximum number of validators before the fee is applied.
        ///
        pub fn set_validator_max_before_fee(&mut self, validator_max_before_fee: u32) {
            self.validator_max_before_fee = validator_max_before_fee;
        }

        /// Get component address of token validator.
        /// 
        /// # Returns
        /// 
        /// * `ComponentAddress` - Component address of token validator.
        /// 
        pub fn get_token_validator_address(&self) -> ComponentAddress {
            self.token_validator.address()
        }

        /// Get component address of fee vaults.
        ///
        /// # Returns
        ///
        /// * `ComponentAddress` - Component address of fee vaults.
        ///
        pub fn get_fee_vaults_address(&self) -> ComponentAddress {
            FEE_VAULTS.address()
        }

        /// Get a vault's balance.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the vault.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Balance of the vault.
        ///
        pub fn get_vault_balance(&self, resource_address: ResourceAddress) -> Option<Decimal> {
            Some(self.vaults.get(&resource_address)?.amount())
        }

        /// Get a reserve vault's balance.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the reserve vault.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Balance of the reserve vault.
        ///
        pub fn get_reserve_vault_balance(
            &self,
            resource_address: ResourceAddress,
        ) -> Option<Decimal> {
            Some(self.reserve_vaults.get(&resource_address)?.amount())
        }

        /// Get the price of a lsu resource to xrd from the internal cache.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the lsu.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Price of the lsu resource to xrd.
        ///
        pub fn get_price_lsu_xrd_cached(
            &self,
            resource_address: ResourceAddress,
        ) -> Option<Decimal> {
            Some(*self.prices_lsu_xrd.get(&resource_address)?)
        }

        /// Get the valuation of the pool in xrd.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Valuation of the pool in xrd.
        ///
        pub fn get_dex_valuation_xrd(&self) -> Decimal {
            self.dex_valuation_xrd
        }

        /// Get the address of the liquidity token.
        ///
        /// # Returns
        ///
        /// * `ResourceAddress` - Address of the liquidity token.
        /// 
        pub fn get_liquidity_token_resource_address(&self) -> ResourceAddress {
            self.liquidity_token_resource_manager.address()
        }

        /// Get the total supply of the liquidity token.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Total supply of the liquidity token.
        /// 
        pub fn get_liquidity_token_total_supply(&self) -> Decimal {
            self.liquidity_token_resource_manager
                .total_supply()
                .unwrap()
        }

        /// Get the address of the credit receipt.
        ///
        /// # Returns
        ///
        /// * `ResourceAddress` - Address of the credit receipt.
        ///
        pub fn get_credit_receipt_resource_address(&self) -> ResourceAddress {
            self.credit_receipt_resource_manager.address()
        }

        /// Get the protocol fee.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Protocol fee.
        ///
        pub fn get_protocol_fee(&self) -> Decimal {
            self.protocol_fee
        }

        /// Get the liquidity fee.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Liquidity fee.
        ///
        pub fn get_liquidity_fee(&self) -> Decimal {
            self.liquidity_fee
        }

        /// Get the reserve fee.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Reserve fee.
        ///
        pub fn get_reserve_fee(&self) -> Decimal {
            self.reserve_fee
        }

        /// Internal helper method to get the price of a lsu resource to xrd.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the lsu.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Price of the lsu resource to xrd.
        ///
        fn get_price_lsu_xrd(&self, resource_address: ResourceAddress) -> Option<Decimal> {
            if resource_address == XRD {
                return Some(Decimal::ONE);
            }
            self.get_validator_price_lsu_xrd(resource_address)
        }

        /// Get the price of a two resources (either lsu or xrd) against each other.
        ///
        /// # Arguments
        ///
        /// * `lhs_resource_address` - Resource address of the lhs resource.
        /// * `rhs_resource_address` - Resource address of the rhs resource.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Price of the two resources against each other.
        ///
        pub fn get_price(
            &self,
            lhs_resource_address: ResourceAddress,
            rhs_resource_address: ResourceAddress,
        ) -> Option<Decimal> {
            if lhs_resource_address == rhs_resource_address {
                return Some(Decimal::ONE);
            }

            let lhs_price = self.get_price_lsu_xrd(lhs_resource_address)?;
            let rhs_price = self.get_price_lsu_xrd(rhs_resource_address)?;
            Some(lhs_price / rhs_price)
        }

        /// Get the Credit Receipt data.
        ///
        /// # Arguments
        ///
        /// * `id` - Non fungible local id of the Credit Receipt.
        ///
        /// # Panics
        ///
        /// * Panics if the Credit Receipt does not exist.
        ///
        /// # Returns
        ///
        /// * `HashMap<ResourceAddress, Decimal>` - Credit Receipt data.
        ///
        pub fn get_nft_data(&self, id: NonFungibleLocalId) -> HashMap<ResourceAddress, Decimal> {
            self.credit_receipt_resource_manager
                .get_non_fungible_data::<CreditReceipt>(&id)
                .resources
        }

        /// Internal helper method to check the if a resource is a lsu resource.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the resource.
        ///
        /// # Returns
        ///
        /// * `Option()` - None if the resource is not a lsu resource.
        ///
        fn check_lsu_token(&self, resource_address: ResourceAddress) -> Option<()> {
            // Get validator from metadata
            let opt_global_address: Option<GlobalAddress> =
                ResourceManager::from_address(resource_address)
                    .get_metadata("validator")
                    .ok()?;

            // Get the possible validator address
            let global_address = opt_global_address?;

            // Convert to component address
            let validator_component_address: ComponentAddress = global_address.try_into().ok()?;

            // Check if the address is for a real validator
            if !self.is_validator(validator_component_address) {
                return None;
            }

            // Convert to validator
            let validator: Global<Validator> =Global::try_from(validator_component_address).ok()?;

            // Get pool unit from metadata
            let opt_global_address: Option<GlobalAddress> =validator.get_metadata("pool_unit").ok()?;
            let pool_unit_global_address = opt_global_address?;

            // Convert into resource address
            let pool_unit_resource_address: ResourceAddress =
                pool_unit_global_address.try_into().ok()?;

            // Compare to be the same
            if resource_address != pool_unit_resource_address {
                return None;
            }

            // Return ok
            Some(())
        }

        /// Check if the resource address is a lsu resource.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the resource.
        ///
        /// # Returns
        ///
        /// * `bool` - True if the resource is a lsu resource.
        ///
        pub fn is_lsu_token(&self, resource_address: ResourceAddress) -> bool {
            if self.lsu_to_validator.get(&resource_address).is_some() {
                return true;
            }

            self.check_lsu_token(resource_address).is_some()
        }

        /// Check if the component address is a native Radix validator.
        ///
        /// # Arguments
        ///
        /// * `component_address` - Component address of the component.
        ///
        /// # Returns
        ///
        /// * `bool` - True if the component is a native Radix validator.
        ///  
        pub fn is_validator(&self, component_address: ComponentAddress) -> bool {
            component_address.as_node_id().entity_type() == Some(EntityType::GlobalValidator)
        }

        /// Get the validator component address from a lsu resource.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the lsu resource.
        ///
        /// # Returns
        ///
        /// * `Option<ComponentAddress>` - Validator component address. None if not a real lsu.
        ///
        pub fn get_validator_address(
            &self,
            resource_address: ResourceAddress,
        ) -> Option<ComponentAddress> {
            // Check the map cache
            if self.lsu_to_validator.get(&resource_address).is_some() {
                return Some(*self.lsu_to_validator.get(&resource_address).unwrap());
            }

            // Check that it is a lsu token
            if !self.is_lsu_token(resource_address) {
                return None;
            }

            // Get validator from metadata
            let opt_global_address: Option<GlobalAddress> =
                ResourceManager::from_address(resource_address)
                    .get_metadata("validator")
                    .ok()?;

            // Get the possible validator address
            let global_address = opt_global_address?;

            // Convert to component address
            let validator_component_address: ComponentAddress = global_address.try_into().ok()?;

            // Check if the address is for a real validator
            if !self.is_validator(validator_component_address) {
                return None;
            }

            // Cache the validator component address
            self.lsu_to_validator.insert(resource_address, validator_component_address);

            // Return the validator component address
            Some(validator_component_address)
        }

        /// Get the validator price of lsu to xrd from a lsu resource.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the resource.
        ///
        /// # Returns
        ///
        /// * `Option<Decimal>` - Validator price of lsu to xrd. None if not a real lsu.
        ///
        pub fn get_validator_price_lsu_xrd(
            &self,
            resource_address: ResourceAddress,
        ) -> Option<Decimal> {
            // Get validator component address from lsu
            if let Some(validator_component_address) = self.get_validator_address(resource_address) {
                // Get validator
                let validator: Global<Validator> = Global::try_from(validator_component_address).ok()?;

                // Get redemption value of 1 LSU
                Some(validator.get_redemption_value(Decimal::ONE))
            } else {
                None
            }
        }

        /// Get the validator price of lsu to xrd from a lsu resource AND updates the valuation of the vault and the internal lsu price to xrd.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address of the resource.
        ///
        /// # Panics
        ///
        /// * Panics if not a real lsu.
        /// * Panics if the vault is found but the price is not found.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Validator price of lsu to xrd.
        /// 
        pub fn get_validator_price_lsu_xrd_and_update_valuation(
            &mut self,
            resource_address: ResourceAddress,
        ) -> Decimal {
            // Get price lsu/xrd from the validator
            let new_price_lsu_xrd = self.get_validator_price_lsu_xrd(resource_address).expect("Getting validator price of resource failed.");

            match (self.prices_lsu_xrd.get(&resource_address), self.vaults.get(&resource_address)) {
                (Some(old_price_lsu_xrd), Some(vault)) => {
                    // Update dex valuation
                    let change = vault.amount() * (new_price_lsu_xrd - *old_price_lsu_xrd);
                    self.dex_valuation_xrd += change;
                }
                (None, Some(_)) => panic!("Vault found, BUT Price not found."),
                _ => {}
            }

            // Update the internal prices
            // NOTE: This is the only place where the price is updated
            self.prices_lsu_xrd.insert(resource_address, new_price_lsu_xrd);

            // Return the price
            new_price_lsu_xrd
        }

        /// Updates multiple validator prices.
        ///
        /// # Arguments
        ///
        /// * `number` - Number of validators to update.
        ///
        pub fn update_multiple_validator_prices(&mut self, number: u32) {
            // Escape if there are no validators
            if self.validator_counter == 0 {
                return;
            }

            // Get pointer
            let mut current_pointer = self.validator_pointer;
            let starting_resource = self.get_validator_address_map(current_pointer);

            // Loop through the number of validators
            for _ in 0..number {
                // Increment pointer
                current_pointer = (current_pointer + 1) % self.validator_counter;

                // Get resource address
                let resource_address = self.get_validator_address_map(current_pointer);

                // If all price have need updated then break
                if resource_address == starting_resource {
                    break;
                }

                // Update the price
                self.get_validator_price_lsu_xrd_and_update_valuation(resource_address);
            }

            // Update the pointer
            self.validator_pointer = current_pointer;
        }

        /// Get the maximum number of validators before the fee is applied.
        ///
        /// # Returns
        ///
        /// * `u32` - Maximum number of validators before the fee is applied.
        ///
        pub fn get_validator_max_before_fee(&self) -> u32 {
            self.validator_max_before_fee
        }

        /// Get the number of validators with vaults.
        ///
        /// # Returns
        ///
        /// * `u32` - Number of validators with vaults.
        ///
        pub fn get_validator_counter(&self) -> u32 {
            self.validator_counter
        }

        /// Get the validator pointer, where the last validator price was updated.
        ///
        /// # Returns
        ///
        /// * `u32` - Validator pointer.
        ///
        pub fn get_validator_pointer(&self) -> u32 {
            self.validator_pointer
        }

        /// Get the validator address map. Maps the validator index to the validator LSU resource address.
        ///
        /// # Arguments
        ///
        /// * `index` - Index of the validator.
        ///
        /// # Panics
        ///
        /// * Panics if the index is out of bounds.
        ///
        /// # Returns
        ///
        /// * `ResourceAddress` - Validator component address.
        ///
        pub fn get_validator_address_map(&self, index: u32) -> ResourceAddress {
            *self.validator_address_map.get(&index).unwrap()
        }

        // CREDIT RECEIPT methods
        
        /// Get the credit receipt id and data from a credit receipt proof.
        ///
        /// # Arguments
        ///
        /// * `proof` - Credit receipt proof.
        ///
        /// # Panics
        ///
        /// * Panics if the proof is invalid.
        ///
        /// # Returns
        ///
        /// * `NonFungibleLocalId` - Credit receipt id.
        /// * `HashMap<ResourceAddress, Decimal>` - Credit receipt data.
        ///
        pub fn get_id_resources_from_credit_proof(
            &self,
            proof: Proof,
        ) -> (NonFungibleLocalId, HashMap<ResourceAddress, Decimal>) {
            // Check proof and panic is invalid
            let checked_proof = proof.check(self.credit_receipt_resource_manager.address());

            // Get the credit receipt object
            let credit_receipt: NonFungible<CreditReceipt> = checked_proof.as_non_fungible().non_fungible();

            // return the ID and data
            (
                credit_receipt.local_id().clone(),
                credit_receipt.data().resources,
            )
        }

        /// Merge the data in two credit receipts to the first credit receipt.
        ///
        /// # Arguments
        ///
        /// * `credit_proof1` - First credit receipt proof.
        /// * `credit_proof2` - Second credit receipt proof.
        ///
        /// # Panics
        ///
        /// * Panics if either proof is invalid.
        ///
        pub fn merge_credit(&mut self, credit_proof1: Proof, credit_proof2: Proof) {
            // Get id and data from the proofs
            let (id1, mut data1) = self.get_id_resources_from_credit_proof(credit_proof1);
            let (id2, data2) = self.get_id_resources_from_credit_proof(credit_proof2);

            // Loop through data2 and add to data1
            for (resource_address, amount) in data2 {
                data1
                    .entry(resource_address)
                    .and_modify(|e| *e += amount)
                    .or_insert(amount);
            }

            // Update both receipt1 with combined data
            self.credit_receipt_resource_manager
                .update_non_fungible_data(&id1, "resources", data1);

            // Update receipt2 with empty data
            let empty_data: HashMap<ResourceAddress, Decimal> = HashMap::new();
            self.credit_receipt_resource_manager
                .update_non_fungible_data(&id2, "resources", empty_data)
        }

        /// Internal method to update the valuation of the pool and emit an event.
        ///
        /// # Arguments
        ///
        /// * `change` - Change in the valuation.
        ///
        /// # Events
        ///
        /// * `ValuationChangeEvent` - Event emitted when the valuation of the pool changes.
        ///
        fn update_dex_valuation_xrd(&mut self, change: Decimal) {
            self.dex_valuation_xrd += change;

            // emit
            Runtime::emit_event(ValuationChangeEvent {
                valuation_change: change,
                valuation_after_change: self.dex_valuation_xrd,
                total_liquidity_token_supply: self.liquidity_token_resource_manager.total_supply().unwrap(),
            });
        }

        /// Deposit tokens into reserve vault.
        ///
        /// # Arguments
        ///
        /// * `tokens` - Tokens to deposit.
        ///
        pub fn deposit_reserve_fee(&mut self, tokens: Bucket) {
            if self.reserve_vaults.get(&tokens.resource_address()).is_none() {
                self.reserve_vaults.insert(tokens.resource_address(), Vault::with_bucket(tokens));
            } else {
                self.reserve_vaults.get_mut(&tokens.resource_address()).unwrap().put(tokens);
            }
        }

        /// Add liquidity to the pool.
        ///
        /// # Arguments
        ///
        /// * `bucket` - Bucket of lsu tokens to add.
        /// * `credit_proof` - Optional proof of credit receipt to add credit to.
        ///
        /// # Returns
        ///
        /// * `Bucket` - Liquidity tokens that represent a share of the pool.
        /// * `Bucket` - Credit receipt if no credit proof was given.
        /// 
        /// # Panics
        ///
        /// * If `bucket` does not contain lsu tokens.
        /// * If token validation fails for the resource address of `bucket`.
        /// 
        /// # Events
        ///
        /// * `AddLiquidityEvent` - Event emitted when liquidity is added.
        /// * `ValuationChangeEvent` - Event emitted when the valuation of the pool changes.
        ///
        pub fn add_liquidity(
            &mut self,
            mut bucket: Bucket,
            credit_proof: Option<Proof>,
        ) -> (Bucket, Bucket) {
            // Update validator prices
            self.update_multiple_validator_prices(NUMBER_VALIDATOR_PRICES_TO_UPDATE);

            // Get bucket resource
            let bucket_resource = bucket.resource_address();

            // Assert that bucket contains lsu tokens
            assert!(
                self.is_lsu_token(bucket_resource),
                "Can only add LSU tokens as liquidity."
            );

            // Validate lsu in bucket, will panic if not valid
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(bucket_resource));

            // CATCH IF TOO MANY VALIDATORS and someone trying to flood system
            // If number validators is greater than the max we charge
            if self.validator_counter > self.validator_max_before_fee {
                let num = self.validator_counter - self.validator_max_before_fee;
                let amount_to_take = Decimal::try_from(num.pow(3u32)).unwrap();
                self.deposit_reserve_fee(bucket.take(amount_to_take));
            }
            // END PENALTY

            // Return gracefully if bucket is empty
            if bucket.is_empty() {
                return (bucket, Bucket::new(bucket_resource));
            }

            // Get price of lsu/xrd
            let price_lsu_xrd = self.get_validator_price_lsu_xrd_and_update_valuation(bucket_resource);
            let bucket_valuation_xrd = bucket.amount() * price_lsu_xrd;

            // Calculate the amount of tokens to mint
            let tokens_to_mint = if self.dex_valuation_xrd == Decimal::ZERO {
                bucket.amount()
            } else {
                // Get total supply of the liquidity tokens
                let liquidity_token_supply = self.liquidity_token_resource_manager.total_supply().unwrap();
                
                // Calculate the amount of tokens to mint
                bucket_valuation_xrd / self.dex_valuation_xrd * liquidity_token_supply 
            };

            // Mint liquidity tokens
            let liquidity_tokens = self.liquidity_token_resource_manager.mint(tokens_to_mint);

            // Update the dex valuation
            self.update_dex_valuation_xrd(bucket_valuation_xrd);

            // Get the credit receipt proof if some
            let credit_receipt_bucket = if credit_proof.is_some() {
                // Get the credit receipt id and resources
                let (credit_receipt_id, mut credit_receipt_resources) = self.get_id_resources_from_credit_proof(credit_proof.unwrap());

                // Update resources data
                credit_receipt_resources
                    .entry(bucket_resource)
                    .and_modify(|e| *e += bucket.amount())
                    .or_insert(bucket.amount());

                // Update the credit receipt
                self.credit_receipt_resource_manager
                    .update_non_fungible_data(
                        &credit_receipt_id,
                        "resources",
                        credit_receipt_resources,
                    );

                // Pass back empty bucket
                Bucket::new(bucket_resource)
            } else {
                // Mint a new credit receipt
                self.credit_receipt_resource_manager
                    .mint_ruid_non_fungible(CreditReceipt {
                        resources: HashMap::<ResourceAddress, Decimal>::from([(bucket_resource, bucket.amount())]),
                    })
            };

            // Emit event
            Runtime::emit_event(AddLiquidityEvent {
                resource_address: bucket_resource,
                amount: bucket.amount(),
                liquidity_token_amount_change: liquidity_tokens.amount(),
            });

            if self.vaults.get(&bucket_resource).is_none() { // New token
                // Create new vault
                self.vaults.insert(bucket_resource, Vault::with_bucket(bucket));

                // Add to validator address map
                self.validator_address_map.insert(self.validator_counter, bucket_resource);

                // Update validator counter
                self.validator_counter += 1;
            } else { // Existing token
                // Put in vault
                self.vaults.get_mut(&bucket_resource).unwrap().put(bucket);
            }

            // Return lp tokens and credit receipt if one was created
            (liquidity_tokens, credit_receipt_bucket)
        }

        /// Remove liquidity from the pool.
        ///
        /// # Arguments
        ///
        /// * `liquidity_tokens` - Liquidity tokens used to claim lsu tokens from the pool.
        /// * `lsu_resource` - Lsu resource to payout.
        /// * `credit_proof` - Optional proof of credit receipt to use.
        /// 
        /// # Returns
        ///
        /// * `Bucket` - Lsu tokens payed out.
        /// * `Bucket` - Any remaining liquidity tokens.
        ///
        /// # Panics
        ///
        /// * Panics if `liquidity_tokens` is invalid.
        /// * Panics if the lsu resource is not in the pool.
        ///
        /// # Events
        ///
        /// * `RemoveLiquidityEvent` - Event emitted when liquidity is removed from the pool.
        /// * `ValuationChangeEvent` - Event emitted when the valuation of the pool changes.
        ///
        pub fn remove_liquidity(
            &mut self,
            mut liquidity_tokens: Bucket,
            lsu_resource: ResourceAddress,
            credit_proof: Option<Proof>,
        ) -> (Bucket, Bucket) {
            // Update validator prices
            self.update_multiple_validator_prices(NUMBER_VALIDATOR_PRICES_TO_UPDATE);

            // Assert valid liquidity tokens
            assert!(liquidity_tokens.resource_address() == self.liquidity_token_resource_manager.address(),
                "Invalid liquidity provider tokens."
            );

            // Return gracefully if bucket is empty
            if liquidity_tokens.is_empty() {
                return (Bucket::new(lsu_resource), liquidity_tokens);
            }

            // Assert we have some lsu resource to payout
            let amount_lsu_vault = self
                .vaults
                .get(&lsu_resource)
                .expect("The pool doesn't have any of this token.")
                .amount();

            // Return gracefully if vault is empty
            if amount_lsu_vault.is_zero() {
                return (Bucket::new(lsu_resource), liquidity_tokens);
            }

            // Get the total supply of the liquidity tokens
            let liquidity_token_total_supply = self
                .liquidity_token_resource_manager
                .total_supply()
                .unwrap();

            // Calculate share of pool
            let liquidity_share = liquidity_tokens.amount() / liquidity_token_total_supply;

            // Get the price of the lsu_resource in xrd from validator
            let price_lsu_xrd = self.get_validator_price_lsu_xrd_and_update_valuation(lsu_resource);

            // Dex valuation in lsu token
            let dex_valuation_lsu = self.dex_valuation_xrd / price_lsu_xrd;

            // Calculate as amount of lsu
            let amount_lsu = dex_valuation_lsu * liquidity_share;

            let (amount_lsu_payout, amount_liquidity_token_burn) =
                if amount_lsu_vault >= amount_lsu { // Enough lsu tokens
                    (amount_lsu, liquidity_tokens.amount())
                } else { // Not enough lsu tokens
                    let amount_liquidity_token_burn = amount_lsu_vault / dex_valuation_lsu * liquidity_token_total_supply;
                    (amount_lsu_vault, amount_liquidity_token_burn)
                };

            liquidity_tokens.take(amount_liquidity_token_burn).burn();

            // Calculate amount of lsu for which to charge a fee
            let amount_lsu_taxable = if credit_proof.is_some() {
                // Get the credit receipt id and resources
                let (credit_receipt_id, mut credit_receipt_resources) = self.get_id_resources_from_credit_proof(credit_proof.unwrap());

                // Calculate the amount to discount if any
                let discount_amount = match credit_receipt_resources.get_mut(&lsu_resource) {
                    Some(amount) => {
                        let discount_amount = amount_lsu_payout.min(*amount);
                        *amount -= discount_amount;
                        discount_amount
                    }
                    None => Decimal::ZERO,
                };

                // Update the credit receipt
                self.credit_receipt_resource_manager
                    .update_non_fungible_data(
                        &credit_receipt_id,
                        "resources",
                        credit_receipt_resources,
                    );
                    amount_lsu_payout - discount_amount
            } else {
                amount_lsu_payout
            };

            // Calculate liquidity fee
            let amount_liquidity_fee = amount_lsu_taxable * self.liquidity_fee;

            // Calculate amount of lsu tokens to remove from the vault
            let amount_lsu_remove = amount_lsu_payout - amount_liquidity_fee;

            // Take LSU tokens from the vault
            let mut lsu_tokens = self
                .vaults
                .get_mut(&lsu_resource)
                .unwrap()
                .take(amount_lsu_remove);

            // Charge protocol fee
            let protocol_amount = amount_lsu_taxable * self.protocol_fee;
            if protocol_amount.is_positive() {
                FEE_VAULTS.deposit(lsu_tokens.take(protocol_amount));
            }

            // Charge reserve fee
            let reserve_amount = amount_lsu_taxable * self.reserve_fee;
            if reserve_amount.is_positive() {
                self.deposit_reserve_fee(lsu_tokens.take(reserve_amount));
            }

            // Update the dex valuation
            self.update_dex_valuation_xrd(-amount_lsu_remove * price_lsu_xrd);

            // Emit event
            Runtime::emit_event(RemoveLiquidityEvent {
                resource_address: lsu_resource,
                amount: -amount_lsu_remove,
                liquidity_token_amount_change: -amount_liquidity_token_burn,
            });

            // Return lsu tokens and any remaining liquidity tokens
            (lsu_tokens, liquidity_tokens)
        }

        /// Swap a lsu token for another lsu token that is in the pool.
        ///
        /// # Arguments
        ///
        /// * `bucket` - The bucket of tokens to swap.
        /// * `lsu_paying` - The lsu token to payout.
        /// 
        ///  # Returns
        ///
        /// * `Bucket` - Lsu tokens payed out.
        /// * `Bucket` - Any remaining tokens not swapped.
        ///
        /// # Panics
        ///
        /// * If `bucket` does not contain lsu tokens.
        /// * If token validation fails for the resource address of `bucket`.
        /// * If `lsu_paying` is not an lsu token.
        ///
        /// # Events
        ///
        /// * `SwapEvent` - Event emitted when tokens are swapped with the pool.
        /// * `ValuationChangeEvent` - Event emitted when the valuation of the pool changes.
        ///
        pub fn swap(
            &mut self,
            mut bucket: Bucket,
            lsu_paying: ResourceAddress,
        ) -> (Bucket, Bucket) {
            // Update validator prices
            self.update_multiple_validator_prices(NUMBER_VALIDATOR_PRICES_TO_UPDATE);

            // Get bucket resource and amount
            let lsu_receiving = bucket.resource_address();
            let amount_receiving = bucket.amount();

            // Assert that bucket contains a lsu.
            assert!(
                self.is_lsu_token(lsu_receiving),
                "Can only swap LSU tokens."
            );

            // Validate lsu receiving, will panic if not valid
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(lsu_receiving));

            // Assert that lsu paying is a real lsu
            assert!(
                self.is_lsu_token(lsu_paying),
                "Can only swap for LSU tokens."
            );

            // Return gracefully if bucket is empty or lsu tokens are the same
            if bucket.is_empty() || lsu_receiving == lsu_paying {
                return (Bucket::new(lsu_receiving), bucket);
            }

            // Get the amount of the lsu paying in vault, if vault does not exist return gracefully
            let amount_vault_paying = if let Some(vault) = self.vaults.get(&lsu_paying) {
                vault.amount()
            } else {
                return (Bucket::new(lsu_receiving), bucket);
            };

            // Get prices of each from validator and update valuation
            let price_receiving_xrd = self.get_validator_price_lsu_xrd_and_update_valuation(lsu_receiving);
            let price_paying_xrd = self.get_validator_price_lsu_xrd_and_update_valuation(lsu_paying);

            // Get price of paying lsu token in receiving lsu token
            let price_paying_receiving = price_paying_xrd / price_receiving_xrd;

            // Calculate amount of lsu paying in vault valued in lsu receiving
            let amount_vault_paying_in_receiving = amount_vault_paying * price_paying_receiving;

            // Calculate amount of receiving tokens to swap
            let amount_swap_receiving = amount_vault_paying_in_receiving.min(amount_receiving);

            // Calculate amount of paying tokens to swap 
            let amount_swap_paying = amount_swap_receiving / price_paying_receiving; 
            let amount_remove_paying = amount_swap_paying * (Decimal::ONE - self.liquidity_fee);

            // Take lsu tokens paying from vault
            let mut lsu_tokens_paying = self
                .vaults
                .get_mut(&lsu_paying)
                .unwrap()
                .take(amount_remove_paying);

            // Charge protocol fee
            let protocol_fee_amount = amount_swap_paying * self.protocol_fee;
            if protocol_fee_amount.is_positive() {
                FEE_VAULTS.deposit(lsu_tokens_paying.take(protocol_fee_amount));
            }

            // Charge reserve fee
            let reserve_fee_amount = amount_swap_paying * self.reserve_fee;
            if reserve_fee_amount.is_positive() {
                self.deposit_reserve_fee(lsu_tokens_paying.take(reserve_fee_amount));
            }

            // Put swapped lsu tokens receiving into vault
            let lsu_tokens_receiving = bucket.take(amount_swap_receiving);

            // Emit swap event
            Runtime::emit_event(SwapEvent {
                user_sell_resource_address: lsu_receiving,
                user_sell_amount: amount_swap_receiving,
                user_buy_resource_address: lsu_paying,
                user_buy_amount: lsu_tokens_paying.amount(),
            });

            if self.vaults.get(&lsu_receiving).is_none() { // New token
                // Create new vault
                self.vaults.insert(lsu_receiving, Vault::with_bucket(lsu_tokens_receiving));

                // Add to validator address map
                self.validator_address_map.insert(self.validator_counter, lsu_receiving);

                // Update validator counter
                self.validator_counter += 1;
            } else { // Existing token
                // Put in vault
                self.vaults.get_mut(&lsu_receiving).unwrap().put(lsu_tokens_receiving);
            }

            // Update the dex valuation
            let change = amount_swap_receiving * price_receiving_xrd - amount_remove_paying * price_paying_xrd;
            self.update_dex_valuation_xrd(change);

            // Return lsu tokens payed out and any remaining tokens not swapped
            (lsu_tokens_paying, bucket)
        }
    }
}
