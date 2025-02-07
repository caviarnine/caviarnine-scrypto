use scrypto::prelude::*;

// Include the generated constants
include!(concat!(env!("OUT_DIR"), "/env_constants.rs"));

pub const INCOMING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToPositiveInfinity);
pub const OUTGOING: WithdrawStrategy = WithdrawStrategy::Rounded(RoundingMode::ToZero);
