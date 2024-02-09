use scrypto::prelude::*;

/// Event emitted when the default protocol fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetProtocolFeeDefaultEvent {
    pub fee: Decimal,
}

/// Event emitted when the default liquidity fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetLiquidityFeeDefaultEvent {
    pub fee: Decimal,
}

/// Event emitted when a protocol fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetProtocolFeeEvent {
    pub package_address: PackageAddress,
    pub fee: Decimal,
}

/// Event emitted when a liquidity fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetLiquidityFeeEvent {
    pub resources: Vec<ResourceAddress>,
    pub fee: Decimal,
}
