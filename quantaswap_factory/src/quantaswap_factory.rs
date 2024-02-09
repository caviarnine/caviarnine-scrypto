use scrypto::prelude::*;

use crate::events::*;
use crate::list::*;

pub type Pair = (ResourceAddress, ResourceAddress);

#[blueprint]
#[events(
    SetOwnerRuleDefaultEvent,
    SetUserRuleDefaultEvent,
    SetTokenValidatorEvent,
    NewPoolEvent,
)]
#[types(
    ComponentAddress,
    Pair,
    List<ComponentAddress>,
    u64,
)]
mod quantaswap_factory {
    // Import QuantaSwap
    extern_blueprint!(
        "package_sim1p5tk86x78nq08k9q8hy9n7w99fv5zefkekeujkyerkrtydzunvrpzu",
        QuantaSwap {
            fn new(owner_rule: AccessRule, user_rule: AccessRule, token_x_address: ResourceAddress, token_y_address: ResourceAddress, bin_span: u32, reservation: Option<GlobalAddressReservation>) -> Global<QuantaSwap>;
            fn get_liquidity_receipt_address(&self) -> ResourceAddress;
        }
    );

    // Set access rules
    enable_method_auth! { 
        roles {
            user => updatable_by: [OWNER];
        },
        methods { 
            set_owner_rule_default => restrict_to: [OWNER];
            set_user_rule_default => restrict_to: [OWNER];
            set_token_validator => restrict_to: [OWNER];
            new_pool => restrict_to: [user];
            get_owner_rule_default => PUBLIC;
            get_user_rule_default => PUBLIC;
            get_token_validator_address => PUBLIC;
            get_pool_count => PUBLIC;
            get_pools => PUBLIC;
            get_pool_pair => PUBLIC;
            get_pools_by_pair => PUBLIC;
        }
    }

    /// QuantaSwap factory component. Used to create QuantaSwap pools that meet the requirements of the 
    /// protocol. This involves validating tokens, setting fee components, and setting the admin badge. 
    /// Can also be used to get information about QuantaSwap pools that have been created.
    /// 
    struct QuantaSwapFactory {
        /// Default access rule for owner of new pool.
        owner_rule_default: AccessRule,
        /// Default access rule for user of new pool.
        user_rule_default: AccessRule,
        /// Token validator component.
        token_validator: Global<AnyComponent>,
        /// List of pools.
        pools: List<ComponentAddress>,
        /// Map of pools to token pairs.
        pools_to_resources: KeyValueStore<ComponentAddress, (ResourceAddress, ResourceAddress)>,
        /// Map of token pairs to vector of pool component addresses.
        resources_to_pools: KeyValueStore<(ResourceAddress, ResourceAddress), List<ComponentAddress>>,
    }

