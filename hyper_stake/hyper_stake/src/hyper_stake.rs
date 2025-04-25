pub mod consts;
pub mod events;
pub mod structs;
pub mod swap_math;

use scrypto::prelude::*;
pub use consts::*;
pub use events::*;
pub use structs::*;
pub use swap_math::*;

#[blueprint]
#[events(
    NewPoolEvent,
    SetFeeShareEvent,
    LiquidityChangeEvent,
    SwapEvent,
)]
mod hyper_stake {
    const OWNER_RESOURCE: ResourceAddress = _OWNER_RESOURCE;
    const LSULP_RESOURCE: ResourceAddress = _LSULP_RESOURCE;
    
    const LSU_POOL_COMPONENT: ComponentAddress = _LSU_POOL_COMPONENT;
    const FEE_VAULTS_COMPONENT: ComponentAddress = _FEE_VAULTS_COMPONENT;
    
    extern_blueprint! {
        FEE_VAULTS_PACKAGE,
        FeeVaults {
            fn deposit(&self, tokens: Bucket);
            fn treasury_deposit(&self, tokens: Bucket);
        }
    }
    extern_blueprint! {
        LSU_POOL_PACKAGE,
        LsuPool {
            fn get_dex_valuation_xrd(&self) -> Decimal;
        }
    }

    enable_method_auth! {
        roles {
            liquidity_user => updatable_by: [OWNER];
            swap_user => updatable_by: [OWNER];
        },
        methods {
            set_protocol_fee_share => restrict_to: [OWNER];
            set_treasury_fee_share => restrict_to: [OWNER];

            get_oracle_price => PUBLIC;
            get_info => PUBLIC;
            get_redemption_value => PUBLIC;

            add_liquidity => restrict_to: [liquidity_user];
            remove_liquidity => PUBLIC;
            swap => restrict_to: [swap_user];
        }
    }

    struct HyperStake {
        resource_x: ResourceAddress,
        resource_y: ResourceAddress,
        lp_resource: ResourceAddress,
        lsu_pool: Global<LsuPool>,
        upper_offset: Decimal,
        lower_offset: Decimal,
        fee: Decimal,
        protocol_fee_share: Decimal,
        treasury_fee_share: Decimal,
        resource_pool: Global<TwoResourcePool>,
        fee_vaults: Global<FeeVaults>,
    }

