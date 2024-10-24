use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BridgeEvent {
    pub amount: Decimal,
}