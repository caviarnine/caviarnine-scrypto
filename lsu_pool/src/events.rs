use scrypto::prelude::*;

/// Event emitted when the valuation of the pool changes.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ValuationChangeEvent {
    /// Net change in xrd.
    pub valuation_change: Decimal,
    /// Total valuation in xrd after the change.
    pub valuation_after_change: Decimal,
    /// Total supply of liquidity provider token.
    pub total_liquidity_token_supply: Decimal,
}

/// Event emitted when liquidity is added to the pool.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct AddLiquidityEvent {
    /// Resource address of the added token.
    pub resource_address: ResourceAddress,
    /// Amount of the added token.
    pub amount: Decimal,
    /// Amount of liquidity provider token minted.
    pub liquidity_token_amount_change: Decimal,
}

/// Event emitted when liquidity is removed from the pool.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct RemoveLiquidityEvent {
    /// Resource address of the removed token.
    pub resource_address: ResourceAddress,
    /// Amount of the removed token.
    pub amount: Decimal,
    /// Amount of liquidity provider token burned.
    pub liquidity_token_amount_change: Decimal,
}

/// Event emitted when a swap is done with the pool.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SwapEvent {
    /// Resource address of the token being put into the pool.
    pub user_sell_resource_address: ResourceAddress,
    /// Amount of the token being put into the pool.
    pub user_sell_amount: Decimal,
    /// Resource address of the token being taken from the pool.
    pub user_buy_resource_address: ResourceAddress,
    /// Amount of the token being taken from the pool.
    pub user_buy_amount: Decimal,
}

/// Event emitted when the token validator is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetTokenValidatorEvent {
    /// Component address of the token validator.
    pub token_validator_component_address: ComponentAddress,
}

/// Event emitted when the protocol fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetProtocolFeeEvent {
    /// New protocol fee.
    pub protocol_fee: Decimal,
}

/// Event emitted when the liquidity fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetLiquidityFeeEvent {
    /// New liquidity fee.
    pub liquidity_fee: Decimal,
}

// Event emitted when the reserve fee is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetReserveFeeEvent {
    /// New reserve fee.
    pub reserve_fee: Decimal,
}
