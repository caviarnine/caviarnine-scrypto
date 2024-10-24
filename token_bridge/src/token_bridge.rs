use scrypto::prelude::*;

use crate::events::*;

#[blueprint]
#[events(BridgeEvent)]
mod token_bridge_mod {
    struct TokenBridge {
        /// Vault to hold old tokens. Token can never be removed from this vault assuming resource permissions.
        old_token_resource_vault: Vault,
        /// Vault to hold new unclaimed tokens.
        new_token_resource_vault: Vault,
    }

    impl TokenBridge {
        /// Instantiate and globalize a new token bridge owned by admin badge.
        ///
        /// # Arguments:
        ///
        /// * `admin_badge_address` - Admin badge resource address to set as owner of bridge and new token.
        /// * `resource_address` - The resource address of the old tokens to be bridged.
        /// * `bridge_token_name` - The name of the new token.
        /// * `bridge_token_symbol` - The symbol of the new token.
        /// * `bridge_token_description` - The description of the new token.
        ///
        /// # Returns:
        ///
        /// * `Global<TokenBridge>` - The new token bridge.
        ///
        /// # Requires:
        ///
        /// * `resource_address` - Is fungible.
        /// * `resource_address` - Has to have a total supply greater than 0.
        ///
        pub fn new(
            admin_badge_address: ResourceAddress,
            resource_address: ResourceAddress,
            bridge_token_name: String,
            bridge_token_symbol: String,
            bridge_token_description: String,
        ) -> Global<TokenBridge> {
            Self::new_local(
                admin_badge_address,
                resource_address,
                bridge_token_name,
                bridge_token_symbol,
                bridge_token_description,
            )
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
            .globalize()
        }

        /// Instantiate a new token bridge.
        ///
        /// # Arguments:
        ///
        /// * `admin_badge_address` - Admin badge resource address to set as owner of the new token.
        /// * `resource_address` - The resource address of the old tokens to be bridged.
        /// * `bridge_token_name` - The name of the new token.
        /// * `bridge_token_symbol` - The symbol of the new token.
        /// * `bridge_token_description` - The description of the new token.
        ///
        /// # Returns:
        ///
        /// * `Owned<TokenBridge>` - The new token bridge.
        ///
        /// # Panics
        ///
        /// * If the resource address is not fungible.
        /// * If the resource address has a total supply of 0.
        ///
        pub fn new_local(
            admin_badge_address: ResourceAddress,
            resource_address: ResourceAddress,
            bridge_token_name: String,
            bridge_token_symbol: String,
            bridge_token_description: String,
        ) -> Owned<TokenBridge> {
            // Get the resource manager for the old tokens
            let resource_manager = ResourceManager::from(resource_address);

            // Assert is fungible
            assert!(
                resource_manager.resource_type().is_fungible(),
                "Must be fungible."
            );

            // Get the total supply of the old tokens
            let total_supply = resource_manager.total_supply().expect("Token not found.");

            // Assert total supply is greater than 0
            assert!(
                total_supply > dec!(0),
                "Invalid total supply. Total supply must be greater than 0."
            );

            // Create the new bridge token
            let new_tokens = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(
                admin_badge_address
            ))))
            .divisibility(DIVISIBILITY_MAXIMUM)
            .metadata(metadata!(
                init {
                    "name" => bridge_token_name, updatable;
                    "symbol" => bridge_token_symbol, updatable;
                    "description" => bridge_token_description, updatable;
                }
            ))
            .burn_roles(burn_roles! {
                burner => rule!(allow_all);
                burner_updater => rule!(deny_all);
            })
            .mint_initial_supply(total_supply);

            Self {
                old_token_resource_vault: Vault::new(resource_address),
                new_token_resource_vault: Vault::with_bucket(new_tokens.into()),
            }
            .instantiate()
        }

        /// Get the old token resource address.
        ///
        /// # Returns
        ///
        /// * `ResourceAddress` - The old token resource address.
        ///
        pub fn get_old_token_address(&self) -> ResourceAddress {
            self.old_token_resource_vault.resource_address()
        }

        /// Get the new token resource address.
        ///
        /// # Returns
        ///
        /// * `ResourceAddress` - The new token resource address.
        ///
        pub fn get_new_token_address(&self) -> ResourceAddress {
            self.new_token_resource_vault.resource_address()
        }

        /// Get the amount of locked old tokens.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The amount of unclaimed new tokens.
        ///
        pub fn get_old_tokens_amount(&self) -> Decimal {
            self.old_token_resource_vault.amount()
        }

        /// Get the amount of unclaimed new tokens.
        ///
        /// # Returns
        ///
        /// * `Decimal` - The amount of unclaimed new tokens.
        ///  
        pub fn get_new_tokens_amount(&self) -> Decimal {
            self.new_token_resource_vault.amount()
        }

        /// Bridge old tokens for new tokens
        ///
        /// # Arguments
        ///
        /// * `tokens` - The old tokens to be bridged.
        ///
        /// # Returns
        ///
        /// * `Bucket` - The new tokens.
        ///
        /// # Panic
        ///
        /// * If the tokens are not of the correct type.
        ///
        /// # Events
        ///
        /// * `BridgeEvent` - Event emitted when tokens are bridged.
        ///
        pub fn bridge(&mut self, tokens: Bucket) -> Bucket {
            // Assert that the tokens are of the correct type
            assert!(
                tokens.resource_address() == self.old_token_resource_vault.resource_address(),
                "Invalid token type."
            );

            // Get amount
            let amount = tokens.amount();

            // Emit bridge event
            Runtime::emit_event(BridgeEvent { amount });

            // Deposit old tokens
            self.old_token_resource_vault.put(tokens);

            // Withdraw and return new tokens
            self.new_token_resource_vault.take(amount)
        }
    }
}
