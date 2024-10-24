use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct RefundReceipt {
    /// hashmap of the resources and amounts
    #[mutable]
    pub resources: HashMap<ResourceAddress, Decimal>,
}
