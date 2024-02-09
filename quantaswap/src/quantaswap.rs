use scrypto::prelude::*;

use crate::bin::*;
use crate::events::*;
use crate::liquidity_receipt::*;
use crate::swap_math::*;
use crate::tick_index::*;
use crate::tick::*;

#[blueprint]
#[events(
    NewPoolEvent,
    MintLiquidityReceiptEvent,
    BurnLiquidityReceiptEvent,
    AddLiquidityEvent,
    RemoveLiquidityEvent,
    SwapEvent,
    ValuationEvent,
    ProtocolFeeEvent,
    LiquidityFeeEvent,
)]
#[types(
    Tick,
    Bin,
    LiquidityReceipt,
    u32,
    IndexNode,
)]
mod quantaswap {
    // Import FeeController to get fee percentages.
    extern_blueprint! {
        "package_sim1pkyls09c258rasrvaee89dnapp2male6v6lmh7en5ynmtnavqdsvk9",
        FeeController {
            fn get_fees(&self, package_address: PackageAddress, resource_addresses: Vec<ResourceAddress>) -> (Decimal, Decimal);
        }
    }

    // Import Fee Vaults to send fee tokens to.
    extern_blueprint! {
        "package_sim1p4nhxvep6a58e88tysfu0zkha3nlmmcp6j8y5gvvrhl5aw47jfsxlt",
        FeeVaults {
            fn deposit(&self, tokens: Bucket);
        }
    }
    
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
            mint_liquidity_receipt => restrict_to: [user];
            add_liquidity => restrict_to: [user];
            add_liquidity_to_receipt => restrict_to: [user];
            swap => restrict_to: [user];
            burn_liquidity_receipt => PUBLIC;
            remove_liquidity => PUBLIC;
            remove_specific_liquidity => PUBLIC;
            get_fee_controller_address => PUBLIC;
            get_fee_vaults_address => PUBLIC;
            get_token_x_address => PUBLIC;
            get_token_y_address => PUBLIC;
            get_liquidity_receipt_address => PUBLIC;
            get_bin_span => PUBLIC;
            get_liquidity_claims => PUBLIC;
            get_amount_x => PUBLIC;
            get_amount_y => PUBLIC;
            get_active_tick => PUBLIC;
            get_price => PUBLIC;
            get_active_bin_price_range => PUBLIC;
            get_active_amounts => PUBLIC;
            get_bins_above => PUBLIC;
            get_bins_below => PUBLIC;
            get_redemption_value => PUBLIC;
            get_redemption_bin_values => PUBLIC;
        }
    }

    // Set withdraw strategies
    const INCOMING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToPositiveInfinity);
    const OUTGOING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToZero);

    struct QuantaSwap {
        /// Span of ticks a bin covers. Bins are aligned to the bin span.
        bin_span: u32,
        /// Index of ticks with active liquidity positions.
        tick_index: TickIndex,
        /// Map of ticks to dormant bins.
        bin_map: KeyValueStore<Tick, Bin>,
        /// Lower price sqrt limit of the active bin.
        lower_limit: Decimal,
        /// Upper price sqrt limit of the active bin.
        upper_limit: Decimal,
        /// Amount of tokens x in the active bin.
        active_x: Decimal,
        /// Amount of tokens y in the active bin.
        active_y: Decimal,
        /// Total claim of liquidity positions in the active bin. This is the sum of all liquidity claims for the active bin.
        active_total_claim: Decimal,
        /// Liquidity receipt manager used for minting and updating liquidity receipts.
        liquidity_receipt_manager: ResourceManager,
        /// Vault for tokens x.
        tokens_x: Vault,
        /// Vault for tokens y.
        tokens_y: Vault,
    }

    impl QuantaSwap {
        /// Instantiate and globalize a new QuantaSwap owned by the admin badge.
        /// 
        /// # Arguments
        /// 
        /// * `owner_rule` - Access rule for the owner.
        /// * `user_rule` - Access rule for the user.
        /// * `token_x_address` - Address of the token x resource.
        /// * `token_y_address` - Address of the token y resource.
        /// * `bin_span` - Span of ticks a bin covers.
        /// * `reservation` - Optional address reservation for component.
        /// 
        /// # Returns
        /// 
        /// * `Global<QuantaSwap>` - The QuantaSwap component.
        /// 
        /// # Access Rules
        /// 
        /// `mint_liquidity_receipt` - User role required.
        /// `add_liquidity` - User role required.
        /// `add_liquidity_to_receipt` - User role required.
        /// `swap` - User role required.
        /// `burn_liquidity_receipt` - Public.
        /// `remove_liquidity` - Public.
        /// `remove_specific_liquidity` - Public.
        /// `get_fee_controller_address` - Public.
        /// `get_fee_vaults_address` - Public.
        /// `get_token_x_address` - Public.
        /// `get_token_y_address` - Public.
        /// `get_liquidity_receipt_address` - Public.
        /// `get_bin_span` - Public.
        /// `get_amount_x` - Public.
        /// `get_amount_y` - Public.
        /// `get_active_tick` - Public.
        /// `get_price` - Public.
        /// `get_active_bin_price_range` - Public.
        /// `get_active_amounts` - Public.
        /// `get_bins_above` - Public.
        /// `get_bins_below` - Public.
        /// `get_liquidity_claims` - Public.
        /// `get_redemption_value` - Public.
        /// `get_redemption_bins` - Public.
        /// 
        /// # Panics
        /// 
        /// * If the bin span is not greater than zero.
        /// 
        /// # Events
        /// 
        /// * `NewPoolEvent` - Event emitted when QuantaSwap is instantiated.
        ///  
        pub fn new(
            owner_rule: AccessRule,
            user_rule: AccessRule,
            token_x_address: ResourceAddress,
            token_y_address: ResourceAddress,
            bin_span: u32,
            reservation: Option<GlobalAddressReservation>
        ) -> Global<QuantaSwap> {
            // Check bin span
            assert!(bin_span > 0, "Bin span must be greater than zero.");

            // Get component address
            let (address_reservation, component_address) = match reservation {
                Some(reservation) => {
                    let component_address: ComponentAddress = Runtime::get_reservation_address(&reservation).try_into().unwrap();
                    (reservation, component_address)
                }
                None => {
                    Runtime::allocate_component_address(QuantaSwap::blueprint_id())
                }
            };

            // Get tokens symbols
            let token_x_manager = ResourceManager::from(token_x_address);
            let token_y_manager = ResourceManager::from(token_y_address);
            let symbol_x: String = token_x_manager.get_metadata("symbol").unwrap_or_default().unwrap_or_default();
            let symbol_y: String = token_y_manager.get_metadata("symbol").unwrap_or_default().unwrap_or_default();

            // Create liquidity receipt resource
            let liquidity_receipt_manager = ResourceBuilder::new_ruid_non_fungible_with_registered_type::<LiquidityReceipt>(OwnerRole::Updatable(owner_rule.clone()))
                .metadata(metadata!(
                    init {
                        "package" => GlobalAddress::from(Runtime::package_address()), locked;
                        "component" => GlobalAddress::from(component_address), locked;
                        "token_x" => GlobalAddress::from(token_x_address), locked;
                        "token_y" => GlobalAddress::from(token_y_address), locked;
                        "name" => format!("Liquidity Receipt {}/{}", symbol_x, symbol_y), updatable;
                        "description" => format!("Used to store liquidity positions for pair {}/{}.", symbol_x, symbol_y), updatable;
                        "tags" => vec!["defi", "dex", "amm", "LP token", "receipt"], updatable;
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

            // Emit new pool event
            Runtime::emit_event(NewPoolEvent {
                component_address,
                liquidity_receipt_address: liquidity_receipt_manager.address(),
                token_x_address,
                token_y_address,
                bin_span,
            });

            // Create component
            Self {
                bin_span,
                tick_index: TickIndex::new(),
                bin_map: KeyValueStore::new_with_registered_type(),
                lower_limit: Decimal::zero(),
                upper_limit: Decimal::zero(),
                active_x: Decimal::zero(),
                active_y: Decimal::zero(),
                active_total_claim: Decimal::zero(),
                liquidity_receipt_manager,
                tokens_x: Vault::new(token_x_address),
                tokens_y: Vault::new(token_y_address),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(owner_rule))
            .metadata(metadata!(
                init {
                    "token_x" => GlobalAddress::from(token_x_address), locked;
                    "token_y" => GlobalAddress::from(token_y_address), locked;
                    "liquidity_receipt" => GlobalAddress::from(liquidity_receipt_manager.address()), locked;
                    "name" => format!("Pool {}/{}", symbol_x, symbol_y), updatable;
                    "description" => format!("Pool for pair {}/{}.", symbol_x, symbol_y), updatable;
                    "tags" => vec!["defi", "dex", "amm", "pool"], updatable;
                }
            ))
            .with_address(address_reservation)
            .roles(roles!(
                user => user_rule;
            ))
            .globalize()
        } 

        /// Get fee controller component address.
        /// 
        /// # Returns
        /// 
        /// * `ComponentAddress` - Address of the fee controller component.
        /// 
        pub fn get_fee_controller_address(&self) -> ComponentAddress {
            FEE_CONTROLLER.address()
        }

        /// Get fee vaults component address.
        /// 
        /// # Returns
        /// 
        /// * `ComponentAddress` - Address of the fee vaults component.
        /// 
        pub fn get_fee_vaults_address(&self) -> ComponentAddress {
            FEE_VAULTS.address()
        }

        /// Get the address of the token x resource.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of the token x resource.
        /// 
        pub fn get_token_x_address(&self) -> ResourceAddress {
            self.tokens_x.resource_address()
        }

        /// Get the address of the token y resource.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of the token y resource.
        /// 
        pub fn get_token_y_address(&self) -> ResourceAddress {
            self.tokens_y.resource_address()
        }

        /// Get the address of the liquidity receipt resource.
        /// 
        /// # Returns
        /// 
        /// * `ResourceAddress` - Address of the liquidity receipt resource.
        /// 
        pub fn get_liquidity_receipt_address(&self) -> ResourceAddress {
            self.liquidity_receipt_manager.address()
        }

        /// Get the span of tick a bin covers.
        /// 
        /// # Returns
        /// 
        /// * `u32` - Span of tick a bin covers.
        /// 
        pub fn get_bin_span(&self) -> u32 {
            self.bin_span
        }

        /// Get total amount of tokens x in protocol.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens x.
        /// 
        pub fn get_amount_x(&self) -> Decimal {
            self.tokens_x.amount()
        }

        /// Get total amount of tokens y in the protocol.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens y.
        /// 
        pub fn get_amount_y(&self) -> Decimal {
            self.tokens_y.amount()
        }

        /// Get the tick of the active bin.
        /// 
        /// # Returns
        /// 
        /// * `Option<u32>` - Tick of the active bin. None if no active tick.
        /// 
        pub fn get_active_tick(&self) -> Option<u32> {
            self.tick_index.current().map(|tick| tick.0)
        }

        /// Get current price.
        /// 
        /// # Returns
        /// 
        /// * `Option<Decimal>` - Current price. None if no active tick.
        /// 
        pub fn get_price(&self) -> Option<Decimal> {
            self.tick_index.current()?;
            let (virtual_x, virtual_y) = self.virtual_amounts();             
            Some(calculate_price(&virtual_x, &virtual_y))
        }

        /// Get the price range of the active bin.
        /// 
        /// # Returns
        /// 
        /// * `Option<(Decimal, Decimal)>` - Price range of the active bin. None if no active tick.
        /// 
        pub fn get_active_bin_price_range(&self) -> Option<(Decimal, Decimal)> {
            self.tick_index.current()?;
            Some((self.lower_limit * self.lower_limit, self.upper_limit * self.upper_limit))
        }

        /// Get amount of tokens x and y in the active bin.
        /// 
        /// # Returns
        /// 
        /// * `Option<(Decimal, Decimal)>` - Amount of tokens x and y in the active bin. None if no active tick.
        /// 
        pub fn get_active_amounts(&self) -> Option<(Decimal, Decimal)> {
            self.tick_index.current()?;
            Some((self.active_x, self.active_y))
        }

        /// Get the amount of tokens x in bins above the current price.
        /// 
        /// # Arguments
        /// 
        /// * `start_tick` - Tick to start from, not inclusive. If None, start from the current tick, inclusive.
        /// * `stop_tick` - Tick to stop at, inclusive.
        /// * `number` - Number of bins to return.
        /// 
        /// # Returns
        /// 
        /// * `Vec<(u32, Decimal)>` - Bins above the current price in the format (tick, amount_x).
        /// 
        pub fn get_bins_above(
            &self,
            start_tick: Option<u32>,
            stop_tick: Option<u32>,
            number: Option<u32>,
        ) -> Vec<(u32, Decimal)> {
            let mut bins: Vec<(u32, Decimal)> = Vec::new();

            // Return if no bins
            let current_tick = self.tick_index.current();
            if current_tick.is_none() {
                return bins;
            }

            // Get start and stop conditions from optional arguments
            let stop_tick: Tick = match stop_tick {
                Some(tick) => Tick(tick),
                None => Tick::MAX,
            };
            let number: u32 = match number {
                Some(number) => number,
                None => u32::MAX,
            };
            let mut count: u32 = 0;

            let start_tick: Option<Tick> = match start_tick {
                Some(tick) => {
                    let tick = Tick(tick);
                    let current_tick: Tick = current_tick.unwrap();
                    if tick < current_tick {
                        // Check stop conditions
                        count += 1;
                        if current_tick > stop_tick || count > number {
                            return bins;
                        }

                        // Get active tick
                        bins.push((current_tick.0, self.active_x));
                        self.tick_index.next_up(current_tick)
                    } else {
                        self.tick_index.next_up(tick)
                    }
                },
                None => {
                    let current_tick: Tick = current_tick.unwrap();

                    // Check stop conditions
                    count += 1;
                    if current_tick > stop_tick || count > number {
                        return bins;
                    }

                    // Get active tick
                    bins.push((current_tick.0, self.active_x));
                    self.tick_index.next_up(current_tick)
                },
            };

            // Loop through bins
            let mut next = start_tick;
            loop {
                // If some next active tick
                match next {
                    Some(tick) => {
                        // Check stop conditions
                        count += 1;
                        if tick > stop_tick || count > number {
                            return bins;
                        }

                        // Get bin
                        let bin = self.bin_map.get(&tick).unwrap();
                        let entry = (tick.0, bin.amount);

                        // Add bin
                        bins.push(entry);

                        // Get next tick
                        next = self.tick_index.next_up(tick);
                    }
                    None => {
                        return bins;
                    }
                }
            }
        }

        /// Get the amount of tokens y in bins below the current price.
        /// 
        /// # Arguments
        /// 
        /// * `start_tick` - Tick to start from, not inclusive. If None, start from the current tick, inclusive.
        /// * `stop_tick` - Tick to stop at, inclusive.
        /// * `number` - Number of bins to return.
        /// 
        /// # Returns
        /// 
        /// * `Vec<(u32, Decimal)>` - Bins below the current price in the format (tick, amount_y).
        /// 
        pub fn get_bins_below(
            &self,
            start_tick: Option<u32>,
            stop_tick: Option<u32>,
            number: Option<u32>,
        ) -> Vec<(u32, Decimal)> {
            let mut bins: Vec<(u32, Decimal)> = Vec::new();

            // Return if no bins
            let current_tick = self.tick_index.current();
            if current_tick.is_none() {
                return bins;
            }

            // Get start and stop conditions from optional arguments
            let stop_tick: Tick = match stop_tick {
                Some(tick) => Tick(tick),
                None => Tick::MIN,
            };
            let number: u32 = match number {
                Some(number) => number,
                None => u32::MAX,
            };
            let mut count: u32 = 0;

            let start_tick: Option<Tick> = match start_tick {
                Some(tick) => {
                    let tick = Tick(tick);
                    let current_tick: Tick = current_tick.unwrap();
                    if current_tick < stop_tick || tick > current_tick {
                        // Check stop conditions
                        count += 1;
                        if current_tick < stop_tick || count > number {
                            return bins;
                        }

                        // Add active bin
                        bins.push((current_tick.0, self.active_y));
                        self.tick_index.next_down(current_tick)
                    } else {
                        self.tick_index.next_down(tick)
                    }
                },
                None => {
                    let current_tick: Tick = current_tick.unwrap();

                    // Check stop conditions
                    count += 1;
                    if current_tick < stop_tick || count > number {
                        return bins;
                    }

                    // Add active bin
                    bins.push((current_tick.0, self.active_y));
                    self.tick_index.next_down(current_tick)
                },
            };
            
            // Loop through bins
            let mut next = start_tick;
            loop {
                // If some next tick
                match next {
                    Some(tick) => {
                        // Check stop conditions
                        count += 1;
                        if tick < stop_tick || count > number {
                            return bins;
                        }

                        // Get bin
                        let bin = self.bin_map.get(&tick).unwrap();
                        let entry = (tick.0, bin.amount);

                        // Add bin
                        bins.push(entry);

                        // Get next tick
                        next = self.tick_index.next_down(tick);
                    }
                    None => {
                        return bins;
                    }
                }
            }
        }

        /// Get the liquidity claims of a liquidity receipt by id.
        ///
        /// # Arguments
        /// 
        /// * `liquidity_receipt_id` - Id of the liquidity receipt.
        /// 
        /// # Returns
        /// 
        /// * `HashMap<u32, Decimal>` - Liquidity claims of the liquidity receipt.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt does not exist.
        /// 
        pub fn get_liquidity_claims(&self, liquidity_receipt_id: NonFungibleLocalId) -> HashMap<u32, Decimal> {
            self.liquidity_receipt_manager
                .get_non_fungible_data::<LiquidityReceipt>(&liquidity_receipt_id)
                .liquidity_claims
        }

        /// Get the redemption value of a liquidity receipt in tokens x and y by id.
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt_id` - Id of the liquidity receipt.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens x.
        /// * `Decimal` - Amount of tokens y.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt does not exist.
        /// 
        pub fn get_redemption_value(&self, liquidity_receipt_id: NonFungibleLocalId) -> (Decimal, Decimal) {
            // Get liquidity claims
            let liquidity_claims = self.liquidity_receipt_manager.get_non_fungible_data::<LiquidityReceipt>(&liquidity_receipt_id).liquidity_claims;

            // Amounts sums
            let mut amount_x = Decimal::zero();
            let mut amount_y = Decimal::zero();

            // Loop through liquidity claims
            if let Some(current_tick) = self.tick_index.current() {
                for (tick, claim) in liquidity_claims {
                    let tick: Tick = Tick(tick);

                    // Withdraw tokens
                    if tick < current_tick { // bin below
                        let bin = self.bin_map.get(&tick).unwrap();
                        amount_y += claim / bin.total_claim * bin.amount;
                    } else if tick > current_tick { // bin above
                        let bin = self.bin_map.get(&tick).unwrap();
                        amount_x += claim / bin.total_claim * bin.amount;
                    } else { // active bin
                        let liquidity_share = claim / self.active_total_claim;
                        amount_x += self.active_x * liquidity_share;
                        amount_y += self.active_y * liquidity_share;
                    }
                }
            }

            // Return amounts
            (amount_x, amount_y)
        }

        /// Get the redemption value per bin of a liquidity receipt in tokens x and y by id.
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt_id` - Id of the liquidity receipt.
        /// 
        /// # Returns
        /// 
        /// * `Vec<(u32, Decimal, Decimal)>` - Redemption values for each claim on a bin in the format (tick, amount_x, amount_y).
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt does not exist.
        /// 
        pub fn get_redemption_bin_values(&self, liquidity_receipt_id: NonFungibleLocalId) -> Vec<(u32, Decimal, Decimal)> {
            // Get liquidity claims
            let liquidity_claims = self.liquidity_receipt_manager.get_non_fungible_data::<LiquidityReceipt>(&liquidity_receipt_id).liquidity_claims;

            // Bin redemption values
            let mut redemptions: Vec<(u32, Decimal, Decimal)> = Vec::new();

            // Loop through liquidity claims
            if let Some(current_tick) = self.tick_index.current() {
                for (tick, claim) in liquidity_claims {
                    let tick: Tick = Tick(tick);

                    // Withdraw tokens
                    if tick < current_tick { // bin below
                        let bin = self.bin_map.get(&tick).unwrap();
                        let amount_y = claim / bin.total_claim * bin.amount;
                        redemptions.push((tick.0, Decimal::zero(), amount_y));
                    } else if tick > current_tick { // bin above
                        let bin = self.bin_map.get(&tick).unwrap();
                        let amount_x = claim / bin.total_claim * bin.amount;
                        redemptions.push((tick.0, amount_x, Decimal::zero()));
                    } else { // active bin
                        let liquidity_share = claim / self.active_total_claim;
                        let amount_x = self.active_x * liquidity_share;
                        let amount_y = self.active_y * liquidity_share;
                        redemptions.push((tick.0, amount_x, amount_y));
                    }
                }
            }

            // Sort redemptions by tick
            redemptions.sort_by(|a, b| a.0.cmp(&b.0));

            // Return bin redemptions
            redemptions
        }

        /// Mint a liquidity receipt that is used to track and manage liquidity positions.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Liquidity receipt.
        /// 
        /// # Events
        /// 
        /// * `MintLiquidityReceiptEvent` - Event emitted when liquidity receipt is minted.
        /// 
        pub fn mint_liquidity_receipt(&mut self) -> Bucket {
            // Mint liquidity receipt
            let liquidity_receipt = self.liquidity_receipt_manager.mint_ruid_non_fungible(LiquidityReceipt { 
                liquidity_claims: HashMap::new() 
            });

            // Emit mint liquidity receipt event
            Runtime::emit_event(MintLiquidityReceiptEvent {
                liquidity_receipt_id: liquidity_receipt.as_non_fungible().non_fungible_local_id(),
            });

            // Return liquidity receipt
            liquidity_receipt
        }

        /// Burn a liquidity receipt.
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt` - Liquidity receipt to burn.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt is invalid.
        /// * If the liquidity receipt has liquidity claims.
        /// 
        /// # Events
        /// 
        /// * `BurnLiquidityReceiptEvent` - Event emitted when a liquidity receipt is burned.
        /// 
        pub fn burn_liquidity_receipt(&mut self, liquidity_receipt: Bucket) {
            assert!(
                liquidity_receipt.resource_address() == self.liquidity_receipt_manager.address(),
                "Invalid liquidity receipt."
            );

            // Get liquidity claims
            let liquidity_receipt = liquidity_receipt.as_non_fungible();
            let liquidity_receipt_id =  liquidity_receipt.non_fungible_local_id();
            let liquidity_receipt_nft = liquidity_receipt.non_fungible::<LiquidityReceipt>();
            let liquidity_claims = liquidity_receipt_nft.data().liquidity_claims;

            // Assert that there are no liquidity claims
            assert!(
                liquidity_claims.is_empty(),
                "Cannot burn liquidity receipt with liquidity claims."
            );

            // Burn liquidity receipt
            liquidity_receipt.burn();

            // Emit burn liquidity receipt event
            Runtime::emit_event(BurnLiquidityReceiptEvent {
                liquidity_receipt_id
            });
        }

        /// Add tokens as liquidity at the specified positions to a liquidity receipt.
        /// If the position is above the active bin, tokens x will be used.
        /// If the position is below the active bin, tokens y will be used.
        /// If the position is at the current price, tokens x and y will be used in proportion current ratio.
        /// If it is the first liquidity position to be added, all tokens will be used and it will become the active bin.
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt` - Liquidity receipt that will be used to track the liquidity claims.
        /// * `tokens_x` - Tokens x to add as liquidity.
        /// * `tokens_y` - Tokens y to add as liquidity.
        /// * `positions` - Positions at which to add liquidity. Format: (tick, amount_x, amount_y).
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Updated liquidity receipt.
        /// * `Bucket` - Tokens x that were not used.
        /// * `Bucket` - Tokens y that were not used.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt is invalid.
        /// * If the tick of a position is not in the range of [0, 54000].
        /// * If the tick of a position is not aligned to the bin span.
        /// * If there are not enough tokens to add liquidity.
        /// * If the total number of liquidity claims for the liquidity receipt exceeds 200.
        /// 
        /// # Events
        /// 
        /// * `AddLiquidityEvent` - Event emitted when liquidity is added.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// 
        pub fn add_liquidity_to_receipt(
            &mut self, 
            liquidity_receipt: Bucket,
            mut tokens_x: Bucket, 
            mut tokens_y: Bucket, 
            positions: Vec<(u32, Decimal, Decimal)>,
        ) -> (Bucket, Bucket, Bucket) {
            assert!(
                liquidity_receipt.resource_address() == self.liquidity_receipt_manager.address(),
                "Invalid liquidity receipt."
            );

            // Get the current tick
            let mut current_tick = self.tick_index.current();

            // Get liquidity claims
            let liquidity_receipt = liquidity_receipt.as_non_fungible();
            let liquidity_receipt_nft = liquidity_receipt.non_fungible::<LiquidityReceipt>();
            let mut liquidity_claims = liquidity_receipt_nft.data().liquidity_claims;

            // Amounts to track tokens used
            let mut amount_x_used = Decimal::zero();
            let mut amount_y_used = Decimal::zero();
            let mut added_x: Vec<(u32, Decimal)> = Vec::new();
            let mut added_y: Vec<(u32, Decimal)> = Vec::new();
            
            for (tick, amount_x, amount_y) in positions {
                let tick: Tick = Tick(tick);
                assert!(tick.is_valid(self.bin_span), "Invalid tick {:?}.", tick);
                assert!(amount_x >= Decimal::zero() && amount_y >= Decimal::zero(), "Amounts must be greater than or equal to zero.");

                // Deposit tokens
                let claim = if current_tick.is_none() {
                    if amount_x == Decimal::zero() && amount_y == Decimal::zero() {
                        continue;
                    }

                    let claim = self.add_initial_liquidity(tick, amount_x, amount_y);
                    current_tick = self.tick_index.current();
                    amount_x_used += amount_x;
                    amount_y_used += amount_y;
                    added_x.push((tick.0, amount_x));
                    added_y.push((tick.0, amount_y));

                    claim
                } else if tick > current_tick.unwrap() && amount_x > Decimal::zero() {
                    let claim = self.add_liquidity_to_bin(tick, amount_x);
                    amount_x_used += amount_x;
                    added_x.push((tick.0, amount_x));

                    claim
                } else if tick < current_tick.unwrap() && amount_y > Decimal::zero() {
                    let claim = self.add_liquidity_to_bin(tick, amount_y);
                    amount_y_used += amount_y;
                    added_y.push((tick.0, amount_y));

                    claim
                } else if tick == current_tick.unwrap() && amount_x > Decimal::zero() && amount_y > Decimal::zero() {
                    let (claim, change_x, change_y) = self.add_active_liquidity(amount_x, amount_y);
                    amount_x_used += change_x;
                    amount_y_used += change_y;
                    added_x.push((tick.0, change_x));
                    added_y.push((tick.0, change_y));

                    claim
                } else {
                    continue;
                };

                // Add to liquidity claim
                if let Some(position) = liquidity_claims.get_mut(&tick.0) {
                    *position += claim;
                } else {
                    liquidity_claims.insert(tick.0, claim);
                }
            }

            // Deposit tokens
            self.tokens_x.put(tokens_x.take_advanced(amount_x_used, INCOMING));
            self.tokens_y.put(tokens_y.take_advanced(amount_y_used, INCOMING));

            // Assert that there are no more than 200 liquidity claims
            assert!(
                liquidity_claims.len() <= 200,
                "Too many liquidity claims for this liquidity receipt."
            );

            // Update liquidity receipt
            self.liquidity_receipt_manager.update_non_fungible_data(
                liquidity_receipt_nft.local_id(), 
                "liquidity_claims", 
                liquidity_claims
            );

            // Emit add liquidity event
            Runtime::emit_event(AddLiquidityEvent {
                liquidity_receipt_id: liquidity_receipt_nft.local_id().clone(),
                amount_change_x: added_x.iter().fold(dec!(0), |sum, &(_, x)| sum + x),
                amount_change_y: added_y.iter().fold(dec!(0), |sum, &(_, y)| sum + y),
                added_x,
                added_y,
            });

            // Emit valuation event
            Runtime::emit_event(ValuationEvent {
                amount_after_x: self.get_amount_x(),
                amount_after_y: self.get_amount_y(),
                price_after: self.get_price().unwrap_or_default(),
            });

            // Return buckets
            (liquidity_receipt.into(), tokens_x, tokens_y)
        }

        /// Add tokens as liquidity at the specified positions.
        /// If the position is above the active bin, tokens x will be used.
        /// If the position is below the active bin, tokens y will be used.
        /// If the position is at the current price, tokens x and y will be used in proportion current ratio.
        /// If it is the first liquidity position to be added, all tokens will be used and it will become the active bin.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_x` - Tokens x to add as liquidity.
        /// * `tokens_y` - Tokens y to add as liquidity.
        /// * `positions` - Positions at which to add liquidity. Format: (tick, amount_x, amount_y).
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Liquidity receipt.
        /// * `Bucket` - Tokens x that were not used.
        /// * `Bucket` - Tokens y that were not used.
        /// 
        /// # Panics
        /// 
        /// * If the tick of a position is not in the range of [0, 54000].
        /// * If the tick of a position is not aligned to the bin span.
        /// * If there are not enough tokens to add liquidity.
        /// * If the total number of liquidity claims for the liquidity receipt exceeds 200.
        /// 
        /// # Events
        /// 
        /// * `MintLiquidityReceiptEvent` - Event emitted when liquidity receipt is minted.
        /// * `AddLiquidityEvent` - Event emitted when liquidity is added.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// 
        pub fn add_liquidity(
            &mut self,
            tokens_x: Bucket,
            tokens_y: Bucket,
            positions: Vec<(u32, Decimal, Decimal)>,
        ) -> (Bucket, Bucket, Bucket) {
            // Mint liquidity receipt
            let liquidity_receipt = self.mint_liquidity_receipt();

            // Add liquidity to receipt and return buckets
            self.add_liquidity_to_receipt(liquidity_receipt, tokens_x, tokens_y, positions)
        }

        /// Remove liquidity from a liquidity receipt using the specified liquidity claims.
        /// Returns (tokens_x, tokens_y).
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt` - Liquidity receipt that contains the liquidity claims.
        /// * `claims` - Liquidity claims to remove. Format: (tick, amount_claim).
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Updated liquidity receipt.
        /// * `Bucket` - Tokens x that were removed.
        /// * `Bucket` - Tokens y that were removed.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt is invalid.
        /// * If the claim amount is not greater than zero.
        /// * If the claim does not exist.
        /// 
        /// # Events
        /// 
        /// * `RemoveLiquidityEvent` - Event emitted when liquidity is removed.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// 
        pub fn remove_specific_liquidity(&mut self, liquidity_receipt: Bucket, claims: Vec<(u32, Decimal)>) -> (Bucket, Bucket, Bucket) {
            assert!(
                liquidity_receipt.resource_address() == self.liquidity_receipt_manager.address(),
                "Invalid liquidity receipt."
            );

            // Get liquidity claims
            let liquidity_receipt = liquidity_receipt.as_non_fungible();
            let liquidity_receipt_nft = liquidity_receipt.non_fungible::<LiquidityReceipt>();
            let mut liquidity_claims = liquidity_receipt_nft.data().liquidity_claims;

            // Amounts to track tokens claimed
            let mut amount_x = Decimal::zero();
            let mut amount_y = Decimal::zero();
            let mut removed_x: Vec<(u32, Decimal)> = Vec::new();
            let mut removed_y: Vec<(u32, Decimal)> = Vec::new();

            // Get current tick
            let mut current_tick = match self.tick_index.current() {
                Some(tick) => tick,
                None => return (liquidity_receipt.into(), Bucket::new(self.tokens_x.resource_address()), Bucket::new(self.tokens_y.resource_address())),
            };

            // Remove liquidity
            for (tick, claim) in claims {
                assert!(
                    claim > Decimal::zero(),
                    "Claim must be greater than zero."
                );

                let available_claim = liquidity_claims.get_mut(&tick).expect("Claim does not exist.");
                let claim = if *available_claim > claim {
                    *available_claim -= claim;
                    claim
                } else {
                    liquidity_claims.remove(&tick).unwrap()
                };
                let tick: Tick = Tick(tick);

                // Withdraw tokens
                if tick > current_tick { // bin above
                    let change_x = self.remove_liquidity_from_bin(tick, claim);
                    amount_x += change_x;
                    removed_x.push((tick.0, -change_x));
                } else if tick < current_tick { // bin below
                    let change_y = self.remove_liquidity_from_bin(tick, claim);
                    amount_y += change_y;
                    removed_y.push((tick.0, -change_y));
                } else { // active bin
                    let (change_x, change_y) = self.remove_active_liquidity(current_tick, claim);
                    amount_x += change_x;
                    amount_y += change_y;
                    removed_x.push((tick.0, -change_x));
                    removed_y.push((tick.0, -change_y));

                    // Update current tick incase it changed
                    let check_tick = self.tick_index.current();
                    if check_tick.is_some() {
                        current_tick = check_tick.unwrap();
                    }
                }
            }

            // Update liquidity receipt
            self.liquidity_receipt_manager.update_non_fungible_data(
                liquidity_receipt_nft.local_id(), 
                "liquidity_claims", 
                liquidity_claims
            );

            // Create buckets
            let tokens_x = self.tokens_x.take_advanced(amount_x, OUTGOING);
            let tokens_y = self.tokens_y.take_advanced(amount_y, OUTGOING);

            // Emit remove liquidity event
            Runtime::emit_event(RemoveLiquidityEvent {
                liquidity_receipt_id: liquidity_receipt_nft.local_id().clone(),
                amount_change_x: removed_x.iter().fold(dec!(0), |sum, &(_, x)| sum + x),
                amount_change_y: removed_y.iter().fold(dec!(0), |sum, &(_, y)| sum + y),
                removed_x,
                removed_y,
            });

            // Emit valuation event
            Runtime::emit_event(ValuationEvent {
                amount_after_x: self.get_amount_x(),
                amount_after_y: self.get_amount_y(),
                price_after: self.get_price().unwrap_or_default(),
            });

            // Return buckets
            (liquidity_receipt.into(), tokens_x, tokens_y)
        }

        /// Remove all liquidity from a liquidity receipt and burn the receipt.
        /// Returns (tokens_x, tokens_y).
        /// 
        /// # Arguments
        /// 
        /// * `liquidity_receipt` - Liquidity receipt that contains the liquidity claims.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Tokens x that were removed.
        /// * `Bucket` - Tokens y that were removed.
        /// 
        /// # Panics
        /// 
        /// * If the liquidity receipt is invalid.
        /// * If the claim amount is not greater than zero.
        /// * If the claim does not exist.
        /// 
        /// # Events
        /// 
        /// * `RemoveLiquidityEvent` - Event emitted when liquidity is removed.
        /// * `BurnLiquidityReceiptEvent` - Event emitted when a liquidity receipt is burned.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// 
        pub fn remove_liquidity(
            &mut self,
            liquidity_receipt: Bucket,
        ) -> (Bucket, Bucket) {
            // Get liquidity claims
            let liquidity_receipt_nft = liquidity_receipt.as_non_fungible().non_fungible::<LiquidityReceipt>();
            let liquidity_claims: Vec<(u32, Decimal)> = liquidity_receipt_nft.data().liquidity_claims.into_iter().collect();

            // Remove liquidity
            let (liquidity_receipt, token_x, tokens_y) = self.remove_specific_liquidity(liquidity_receipt, liquidity_claims);

            // Burn liquidity receipt
            self.burn_liquidity_receipt(liquidity_receipt);

            // Return buckets
            (token_x, tokens_y)
        }

        /// Swap tokens.
        /// 
        /// # Arguments
        /// 
        /// * `tokens` - Tokens to swap.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Tokens bought.
        /// * `Bucket` - Tokens leftover.
        /// 
        /// # Panics
        /// 
        /// * If the tokens are not tokens x or tokens y.
        /// * If the swap would result in a breaking state change due to extreme liquidity conditions.
        /// 
        /// # Events
        /// 
        /// * `SwapEvent` - Event emitted when tokens are swapped.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// * `LiquidityFeeEvent` - Event emitted when liquidity fee is collected.
        /// 
        pub fn swap(&mut self, tokens: Bucket) -> (Bucket, Bucket) {
            let token_address = tokens.resource_address();
            if token_address == self.tokens_x.resource_address() {
                self.swap_x(tokens)
            } else if token_address == self.tokens_y.resource_address() {
                self.swap_y(tokens)
            } else {
                panic!("Invalid token address.")
            }
        }

        /// Swap tokens x for tokens y.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_x` - Tokens x to swap.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Tokens y bought.
        /// * `Bucket` - Tokens x leftover.
        /// 
        /// # Requires
        /// 
        /// * `tokens_x` - Tokens x are of the correct type.
        /// 
        /// # Events
        /// 
        /// * `SwapEvent` - Event emitted when tokens are swapped.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// * `LiquidityFeeEvent` - Event emitted when liquidity fee is collected.
        ///
        fn swap_x(&mut self, mut tokens_x: Bucket) -> (Bucket, Bucket) {
            // Get the current tick
            let current_tick = self.tick_index.current();
            if current_tick.is_none() {
                return (Bucket::new(self.tokens_y.resource_address()), tokens_x);
            }

            // Get fee percentages
            let (protocol_fee, liquidity_fee) = FEE_CONTROLLER.get_fees(
                Runtime::package_address(), 
                vec![self.tokens_x.resource_address(), self.tokens_y.resource_address()]);
            
            // Separate fee tokens
            let mut protocol_fee_tokens = tokens_x.take_advanced(tokens_x.amount() * protocol_fee, INCOMING);
            let mut liquidity_fee_tokens = tokens_x.take_advanced(tokens_x.amount() * liquidity_fee, INCOMING);
            let amount_x_input = tokens_x.amount();

            // Variables to track token amounts
            let mut amount_x = amount_x_input;
            let mut amount_y = Decimal::zero();
            
            // Loop through bins
            let mut tick: Tick = current_tick.unwrap();
            loop {
                // Get virtual amounts
                let (virtual_x, virtual_y) = self.virtual_amounts();

                // Calculate swap
                let amount_y_swap = calculate_swap(&amount_x, &virtual_x, &virtual_y);

                // Check if swap is not possible, meaning bounds of the active bin have been reached
                if amount_y_swap >= self.active_y {
                    // Calculate inverse swap, round up from zero
                    let amount_x_swap = calculate_swap_inverse(&self.active_y, &virtual_x, &virtual_y);
                    let amount_x_swap = if amount_x_swap > Decimal::zero() {
                        amount_x_swap
                    } else if self.active_y == Decimal::zero() {
                        Decimal::ZERO
                    } else {
                        Decimal(I192::from_digits([1, 0, 0]))
                    };
                    let amount_x_swap_with_fee = amount_x_swap / (Decimal::ONE - liquidity_fee);

                    // Update amounts
                    amount_x -= amount_x_swap;
                    amount_y += self.active_y;
                    
                    // Check if there is another bin
                    let next_tick: Option<Tick> = self.tick_index.move_down();
                    if next_tick.is_some() {
                        // Save active bin
                        self.bin_map.insert(tick, 
                            Bin {
                                amount: self.active_x + amount_x_swap_with_fee, 
                                total_claim: self.active_total_claim
                            }
                        );

                        // Get next bin
                        tick = next_tick.unwrap();
                        let bin = self.bin_map.get(&tick).unwrap();

                        // Update active bin state
                        self.active_x = Decimal::zero();
                        self.active_y = bin.amount;
                        self.active_total_claim = bin.total_claim;
                        self.lower_limit = tick.into();
                        self.upper_limit = tick.tick_upper(self.bin_span).into();
                    } else {
                        // Update active bin state
                        self.active_x += amount_x_swap_with_fee;
                        self.active_y = Decimal::zero();

                        break;
                    }
                } else {
                    let amount_x_swap_with_fee = amount_x / (Decimal::ONE - liquidity_fee);

                    // Update amounts
                    amount_x = Decimal::zero();
                    amount_y += amount_y_swap;

                    // Update active bin state
                    self.active_x += amount_x_swap_with_fee;
                    self.active_y -= amount_y_swap;

                    break;
                }
            }

            // Create withdraw and deposit tokens
            let tokens_y = self.tokens_y.take_advanced(amount_y, OUTGOING);
            let amount_y_swapped = tokens_y.amount();
            let tokens_x_swapped = tokens_x.take_advanced(tokens_x.amount() - amount_x, INCOMING);
            let amount_x_swapped = tokens_x_swapped.amount();
            self.tokens_x.put(tokens_x_swapped);

            // Replace unused component fee
            if amount_x != Decimal::zero() {
                let unused = tokens_x.amount() / amount_x_input;
                tokens_x.put(protocol_fee_tokens.take_advanced(protocol_fee_tokens.amount() * unused, OUTGOING));
                tokens_x.put(liquidity_fee_tokens.take_advanced(liquidity_fee_tokens.amount() * unused, OUTGOING));
            }

            // Deposit fees
            let amount_protocol_fee = protocol_fee_tokens.amount();
            let amount_liquidity_fee = liquidity_fee_tokens.amount();
            FEE_VAULTS.deposit(protocol_fee_tokens);
            self.tokens_x.put(liquidity_fee_tokens);

            // Emit fee events
            Runtime::emit_event(ProtocolFeeEvent {
                token_address: self.tokens_x.resource_address(),
                amount: amount_protocol_fee,
            });
            Runtime::emit_event(LiquidityFeeEvent {
                token_address: self.tokens_x.resource_address(),
                amount: amount_liquidity_fee,
            });

            // Get price after swap
            let price_after = self.get_price().unwrap_or_default();

            // Emit swap event
            Runtime::emit_event(SwapEvent {
                amount_change_x: amount_x_swapped + amount_liquidity_fee,
                amount_change_y: -amount_y_swapped,
                price_after,
            });

            // Emit valuation event
            Runtime::emit_event(ValuationEvent {
                amount_after_x: self.get_amount_x(),
                amount_after_y: self.get_amount_y(),
                price_after,
            });

            // Return tokens
            (tokens_y, tokens_x)
        }

        /// Swap tokens y for tokens x.
        /// 
        /// # Arguments
        /// 
        /// * `tokens_y` - Tokens y to swap.
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - Tokens x bought.
        /// * `Bucket` - Tokens y leftover.
        /// 
        /// # Requires
        /// 
        /// * `tokens_y` - Tokens y are of the correct type.
        /// 
        /// # Events
        /// 
        /// * `SwapEvent` - Event emitted when tokens are swapped.
        /// * `ValuationEvent` - Event emitted when the value of the pool changes.
        /// * `ProtocolFeeEvent` - Event emitted when protocol fee is collected.
        /// * `LiquidityFeeEvent` - Event emitted when liquidity fee is collected.
        /// 
        fn swap_y(&mut self, mut tokens_y: Bucket) -> (Bucket, Bucket) {
            // Get the current tick
            let current_tick = self.tick_index.current();
            if current_tick.is_none() {
                return (Bucket::new(self.tokens_x.resource_address()), tokens_y);
            }

            // Get fee percentages
            let (protocol_fee, liquidity_fee) = FEE_CONTROLLER.get_fees(
                Runtime::package_address(), 
                vec![self.tokens_x.resource_address(), self.tokens_y.resource_address()]);

            // Separate fee tokens
            let mut protocol_fee_tokens = tokens_y.take_advanced(tokens_y.amount() * protocol_fee, INCOMING);
            let mut liquidity_fee_tokens = tokens_y.take_advanced(tokens_y.amount() * liquidity_fee, INCOMING);
            let amount_y_input = tokens_y.amount();

            // Variables to track token amounts
            let mut amount_y = amount_y_input;
            let mut amount_x = Decimal::zero();
            
            // Loop through liquidity pots
            let mut tick = current_tick.unwrap();
            loop {
                // Get virtual amounts
                let (virtual_x, virtual_y) = self.virtual_amounts();

                // Calculate swap
                let amount_x_swap = calculate_swap(&amount_y, &virtual_y, &virtual_x);

                // Check if swap is not possible, meaning bounds of the active bin have been reached
                if amount_x_swap >= self.active_x {
                    // Calculate inverse swap, round up from zero
                    let amount_y_swap = calculate_swap_inverse(&self.active_x, &virtual_y, &virtual_x);
                    let amount_y_swap = if amount_y_swap > Decimal::zero() {
                        amount_y_swap
                    } else if self.active_x == Decimal::zero() {
                        Decimal::ZERO
                    } else {
                        Decimal(I192::from_digits([1, 0, 0]))
                    };
                    let amount_y_swap_with_fee = amount_y_swap / (Decimal::ONE - liquidity_fee);

                    // Update amounts
                    amount_x += self.active_x;
                    amount_y -= amount_y_swap;
                    
                    // Check if there is another bin
                    let next_tick: Option<Tick> = self.tick_index.move_up();
                    if next_tick.is_some() {
                        // Save active bin
                        self.bin_map.insert(tick, 
                            Bin {
                                amount: self.active_y + amount_y_swap_with_fee,
                                total_claim: self.active_total_claim
                            }
                        );

                        // Get next bin
                        tick = next_tick.unwrap();
                        let bin = self.bin_map.get(&tick).unwrap();

                        // Update active bin state
                        self.active_x = bin.amount;
                        self.active_y = Decimal::zero();
                        self.active_total_claim = bin.total_claim;
                        self.lower_limit = tick.into();
                        self.upper_limit = tick.tick_upper(self.bin_span).into();
                    } else {
                        // Update active bin state
                        self.active_x = Decimal::zero();
                        self.active_y += amount_y_swap_with_fee;

                        break;
                    }
                } else {
                    let amount_y_swap_with_fee = amount_y / (Decimal::ONE - liquidity_fee);

                    // Update amounts
                    amount_y = Decimal::zero();
                    amount_x += amount_x_swap;

                    // Update active bin state
                    self.active_x -= amount_x_swap;
                    self.active_y += amount_y_swap_with_fee;

                    break;
                }
            }

            // Create withdraw and deposit tokens
            let tokens_x = self.tokens_x.take_advanced(amount_x, OUTGOING);
            let amount_x_swapped = tokens_x.amount();
            let tokens_y_swapped = tokens_y.take_advanced(tokens_y.amount() - amount_y, INCOMING);
            let amount_y_swapped = tokens_y_swapped.amount();
            self.tokens_y.put(tokens_y_swapped);

            // Replace unused component fee
            if amount_y != Decimal::zero() {
                let unused = tokens_y.amount() / amount_y_input;
                tokens_y.put(protocol_fee_tokens.take_advanced(protocol_fee_tokens.amount() * unused, OUTGOING));
                tokens_y.put(liquidity_fee_tokens.take_advanced(liquidity_fee_tokens.amount() * unused, OUTGOING));
            }

            // Deposit fees
            let amount_protocol_fee = protocol_fee_tokens.amount();
            let amount_liquidity_fee = liquidity_fee_tokens.amount();
            FEE_VAULTS.deposit(protocol_fee_tokens);
            self.tokens_y.put(liquidity_fee_tokens);

            // Emit fee events
            Runtime::emit_event(ProtocolFeeEvent {
                token_address: self.tokens_y.resource_address(),
                amount: amount_protocol_fee,
            });
            Runtime::emit_event(LiquidityFeeEvent {
                token_address: self.tokens_y.resource_address(),
                amount: amount_liquidity_fee,
            });

            // Get price after swap
            let price_after = self.get_price().unwrap_or_default();

            // Emit swap event
            Runtime::emit_event(SwapEvent {
                amount_change_x: -amount_x_swapped,
                amount_change_y: amount_y_swapped + amount_liquidity_fee,
                price_after,
            });

            // Emit valuation event
            Runtime::emit_event(ValuationEvent {
                amount_after_x: self.get_amount_x(),
                amount_after_y: self.get_amount_y(),
                price_after,
            });

            // Return tokens
            (tokens_x, tokens_y)
        }

        /// Helper method to add initial liquidity.
        /// 
        /// # Arguments
        /// 
        /// * `tick` - Tick of the initial liquidity.
        /// * `amount_x` - Amount of tokens x to add.
        /// * `amount_y` - Amount of tokens y to add.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of claim.
        /// 
        /// # Requires
        /// 
        /// * `tick` - Tick is valid.
        /// * Either `amount_x` or `amount_y` is greater than zero.
        /// * Neither `amount_x` or `amount_y` is less than zero.
        /// 
        fn add_initial_liquidity(&mut self, tick: Tick, amount_x: Decimal, amount_y: Decimal) -> Decimal {
            // Insert tick into tick index
            self.tick_index.insert(tick);

            // Find upper and lower limits of bin
            self.lower_limit = tick.into();
            self.upper_limit = tick.tick_upper(self.bin_span).into();
            
            // Calculate claim amount
            let claim = amount_x.max(amount_y);
            self.active_total_claim = claim;

            // Set liquidity for active bin
            self.active_x = amount_x;
            self.active_y = amount_y;

            // Return claim
            claim
        }

        /// Helper method to add liquidity to the active bin.
        /// 
        /// # Arguments
        /// 
        /// * `amount_x` - Amount of tokens x to add.
        /// * `amount_y` - Amount of tokens y to add.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of claim.
        /// * `Decimal` - Amount of tokens x that were used.
        /// * `Decimal` - Amount of tokens y that were used.
        /// 
        /// # Requires
        /// 
        /// * Both `amount_x` and `amount_y` are greater than zero.
        /// 
        fn add_active_liquidity(&mut self, amount_x: Decimal, amount_y: Decimal) -> (Decimal, Decimal, Decimal) {
            if self.active_x == Decimal::zero() { // Only add tokens y
                // Calculate claim amount
                let claim: Decimal = amount_y / self.active_y * self.active_total_claim;
                self.active_total_claim += claim;

                // Add liquidity to active bin
                self.active_y += amount_y;

                // Return claim and amounts used
                (claim, Decimal::zero(), amount_y)
            } else if self.active_y == Decimal::zero() { // Only add tokens x
                // Calculate claim amount
                let claim: Decimal = amount_x / self.active_x * self.active_total_claim;
                self.active_total_claim += claim;

                // Add liquidity to active bin
                self.active_x += amount_x;

                // Return claim and amounts used
                (claim, amount_x, Decimal::zero())
            } else { // Add tokens x and y
                // Calculate ratio of tokens in active bin
                let token_ratio = self.active_y / self.active_x;

                // Determine if amount x or y is the limiting factor
                let amount_x_in_y = amount_x * token_ratio;
                let (amount_x, amount_y, claim) = if amount_x_in_y > amount_y { // Amount y is limiting factor
                    // Calculate claim and token amounts based on tokens y
                    let claim = amount_y / self.active_y * self.active_total_claim;
                    let amount_y_in_x = amount_y / token_ratio;
                    (amount_y_in_x, amount_y, claim)
                } else { // Amount x is limiting factor
                    // Calculate claim and token amounts based on tokens x
                    let claim = amount_x / self.active_x * self.active_total_claim;
                    (amount_x, amount_x_in_y, claim)
                };

                // Add liquidity to active bin
                self.active_x += amount_x;
                self.active_y += amount_y;
                self.active_total_claim += claim;

                // Return claim and amounts used
                (claim, amount_x, amount_y)
            }
        }

        /// Helper method to add liquidity to a dormant bin.
        /// 
        /// # Arguments
        /// 
        /// * `tick` - Tick of the bin.
        /// * `amount` - Amount of tokens to add.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of claim.
        /// 
        /// # Requires
        /// 
        /// * `tick` - Tick is valid.
        /// * `amount` - Amount is greater than zero.
        ///  
        fn add_liquidity_to_bin(&mut self, tick: Tick, amount: Decimal) -> Decimal {
            let (bin, claim) = match self.bin_map.get(&tick) {
                Some(bin) => {
                    let mut bin = *bin;
                    if bin.amount == Decimal::zero() { // Bin is empty
                        // Insert tick into tick index
                        self.tick_index.insert(tick);
    
                        // Set liquidity for bin
                        bin.amount = amount;
                        bin.total_claim = amount;
                        
                        (bin, amount)
                    } else { // Bin is not empty
                        // Calculate claim amount
                        let claim: Decimal = amount / bin.amount * bin.total_claim;
                        bin.total_claim += claim;
    
                        // Add liquidity to bin
                        bin.amount += amount;
                        
                        (bin, claim)
                    }
                },
                None => {
                    // Insert tick into tick index
                    self.tick_index.insert(tick);

                    (Bin {amount, total_claim: amount}, amount)
                }
            };
            self.bin_map.insert(tick, bin);

            // Return claim amount
            claim
        }

        /// Helper method to remove liquidity from the active bin.
        /// 
        /// # Arguments
        /// 
        /// * `current_tick` - Current tick.
        /// * `claim` - Amount of claim to remove.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens x to remove.
        /// * `Decimal` - Amount of tokens y to remove.
        /// 
        fn remove_active_liquidity(&mut self, current_tick: Tick, claim: Decimal) -> (Decimal, Decimal) {
            if claim == self.active_total_claim { // Remove all liquidity from active bin
                // Get token amounts to remove
                let amount_x_claim = self.active_x;
                let amount_y_claim = self.active_y;
                
                // Remove from price index
                self.tick_index.remove(current_tick);
                self.bin_map.insert(current_tick, 
                    Bin {
                        amount: Decimal::zero(), 
                        total_claim: Decimal::zero()
                    }
                );

                // Get next bin
                let next_tick: Option<Tick> = self.tick_index.current();
                if next_tick.is_some() { // There is another bin
                    // Set active bin to next bin
                    let next_tick = next_tick.unwrap();
                    let bin = self.bin_map.get(&next_tick).unwrap();

                    // Update active bin state
                    if next_tick > current_tick {
                        self.active_x = bin.amount;
                        self.active_y = Decimal::zero();
                    } else {
                        self.active_x = Decimal::zero();
                        self.active_y = bin.amount;
                    }
                    self.active_total_claim = bin.total_claim;
 
                    // Calculate new active bin limits
                    self.lower_limit = next_tick.into();
                    self.upper_limit = next_tick.tick_upper(self.bin_span).into();
                } else { // There is no other bin
                    // Zero active bin state
                    self.active_x = Decimal::zero();
                    self.active_y = Decimal::zero();
                    self.active_total_claim = Decimal::zero();

                    self.lower_limit = Decimal::zero();
                    self.upper_limit = Decimal::zero();
                }

                // Return amounts of tokens to remove
                (amount_x_claim, amount_y_claim)
            } else { // Remove part liquidity from active bin
                // Calculate amounts to remove
                let liquidity_share = claim / self.active_total_claim;
                let amount_x_claim = self.active_x * liquidity_share;
                let amount_y_claim = self.active_y * liquidity_share;

                // Update active bin state
                self.active_x -= amount_x_claim;
                self.active_y -= amount_y_claim;
                self.active_total_claim -= claim;

                // Return amounts of tokens to remove
                (amount_x_claim, amount_y_claim)
            }
        }

        /// Helper method to remove liquidity from a dormant bin.
        /// 
        /// # Arguments
        /// 
        /// * `tick` - Tick of the bin.
        /// * `claim` - Amount of claim to remove.
        /// 
        /// # Returns
        /// 
        /// * `Decimal` - Amount of tokens to remove.
        /// 
        fn remove_liquidity_from_bin(&mut self, tick: Tick, claim: Decimal) -> Decimal {
            // Get bin
            let bin: &mut Bin = &mut self.bin_map.get_mut(&tick).unwrap();
            if claim == bin.total_claim { // Remove all liquidity from bin
                // Remove from price index
                self.tick_index.remove(tick);

                // Get amount of tokens to remove
                let amount = bin.amount;

                // Update bin state
                bin.amount = Decimal::zero();
                bin.total_claim = Decimal::zero();

                // Return amount of tokens to remove
                amount
            } else { // Remove part liquidity from bin
                // Calculate amount of tokens to remove
                let amount = claim / bin.total_claim * bin.amount;

                // Update bin state
                bin.amount -= amount;
                bin.total_claim -= claim;
                
                // Return amount of tokens to remove
                amount
            }
        }

        /// Get the virtual amounts of the active bin. 
        /// 
        /// # Returns
        /// 
        /// * `I512` - Virtual amount of tokens x in `I512` with base `10^36`.
        /// * `I512` - Virtual amount of tokens y in `I512` with base `10^36`.
        /// 
        fn virtual_amounts(&self) -> (I512, I512) {
            // Calculate virtual amounts
            calculate_virtual_amounts(&self.active_x, &self.active_y, &self.upper_limit, &self.lower_limit)
        }
    }
}
