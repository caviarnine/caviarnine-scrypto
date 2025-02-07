pub mod consts;
pub mod events;
pub mod structs;
pub mod swap_math;
pub mod util_math;

use scrypto::prelude::*;
pub use consts::*;
pub use events::*;
pub use structs::*;
pub use swap_math::*;
pub use util_math::*;

#[blueprint]
#[events(
    NewPoolEvent,
    SwapEvent,
)]
mod weighted_pool {
    const OWNER_RESOURCE: ResourceAddress = _OWNER_RESOURCE;
    const FEE_VAULTS_COMPONENT: ComponentAddress = _FEE_VAULTS_COMPONENT;
    
    extern_blueprint! {
        FEE_VAULTS_PACKAGE,
        FeeVaults {
            fn deposit(&self, tokens: Bucket);
            fn treasury_deposit(&self, tokens: Bucket);
        }
    }

    enable_method_auth! {
        roles {
            user => updatable_by: [OWNER];
        },
        methods {
            set_protocol_fee_share => restrict_to: [OWNER];
            set_treasury_fee_share => restrict_to: [OWNER];

            get_info => PUBLIC;
            get_redemption_value => PUBLIC;

            add_liquidity => restrict_to: [user];
            remove_liquidity => PUBLIC;
            swap => restrict_to: [user];
        }
    }

    struct WeightedPool {
        weights: IndexMap<ResourceAddress, Decimal>,
        fee: Decimal,
        protocol_fee_share: Decimal,
        treasury_fee_share: Decimal,
        resource_pool: Global<TwoResourcePool>,
        fee_vaults: Global<FeeVaults>,
    }

