use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PoolInfo {
    pub price: Decimal,
    pub resource_x: ResourceAddress,
    pub resource_y: ResourceAddress,
    pub reserve_x: Decimal,
    pub reserve_y: Decimal,
    pub oracle_price: Decimal,
    pub upper_offset: Decimal,
    pub lower_offset: Decimal,
    pub fee: Decimal,
    pub protocol_fee_share: Decimal,
    pub treasury_fee_share: Decimal,
    pub pool_component: ComponentAddress,
    pub lp_resource: ResourceAddress,
}
