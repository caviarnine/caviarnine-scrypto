use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct PoolInfo {
    pub price: Decimal,
    pub resource_x: ResourceAddress,
    pub resource_y: ResourceAddress,
    pub reserve_x: Decimal,
    pub reserve_y: Decimal,
    pub weight_x: Decimal,
    pub weight_y: Decimal,
    pub fee: Decimal,
    pub protocol_fee_share: Decimal,
    pub treasury_fee_share: Decimal,
    pub pool_component: ComponentAddress,
    pub lp_resource: ResourceAddress,
}