    impl WeightedPool {
        /// Instantiates a new WeightedPool
        /// 
        /// # Arguments
        /// 
        /// * `resource_x` - The address of the first resource
        /// * `resource_y` - The address of the second resource
        /// * `weight_x` - The weight of the first resource
        /// * `fee` - The fee
        /// * `reservation` - The reservation to use for the component address
        /// 
        /// # Returns
        /// 
        /// * `Global<WeightedPool>` - The pool global
        /// 
        /// # Panics
        /// 
        /// * If the weights are not between 0.1 and 0.9
        /// * If the fee is not between 0.0001 and 0.01
        /// 
        pub fn new(
            resource_x: ResourceAddress,
            resource_y: ResourceAddress,
            weight_x: Decimal,
            fee: Decimal,
            reservation: Option<GlobalAddressReservation>,
        ) -> Global<WeightedPool> {
            let owner_role = OwnerRole::Updatable(rule!(require(OWNER_RESOURCE)));

            let (component_reservation, this) = match reservation {
                Some(reservation) => {
                    let this = Runtime::get_reservation_address(&reservation).try_into().unwrap();
                    (reservation, this)
                },
                None => Runtime::allocate_component_address(WeightedPool::blueprint_id())
            };

            assert!(
                weight_x >= dec!(0.1) && weight_x <= dec!(0.9),
                "Weights must be between 0.1 and 0.9"
            );
            let weight_y = Decimal::ONE - weight_x;
            let weights = indexmap![
                resource_x => weight_x,
                resource_y => weight_y,
            ];
            assert!(
                fee >= dec!(0.0001) && fee <= dec!(0.02),
                "Fee must be between 0.0001 and 0.02"
            );

            let resource_pool = Blueprint::<TwoResourcePool>::instantiate(
                owner_role.clone(),
                rule!(require(global_caller(this))),
                (resource_x, resource_y),
                None,
            );
            let lp_resource: ResourceAddress = resource_pool.get_metadata::<&str, GlobalAddress>("pool_unit").unwrap().unwrap().try_into().unwrap();
            let fee_vaults: Global<FeeVaults> = Global::from(FEE_VAULTS_COMPONENT);

            let pool = Self {
                weights,
                fee,
                protocol_fee_share: dec!(0.1),
                treasury_fee_share: dec!(0.1),
                resource_pool,
                fee_vaults,
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .roles(roles!(
                user => rule!(allow_all);
            ))
            .metadata(metadata!(
                init {
                    "resource_x" => resource_x, locked;
                    "resource_y" => resource_y, locked;
                    "weight_x" => weight_x, locked;
                    "weight_y" => weight_y, locked;
                    "fee" => fee, locked;
                    "lp_resource" => lp_resource, locked;
                    "pool_component" => resource_pool.address(), locked;
                }
            ))
            .with_address(component_reservation)
            .globalize();

            Runtime::emit_event(
                NewPoolEvent {
                    resource_x,
                    resource_y,
                    weight_x,
                    weight_y,
                    fee,
                    swap_component: pool.address(),
                    pool_component: resource_pool.address(),
                    lp_resource,
                }
            );

            pool
        }

        /// Instantiates a new WeightedPool with tokens
        /// 
        /// # Arguments
        /// 
        /// * `token_x` - The first token
        /// * `token_y` - The second token
        /// * `weight_x` - The weight of the first token
        /// * `fee` - The fee
        /// * `reservation` - The reservation to use for the component address
        /// 
        /// # Returns
        /// 
        /// * `(Global<WeightedPool>, Bucket)` - The pool global and the LP tokens
        /// 
        /// # Panics
        /// 
        /// * If the weights are not between 0.1 and 0.9
        /// * If the fee is not between 0.0001 and 0.01
        /// 
        pub fn new_with_tokens(
            token_x: Bucket,
            token_y: Bucket,
            weight_x: Decimal,
            fee: Decimal,
            reservation: Option<GlobalAddressReservation>,
        ) -> (Global<WeightedPool>, Bucket) {
            let pool = Self::new(
                token_x.resource_address(),
                token_y.resource_address(),
                weight_x,
                fee,
                reservation
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
            self.treasury_fee_share = treasury_fee_share;
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
            let resources: IndexMap<ResourceAddress, (Decimal, Decimal)> = self.weights.iter().map(|(resource, weight)| (*resource, (reserves[resource], *weight))).collect();
            let (&resource_x, &(reserve_x, weight_x)) = resources.get_index(0).unwrap();
            let (&resource_y, &(reserve_y, weight_y)) = resources.get_index(1).unwrap();

            let price = if reserve_x.is_zero() || reserve_y.is_zero() {
                Decimal::ZERO
            } else {
                (reserve_y / weight_y) / (reserve_x / weight_x)
            };

            PoolInfo {
                price,
                resource_x,
                resource_y,
                reserve_x,
                reserve_y,
                weight_x,
                weight_y,
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
        /// * `(Bucket, Option<Bucket>)` - The LP tokens and the change
        /// 
        pub fn add_liquidity(&mut self, token_x: Bucket, token_y: Bucket) -> (Bucket, Option<Bucket>) {
            self.resource_pool.contribute((token_x, token_y))
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
        pub fn remove_liquidity(&mut self, token: Bucket) -> (Bucket, Bucket) {
            self.resource_pool.redeem(token)
        }

        /// USER
        /// 
        /// Swaps input token for the other token in the pool
        /// 
        /// # Arguments
        /// 
        /// * `input_token` - The input token
        /// 
        /// # Returns
        /// 
        /// * `Bucket` - The output token
        /// 
        /// # Panics
        /// 
        /// * If unequal weights and the input amount is greater than 60% of the tokens in the pool
        /// 
        pub fn swap(&mut self, mut input_token: Bucket) -> Bucket {
            let mut reserves = self.resource_pool.get_vault_amounts();
            
            let input_resource = input_token.resource_address();
            let input_reserve = reserves.shift_remove(&input_resource).expect("Invalid input token");
            let input_weight = self.weights[&input_resource];
            
            let (&output_resource, &output_reserve) = reserves.first().unwrap();
            let output_weight = self.weights[&output_resource];
            
            let input_amount = input_token.amount();
            let fee = input_amount * self.fee;
            let protocol_fee = fee * self.protocol_fee_share;
            let treasury_fee = fee * self.treasury_fee_share;
            let liquidity_fee = fee - protocol_fee - treasury_fee;

            let output_amount = calculate_swap(
                input_reserve,
                output_reserve,
                input_weight,
                output_weight,
                input_amount - fee
            );

            self.fee_vaults.deposit(input_token.take_advanced(protocol_fee, OUTGOING));
            self.fee_vaults.treasury_deposit(input_token.take_advanced(treasury_fee, OUTGOING));
            
            self.resource_pool.protected_deposit(input_token);
            let output_token = self.resource_pool.protected_withdraw(output_resource, output_amount, OUTGOING);
            let output_amount = output_token.amount();

            Runtime::emit_event(
                SwapEvent {
                    input_resource,
                    output_resource,
                    input_amount,
                    output_amount,
                    input_reserve,
                    output_reserve,
                    liquidity_fee,
                    protocol_fee,
                    treasury_fee,
                }
            );

            output_token
        }
    }
}
