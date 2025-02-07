use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct NewPoolEvent {
    pub swap_component: ComponentAddress,
    pub pool_component: ComponentAddress,
    pub lp_resource: ResourceAddress,
    pub resource_x: ResourceAddress,
    pub resource_y: ResourceAddress,
    pub weight_x: Decimal,
    pub weight_y: Decimal,
    pub fee: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent, Debug)]
pub struct SwapEvent {
    pub input_resource: ResourceAddress,
    pub output_resource: ResourceAddress,
    pub input_amount: Decimal,
    pub output_amount: Decimal,
    pub input_reserve: Decimal,
    pub output_reserve: Decimal,
    pub liquidity_fee: Decimal,
    pub protocol_fee: Decimal,
    pub treasury_fee: Decimal,
}
