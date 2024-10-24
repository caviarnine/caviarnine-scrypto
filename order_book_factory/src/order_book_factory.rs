use scrypto::prelude::*;

use crate::events::*;
use crate::list::*;

pub type Pair = (ResourceAddress, ResourceAddress);

#[blueprint]
#[events(
    SetOwnerRuleDefaultEvent,
    SetUserRuleDefaultEvent,
    SetTokenValidatorEvent,
    NewOrderBookEvent,
)]
#[types(
    ComponentAddress,
    Pair,
    List<ComponentAddress>,
    u64,
)]
mod order_book_factory {    
    // Import OrderBook
    extern_blueprint!(
        "package_sim1p5tk86x78nq08k9q8hy9n7w99fv5zefkekeujkyerkrtydzunvrpzu",
        OrderBook {
            fn new(owner_rule_default: AccessRule, user_rule_default: AccessRule, token_x_address: ResourceAddress, token_y_address: ResourceAddress, reservation: Option<GlobalAddressReservation>) -> Global<OrderBook>;
            fn get_order_receipt_address(&self) -> ResourceAddress;
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
            new_order_book => restrict_to: [user];
            get_owner_rule_default => PUBLIC;
            get_user_rule_default => PUBLIC;
            get_token_validator_address => PUBLIC;
            get_order_book_count => PUBLIC;
            get_order_books => PUBLIC;
            get_order_book_pair => PUBLIC;
            get_order_books_by_pair => PUBLIC;
        }
    }

    /// Order book factory component. Used to create order books that meet the requirements of the 
    /// protocol. This involves validating tokens, setting fee components, and setting the admin badge. 
    /// Can also be used to get information about order books that have been created.
    /// 
    struct OrderBookFactory {
        /// Default access rule for owner of new order book.
        owner_rule_default: AccessRule,
        /// Default access rule for user of new order book.
        user_rule_default: AccessRule,
        /// Token validator component.
        token_validator: Global<AnyComponent>,
        /// List of order books.
        order_books_list: List<ComponentAddress>,
        /// Map of order books to token pairs.
        order_books_to_resources: KeyValueStore<ComponentAddress, (ResourceAddress, ResourceAddress)>,
        /// Map of token pairs to vector of order books.
        resources_to_order_book: KeyValueStore<(ResourceAddress, ResourceAddress), List<ComponentAddress>>,
    }