    impl HyperStake {
        /// Instantiates a new lsulp tethered pool
        /// 
        /// # Arguments
        /// 
        /// * `upper_offset` - The upper offset
        /// * `lower_offset` - The lower offset
        /// * `fee` - The swap fee
        /// * `reservation` - The reservation to use for the component address
        /// 
        /// # Returns
        /// 
        /// * `Global<LsulpTetheredPool>` - The pool global
        /// 
        /// # Panics
        /// 
        /// * If the fee is not between 0.0001 and 0.02
        /// * If the upper offset is not greater than the lower offset by a factor of 1.001
        /// * If upper or lower offset is not positive
        /// 
        pub fn new(
            upper_offset: Decimal,
            lower_offset: Decimal,
            fee: Decimal,
            reservation: Option<GlobalAddressReservation>,
        ) -> Global<HyperStake> {
            let (component_reservation, this) = match reservation {
                Some(reservation) => {
                    let this = Runtime::get_reservation_address(&reservation).try_into().unwrap();
                    (reservation, this)
                },
                None => Runtime::allocate_component_address(HyperStake::blueprint_id())
            };
 
            assert!(
                fee >= dec!(0.0001) && fee <= dec!(0.02),
                "Fee must be between 0.0001 and 0.02"
            );

            assert!(
                upper_offset >= lower_offset * dec!(1.001) ,
                "Upper offset must be greater than lower offset by a factor of 1.001"
            );

            assert!(
                upper_offset.is_positive() && lower_offset.is_positive(),
                "Both upper and lower offset must be positive"
            );  

            let resource_x: ResourceAddress = LSULP_RESOURCE;
            let resource_y: ResourceAddress = XRD;
            let resource_pool = Blueprint::<TwoResourcePool>::instantiate(
                OwnerRole::Updatable(rule!(require(global_caller(HyperStake::blueprint_id())))),
                rule!(require(global_caller(this))),
                (resource_x, resource_y),
                None,
            );
            let lp_resource: ResourceAddress = resource_pool.get_metadata::<&str, GlobalAddress>("pool_unit").unwrap().unwrap().try_into().unwrap();

            let resource_x_manager = ResourceManager::from(resource_x);
            let resource_y_manager = ResourceManager::from(resource_y);
            let lp_manager = ResourceManager::from_address(lp_resource);
            let symbol_x: String = resource_x_manager.get_metadata("symbol").unwrap_or_default().unwrap_or_default();
            let symbol_y: String = resource_y_manager.get_metadata("symbol").unwrap_or_default().unwrap_or_default();

            resource_pool.set_metadata("name", format!("Hyper Stake {}/{}", symbol_x, symbol_y));
            resource_pool.set_metadata("description", format!("Hyper stake pool for the pair {}/{}, with a fee of {}%, and offsets of {}/{}.", symbol_x, symbol_y, fee * 100, upper_offset, lower_offset));
            resource_pool.set_metadata("swap_component", GlobalAddress::from(this));
            resource_pool.set_metadata("info_url", Url::of("https://www.caviarnine.com/earn/hyper-stake"));

            lp_manager.set_metadata("tags", vec!["dex".to_string(), "LP token".to_string()]);
            lp_manager.set_metadata("symbol", format!("HLP"));
            lp_manager.set_metadata("name", format!("Hyper Stake {}/{}", symbol_x, symbol_y));
            lp_manager.set_metadata("description", format!("Hyper stake liquidity provider token for the pair {}/{}, with a fee of {}%, and offsets of {}/{}.", symbol_x, symbol_y, fee * 100, upper_offset, lower_offset));
            lp_manager.set_metadata("swap_component", GlobalAddress::from(this));
            lp_manager.set_metadata("icon_url", Url::of("https://www.caviarnine.com/images/hyper_stake_lp_icon.png"));
            lp_manager.set_metadata("info_url", Url::of("https://www.caviarnine.com/earn/hyper-stake"));

            resource_pool.set_owner_role(rule!(require(OWNER_RESOURCE)));
            lp_manager.set_owner_role(rule!(require(OWNER_RESOURCE)));

            let lsu_pool: Global<LsuPool> = Global::from(LSU_POOL_COMPONENT);
            let fee_vaults: Global<FeeVaults> = Global::from(FEE_VAULTS_COMPONENT);

            let pool = Self {
                resource_x,
                resource_y,
                lp_resource,
                lsu_pool,
                upper_offset,
                lower_offset,
                fee,
                protocol_fee_share: dec!(0.1),
                treasury_fee_share: dec!(0.1),
                resource_pool,
                fee_vaults,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(OWNER_RESOURCE))))
            .roles(roles!(
                liquidity_user => rule!(allow_all);
                swap_user => rule!(allow_all);
            ))
            .metadata(metadata!(
                init {
                    "resource_x" => resource_x, locked;
                    "resource_y" => resource_y, locked;
                    "fee" => fee, locked;
                    "lp_resource" => lp_resource, locked;
                    "pool_component" => resource_pool.address(), locked;
                    "oracle_component" => LSU_POOL_COMPONENT, locked;
                    "upper_offset" => upper_offset, locked;
                    "lower_offset" => lower_offset, locked;
                }
            ))
            .with_address(component_reservation)
            .globalize();

            Runtime::emit_event(
                NewPoolEvent {
                    swap_component: pool.address(),
                    pool_component: resource_pool.address(),
                    lp_resource,
                    resource_x,
                    resource_y,
                    oracle_component: LSU_POOL_COMPONENT,
                    upper_offset,
                    lower_offset,
                    fee,
                }
            );

