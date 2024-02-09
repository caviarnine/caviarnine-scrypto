use scrypto::prelude::*;

use crate::events::*;
use crate::util::*;

#[blueprint]
#[events(
    SetProtocolFeeDefaultEvent, 
    SetLiquidityFeeDefaultEvent, 
    SetProtocolFeeEvent, 
    SetLiquidityFeeEvent
)]
#[types(
    ResourcesKey,
    PackageAddress,
    u16,
)]
mod fee_controller_mod {
    enable_method_auth! {
        roles {
            fee_manager => updatable_by: [OWNER];
        },
        methods {
            set_protocol_fee_default => restrict_to: [fee_manager];
            set_liquidity_fee_default => restrict_to: [fee_manager];
            set_protocol_fee => restrict_to: [fee_manager];
            set_liquidity_fee => restrict_to: [fee_manager];
            get_protocol_fee_default => PUBLIC;
            get_liquidity_fee_default => PUBLIC;
            get_protocol_fee => PUBLIC;
            get_liquidity_fee => PUBLIC;
            get_fees => PUBLIC;
        }
    }

    // Maximum protocol fee of 1% in basis points hundredths.
    const MAX_PROTOCOL_FEE: u16 = 10000u16;
    // Maximum liquidity fee of 5% in basis points hundredths.
    const MAX_LIQUIDITY_FEE: u16 = 50000u16;

    struct FeeController {
        /// Default protocol fee.
        protocol_fee_default: u16,
        /// Default liquidity fee.
        liquidity_fee_default: u16,
        /// Protocol fee for a package.
        protocol_fees: KeyValueStore<PackageAddress, u16>,
        /// Liquidity fee for a combination of resources.
        liquidity_fees: KeyValueStore<ResourcesKey, u16>,
    }

