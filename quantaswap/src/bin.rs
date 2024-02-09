use scrypto::prelude::*;

/// Bin that stores a dormant liquidity position.
#[derive(ScryptoSbor, Clone, Copy)]
pub struct Bin {
    /// Amount of tokens in the bin. May be tokens x or y depending on if the bin is above or below the active bin.
    pub amount: Decimal,
    /// Total claim amount for the bin. This is the sum of all liquidity claims for the bin.
    pub total_claim: Decimal,
}
