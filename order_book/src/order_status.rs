use scrypto::prelude::*;

/// Order data.
#[derive(ScryptoSbor, Clone, Copy)]
pub struct OrderData {
    /// Is ask or bid order.
    pub is_ask: bool,
    /// Price of order.
    pub price: Decimal,
    /// Filled amount of tokens for order calculated in tokens x.
    pub amount_filled: Decimal,
    /// Total amount of tokens for order calculated in tokens x.
    pub amount_total: Decimal,
}

/// Order status.
///
/// * `Open(OrderData)` - Order is open and has the contained order data.
/// * `Filled(OrderData)` - Order has been filled and has the contained order data.
/// * `Claimed` - Order has been claimed and no longer exists.
/// * `Invalid` - Order id is invalid.
#[derive(ScryptoSbor)]
pub enum OrderStatus {
    Open(OrderData),
    Filled(OrderData),
    Claimed,
    Invalid,
}
