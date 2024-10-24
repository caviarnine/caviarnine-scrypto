use scrypto::prelude::*;

/// Event emitted when the owner rule default is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetOwnerRuleDefaultEvent {
    /// The new owner rule default.
    pub owner_rule_default: AccessRule,
}

/// Event emitted when the user rule default is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetUserRuleDefaultEvent {
    /// The new user rule default.
    pub user_rule_default: AccessRule,
}

/// Event emitted when the token validator is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetTokenValidatorEvent {
    /// The new token validator.
    pub token_validator_address: ComponentAddress,
}

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