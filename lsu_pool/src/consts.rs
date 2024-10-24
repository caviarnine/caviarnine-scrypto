use scrypto::prelude::*;

// Max protocol fee is 100 bp
pub const PROTOCOL_FEE_MAX: Decimal = Decimal(I192::from_digits([10000000000000000, 0, 0]));

// Max liquidity fee is 100 bp
pub const LIQUIDITY_FEE_MAX: Decimal = Decimal(I192::from_digits([10000000000000000, 0, 0]));

// Max reserve fee is 100 bp
pub const RESERVE_FEE_MAX: Decimal = Decimal(I192::from_digits([10000000000000000, 0, 0]));

// Number of prices to update 
pub const NUMBER_VALIDATOR_PRICES_TO_UPDATE: u32 = 5;
