use scrypto::prelude::*;

use crate::events::*;

pub type Void = ();

#[blueprint]
#[events(
    UpdateWhiteListEvent,
    UpdateBlackListEvent,
    SetRestrictRecallableEvent,
    SetRestrictFreezableEvent,
    SetMinimumDivisibilityEvent,
)]
#[types(
    ResourceAddress,
    Void,
)]
mod token_validator_mod {
    enable_method_auth! {
        methods {
            update_white_list => restrict_to: [OWNER];
            update_black_list => restrict_to: [OWNER];
            set_restrict_recallable => restrict_to: [OWNER];
            set_restrict_freezable => restrict_to: [OWNER];
            set_minimum_divisibility => restrict_to: [OWNER];
            get_white_listed => PUBLIC;
            get_black_listed => PUBLIC;
            get_restrict_recallable => PUBLIC;
            get_restrict_freezable => PUBLIC;
            get_minimum_divisibility => PUBLIC;
            validate_token => PUBLIC;
        }
    }

    struct TokenValidator {
        /// Whitelisted tokens.
        white_list: KeyValueStore<ResourceAddress, ()>,
        /// Blacklisted tokens.
        black_list: KeyValueStore<ResourceAddress, ()>,
        /// If recallable tokens are restricted to require whitelist.
        restrict_recallable: bool,
        /// If freezable tokens are restricted to require whitelist.
        restrict_freezable: bool,
        /// Minimum divisibility required for tokens.
        minimum_divisibility: u8,
    }

    impl TokenValidator {
        /// Instantiate and globalize a token validator component owned by admin badge.
        /// 
        /// # Arguments
        /// 
        /// * `admin_badge_address` - Admin badge resource address to set as owner.
        /// 
        /// # Returns
        /// 
        /// * `Global<TokenValidator>` - The token validator.
        /// 
        /// # Access Rules
        /// 
        /// * `update_white_list` - Owner required.
        /// * `update_black_list` - Owner required.
        /// * `set_restrict_recallable` - Owner required.
        /// * `set_restrict_freezable` - Owner required.
        /// * `set_minimum_divisibility` - Owner required.
        /// * `get_white_listed` - Public.
        /// * `get_black_listed` - Public.
        /// * `get_restrict_recallable` - Public.
        /// * `get_restrict_freezable` - Public.
        /// * `get_minimum_divisibility` - Public.
        /// * `validate_token` - Public.
        /// 
        pub fn new(admin_badge_address: ResourceAddress) -> Global<TokenValidator> {
            Self::new_local()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
            .globalize()
        }

        /// Instantiate a token validator component.
        /// 
        /// # Returns
        /// 
        /// * `Owned<TokenValidator>` - The token validator.
        /// 
        pub fn new_local() -> Owned<TokenValidator> {
            Self {
                white_list: KeyValueStore::new_with_registered_type(),
                black_list: KeyValueStore::new_with_registered_type(),
                restrict_recallable: true,
                restrict_freezable: true,
                minimum_divisibility: 6,
            }
            .instantiate()
        }

        /// OWNER: Update whitelist status of resource address.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` - Resource address to update.
        /// * `contain` - If token should be in white list or not.
        /// 
        /// # Events
        /// 
        /// * `UpdateWhiteListEvent` - Event emitted when whitelist is updated.
        /// 
        pub fn update_white_list(&mut self, resource_address: ResourceAddress, contain: bool) {
            // Update white list
            if contain {
                self.white_list.insert(resource_address, ());
            } else {
                self.white_list.remove(&resource_address);
            }

            // Emit update white list event
            Runtime::emit_event(UpdateWhiteListEvent {
                resource_address,
                contain,
            });
        }

        /// OWNER: Update blacklist status of resource address.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` - Resource address to update.
        /// * `contain` - If token should be in black list or not.
        /// 
        /// # Events
        /// 
        /// * `UpdateBlackListEvent` - Event emitted when blacklist is updated.
        /// 
        pub fn update_black_list(&mut self, resource_address: ResourceAddress, contain: bool) {
            // Update black list
            if contain {
                self.black_list.insert(resource_address, ());
            } else {
                self.black_list.remove(&resource_address);
            }

            // Emit update black list event
            Runtime::emit_event(UpdateBlackListEvent {
                resource_address,
                contain,
            });
        }

        /// OWNER: Set if recallable tokens are restricted.
        /// 
        /// # Arguments
        /// 
        /// * `restrict` - If recallable tokens are restricted.
        /// 
        /// # Events
        /// 
        /// * `SetRestrictRecallableEvent` - Event emitted when recallable restriction is set.
        /// 
        pub fn set_restrict_recallable(&mut self, restrict: bool) {
            // Set restrict recallable
            self.restrict_recallable = restrict;

            // Emit set restrict recallable event
            Runtime::emit_event(SetRestrictRecallableEvent { 
                restrict 
            });
        }

        /// OWNER: Set if freezable tokens are restricted.
        /// 
        /// # Arguments
        /// 
        /// * `restrict` - If freezable tokens are restricted.
        /// 
        /// # Events
        /// 
        /// * `SetRestrictFreezableEvent` - Event emitted when freezable restriction is set.
        /// 
        pub fn set_restrict_freezable(&mut self, restrict: bool) {
            // Set restrict freezable
            self.restrict_freezable = restrict;

            // Emit set restrict freezable event
            Runtime::emit_event(SetRestrictFreezableEvent { 
                restrict 
            });
        }

