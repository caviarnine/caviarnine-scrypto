use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct UpdateWhiteListEvent {
    pub resource_address: ResourceAddress,
    pub contain: bool,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct UpdateBlackListEvent {
    pub resource_address: ResourceAddress,
    pub contain: bool,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetRestrictRecallableEvent {
    pub restrict: bool,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetRestrictFreezableEvent {
    pub restrict: bool,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetMinimumDivisibilityEvent {
    pub minimum_divisibility: u8,
}