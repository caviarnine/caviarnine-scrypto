use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct NewPoolEvent {
    pub swap_component: ComponentAddress,
    pub pool_component: ComponentAddress,
    pub lp_resource: ResourceAddress,
    pub resource_x: ResourceAddress,
    pub resource_y: ResourceAddress,
    pub oracle_component: ComponentAddress,
    pub upper_offset: Decimal,
    pub lower_offset: Decimal,
    pub fee: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct SetFeeShareEvent {
    pub old_protocol_fee_share: Decimal,
    pub new_protocol_fee_share: Decimal,
    pub old_treasury_fee_share: Decimal,
    pub new_treasury_fee_share: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct LiquidityChangeEvent {
    pub lp_resource: ResourceAddress,
    pub resource_x: ResourceAddress,
    pub resource_y: ResourceAddress,
    pub amount_lp: Decimal,
    pub amount_x: Decimal,
    pub amount_y: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct SwapEvent {
    pub input_resource: ResourceAddress,
    pub output_resource: ResourceAddress,
    pub input_amount: Decimal,
    pub output_amount: Decimal,
    pub input_reserve: Decimal,
    pub output_reserve: Decimal,
    pub oracle_price: Decimal,
    pub liquidity_fee: Decimal,
    pub protocol_fee: Decimal,
    pub treasury_fee: Decimal,
}
