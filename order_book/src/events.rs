use scrypto::prelude::*;

/// Event emitted when a new order book is created.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct NewOrderBookEvent {
    /// The address of the new order book.
    pub component_address: ComponentAddress,
    /// The address of order receipts for the new order book.
    pub order_receipt_address: ResourceAddress,
    /// The address of the token x for the new order book.
    pub token_x_address: ResourceAddress,
    /// The address of the token x for the new order book.
    pub token_y_address: ResourceAddress,
}

/// Event emitted when a limit order is placed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct LimitOrderEvent {
    /// The id of the created order receipt.
    pub order_id: NonFungibleLocalId,
    /// Whether the limit order is on the ask or bid side of the order book.
    pub is_ask: bool, 
    /// The price of the limit order calculated as `tokens_y / token_x`.
    pub price: Decimal,
    /// The amount the limit order is for valued in tokens x.
    pub amount: Decimal,
}

/// Event emitted when a market order is placed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct MarketOrderEvent {
    /// Whether the market order is buying into the ask or selling into bid side of the order book.
    pub is_buy: bool, 
    /// Vector of fills for the market order. Each fill is a tuple of `(price, amount)`, where `amount` is valued in tokens x.
    pub fills: Vec<(Decimal, Decimal)>,
}

/// Event emitted when an order is claimed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ClaimOrderEvent {
    /// The id of the claimed order receipt.
    pub order_id: NonFungibleLocalId,
    /// Whether the claimed limit order was on the ask or bid side of the order book.
    pub is_ask: bool,
    /// The price of the claimed limit order calculated as `tokens_y/token_x`.
    pub price: Decimal,
    /// The amount the claimed limit order was for valued in tokens x.
    pub amount_canceled: Decimal,
    /// The amount the claimed limit order was filled valued in tokens x.
    pub amount_filled: Decimal,
    /// The amount of tokens x received for the claimed limit order.
    pub amount_x: Decimal,
    /// The amount of tokens y received for the claimed limit order.
    pub amount_y: Decimal,
}

/// Event emitted when protocol fee is collected.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ProtocolFeeEvent {
    /// Fee token address.
    pub token_address: ResourceAddress,
    /// Fee amount.
    pub amount: Decimal,
}