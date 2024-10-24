use scrypto::prelude::*;

/// Event emitted when an lsu is added or removed from the active set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct UpdateActiveSetEvent {
    /// Resource address of lsu to updated.
    pub resource_address: ResourceAddress,
    /// If the lsu is in the active set or not.
    pub contain: bool,
}

/// Event emitted when require active is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetRequireActiveEvent {
    /// If being in active set is required for token validation.
    pub require_active: bool,
}

/// Event emitted when require lsu is set.
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct SetRequireLsuEvent {
    /// If being an lsu is required for token validation.
    pub require_lsu: bool,
}