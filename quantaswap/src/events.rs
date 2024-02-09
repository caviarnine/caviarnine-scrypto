use scrypto::prelude::*;

/// Event emitted when a new pool is created.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct NewPoolEvent {
    /// The address of the new pool.
    pub component_address: ComponentAddress,
    /// The address of liquidity receipts for the new pool.
    pub liquidity_receipt_address: ResourceAddress,
    /// The address of the token x for the new pool.
    pub token_x_address: ResourceAddress,
    /// The address of the token x for the new pool.
    pub token_y_address: ResourceAddress,
    /// The bin span for the new pool.
    pub bin_span: u32,
}

/// Event emitted when a liquidity receipt is minted.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct MintLiquidityReceiptEvent {
    /// The id of the minted liquidity receipt.
    pub liquidity_receipt_id: NonFungibleLocalId,
}

/// Event emitted when a liquidity receipt is burned.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct BurnLiquidityReceiptEvent {
    /// The id of the burned liquidity receipt.
    pub liquidity_receipt_id: NonFungibleLocalId,
}

/// Event emitted when liquidity is added.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct AddLiquidityEvent {
    /// The id of the liquidity receipt to which liquidity was added.
    pub liquidity_receipt_id: NonFungibleLocalId,
    /// The change in tokens x for the pool.
    pub amount_change_x: Decimal,
    /// The change in tokens y for the pool.
    pub amount_change_y: Decimal,
    /// The positions tokens x were added to.
    pub added_x: Vec<(u32, Decimal)>,
    /// The positions tokens y were added to.
    pub added_y: Vec<(u32, Decimal)>,
}

/// Event emitted when liquidity is removed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct RemoveLiquidityEvent {
    /// The id of the liquidity receipt from which liquidity was removed.
    pub liquidity_receipt_id: NonFungibleLocalId,
    /// The change in tokens x for the pool.
    pub amount_change_x: Decimal,
    /// The change in tokens y for the pool.
    pub amount_change_y: Decimal,
    /// The positions tokens x were removed from.
    pub removed_x: Vec<(u32, Decimal)>,
    /// The positions tokens y were removed from.
    pub removed_y: Vec<(u32, Decimal)>,
}

/// Event emitted when a swap is performed.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SwapEvent {
    /// The change in tokens x for the pool.
    pub amount_change_x: Decimal,
    /// The change in tokens y for the pool.
    pub amount_change_y: Decimal,
    /// The price after the swap.
    pub price_after: Decimal,
}

/// Event emitted when the value of a pool changes.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ValuationEvent {
    /// Total tokens x in the pool after the change.
    pub amount_after_x: Decimal,
    /// Total tokens y in the pool after the change.
    pub amount_after_y: Decimal,
    /// The price after the change.
    pub price_after: Decimal,
}

/// Event emitted when protocol fee is collected.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ProtocolFeeEvent {
    /// Fee token address.
    pub token_address: ResourceAddress,
    /// Fee amount.
    pub amount: Decimal,
}

/// Event emitted when liquidity fee is collected.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct LiquidityFeeEvent {
    /// Fee token address.
    pub token_address: ResourceAddress,
    /// Fee amount.
    pub amount: Decimal,
}
