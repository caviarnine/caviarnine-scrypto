use scrypto::prelude::*;

/// Soul bound credit receipt that can be redeemed to no pay fees when removing liquidity.
#[derive(ScryptoSbor, NonFungibleData)]
pub struct CreditReceipt {
    /// Hashmap of resources and amounts.
    #[mutable]
    pub resources: HashMap<ResourceAddress, Decimal>,
}
