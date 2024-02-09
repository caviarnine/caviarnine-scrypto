use scrypto::prelude::*;

/// Liquidity receipt NFT that stores liquidity claims.
/// Each claim is a pair of the tick and the claim amount on the bin which the tick maps to.
/// Liquidity claims can be added, removed, and adjusted without need for another liquidity 
/// receipt so long as the total number of claims does not exceed the maximum allowed number.
/// This maximum is enforced to prevent unbounded gas costs.
#[derive(ScryptoSbor, NonFungibleData)]
pub struct LiquidityReceipt {
    /// Map of ticks to liquidity claim amounts.
    #[mutable] pub liquidity_claims: HashMap<u32, Decimal>,
}