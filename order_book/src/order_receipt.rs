use scrypto::prelude::*;

/// Order receipt NFT that represents a outstanding limit order.
/// Tokens are owned by the order receipt and can be claimed using the order receipt.
#[derive(ScryptoSbor, NonFungibleData)]
pub struct OrderReceipt {
    /// Is ask or bid order.
    pub is_ask: bool,
    /// Price of order.
    pub price: Decimal,
    /// Amount of tokens in order calculated in tokens x.
    pub amount: Decimal,
    /// Id of next order in FIFO ordering linked list.
    #[mutable]
    pub next_id: u64,
    /// Id of previous order in FIFO ordering linked list.
    #[mutable]
    pub prev_id: u64,
}