        /// OWNER: Set minimum divisibility required for tokens.
        /// 
        /// # Arguments
        /// 
        /// * `minimum_divisibility` - Minimum divisibility required for tokens.
        /// 
        /// # Events
        /// 
        /// * `SetMinimumDivisibilityEvent` - Event emitted when minimum divisibility is set.
        /// 
        pub fn set_minimum_divisibility(&mut self, minimum_divisibility: u8) {
            // Set minimum divisibility
            self.minimum_divisibility = minimum_divisibility;
        
            // Emit set minimum divisibility event
            Runtime::emit_event(SetMinimumDivisibilityEvent { 
                minimum_divisibility 
            });
        }

        /// Get if token address is whitelisted.
        /// 
        /// # Arguments
        /// 
        /// * `token_address` - Token address to check.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If token address is whitelisted.
        /// 
        pub fn get_white_listed(&self, token_address: ResourceAddress) -> bool {
            self.white_list.get(&token_address).is_some()
        }

        /// Get if token address is blacklisted.
        /// 
        /// # Arguments
        /// 
        /// * `token_address` - Token address to check.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If token address is blacklisted.
        /// 
        pub fn get_black_listed(&self, token_address: ResourceAddress) -> bool {
            self.black_list.get(&token_address).is_some()
        }

        /// Get if recallable tokens are restricted.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If recallable tokens are restricted.
        /// 
        pub fn get_restrict_recallable(&self) -> bool {
            self.restrict_recallable
        }

        /// Get if freezable tokens are restricted.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If freezable tokens are restricted.
        /// 
        pub fn get_restrict_freezable(&self) -> bool {
            self.restrict_freezable
        }

        /// Get minimum divisibility required for tokens.
        /// 
        /// # Returns
        /// 
        /// * `u8` - Minimum divisibility required for tokens.
        /// 
        pub fn get_minimum_divisibility(&self) -> u8 {
            self.minimum_divisibility
        }

        /// Validate token address.
        /// 
        /// # Arguments
        /// 
        /// * `token_address` - Token address to validate.
        /// 
        /// # Panics
        /// 
        /// * If token validation fails.
        /// 
        pub fn validate_token(&self, token_address: ResourceAddress) {
            // Check if token is blacklisted
            assert!(
                self.black_list.get(&token_address).is_none(), 
                "Token is blacklisted."
            );

            let token_manager = ResourceManager::from(token_address);

            // Check if token is fungible
            assert!(
                token_manager.resource_type().is_fungible(),
                "Token is not fungible."
            );

            // Get white list status
            let white_listed = self.white_list.get(&token_address).is_some();

            // Check if token is divisible
            assert!(
                token_manager.resource_type().divisibility().unwrap() >= self.minimum_divisibility,
                "Token is not divisible by at least {} decimals.",
                self.minimum_divisibility
            );
            
            // Check recallable
            let recaller_role = token_manager.get_role(RECALLER_ROLE);
            let recallable_some = recaller_role.is_some() && recaller_role.unwrap() != AccessRule::DenyAll;
            let recaller_updater_role = token_manager.get_role(RECALLER_UPDATER_ROLE);
            let recall_updatable_some = recaller_updater_role.is_some() && recaller_updater_role.unwrap() != AccessRule::DenyAll;
            if self.restrict_recallable && (recallable_some || recall_updatable_some) {
                assert!(white_listed, "Only whitelisted tokens are allowed to be recallable.")
            }

            // Check freezable
            let freezer_role = token_manager.get_role(FREEZER_ROLE);
            let freezable_some = freezer_role.is_some() && freezer_role.unwrap() != AccessRule::DenyAll;
            let freezer_updater_role = token_manager.get_role(FREEZER_UPDATER_ROLE);
            let freeze_updatable_some = freezer_updater_role.is_some() && freezer_updater_role.unwrap() != AccessRule::DenyAll;
            if self.restrict_freezable && (freezable_some || freeze_updatable_some) {
                assert!(white_listed, "Only whitelisted tokens are allowed to be freezable.")
            }
            
            // Check depositable
            let depositor_role = token_manager.get_role(DEPOSITOR_ROLE);
            let depositable_any = depositor_role.is_none() || depositor_role.unwrap() == AccessRule::AllowAll;
            let depositor_updater_role = token_manager.get_role(DEPOSITOR_UPDATER_ROLE);
            let deposit_updatable_some = depositor_updater_role.is_some() && depositor_updater_role.unwrap() != AccessRule::DenyAll;
            assert!(depositable_any, "Token is not depositable.");
            if self.restrict_freezable && deposit_updatable_some {
                assert!(white_listed, "Only whitelisted tokens are allowed to have depositable updatable.")
            }

            // Check withdrawable
            let withdrawer_role = token_manager.get_role(WITHDRAWER_ROLE);
            let withdrawable_any = withdrawer_role.is_none() || withdrawer_role.unwrap() == AccessRule::AllowAll;
            let withdrawer_updater_role = token_manager.get_role(WITHDRAWER_UPDATER_ROLE);
            let withdraw_updatable_some = withdrawer_updater_role.is_some() && withdrawer_updater_role.unwrap() != AccessRule::DenyAll;
            assert!(withdrawable_any, "Token is not withdrawable.");
            if self.restrict_freezable && withdraw_updatable_some {
                assert!(white_listed, "Only whitelisted tokens are allowed to have withdrawable updatable.")
            }
        }
    }
}
