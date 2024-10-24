use scrypto::prelude::*;

use crate::events::*;
use crate::limit::*;
use crate::order_receipt::*;
use crate::price::*;
use crate::price_index::*;
use crate::order_status::*;

#[blueprint]
#[events(
    NewOrderBookEvent,
    LimitOrderEvent, 
    MarketOrderEvent, 
    ClaimOrderEvent,
    ProtocolFeeEvent,
)]
#[types(
    Price,
    Limit,
    OrderReceipt,
    u32,
    IndexNode,
)]
mod order_book {
    // Import FeeController to get fee percentage.
    extern_blueprint!(
        "package_sim1pkyls09c258rasrvaee89dnapp2male6v6lmh7en5ynmtnavqdsvk9",
        FeeController {
            fn get_protocol_fee(&self, package_address: PackageAddress) -> Decimal;
        }
    );

    // Import Fee Vaults to send fee tokens to.
    extern_blueprint!(
        "package_sim1p4nhxvep6a58e88tysfu0zkha3nlmmcp6j8y5gvvrhl5aw47jfsxlt",
        FeeVaults {
            fn deposit(&self, tokens: Bucket);
        }
    );

    // Set fee controller component.
    const FEE_CONTROLLER: Global<FeeController> = global_component!(
        FeeController,
        "component_sim1czc0e8f9yhlvpv38s2ymrplu7q366y3k8zc53zf2srlm7qm604g029"
    );

    // Set fee vaults component.
    const FEE_VAULTS: Global<FeeVaults> = global_component!(
        FeeVaults,
        "component_sim1crjv96xvssf2sw3c2tmxeajlt0tap07p8lx2apa7hkktsppz838kw6"
    );

    // Set access rules
    enable_method_auth! { 
        roles {
            user => updatable_by: [OWNER];
        },
        methods { 
            limit_order => restrict_to: [user];
            limit_order_batch => restrict_to: [user];
            market_order => restrict_to: [user];
            claim_orders => PUBLIC;
            get_fee_controller_address => PUBLIC;
            get_fee_vaults_address => PUBLIC;
            get_token_x_address => PUBLIC;
            get_token_y_address => PUBLIC;
            get_order_receipt_address => PUBLIC;
            get_amount_x => PUBLIC;
            get_amount_y => PUBLIC;
            get_last_price => PUBLIC;
            get_current_ask_price => PUBLIC;
            get_current_bid_price => PUBLIC;
            get_ask_limits => PUBLIC;
            get_bid_limits => PUBLIC;
            get_order_status => PUBLIC;
            get_order_statuses => PUBLIC;
        }
    }