            pool
        }

        /// Instantiates a new lsulp tethered pool with tokens
        /// 
        /// # Arguments
        /// 
        /// * `token_x` - The first token
        /// * `token_y` - The second token
        /// * `upper_offset` - The upper offset
        /// * `lower_offset` - The lower offset
        /// * `fee` - The swap fee
        /// * `reservation` - The reservation to use for the component address
        ///
        /// # Returns
        /// 
        /// * `(Global<LsulpTetheredPool>, Bucket)` - The pool global and the LP tokens
        /// 
        /// # Panics
        /// 
        /// * If the fee is not between 0.0001 and 0.02
        /// * If the upper offset is not greater than the lower offset by a factor of 1.001
        /// 
        pub fn new_with_tokens(
            token_x: Bucket,
            token_y: Bucket,
            upper_offset: Decimal,
            lower_offset: Decimal,
            fee: Decimal,
            reservation: Option<GlobalAddressReservation>,
        ) -> (Global<HyperStake>, Bucket) {
            let pool = Self::new(
                upper_offset,
                lower_offset,
                fee,
                reservation,
            );
            let (pool_unit, _) = pool.add_liquidity(token_x, token_y);

            (pool, pool_unit)
        }
            

        /// OWNER
        /// 
        /// Sets the protocol fee share
        /// 
        /// # Arguments
        /// 
        /// * `protocol_fee_share` - The protocol fee share
        /// 
        pub fn set_protocol_fee_share(&mut self, protocol_fee_share: Decimal) {
            assert!(
                protocol_fee_share >= dec!(0) && protocol_fee_share <= dec!(0.1),
                "Protocol fee share must be between 0.0 and 0.1"
            );

            Runtime::emit_event(
                SetFeeShareEvent {
                    old_protocol_fee_share: self.protocol_fee_share,
                    new_protocol_fee_share: protocol_fee_share,
                    old_treasury_fee_share: self.treasury_fee_share,
                    new_treasury_fee_share: self.treasury_fee_share,
                }
            );
            
            self.protocol_fee_share = protocol_fee_share;
        }

        /// OWNER
        /// 
        /// Sets the treasury fee share
        /// 
        /// # Arguments
        /// 
        /// * `treasury_fee_share` - The treasury fee share
        /// 
        pub fn set_treasury_fee_share(&mut self, treasury_fee_share: Decimal) {
            assert!(
                treasury_fee_share >= dec!(0) && treasury_fee_share <= dec!(0.1),
                "Treasury fee share must be between 0.0 and 0.1"
            );

            Runtime::emit_event(
                SetFeeShareEvent {
                    old_protocol_fee_share: self.protocol_fee_share,
                    new_protocol_fee_share: self.protocol_fee_share,
                    old_treasury_fee_share: self.treasury_fee_share,
                    new_treasury_fee_share: treasury_fee_share,
                }
            );

            self.treasury_fee_share = treasury_fee_share;
        }

        /// PUBLIC
        /// 
        /// Gets the oracle price
        /// 
        /// # Returns
        ///
        /// * `Decimal` - The oracle price
        /// 
        pub fn get_oracle_price(&self) -> Decimal {
            let dex_valuation_xrd = self.lsu_pool.get_dex_valuation_xrd();
            let lsulp_supply = ResourceManager::from_address(LSULP_RESOURCE).total_supply().unwrap();

            dex_valuation_xrd / lsulp_supply
        }

        /// PUBLIC
        /// 
        /// Gets the pool info
        /// 
        /// # Returns
        /// 
        /// * `PoolInfo` - The pool info
        /// 
        pub fn get_info(&self) -> PoolInfo {
            let reserves = self.resource_pool.get_vault_amounts();
            let reserve_x = reserves[&self.resource_x];
            let reserve_y = reserves[&self.resource_y];
            let oracle_price = self.get_oracle_price();

            let price = if reserve_x.is_zero() && reserve_y.is_zero() {
                Decimal::ZERO
            } else {
                let upper_limit = (oracle_price * self.upper_offset).checked_sqrt().unwrap();
                let lower_limit = (oracle_price * self.lower_offset).checked_sqrt().unwrap();
                let (virtual_x, virtual_y) = calculate_virtual_amounts(&reserve_x, &reserve_y, &upper_limit, &lower_limit);
                calculate_price(&virtual_x, &virtual_y)
            };

            PoolInfo {
                price,
                resource_x: self.resource_x,
                resource_y: self.resource_y,
                reserve_x,
                reserve_y,
                oracle_price,
                upper_offset: self.upper_offset,
                lower_offset: self.lower_offset,
                fee: self.fee,
                protocol_fee_share: self.protocol_fee_share,
                treasury_fee_share: self.treasury_fee_share,
                pool_component: self.resource_pool.address(),
                lp_resource: self.resource_pool.get_metadata::<&str, GlobalAddress>("pool_unit").unwrap().unwrap().try_into().unwrap(),
            }
        }

        /// PUBLIC
        /// 
        /// Gets the redemption value
        /// 
        /// # Arguments
        /// 
        /// * `amount` - The amount of LP tokens
        /// 
        /// # Returns
        /// 
        /// * `IndexMap<ResourceAddress, Decimal>` - The redemption value
        /// 
        pub fn get_redemption_value(&self, amount: Decimal) -> IndexMap<ResourceAddress, Decimal> {
            self.resource_pool.get_redemption_value(amount)
        }

        /// USER
        /// 
        /// Adds liquidity
        /// 
        /// # Arguments
        /// 
        /// * `token_x` - The first token
        /// * `token_y` - The second token
        /// 
        /// # Returns
        /// 
        /// * `(Bucket, Option<Bucket>)` - The LP tokens and the remainder
        /// 
        pub fn add_liquidity(&mut self, token_x: Bucket, token_y: Bucket) -> (Bucket, Option<Bucket>) {
            let mut amount_x = token_x.amount();
            let mut amount_y = token_y.amount();

            let (token_lp, remainder) = self.resource_pool.contribute((token_x, token_y));

            if let Some(remainder) = &remainder {
                if remainder.resource_address() == self.resource_x {
                    amount_x -= remainder.amount();
                } else {
                    amount_y -= remainder.amount();
                }
            }

            Runtime::emit_event(
                LiquidityChangeEvent {
                    lp_resource: self.lp_resource,
                    resource_x: self.resource_x,
                    resource_y: self.resource_y,
                    amount_lp: token_lp.amount(),
                    amount_x,
                    amount_y,
                }
            );

            (token_lp, remainder)
        }

        /// PUBLIC
        /// 
        /// Removes liquidity
        /// 
        /// # Arguments
        /// 
        /// * `token` - LP tokens
        /// 
        /// # Returns
        /// 
        /// * `(Bucket, Bucket)` - The first token and the second token
        /// 
        pub fn remove_liquidity(&mut self, token_lp: Bucket) -> (Bucket, Bucket) {
            let amount_lp = token_lp.amount();
            let (token_x, token_y) = self.resource_pool.redeem(token_lp);

            Runtime::emit_event(
                LiquidityChangeEvent {
                    lp_resource: self.lp_resource,
                    resource_x: self.resource_x,
                    resource_y: self.resource_y,
                    amount_lp: -amount_lp,
                    amount_x: -token_x.amount(),
                    amount_y: -token_y.amount(),
                }
            );

            (token_x, token_y)
        }

        pub fn swap(&mut self, input_token: Bucket) -> (Bucket, Bucket) {
            if input_token.resource_address() == self.resource_x {
                self._swap_x_for_y(input_token)
            } else if input_token.resource_address() == self.resource_y {
                self._swap_y_for_x(input_token)
            } else {
                panic!("Invalid input token");
            }
        }

        fn _swap_x_for_y(&mut self, mut input_token_x: Bucket) -> (Bucket, Bucket) {
            let reserves = self.resource_pool.get_vault_amounts();
            let reserve_x = reserves[&self.resource_x];
            let reserve_y = reserves[&self.resource_y];

            if reserve_y == Decimal::ZERO {
                return (Bucket::new(self.resource_y), input_token_x);
            }

            let oracle_price = self.get_oracle_price();
            let upper_limit = (oracle_price * self.upper_offset).checked_sqrt().unwrap();
            let lower_limit = (oracle_price * self.lower_offset).checked_sqrt().unwrap();
            let (virtual_x, virtual_y) = calculate_virtual_amounts(&reserve_x, &reserve_y, &upper_limit, &lower_limit);

            let input_amount_x = input_token_x.amount();

            let swap_amount_y = calculate_swap(&(input_amount_x * (Decimal::ONE - self.fee)), &virtual_x, &virtual_y);
            let (used_amount_x, output_amount_y) = if swap_amount_y <= reserve_y {
                (input_amount_x, swap_amount_y)
            } else {
                let swap_amount_x = calculate_swap_inverse(&reserve_y, &virtual_x, &virtual_y);
                let swap_amount_x = if swap_amount_x > Decimal::zero() {
                    swap_amount_x
                } else {
                    Decimal(I192::from_digits([1, 0, 0]))
                };
                let swap_amount_x_with_fee = swap_amount_x / (Decimal::ONE - self.fee);

                (swap_amount_x_with_fee, reserve_y)
            };

            let fee = used_amount_x * self.fee;
            let protocol_fee = fee * self.protocol_fee_share;
            let treasury_fee = fee * self.treasury_fee_share;
            let liquidity_fee = fee - protocol_fee - treasury_fee;

            self.fee_vaults.deposit(input_token_x.take_advanced(protocol_fee, OUTGOING));
            self.fee_vaults.treasury_deposit(input_token_x.take_advanced(treasury_fee, OUTGOING));
            
            self.resource_pool.protected_deposit(input_token_x.take_advanced(used_amount_x - protocol_fee - treasury_fee, INCOMING));
            let output_token_y = self.resource_pool.protected_withdraw(self.resource_y, output_amount_y, OUTGOING);

            let input_amount = input_amount_x - input_token_x.amount();
            let output_amount = output_token_y.amount();

            Runtime::emit_event(
                SwapEvent {
                    input_resource: self.resource_x,
                    output_resource: self.resource_y,
                    input_amount,
                    output_amount,
                    input_reserve: reserve_x,
                    output_reserve: reserve_y,
                    oracle_price,
                    liquidity_fee,
                    protocol_fee,
                    treasury_fee,
                }
            );

            (output_token_y, input_token_x)
        }

        fn _swap_y_for_x(&mut self, mut input_token_y: Bucket) -> (Bucket, Bucket) {
            let reserves = self.resource_pool.get_vault_amounts();
            let reserve_x = reserves[&self.resource_x];
            let reserve_y = reserves[&self.resource_y];

            if reserve_x == Decimal::ZERO {
                return (Bucket::new(self.resource_x), input_token_y);
            }

            let oracle_price = self.get_oracle_price();
            let upper_limit = (oracle_price * self.upper_offset).checked_sqrt().unwrap();
            let lower_limit = (oracle_price * self.lower_offset).checked_sqrt().unwrap();
            let (virtual_x, virtual_y) = calculate_virtual_amounts(&reserve_x, &reserve_y, &upper_limit, &lower_limit);

            let input_amount_y = input_token_y.amount();
            let swap_amount_x = calculate_swap(&(input_amount_y * (Decimal::ONE - self.fee)), &virtual_y, &virtual_x);
            let (used_amount_y, output_amount_x) = if swap_amount_x <= reserve_x {
                (input_amount_y, swap_amount_x)
            } else {
                let swap_amount_y = calculate_swap_inverse(&reserve_x, &virtual_y, &virtual_x);
                let swap_amount_y = if swap_amount_y > Decimal::zero() {
                    swap_amount_y
                } else {
                    Decimal(I192::from_digits([1, 0, 0]))
                };
                let swap_amount_y_with_fee = swap_amount_y / (Decimal::ONE - self.fee);

                (swap_amount_y_with_fee, reserve_x)
            };

            let fee = used_amount_y * self.fee;
            let protocol_fee = fee * self.protocol_fee_share;
            let treasury_fee = fee * self.treasury_fee_share;
            let liquidity_fee = fee - protocol_fee - treasury_fee;

            self.fee_vaults.deposit(input_token_y.take_advanced(protocol_fee, OUTGOING));
            self.fee_vaults.treasury_deposit(input_token_y.take_advanced(treasury_fee, OUTGOING));
            
            self.resource_pool.protected_deposit(input_token_y.take_advanced(used_amount_y - protocol_fee - treasury_fee, INCOMING));
            let output_token_x = self.resource_pool.protected_withdraw(self.resource_x, output_amount_x, OUTGOING);

            let input_amount = input_amount_y - input_token_y.amount();
            let output_amount = output_token_x.amount();

            Runtime::emit_event(
                SwapEvent {
                    input_resource: self.resource_y,
                    output_resource: self.resource_x,
                    input_amount,
                    output_amount,
                    input_reserve: reserve_y,
                    output_reserve: reserve_x,
                    oracle_price,
                    liquidity_fee,
                    protocol_fee,
                    treasury_fee,
                }
            );

            (output_token_x, input_token_y)
        }
    }
}