    impl QuantaSwapFactory {
        /// Instantiate and globalize new quantaswap factory owed by admin badge.
        /// 
        /// # Arguments
        /// 
        /// * `admin_badge_address` - Admin badge resource address to set as owner.
        /// * `token_validator_address` - Token validator component address.
        /// 
        /// # Returns
        /// 
        /// * `Global<QuantaSwapFactory>` - The QuantaSwapFactory.
        /// 
        /// # Access Rules
        /// 
        /// * `set_owner_rule_default` - Owner required.
        /// * `set_user_rule_default` - Owner required.
        /// * `set_token_validator` - Owner required.
        /// * `new_pool` - User role required.
        /// * `get_owner_rule_default` - Public.
        /// * `get_user_rule_default` - Public.
        /// * `get_fee_vaults_address` - Public.
        /// * `get_fee_controller_address` - Public.
        /// * `get_token_validator_address` - Public.
        /// * `get_pool_count` - Public.
        /// * `get_pools` - Public.
        /// * `get_pool_pair` - Public.
        /// * `get_pools_by_pair` - Public.
        /// 
        pub fn new(
            admin_badge_address: ResourceAddress, 
            token_validator_address: ComponentAddress,
        ) -> Global<QuantaSwapFactory> {
            // Instantiate and globalize order book factory
            Self {
                owner_rule_default: rule!(require(admin_badge_address)),
                user_rule_default: rule!(allow_all),
                token_validator: Global::from(token_validator_address),
                pools: List::new(),
                pools_to_resources: KeyValueStore::new_with_registered_type(),
                resources_to_pools: KeyValueStore::new_with_registered_type(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
            .roles(roles!(
                user => rule!(allow_all);
            ))
            .globalize()
        }

        /// OWNER: Set owner rule default.
        /// 
        /// # Arguments
        /// 
        /// * `owner_rule_default` - Owner rule default.
        /// 
        /// # Events
        /// 
        /// * `SetOwnerRuleDefaultEvent` - Set owner rule default event.
        /// 
        pub fn set_owner_rule_default(&mut self, owner_rule_default: AccessRule) {
            // Set owner rule default
            self.owner_rule_default = owner_rule_default;

            // Emit set owner rule default event
            Runtime::emit_event(SetOwnerRuleDefaultEvent {
                owner_rule_default: self.owner_rule_default.clone(),
            });
        }

        /// OWNER: Set user rule default.
        /// 
        /// # Arguments
        /// 
        /// * `user_rule_default` - User rule default.
        /// 
        /// # Events
        /// 
        /// * `SetUserRuleDefaultEvent` - Set user rule default event.
        /// 
        pub fn set_user_rule_default(&mut self, user_rule_default: AccessRule) {
            // Set user rule default
            self.user_rule_default = user_rule_default;

            // Emit set user rule default event
            Runtime::emit_event(SetUserRuleDefaultEvent {
                user_rule_default: self.user_rule_default.clone(),
            });
        }

        /// OWNER: Set token validator component.
        /// 
        /// # Arguments
        /// 
        /// * `token_validator_address` - Token validator component address.
        /// 
        /// # Events
        /// 
        /// * `SetTokenValidatorEvent` - Set token validator event.
        /// 
        pub fn set_token_validator(&mut self, token_validator_address: ComponentAddress) {
            // Set token validator component
            self.token_validator = Global::from(token_validator_address);

            // Emit set token validator event
            Runtime::emit_event(SetTokenValidatorEvent {
                token_validator_address: self.token_validator.address(),
            });
        }

        /// Get owner rule default.
        /// 
        /// # Returns
        /// 
        /// * `AccessRule` - Owner rule default.
        /// 
        pub fn get_owner_rule_default(&self) -> AccessRule {
            self.owner_rule_default.clone()
        }

        /// Get user rule default.
        /// 
        /// # Returns
        /// 
        /// * `AccessRule` - User rule default.
        /// 
        pub fn get_user_rule_default(&self) -> AccessRule {
            self.user_rule_default.clone()
        }

        /// Get token validator component address.
        /// 
        /// # Returns
        /// 
        /// * `ComponentAddress` - Token validator component address.
        /// 
        pub fn get_token_validator_address(&self) -> ComponentAddress {
            self.token_validator.address()
        }

        /// Get number of QuantaSwap pools.
        /// 
        /// # Returns
        /// 
        /// * `u64` - Number of QuantaSwap pools.
        /// 
        pub fn get_pool_count(&self) -> u64 {
            self.pools.len()
        }

        /// Get vector of QuantaSwap pool addresses.
        /// 
        /// # Arguments
        /// 
        /// * `start` - Optional start index of range to return, included.
        /// * `end` - Optional end index of range to return, excluded.
        /// 
        /// # Returns
        /// 
        /// * `Vec<ComponentAddress>` - Vector of QuantaSwap pool addresses.
        /// 
        pub fn get_pools(&self, start: Option<u64>, end: Option<u64>) -> Vec<ComponentAddress> {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(self.pools.len());
            
            self.pools.range(start, end)
        }

        /// Get token pair for a given QuantaSwap pool.
        /// 
        /// # Arguments
        /// 
        /// * `pool_address` - Pool component address.
        /// 
        /// # Returns
        /// 
        /// * `Option<(ResourceAddress, ResourceAddress)>` - Token pair if pool exists, otherwise None.
        /// 
        pub fn get_pool_pair(&self, pool_address: ComponentAddress) -> Option<(ResourceAddress, ResourceAddress)> {
            self.pools_to_resources.get(&pool_address).map(|resources| *resources)
        }

        /// Get vector of QuantaSwap pool addresses for a given token pair.
        /// 
        /// # Arguments
        /// 
        /// * `token_x_address` - Token x resource address.
        /// * `token_y_address` - Token y resource address.
        /// * `start` - Optional start index of range to return, included.
        /// * `end` - Optional end index of range to return, excluded.
        /// 
        pub fn get_pools_by_pair(&self, token_x_address: ResourceAddress, token_y_address: ResourceAddress, start: Option<u64>, end: Option<u64>) -> Vec<ComponentAddress> {
            if let Some(pools) = self.resources_to_pools.get(&(token_x_address, token_y_address)) {
                let start = start.unwrap_or(0);
                let end = end.unwrap_or(pools.len());
                
                pools.range(start, end)
            } else {
                vec![]
            }
        }

        /// USER: Instantiate and globalize a new QuantaSwap pool.
        /// 
        /// # Arguments
        /// 
        /// * `token_x_address` - Token x resource address.
        /// * `token_y_address` - Token y resource address.
        /// * `bin_span` - Tick width of bins for the pool.
        /// * `reservation` - Optional global address reservation.
        /// 
        /// # Returns
        /// 
        /// * `Global<QuantaSwap>` - The new QuantaSwap pool.
        /// 
        /// # Panics
        /// 
        /// * If tokens are invalid.
        /// 
        /// # Events
        /// 
        /// * `NewPoolEvent` - New pool event.
        /// 
        pub fn new_pool(           
            &mut self, 
            token_x_address: ResourceAddress,
            token_y_address: ResourceAddress,
            bin_span: u32,
            reservation: Option<GlobalAddressReservation>,
        ) -> Global<QuantaSwap> {
            // Validate tokens
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(token_x_address));
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(token_y_address));

            // Instantiate QuantaSwap pool component
            let pool: Global::<QuantaSwap> = Blueprint::<QuantaSwap>::new(
                self.owner_rule_default.clone(),
                self.user_rule_default.clone(),
                token_x_address,
                token_y_address,
                bin_span,
                reservation,
            );

            // Insert into pools list
            self.pools.push(pool.address());

            // Insert into pool to resources map
            self.pools_to_resources.insert(pool.address(), (token_x_address, token_y_address));
            
            // Insert into resources to pools map
            let exists = self.resources_to_pools.get_mut(&(token_x_address, token_y_address)).is_some();
            if exists {
                let mut pools = self.resources_to_pools.get_mut(&(token_x_address, token_y_address)).unwrap();
                pools.push(pool.address());
            } else {
                let mut pools = List::new();
                pools.push(pool.address());
                self.resources_to_pools.insert((token_x_address, token_y_address), pools);
            }

            // Emit new pool event
            Runtime::emit_event(NewPoolEvent {
                component_address: pool.address(),
                liquidity_receipt_address: pool.get_liquidity_receipt_address(),
                token_x_address,
                token_y_address,
                bin_span,
            });

            // Return QuantaSwap pool
            pool
        }
    }
}