    // Set withdraw strategies
    const INCOMING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToPositiveInfinity);
    const OUTGOING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToZero);

    /// Order book component that creates a market for two fungible tokens. 
    /// Limit orders are placed at a specific price and executed in a FIFO ordering at that price. 
    /// There can never be overlap between ask and bid limits. Market orders execute 
    /// against the best available limit orders. When a limit order is placed an order 
    /// receipt is minted. The order receipt can be used to claim tokens from the limit 
    /// order. This consumes the order receipt. Limit orders are bundled together into a 
    /// limit. This allows for efficient execution of market orders. A price index is used 
    /// to keep track of available limit prices. The price index is a efficient data
    /// structure that allows for quick lookup of the best available limit prices and has 
    /// a low cost to maintain. Protocol fees are set by the external fee controller component 
    /// and collected by the external fee vaults component.
    /// 
    /// Price is calculated as `tokens_y / tokens_x`.
    /// Amounts are tracked in tokens x.
    /// 
    struct OrderBook {
        /// Number used to create order ids. The value 0 is reserved as a special value and is used as a null id.
        nonce: u64,
        /// Last price a limit was filled at.
        last_price: Price,
        /// Index of active prices for the order book.
        price_index: PriceIndex,
        /// Map of prices to ask limits. Ask limits sell tokens x for tokens y.
        ask_limit_map: KeyValueStore<Price, Limit>,
        /// Map of prices to bid limits. Bid limits buy tokens x for tokens y.
        bid_limit_map: KeyValueStore<Price, Limit>,
        /// Order receipt manager for minting and updating order receipts.
        order_receipt_manager: ResourceManager,
        /// Vault of tokens x. Includes both tokens from unfilled ask limit orders and filled bid limit orders.
        tokens_x: Vault,
        /// Vault of tokens y. Includes both tokens from unfilled bid limit orders and filled ask limit orders.
        tokens_y: Vault,
    }

    impl OrderBook {
        /// Instantiate and globalize new order book with access rules.
        /// 
        /// # Arguments
        /// 
        /// * `owner_rule` - Owner access rule.
        /// * `user_rule` - User role access rule.
        /// * `token_x_address` - Address of token x.
        /// * `token_y_address` - Address of token y.
        /// * `reservation` - Optional address reservation for component.
        /// 
        /// # Returns
        /// 
        /// * `Global<OrderBook>` - Order book component.
        /// 
        /// # Access Rules
        /// 
        /// * `limit_order` - User required.
        /// * `limit_order_batch` - User required
        /// * `market_order` - User required.
        /// * `claim_order` - Public.
        /// * `get_fee_controller_address` - Public.
        /// * `get_fee_vaults_address` - Public.
        /// * `get_token_x_address` - Public.
        /// * `get_token_y_address` - Public.
        /// * `get_order_receipt_address` - Public.
        /// * `get_amount_x` - Public.
        /// * `get_amount_y` - Public.
        /// * `get_last_price` - Public.
        /// * `get_current_ask_price` - Public.
        /// * `get_current_bid_price` - Public.
        /// * `get_ask_limits` - Public.
        /// * `get_bid_limits` - Public.
        /// * `get_order_status` - Public.
        /// * `get_order_statuses` - Public.
        /// 
        /// # Events
        /// 
        /// * `NewOrderBookEvent` - Event emitted when order book is created.
        /// 
        pub fn new(
            owner_rule: AccessRule,
            user_rule: AccessRule,
            token_x_address: ResourceAddress,
            token_y_address: ResourceAddress,
            reservation: Option<GlobalAddressReservation>
        ) -> Global<OrderBook> {
            // Get component address
            let (address_reservation, component_address) = match reservation {
                Some(reservation) => {
                    let component_address: ComponentAddress = Runtime::get_reservation_address(&reservation).try_into().unwrap();
                    (reservation, component_address)
                }
                None => {
                    Runtime::allocate_component_address(OrderBook::blueprint_id())
                }
            };

            // Get tokens symbols
            let token_x_manager = ResourceManager::from_address(token_x_address);
            let token_y_manager = ResourceManager::from_address(token_y_address);
            let symbol_x: String = token_x_manager.get_metadata::<String, String>("symbol".to_string()).unwrap_or_default().unwrap_or_default();
            let symbol_y: String = token_y_manager.get_metadata::<String, String>("symbol".to_string()).unwrap_or_default().unwrap_or_default();

            // Create order receipt resource
            let order_receipt_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<OrderReceipt>(OwnerRole::Updatable(owner_rule.clone()))
                .metadata(metadata!(
                    init {
                        "package" => GlobalAddress::from(Runtime::package_address()), locked;
                        "component" => GlobalAddress::from(component_address), locked;
                        "token_x" => GlobalAddress::from(token_x_address), locked;
                        "token_y" => GlobalAddress::from(token_y_address), locked;
                        "name" => format!("Order Receipt {}/{}", symbol_x, symbol_y), updatable;
                        "description" => format!("Used to claim tokens from a limit order for pair {}/{}.", symbol_x, symbol_y), updatable;
                        "tags" => vec!["defi", "dex", "order book", "receipt"], updatable;
                    }
                ))
                .mint_roles(mint_roles!{
                        minter => rule!(require(global_caller(component_address))); 
                        minter_updater => rule!(deny_all);
                    }
                )
                .burn_roles(burn_roles!{
                        burner => rule!(require(global_caller(component_address))); 
                        burner_updater => rule!(deny_all);
                    }
                )
                .non_fungible_data_update_roles(non_fungible_data_update_roles!{
                        non_fungible_data_updater => rule!(require(global_caller(component_address))); 
                        non_fungible_data_updater_updater => rule!(deny_all);
                    }
                )
                .create_with_no_initial_supply();

            // Emit new order book event
            Runtime::emit_event(NewOrderBookEvent {
                component_address,
                order_receipt_address: order_receipt_manager.address(),
                token_x_address,
                token_y_address,
            });

            // Instantiate and globalize order book
            Self {
                nonce: 1,
                last_price: Price::MIN,
                price_index: PriceIndex::new(),
                ask_limit_map: KeyValueStore::new_with_registered_type(),
                bid_limit_map: KeyValueStore::new_with_registered_type(),
                order_receipt_manager,
                tokens_x: Vault::new(token_x_address),
                tokens_y: Vault::new(token_y_address),
            }.instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(owner_rule))
            .metadata(metadata!(
                init {
                    "token_x" => GlobalAddress::from(token_x_address), locked;
                    "token_y" => GlobalAddress::from(token_y_address), locked;
                    "order_receipt" => GlobalAddress::from(order_receipt_manager.address()), locked;
                    "name" => format!("Order Book {}/{}", symbol_x, symbol_y), updatable;
                    "description" => format!("Order book for pair {}/{}.", symbol_x, symbol_y), updatable;
                    "tags" => vec!["defi", "dex", "order book"], updatable;
                }
            ))
            .with_address(address_reservation)
            .roles(roles!(
                user => user_rule;
            ))
            .globalize()
        }

        /// Get component address of fee controller.
        /// 
        /// # Returns
        /// 
        /// * `ComponentAddress` - Component address of fee controller.
        /// 
        pub fn get_fee_controller_address(&self) -> ComponentAddress {
            FEE_CONTROLLER.address()
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

        /// Get the address of token x.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of token x.
        /// 
        pub fn get_token_x_address(&self) -> ResourceAddress {
            self.tokens_x.resource_address()
        }

        /// Get the address of token y.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of token y.
        /// 
        pub fn get_token_y_address(&self) -> ResourceAddress {
            self.tokens_y.resource_address()
        }

        /// Get the order receipt address.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of order_receipt.
        /// 
        pub fn get_order_receipt_address(&self) -> ResourceAddress {
            self.order_receipt_manager.address()
        }

        /// Get amount of tokens x.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens x.
        /// 
        pub fn get_amount_x(&self) -> Decimal {
            self.tokens_x.amount()
        }

        /// Get amount of tokens y.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens y.
        /// 
        pub fn get_amount_y(&self) -> Decimal {
            self.tokens_y.amount()
        }

        /// Get last price a limit order was filled at.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// 
        /// # Returns
        ///
        /// * `Decimal` - Last price a limit order was filled at.
        /// 
        pub fn get_last_price(&self) -> Decimal {
            self.last_price.into()
        }

        /// Get best current ask price.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// 
        /// # Returns
        /// 
        /// * `Option<Decimal>` - Best current ask price.
        /// 
        pub fn get_current_ask_price(&self) -> Option<Decimal> {
            self.price_index.current_ask().map(|price| price.into())
        }

        /// Get best current bid price.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// 
        /// # Returns
        /// 
        /// * `Option<Decimal>` - Best current bid price.
        /// 
        pub fn get_current_bid_price(&self) -> Option<Decimal> {
            self.price_index.current_bid().map(|price| price.into())
        }

        /// Get ask limits.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Amount is calculated in tokens x.
        /// 
        /// # Arguments
        /// 
        /// * `start_price` - Optional start price, not inclusive. If none, start at current ask price.
        /// * `stop_price` - Optional stop price, inclusive. If none, return all limits.
        /// * `number` - Optional number of limits to return. If none, return all limits.
        /// 
        /// # Returns
        /// 
        /// * `Vec<(Decimal, Decimal)>` - Vector of ask limits in format (price, amount).
        /// 
        pub fn get_ask_limits(
            &self,
            start_price: Option<Decimal>,
            stop_price: Option<Decimal>,
            number: Option<u32>,
        ) -> Vec<(Decimal, Decimal)> {
            // Return if no ask limits
            let current_ask = self.price_index.current_ask();
            if current_ask.is_none() {
                return Vec::new();
            }

            // Get start and stop conditions from optional arguments
            let start_price: Option<Price> = match start_price {
                Some(price_dec) => {
                    let price: Price = price_dec.round_to_price_range().into();
                    if price < current_ask.unwrap() {
                        current_ask
                    } else {
                        self.price_index.next_up(price)
                    }
                },
                None => current_ask,
            };
            let stop_price: Price = match stop_price {
                Some(price_dec) => price_dec.round_to_price_range().into(),
                None => Price::MAX,
            };
            let number: u32 = match number {
                Some(number) => number,
                None => u32::MAX,
            };

            // Create vector of limits
            let mut limits: Vec<(Decimal, Decimal)> = Vec::new();
            let mut count: u32 = 0;

            // Loop through limits
            let mut next = start_price;
            loop {
                // If some next price
                match next {
                    Some(price) => {                            
                        // Check stop conditions
                        count += 1;
                        if price > stop_price || count > number {
                            // Return list of limits
                            return limits;
                        }

                        // Get limit
                        let limit = self.ask_limit_map.get(&price).unwrap();
                        let entry = (Decimal::from(price), limit.get_amount_x());
                        
                        // Add limit
                        limits.push(entry);

                        // Get next price
                        next = self.price_index.next_up(price);
                    }
                    None => {
                        // Return list of limits
                        return limits;
                    }
                }
            }
        }

        /// Get bid limits.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Amount is calculated in tokens x.
        /// 
        /// # Arguments
        /// 
        /// * `start_price` - Optional start price, not inclusive. If none, start at current bid price.
        /// * `stop_price` - Optional stop price, inclusive. If none, return all limits.
        /// * `number` - Optional number of limits to return. If none, return all limits.
        /// 
        /// # Returns
        /// 
        /// * `Vec<(Decimal, Decimal)>` - Vector of bid limits in format (price, amount).
        /// 
        pub fn get_bid_limits(
            &self,
            start_price: Option<Decimal>,
            stop_price: Option<Decimal>,
            number: Option<u32>,
        ) -> Vec<(Decimal, Decimal)> {
            // Return if no bid limits
            let current_bid = self.price_index.current_bid();
            if current_bid.is_none() {
                return Vec::new();
            }

            // Get start and stop conditions from optional arguments
            let start_price: Option<Price> = match start_price {
                Some(price_dec) => {
                    let price: Price = price_dec.round_to_price_range().into();
                    if price > current_bid.unwrap() {
                        current_bid
                    } else {
                        self.price_index.next_down(price)
                    }
                },
                None => current_bid,
            };
            let stop_price: Price = match stop_price {
                Some(price_dec) => price_dec.round_to_price_range().into(),
                None => Price::MIN,
            };
            let number: u32 = match number {
                Some(number) => number,
                None => u32::MAX,
            };

            let mut limits: Vec<(Decimal, Decimal)> = Vec::new();
            let mut count: u32 = 0;

            // Loop through limits
            let mut next = start_price;
            loop {
                // If some next price
                match next {
                    Some(price) => {                            
                        // Check stop conditions
                        count += 1;
                        if price < stop_price || count > number {
                            // Return list of limits
                            return limits;
                        }

                        // Get limit
                        let limit = self.bid_limit_map.get(&price).unwrap();
                        let entry = (Decimal::from(price), limit.get_amount_x());
                        
                        // Add limit
                        limits.push(entry);

                        // Get next price
                        next = self.price_index.next_down(price);
                    }
                    None => {
                        // Return list of limits
                        return limits;
                    }
                }
            }
        }

        /// Get the status of a limit order.
        /// Amount is calculated in tokens x.
        /// 
        /// # Arguments
        /// 
        /// * `order_receipt_id` - Id of order receipt to get the status of.
        /// 
        /// # Returns
        /// 
        /// * `OrderStatus` - Status of order.
        /// 
        /// # Order Status
        /// 
        /// * `Open(OrderData)` - Order is open and has the contained order data.
        /// * `Filled(OrderData)` - Order has been filled and has the contained order data.
        /// * `Claimed` - Order has been claimed and no longer exists.
        /// * `Invalid` - Order id is invalid.
        /// 
        /// # Order Data
        /// 
        /// * `is_ask` - Is ask or bid order.
        /// * `price` - Price of order.
        /// * `amount_filled` - Filled amount of tokens for order calculated in tokens x.
        /// * `amount_total` - Total amount of tokens for order calculated in tokens x.
        /// 
        pub fn get_order_status(&self, order_receipt_id: NonFungibleLocalId) -> OrderStatus {
            let order_id: u64 = match order_receipt_id.clone() {
                NonFungibleLocalId::Integer(id) => id.value(),
                _ => return OrderStatus::Invalid,
            };

            // If order has not existed
            if order_id >= self.nonce || order_id == 0 {
                return OrderStatus::Invalid;
            }

            // If order no long exists
            if !self.order_receipt_manager.non_fungible_exists(&order_receipt_id) {
                return OrderStatus::Claimed;
            }

            // Get order receipt
            let order_data: OrderReceipt = self.order_receipt_manager.get_non_fungible_data(&order_receipt_id);

            // Get limit
            let price: Price = order_data.price.into();
            let limit = if order_data.is_ask {
                self.ask_limit_map.get(&price).unwrap()
            } else {
                self.bid_limit_map.get(&price).unwrap()
            };
            
            // Check position in relation to active order
            let head_id: u64 = limit.get_head_id();
            if head_id == 0 || order_id < head_id {
                OrderStatus::Filled(
                    OrderData {
                        is_ask: order_data.is_ask,
                        price: order_data.price,
                        amount_filled: order_data.amount,
                        amount_total: order_data.amount,
                    }
                )
            } else if order_id == head_id {
                OrderStatus::Open(OrderData {
                    is_ask: order_data.is_ask,
                    price: order_data.price,
                    amount_filled: limit.get_amount_x_unallocated(),
                    amount_total: order_data.amount,
                })
            } else {
                OrderStatus::Open(OrderData {
                    is_ask: order_data.is_ask,
                    price: order_data.price,
                    amount_filled: Decimal::zero(),
                    amount_total: order_data.amount,
                })
            }
        }

        /// Get the status of a batch of limit orders.
        /// 
        /// # Arguments
        /// 
        /// * `order_receipt_ids` - Ids of order receipts to get the status of.
        /// 
        /// # Returns
        /// 
        /// * `Vec<OrderStatus>` - Statuses of orders.
        /// 
        /// # Order Status
        /// 
        /// * `Open(amount)` - Order is open and has had amount of tokens filled.
        /// * `Filled` - Order has been filled.
        /// * `Claimed` - Order has been claimed and no longer exists.
        /// * `Invalid` - Order id is invalid.
        /// 
        pub fn get_order_statuses(&self, order_receipt_ids: Vec<NonFungibleLocalId>) -> Vec<OrderStatus> {
            order_receipt_ids.into_iter().map(|id| self.get_order_status(id)).collect()
        }

        /// Place a limit order into the order book.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Price is truncated to 5 significant figures.
        /// 
        /// # Arguments
        /// 
        /// * `tokens` - Tokens used to place the limit order.
        /// * `price` - Price at which to place limit order.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Possibly contains an order_receipt. Will not contain an order receipt if order is filled immediately.
        /// * `Bucket` - Contains any tokens immediately bought.
        /// * `Bucket` - Contains any tokens leftover.
        /// 
        /// # Panics
        /// 
        /// * If `tokens` are not tokens x or tokens y.
        /// * If order size is not greater than zero.
        /// * If `price` is not in the valid range of [0.00000000001, 100000000000].
        /// 
        /// # Events
        /// 
        /// * `LimitOrderEvent` - Event emitted when limit order is placed.
        /// * `MarketOrderEvent` - Event emitted when market order is placed.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// 
        pub fn limit_order(&mut self, tokens: Bucket, price: Decimal) -> (Bucket, Bucket, Bucket) { 
            // Check if tokens_x or tokens_y
            if tokens.resource_address() == self.tokens_x.resource_address() {
                // Market order to clear overlap
                let price: Price = price.into();
                let current_bid: Option<Price> = self.price_index.current_bid();
                let (tokens_y, mut tokens_x) = if current_bid.is_some() && price <= current_bid.unwrap() {
                    self.market_order_x_to_y(tokens, price)
                } else {
                    (Bucket::new(self.tokens_y.resource_address()), tokens)
                };

                // If remaining tokens place limit order
                let order_receipt = if tokens_x.amount() > Decimal::ZERO {
                    self.limit_order_x_to_y(tokens_x.take(tokens_x.amount()), price)
                } else {
                    Bucket::new(self.order_receipt_manager.address())
                };

                // Return buckets
                (order_receipt, tokens_y, tokens_x)
            } else if tokens.resource_address() == self.tokens_y.resource_address() {
                // Market order to clear overlap
                let price: Price = price.into();
                let current_ask: Option<Price> = self.price_index.current_ask();
                let (tokens_x, mut tokens_y) = if current_ask.is_some() && price >= current_ask.unwrap() {
                    self.market_order_y_to_x(tokens, price)
                } else {
                    (Bucket::new(self.tokens_x.resource_address()), tokens)
                };

                // If remaining tokens place limit order
                let price_dec: Decimal = price.into();
                let order_receipt = if tokens_y.amount() / price_dec > Decimal::ZERO {
                    self.limit_order_y_to_x(tokens_y.take(tokens_y.amount()), price)
                } else {
                    Bucket::new(self.order_receipt_manager.address())
                };

                // Return buckets
                (order_receipt, tokens_x, tokens_y)
            } else {
                panic!("Invalid token address.");
            }
        }

        /// Place a batch of limit orders into the order book.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Price is truncated to 5 significant figures.
        /// 
        /// # Arguments
        /// 
        /// * `limits` - Vector of limits to place in format (tokens, price).
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any order receipts.
        /// * `Bucket` - Contains any tokens x bought or remaining.
        /// * `Bucket` - Contains any tokens y bought or remaining.
        /// 
        /// # Panics
        /// 
        /// * If `tokens` are not tokens x or tokens y.
        /// * If order size is not greater than zero.
        /// * If `price` is not in the valid range of [0.00000000001, 100000000000].
        /// 
        /// # Events
        /// 
        /// * `LimitOrderEvent` - Event emitted when limit order is placed.
        /// * `MarketOrderEvent` - Event emitted when market order is placed.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// 
        pub fn limit_order_batch(&mut self, limits: Vec<(Bucket, Decimal)>) -> (Bucket, Bucket, Bucket) {
            // Create buckets
            let mut order_receipts: Bucket = Bucket::new(self.order_receipt_manager.address());
            let mut tokens_x: Bucket = Bucket::new(self.tokens_x.resource_address());
            let mut tokens_y: Bucket = Bucket::new(self.tokens_y.resource_address());

            // Loop through limits
            for (tokens, price) in limits {
                // Place limit order
                let (order_receipt, tokens_a, tokens_b) = self.limit_order(tokens, price);

                // Deposit order receipt
                order_receipts.put(order_receipt);

                // Deposit tokens
                if tokens_a.resource_address() == self.tokens_x.resource_address() {
                    tokens_x.put(tokens_a);
                    tokens_y.put(tokens_b);
                } else {
                    tokens_x.put(tokens_b);
                    tokens_y.put(tokens_a);
                }
            }   

            // Return order receipts and tokens
            (order_receipts, tokens_x, tokens_y)
        }

        /// Execute a market order on the order book.
        /// A fee is taken from the input tokens.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// 
        /// # Arguments
        /// 
        /// * `tokens` - Tokens used to execute the market order.
        /// * `stop_price` - Optional stop price. If none, market order will not stop until filled.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens bought.
        /// * `Bucket` - Contains any tokens remaining.
        /// 
        /// # Panics
        /// 
        /// * If amount of tokens is not greater than zero.
        /// * If tokens are not tokens x or tokens y.
        /// 
        /// # Events
        /// 
        /// * `MarketOrderEvent` - Event emitted when market order is placed.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// 
        pub fn market_order(&mut self, tokens: Bucket, stop_price: Option<Decimal>) -> (Bucket, Bucket) {
            // Check parameters
            assert!(tokens.amount() > Decimal::zero(), "Order size must be greater than zero.");

            // Check if tokens_x or tokens_y
            if tokens.resource_address() == self.tokens_x.resource_address() {
                let stop_price: Price = match stop_price {
                    Some(price_dec) => price_dec.round_to_price_range().into(),
                    None => Price::MIN,
                };

                self.market_order_x_to_y(tokens, stop_price)
            } else if tokens.resource_address() == self.tokens_y.resource_address() {
                let stop_price: Price = match stop_price {
                    Some(price_dec) => price_dec.round_to_price_range().into(),
                    None => Price::MAX,
                };

                self.market_order_y_to_x(tokens, stop_price)
            } else {
                panic!("Invalid token address.");
            }
        }

        /// Claim tokens owned by order receipts.
        /// 
        /// # Arguments
        /// 
        /// * `order_receipts` - Order receipts.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens x from claimed orders.
        /// * `Bucket` - Contains any tokens y from claimed orders.
        /// 
        /// # Panics
        /// 
        /// * If an order receipt is invalid.
        /// * If an order receipt is for a different order book.
        /// 
        /// # Events
        /// 
        /// * `ClaimOrderEvent` - Event emitted when order is claimed.
        /// 
        pub fn claim_orders(&mut self, order_receipts: Bucket) -> (Bucket, Bucket) {
            // Check is valid order receipt
            assert!(order_receipts.resource_address() == self.order_receipt_manager.address(), "Invalid order receipt.");

            // As non fungible bucket
            let order_receipts = order_receipts.as_non_fungible();

            // Create buckets
            let mut tokens_x: Bucket = Bucket::new(self.tokens_x.resource_address());
            let mut tokens_y: Bucket = Bucket::new(self.tokens_y.resource_address());

            // Loop through order receipts
            let order_id_list: Vec<NonFungibleLocalId> = order_receipts.non_fungible_local_ids().into_iter().collect();
            let order_data_list: Vec<NonFungible<OrderReceipt>> = order_receipts.non_fungibles();
            for i in 0..order_id_list.len() {
                // Get order
                let order_id: u64 = match &order_id_list[i] {
                    NonFungibleLocalId::Integer(id) => id.value(),
                    _ => unreachable!(),
                };
                let order_data: OrderReceipt = order_data_list[i].data();

                // Remove order from chain
                self.remove_from_order_chain(&order_data);

                // Claim and return tokens
                let (tokens_a, tokens_b) = if order_data.is_ask {
                    self.claim_order_ask(order_id, &order_data)
                } else {
                    self.claim_order_bid(order_id, &order_data)
                };
                tokens_x.put(tokens_a);
                tokens_y.put(tokens_b);
            }

            // Burn order receipts
            order_receipts.burn();

            // Return tokens
            (tokens_x, tokens_y)
        }

        /// Helper method to place an ask limit order into the order book.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Price is truncated to 5 significant figures.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_x` - Tokens x used to place the limit order.
        /// * `price` - Price at which to place limit order.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains an order receipt.
        /// 
        /// # Requires
        /// 
        /// * `tokens_x` - Must be tokens x.
        /// * `tokens_x` - Must not be empty.
        /// * `price` - Must be greater than current bid price.
        /// 
        /// # Events
        /// 
        /// * `LimitOrderEvent` - Event emitted when limit order is placed.
        /// 
        fn limit_order_x_to_y(&mut self, tokens_x: Bucket, price: Price) -> Bucket {
            // Deposit tokens
            let amount_x: Decimal = tokens_x.amount();
            self.tokens_x.put(tokens_x);

            // Get order nonce to make order receipt id
            let order_id: u64 = self.nonce;
            self.nonce += 1;

            // Get limit
            let mut limit = match self.ask_limit_map.get(&price) {
                Some(limit) => {
                    // If empty limit add price to price tree
                    if limit.is_empty() {
                        self.price_index.insert(price, true);
                    }

                    // Return limit
                    *limit
                },
                None => {
                    // Add price to price tree
                    self.price_index.insert(price, true);

                    // Return new limit
                    Limit::new()
                }
            };

            // Add order to limit
            let tail_id: u64 = limit.add_order(order_id, &amount_x);
            self.ask_limit_map.insert(price, limit);

            // Update tail order receipt
            if tail_id != 0 {
                self.order_receipt_manager.update_non_fungible_data(&NonFungibleLocalId::integer(tail_id), "next_id", order_id);
            }

            // Mint new order receipt
            let price = Decimal::from(price);
            let order_receipt: Bucket = self.order_receipt_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(order_id),
                OrderReceipt {
                    is_ask: true,
                    price,
                    amount: amount_x,
                    next_id: 0,
                    prev_id: tail_id,
                },
            );

            // Emit limit order event
            Runtime::emit_event(LimitOrderEvent {
                order_id: NonFungibleLocalId::integer(order_id),
                is_ask: true,
                price,
                amount: amount_x,
            });

            // Return order receipt
            order_receipt
        }

        /// Helper method to place a bid limit order into the order book.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Price is truncated to 5 significant figures.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_y` - Tokens y used to place the limit order.
        /// * `price` - Price at which to place limit order.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains an order receipt.
        /// 
        /// # Requires
        /// 
        /// * `tokens_y` - Must be tokens y.
        /// * `tokens_y` - Must not be empty.
        /// * `price` - Must be less than current ask price.
        /// 
        /// # Events
        /// 
        /// * `LimitOrderEvent` - Event emitted when limit order is placed.
        /// 
        fn limit_order_y_to_x(&mut self, tokens_y: Bucket, price: Price) -> Bucket {
            // Deposit tokens
            let amount_y: Decimal = tokens_y.amount();
            let price_dec: Decimal = price.into();
            let amount_x: Decimal = amount_y / price_dec;
            self.tokens_y.put(tokens_y);

            // Get order nonce to make order receipt id
            let order_id: u64 = self.nonce;
            self.nonce += 1;

            // Get limit
            let mut limit = match self.bid_limit_map.get(&price) {
                Some(limit) => {
                    // If empty limit add price to price tree
                    if limit.is_empty() {
                        self.price_index.insert(price, false);
                    }

                    // Return limit
                    *limit
                },
                None => {
                    // Add price to price tree
                    self.price_index.insert(price, false);

                    // Return new limit
                    Limit::new()
                }
            };

            // Add order to limit
            let tail_id: u64 = limit.add_order(order_id, &amount_x);
            self.bid_limit_map.insert(price, limit);
    
            // Update tail order receipt
            if tail_id != 0 {
                self.order_receipt_manager.update_non_fungible_data(&NonFungibleLocalId::integer(tail_id), "next_id", order_id);
            }

            // Mint new order receipt
            let price = Decimal::from(price);
            let order_receipt: Bucket = self.order_receipt_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(order_id),
                OrderReceipt {
                    is_ask: false,
                    price,
                    amount: amount_x,
                    next_id: 0,
                    prev_id: tail_id,
                },
            );

            // Emit limit order event
            Runtime::emit_event(LimitOrderEvent {
                order_id: NonFungibleLocalId::integer(order_id),
                is_ask: false,
                price,
                amount: amount_x,
            });

            // Return order receipt
            order_receipt
        }

        /// Helper method to execute a market order on ask limits in the order book.
        /// A fee is taken from the input tokens.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_y` - Tokens y used to execute the market order.
        /// * `stop_price` - Stop price at which to stop executing market order.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens x bought.
        /// * `Bucket` - Contains any tokens y remaining.
        /// 
        /// # Requires
        /// 
        /// * `tokens_y` - Must be tokens y.
        /// * `tokens_y` - Must not be empty.
        /// 
        /// # Events
        /// 
        /// * `MarketOrderEvent` - Event emitted when market order is placed.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// 
        fn market_order_y_to_x(&mut self, mut tokens_y: Bucket, stop_price: Price) -> (Bucket, Bucket) {
            // Take fee
            let protocol_fee: Decimal =  FEE_CONTROLLER.get_protocol_fee(Runtime::package_address());
            let mut tokens_fee: Bucket = tokens_y.take_advanced(tokens_y.amount() * protocol_fee, INCOMING);
            let amount_y_input: Decimal = tokens_y.amount();

            // Initialize amounts
            let mut amount_x_bought: Decimal = Decimal::zero();
            let mut amount_y_order: Decimal = tokens_y.amount();
            let mut fills: Vec<(Decimal, Decimal)> = Vec::new();

            // Loop through limits
            while let Some(price) = self.price_index.current_ask() {
                // Decompress price
                let price_dec: Decimal = price.into();

                // Check if price is past stop price
                if price > stop_price {
                    break;
                }

                // Update last traded price
                self.last_price = price;

                // Get limit
                let limit: &mut Limit = &mut self.ask_limit_map.get_mut(&price).unwrap();

                // Calculate amounts
                let amount_x_limit: Decimal = limit.get_amount_x();
                let amount_y_limit: Decimal = amount_x_limit * price_dec;

                // If order amount is greater or equal to the limit amount fill limit and continue,
                // else partly fill limit and break
                if amount_y_order >= amount_y_limit {
                    limit.fully_fill();
                    fills.push((price_dec, amount_x_limit));
                    amount_x_bought += amount_x_limit;
                    amount_y_order -= amount_y_limit;

                    self.price_index.remove(price);
                } else {
                    let amount_x_order: Decimal = amount_y_order / price_dec;
                    limit.fill(&amount_x_order, &self.order_receipt_manager);
                    fills.push((price_dec, amount_x_order));
                    amount_x_bought += amount_x_order;
                    amount_y_order = Decimal::zero();

                    break;
                }
            }

            // Deposit and withdraw tokens
            let tokens_x: Bucket = self.tokens_x.take_advanced(amount_x_bought, OUTGOING);
            self.tokens_y.put(tokens_y.take_advanced(tokens_y.amount() - amount_y_order, INCOMING));
            
            // If fee amount is not zero
            if !tokens_fee.is_empty() {
                // Return fee if tokens not used
                if tokens_y.amount() != Decimal::zero() {
                    let amount_unused: Decimal = tokens_y.amount() / amount_y_input * tokens_fee.amount();
                    tokens_y.put(tokens_fee.take_advanced(amount_unused, OUTGOING));
                }

                // Emit fee event
                Runtime::emit_event(ProtocolFeeEvent {
                    token_address: self.tokens_y.resource_address(),
                    amount: tokens_fee.amount(),
                });

                // Deposit fee
                FEE_VAULTS.deposit(tokens_fee);
            } else {
                tokens_fee.drop_empty();
            }

            // Emit market order event
            Runtime::emit_event(MarketOrderEvent {
                is_buy: true,
                fills,
            });

            // Return tokens
            (tokens_x, tokens_y)
        }

        /// Helper method to execute a market order on bid limits in the order book.
        /// A fee is taken from the input tokens.
        /// Price is calculated as `tokens_y / tokens_x`.
        /// Buys tokens_y for tokens_x from limit orders until order is filled or stop_price is reached.
        /// Returns (tokens_y, tokens_x).
        /// 
        /// # Arguments
        /// 
        /// * `tokens_x` - Tokens x used to execute the market order.
        /// * `stop_price` - Stop price at which to stop executing market order.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens y bought.
        /// * `Bucket` - Contains any tokens x remaining.
        /// 
        /// # Requires
        /// 
        /// * `tokens_x` - Must be tokens x.
        /// * `tokens_x` - Must not be empty.
        /// 
        /// # Events
        /// 
        /// * `MarketOrderEvent` - Event emitted when market order is placed.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// 
        fn market_order_x_to_y(&mut self, mut tokens_x: Bucket, stop_price: Price) -> (Bucket, Bucket) {
            // Take fee
            let protocol_fee: Decimal =  FEE_CONTROLLER.get_protocol_fee(Runtime::package_address());
            let mut tokens_fee: Bucket = tokens_x.take_advanced(tokens_x.amount() * protocol_fee, INCOMING);
            let amount_x_input: Decimal = tokens_x.amount();

            // Initialize amounts
            let mut amount_y_bought: Decimal = Decimal::zero();
            let mut amount_x_order: Decimal = tokens_x.amount();
            let mut fills: Vec<(Decimal, Decimal)> = Vec::new();

            // Loop through limits
            while let Some(price) = self.price_index.current_bid() {
                // Decompress price
                let price_dec: Decimal = price.into();

                // Check stop price
                if price < stop_price {
                    break;
                }

                // Update last traded price
                self.last_price = price;

                // Get limit
                let limit: &mut Limit = &mut self.bid_limit_map.get_mut(&price).unwrap();

                // Calculate amounts
                let amount_x_limit: Decimal = limit.get_amount_x();

                // If order amount is greater or equal to the limit amount fill limit and continue,
                // else partly fill limit and break
                if amount_x_order >= amount_x_limit {
                    limit.fully_fill();
                    fills.push((price_dec, amount_x_limit));
                    let amount_y_limit: Decimal = amount_x_limit * price_dec;
                    amount_y_bought += amount_y_limit;
                    amount_x_order -= amount_x_limit;

                    self.price_index.remove(price);
                } else {
                    limit.fill(&amount_x_order, &self.order_receipt_manager);
                    fills.push((price_dec, amount_x_order));
                    let amount_y_order: Decimal = amount_x_order * price_dec;
                    amount_y_bought += amount_y_order;
                    amount_x_order = Decimal::zero();

                    break;
                }
            }

            // Deposit and withdraw tokens
            let tokens_y: Bucket = self.tokens_y.take_advanced(amount_y_bought, OUTGOING);
            self.tokens_x.put(tokens_x.take_advanced(tokens_x.amount() - amount_x_order, INCOMING));

            // If fee amount is not zero
            if !tokens_fee.is_empty() {
                // Return fee if tokens not used
                if tokens_x.amount() != Decimal::zero() {
                    let amount_unused: Decimal = tokens_x.amount() / amount_x_input * tokens_fee.amount();
                    tokens_x.put(tokens_fee.take_advanced(amount_unused, OUTGOING));
                }

                // Emit fee event
                Runtime::emit_event(ProtocolFeeEvent {
                    token_address: self.tokens_x.resource_address(),
                    amount: tokens_fee.amount(),
                });

                // Deposit fee
                FEE_VAULTS.deposit(tokens_fee);
            } else {
                tokens_fee.drop_empty();
            }

            // Emit market order event
            Runtime::emit_event(MarketOrderEvent {
                is_buy: false,
                fills,
            });

            // Return tokens
            (tokens_y, tokens_x)
        }

        /// Helper method to remove an order receipt from the limit order chain.
        /// 
        /// # Arguments
        /// 
        /// * `order_data` - Data of order receipt to remove.
        /// 
        /// # Requires
        /// 
        /// * `order_data` - Must be a valid order receipt data.
        /// 
        fn remove_from_order_chain(&mut self, order_data: &OrderReceipt) {
            if order_data.prev_id != 0 {
                self.order_receipt_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(order_data.prev_id),
                    "next_id",
                    order_data.next_id,
                );
            }
            if order_data.next_id != 0 {
                self.order_receipt_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(order_data.next_id),
                    "prev_id",
                    order_data.prev_id,
                );
            }
        }

        /// Helper method to claim tokens owned by an ask limit order receipt.
        /// 
        /// # Arguments
        /// 
        /// * `order_id` - Id of order.
        /// * `order_data` - Data of order receipt.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens x from canceled order.
        /// * `Bucket` - Contains any tokens y from filled order.
        /// 
        /// # Requires
        /// 
        /// * `order_id` - Must be a valid order id.
        /// * `order_data` - Must match the order receipt id.
        /// 
        /// # Events
        /// 
        /// * `ClaimOrderEvent` - Event emitted when order is claimed.
        /// 
        fn claim_order_ask(&mut self, order_id: u64, order_data: &OrderReceipt) -> (Bucket, Bucket) {
            // Get claim amounts
            let price: Price = order_data.price.into();
            let limit: &mut Limit = &mut self.ask_limit_map.get_mut(&price).unwrap();
            let (amount_canceled, amount_filled) = limit.claim_order(order_id, order_data);
            let amount_x = amount_canceled;
            let amount_y = amount_filled * order_data.price;

            // If limit is empty remove from price tree
            if limit.is_empty() {
                let price_struct: Price = order_data.price.into();
                self.price_index.remove(price_struct);
            }

            // Withdraw tokens
            let tokens_x: Bucket = self.tokens_x.take_advanced(amount_x, OUTGOING);
            let tokens_y: Bucket = self.tokens_y.take_advanced(amount_y, OUTGOING);

            // Emit claim order event
            Runtime::emit_event(ClaimOrderEvent {
                order_id: NonFungibleLocalId::integer(order_id),
                is_ask: true,
                price: order_data.price,
                amount_canceled,
                amount_filled,
                amount_x,
                amount_y,
            });

            // Return tokens
            (tokens_x, tokens_y)
        }

        /// Helper method to claim tokens owned by an bid limit order receipt.
        /// 
        /// # Arguments
        /// 
        /// * `order_id` - Id of order.
        /// * `order_data` - Data of order receipt.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Contains any tokens x from filled order.
        /// * `Bucket` - Contains any tokens y from canceled order.
        /// 
        /// # Requires
        /// 
        /// * `order_id` - Must be a valid order id.
        /// * `order_data` - Must match the order receipt id.
        /// 
        /// # Events
        /// 
        /// * `ClaimOrderEvent` - Event emitted when order is claimed.
        /// 
        fn claim_order_bid(&mut self, order_id: u64, order_data: &OrderReceipt) -> (Bucket, Bucket) {
            // Get claim amounts
            let price: Price = order_data.price.into();
            let limit: &mut Limit = &mut self.bid_limit_map.get_mut(&price).unwrap();
            let (amount_canceled, amount_filled) = limit.claim_order(order_id, order_data);
            let amount_y = amount_canceled * order_data.price;
            let amount_x = amount_filled;

            // If limit is empty remove from price tree
            if limit.is_empty() {
                let price_struct: Price = order_data.price.into();
                self.price_index.remove(price_struct);
            }

            // Withdraw tokens
            let tokens_y: Bucket = self.tokens_y.take_advanced(amount_y, OUTGOING);
            let tokens_x: Bucket = self.tokens_x.take_advanced(amount_x, OUTGOING);

            // Emit claim order event
            Runtime::emit_event(ClaimOrderEvent {
                order_id: NonFungibleLocalId::integer(order_id),
                is_ask: false,
                price: order_data.price,
                amount_canceled,
                amount_filled,
                amount_x,
                amount_y,
            });

            // Return tokens
            (tokens_x, tokens_y)
        }
    }
}
