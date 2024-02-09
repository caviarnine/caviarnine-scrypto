use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetTreasuryPercentageEvent {
    pub treasury_percentage: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetBurnPercentageEvent {
    pub burn_percentage: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetSwapAmountEvent {
    pub swap_amount: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetMaxEpochsEvent {
    pub max_epochs: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct TreasuryWithdrawEvent {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ReserveWithdrawEvent {
    pub amount: Decimal,
    pub new_balance: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct TreasuryDepositEvent {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
    pub new_balance: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SwapVaultDepositEvent {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
    pub new_balance: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ReserveDepositEvent {
    pub amount: Decimal,
    pub new_balance: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BurnEvent {
    pub amount: Decimal,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SwapEvent {
    pub resource_address: ResourceAddress,
    pub swap_price: Decimal,
    pub amount: Decimal,
}