    impl OrderBookFactory {
        /// Instantiate and globalize new order book factory component with access rules.
        /// 
        /// # Arguments
        /// 
        /// * `admin_badge_address` - Admin badge resource address to set as owner.
        /// * `token_validator_address` - Token validator component address.
        /// 
        /// # Returns
        /// 
        /// * `Global<OrderBookFactory>` - The new order book factory.
        /// 
        /// # Access Rules
        /// 
        /// * `set_owner_rule_default` - Owner required.
        /// * `set_user_rule_default` - Owner required.
        /// * `set_token_validator` - Owner required.
        /// * `new_order_book` - User role required.
        /// * `get_owner_rule_default` - Public.
        /// * `get_user_rule_default` - Public.
        /// * `get_fee_vaults_address` - Public.
        /// * `get_fee_controller_address` - Public.
        /// * `get_token_validator_address` - Public.
        /// * `get_order_books_count` - Public.
        /// * `get_order_books` - Public.
        /// * `get_order_book_pair` - Public.
        /// * `get_order_books_by_pair` - Public.
        /// 
        pub fn new(
            admin_badge_address: ResourceAddress, 
            token_validator_address: ComponentAddress,
        ) -> Global<OrderBookFactory> {
            // Instantiate and globalize order book factory
            Self {
                owner_rule_default: rule!(require(admin_badge_address)),
                user_rule_default: rule!(allow_all),
                token_validator: Global::from(token_validator_address),
                order_books_list: List::new(),
                order_books_to_resources: KeyValueStore::new_with_registered_type(),
                resources_to_order_book: KeyValueStore::new_with_registered_type(),
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
        /// * `SetOwnerRuleDefaultEvent` - Event emitted when owner rule default is set.
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
        /// * `SetUserRuleDefaultEvent` - Event emitted when user rule default is set.
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
        /// * `SetTokenValidatorEvent` - Event emitted when token validator component is set.
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

        /// Get number of order books.
        /// 
        /// # Returns
        /// 
        /// * `u64` - Number of order books.
        /// 
        pub fn get_order_book_count(&self) -> u64 {
            self.order_books_list.len()
        }

        /// Get vector of order book addresses.
        /// 
        /// # Arguments
        /// 
        /// * `start` - Optional start index of range to return, included.
        /// * `end` - Optional end index of range to return, excluded.
        /// 
        /// # Returns
        /// 
        /// * `Vec<ComponentAddress>` - Vector of order book addresses.
        /// 
        pub fn get_order_books(&self, start: Option<u64>, end: Option<u64>) -> Vec<ComponentAddress> {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(self.order_books_list.len());
            
            self.order_books_list.range(start, end)
        }

        /// Get order book pair for a given order book.
        /// 
        /// # Arguments
        /// 
        /// * `order_book_address` - Order book component address.
        /// 
        /// # Returns
        /// 
        /// * `Option<(ResourceAddress, ResourceAddress)>` - Token pair if order book exists, otherwise None.
        /// 
        pub fn get_order_book_pair(&self, order_book_address: ComponentAddress) -> Option<(ResourceAddress, ResourceAddress)> {
            self.order_books_to_resources.get(&order_book_address).map(|resources| *resources)
        }

        /// Get vector of order book addresses for a given token pair.
        /// 
        /// # Arguments
        /// 
        /// * `token_x_address` - Token x resource address.
        /// * `token_y_address` - Token y resource address.
        /// * `start` - Optional start index of range to return, included.
        /// * `end` - Optional end index of range to return, excluded.
        /// 
        /// # Returns
        /// 
        /// * `Vec<ComponentAddress>` - Vector of order book addresses.
        /// 
        pub fn get_order_books_by_pair(&self, token_x_address: ResourceAddress, token_y_address: ResourceAddress, start: Option<u64>, end: Option<u64>) -> Vec<ComponentAddress> {
            if let Some(pools) = self.resources_to_order_book.get(&(token_x_address, token_y_address)) {
                let start = start.unwrap_or(0);
                let end = end.unwrap_or(pools.len());
                
                pools.range(start, end)
            } else {
                vec![]
            }
        }

        /// USER: Instantiate and globalize new order book.
        /// 
        /// # Arguments
        /// 
        /// * `token_x_address` - Token x resource address.
        /// * `token_y_address` - Token y resource address.
        /// * `reservation` - Optional global address reservation.
        /// 
        /// # Returns
        /// 
        /// * `Global<OrderBook>` - The new order book.
        /// 
        /// # Panics
        /// 
        /// * If tokens are invalid.
        /// 
        /// # Events
        /// 
        /// * `NewOrderBookEvent` - Event emitted when new order book is created.
        /// 
        pub fn new_order_book(           
            &mut self, 
            token_x_address: ResourceAddress,
            token_y_address: ResourceAddress,
            reservation: Option<GlobalAddressReservation>,
        ) -> Global<OrderBook> {
            // Validate tokens
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(token_x_address));
            self.token_validator.call_raw::<()>("validate_token", scrypto_args!(token_y_address));

            // Instantiate order book
            let order_book = Blueprint::<OrderBook>::new(
                self.owner_rule_default.clone(),
                self.user_rule_default.clone(),
                token_x_address,
                token_y_address,
                reservation,
            );

            // Insert into order books list
            self.order_books_list.push(order_book.address());

            // Insert into order books to resources map
            self.order_books_to_resources.insert(order_book.address(), (token_x_address, token_y_address));

            // Insert into resources to order books map
            let exists = self.resources_to_order_book.get(&(token_x_address, token_y_address)).is_some();
            if exists {
                let mut order_books = self.resources_to_order_book.get_mut(&(token_x_address, token_y_address)).unwrap();
                order_books.push(order_book.address());
            } else {
                let mut order_books = List::new();
                order_books.push(order_book.address());
                self.resources_to_order_book.insert((token_x_address, token_y_address), order_books);
            }

            // Emit new order book event
            Runtime::emit_event(NewOrderBookEvent {
                component_address: order_book.address(),
                order_receipt_address: order_book.get_order_receipt_address(),
                token_x_address,
                token_y_address,
            });

            order_book
        }
    }
}
