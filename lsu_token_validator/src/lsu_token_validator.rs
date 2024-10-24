use scrypto::prelude::*;

use crate::events::*;

pub type Void = ();

#[blueprint]
#[events(
    UpdateActiveSetEvent,
    SetRequireActiveEvent,
)]
#[types(
    ResourceAddress,
    Void,
)]
mod lsu_token_validator {
    enable_method_auth! {
        methods {
            update_active_set => restrict_to: [OWNER];
            set_require_active => restrict_to: [OWNER];
            get_in_active_set => PUBLIC;
            get_is_lsu_token => PUBLIC;
            get_require_active => PUBLIC;
            validate_token => PUBLIC;
        }
    }

    struct LsuTokenValidator {
        /// Set of LSUs that are for active validators.
        active_set: KeyValueStore<ResourceAddress, ()>,
        /// If being in active set is required for token validation.
        require_active: bool,
    }

    impl LsuTokenValidator {
        /// Instantiate and globalize a lsu token validator component owned by admin badge.
        /// 
        /// # Arguments
        /// 
        /// * `admin_badge_address` - Address of admin badge.
        /// 
        /// # Returns
        /// 
        /// * `Global<LsuValidator>` - Globalized lsu token validator component.
        /// 
        /// # Access Rules
        /// 
        /// * `update_active_set` - Owner required.
        /// * `set_require_active` - Owner required.
        /// * `get_in_active_set` - Public.
        /// * `get_is_lsu_token` - Public.
        /// * `get_require_active` - Public.
        /// * `validate_token` - Public.
        /// 
        pub fn new(
            admin_badge_address: ResourceAddress,
        ) -> Global<LsuTokenValidator> {
            Self {
                active_set: KeyValueStore::new_with_registered_type(),
                require_active: true,
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
                .globalize()
        }

        /// OWNER: Update if lsu is active or not.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` - Resource address of lsu to update.
        /// * `contain` - If the lsu should be in the active set or not.
        /// 
        /// # Panics
        /// 
        /// * If the resource address is not for a lsu.
        /// 
        /// # Events
        /// 
        /// * `UpdateActiveSetEvent` - Event emitted when the active set is updated.
        /// 
        pub fn update_active_set(&mut self, resource_address: ResourceAddress, contain: bool) {
            // Update active set
            if contain {
                assert!(
                    self.get_is_lsu_token(resource_address),
                    "Can only add LSUs to the active set."
                );

                self.active_set.insert(resource_address, ());
            } else {
                self.active_set.remove(&resource_address);
            }

            // Emit update active set event
            Runtime::emit_event(UpdateActiveSetEvent {
                resource_address,
                contain,
            });
        }

        /// OWNER: Set if being in active set is required for token validation.
        /// 
        /// # Arguments
        /// 
        /// * `require_active` - If being in active set is required for token validation.
        /// 
        /// # Events
        /// 
        /// * `SetRequireActiveEvent` - Event emitted when require active is set.
        /// 
        pub fn set_require_active(&mut self, require_active: bool) {
            self.require_active = require_active;

            // Emit set require active event
            Runtime::emit_event(SetRequireActiveEvent {
                require_active,
            });
        }

        /// Get if resource address is in active set.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` - Resource address to check.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If resource address is in active set.
        /// 
        pub fn get_in_active_set(&self, resource_address: ResourceAddress) -> bool {
            self.active_set.get(&resource_address).is_some()
        }

        /// Check if the resource address is a lsu.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address to check.
        ///
        /// # Returns
        ///
        /// * `bool` - True if the token is a lsu.
        ///
        pub fn get_is_lsu_token(&self, resource_address: ResourceAddress) -> bool {
            self.check_lsu_token(resource_address).is_some()
        }

        /// Get if it is required to be in active set for token validation.
        /// 
        /// # Returns
        /// 
        /// * `bool` - If it is required to be in active set for token validation.
        /// 
        pub fn get_require_active(&self) -> bool {
            self.require_active
        }

        /// Validate resource address.
        /// 
        /// # Arguments
        /// 
        /// * `resource_address` - Resource address to validate.
        /// 
        /// # Panics
        /// 
        /// * If token validation fails.
        /// 
        pub fn validate_token(&self, resource_address: ResourceAddress) {
            // If active set is required
            if self.require_active {
                // Assert in active set
                assert!(
                    self.get_in_active_set(resource_address), 
                    "LSU must be for an active validator."
                );
            }
        }

        /// Internal helper method to check the if a resource address is a lsu.
        ///
        /// # Arguments
        ///
        /// * `resource_address` - Resource address to check.
        ///
        /// # Returns
        ///
        /// * `Option()` - None if the resource address is not a lsu.
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
            if validator_component_address.as_node_id().entity_type() != Some(EntityType::GlobalValidator) {
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
    }
}