    impl FeeController {
        /// Instantiate and globalize a new fee controller owned by admin badge.
        ///
        /// # Arguments
        ///
        /// * `admin_badge_address` - Admin badge resource address to set as owner.
        ///
        /// # Returns
        ///
        /// * `Global<FeeController>` - The new fee controller.
        ///
        /// # Access Rules
        ///
        /// * `set_protocol_fee_default` - Fee manager required.
        /// * `set_liquidity_fee_default` - Fee manager required.
        /// * `set_protocol_fee` - Fee manager required.
        /// * `set_liquidity_fee` - Fee manager required.
        /// * `get_protocol_fee_default` - Public.
        /// * `get_liquidity_fee_default` - Public.
        /// * `get_protocol_fee` - Public.
        /// * `get_liquidity_fee` - Public.
        /// * `get_fees` - Public.
        ///
        pub fn new(admin_badge_address: ResourceAddress) -> Global<FeeController> {
            // Instantiate component
            Self::new_local()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge_address))))
                .roles(roles!(
                    fee_manager => rule!(require(admin_badge_address));
                ))
                .globalize()
        }

        /// Instantiate a new fee controller.
        ///
        /// # Returns
        ///
        /// * `Owned<FeeController>` - The new fee controller.
        ///
        pub fn new_local() -> Owned<FeeController> {
            // Instantiate component
            Self {
                protocol_fee_default: 300u16,
                liquidity_fee_default: 3000u16,
                protocol_fees: KeyValueStore::new_with_registered_type(),
                liquidity_fees: KeyValueStore::new_with_registered_type(),
            }
            .instantiate()
        }

        /// FEE MANAGER: Set default protocol fee.
        ///
        /// # Arguments
        ///
        /// * `fee` - Default protocol fee.
        ///
        /// # Panics
        ///
        /// * If `fee` is greater than 1%.
        ///
        /// # Events
        /// 
        /// * `SetProtocolFeeDefaultEvent` - Event emitted when default protocol fee is set.
        /// 
        pub fn set_protocol_fee_default(&mut self, fee: u16) {
            assert!(
                fee <= MAX_PROTOCOL_FEE,
                "Protocol fee must be less than or equal to 1%"
            );

            self.protocol_fee_default = fee;

            // Runtime::emit_event(SetProtocolFeeDefaultEvent { 
            //     fee: fee.into() 
            // });
        }

        /// FEE MANAGER: Set default liquidity fee.
        ///
        /// # Arguments
        ///
        /// * `fee` - Default liquidity fee.
        /// 
        /// # Panics
        ///
        /// * If `fee` is greater than 5%.
        ///
        /// # Events
        /// 
        /// * `SetLiquidityFeeDefaultEvent` - Event emitted when default liquidity fee is set.
        /// 
        pub fn set_liquidity_fee_default(&mut self, fee: u16) {
            assert!(
                fee <= MAX_LIQUIDITY_FEE,
                "Liquidity fee must be less than or equal to 5%"
            );

            self.liquidity_fee_default = fee;

            Runtime::emit_event(SetLiquidityFeeDefaultEvent { 
                fee: fee.into() 
            });
        }

        /// FEE MANAGER: Set protocol fee.
        ///
        /// # Arguments
        ///
        /// * `package_address` - Package address for protocol.
        /// * `fee` - Protocol fee.
        ///
        /// # Panics
        ///
        /// * If `fee` is greater than 1%.
        /// 
        /// # Events
        /// 
        /// * `SetProtocolFeeEvent` - Event emitted when protocol fee is set.
        ///
        pub fn set_protocol_fee(&mut self, package_address: PackageAddress, fee: u16) {
            assert!(
                fee <= MAX_PROTOCOL_FEE,
                "Protocol fee must be less than or equal to 1%"
            );

            self.protocol_fees.insert(package_address, fee);

            Runtime::emit_event(SetProtocolFeeEvent {
                package_address,
                fee: fee.into() 
            });
        }

        /// FEE MANAGER: Set liquidity fee.
        ///
        /// # Arguments
        ///
        /// * `resource_addresses` - Resource addresses for liquidity pool.
        /// * `fee` - Liquidity fee.
        ///
        /// # Panics
        ///
        /// * If `fee` is greater than 5%.
        /// 
        /// # Events
        /// 
        /// * `SetLiquidityFeeEvent` - Event emitted when liquidity fee is set.
        ///
        pub fn set_liquidity_fee(&mut self, resource_addresses: Vec<ResourceAddress>, fee: u16) {
            assert!(
                fee <= MAX_LIQUIDITY_FEE,
                "Liquidity fee must be less than or equal to 5%"
            );

            let key = ResourcesKey::from(resource_addresses.clone());
            self.liquidity_fees.insert(key, fee);

            let mut resource_addresses = resource_addresses;
            resource_addresses.sort();
            resource_addresses.dedup();
            Runtime::emit_event(SetLiquidityFeeEvent {
                resources: resource_addresses,
                fee: fee.into() 
            });
        }

        /// Get protocol fee default.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Protocol fee default as ratio.
        ///
        pub fn get_protocol_fee_default(&self) -> Decimal {
            Decimal::from_basis_point_hundredths(self.protocol_fee_default)
        }

        /// Get liquidity fee default.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Liquidity fee default as ratio.
        ///
        pub fn get_liquidity_fee_default(&self) -> Decimal {
            Decimal::from_basis_point_hundredths(self.liquidity_fee_default)
        }

        /// Get protocol fee.
        ///
        /// # Arguments
        ///
        /// * `package_address` - Package address for protocol.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Protocol fee as ratio.
        ///
        pub fn get_protocol_fee(&self, package_address: PackageAddress) -> Decimal {
            match self.protocol_fees.get(&package_address) {
                Some(fee) => Decimal::from_basis_point_hundredths(*fee),
                None => Decimal::from_basis_point_hundredths(self.protocol_fee_default),
            }
        }

        /// Get liquidity fee.
        ///
        /// # Arguments
        ///
        /// * `resource_addresses` - Resource addresses for liquidity pool.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Liquidity fee as ratio.
        ///
        pub fn get_liquidity_fee(&self, resource_addresses: Vec<ResourceAddress>) -> Decimal {
            let key = ResourcesKey::from(resource_addresses);
            match self.liquidity_fees.get(&key) {
                Some(fee) => Decimal::from_basis_point_hundredths(*fee),
                None => Decimal::from_basis_point_hundredths(self.liquidity_fee_default),
            }
        }

        /// Get fees.
        ///
        /// # Arguments
        ///
        /// * `package_address` - Package address for protocol.
        /// * `resource_addresses` - Resource addresses for liquidity pool.
        ///
        /// # Returns
        ///
        /// * `Decimal` - Protocol fee as ratio.
        /// * `Decimal` - Liquidity fee as ratio.
        ///
        pub fn get_fees(
            &self,
            package_address: PackageAddress,
            resource_addresses: Vec<ResourceAddress>,
        ) -> (Decimal, Decimal) {
            (
                self.get_protocol_fee(package_address),
                self.get_liquidity_fee(resource_addresses),
            )
        }
    }
}
