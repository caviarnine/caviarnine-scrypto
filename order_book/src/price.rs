use std::cmp::Ordering;
use std::convert::From;
use scrypto::prelude::*;

/// Represents a compressed decimal price in the range of [0.00000000001, 100000000000].
/// Uses the format value * 10 ^ (exp - (18 - PRECISION + 1)).
/// PRECISION is set to 5, meaning 5 significant figures of precision.
#[derive(ScryptoSbor, Copy, Clone, Debug)]
pub struct Price(
    // Data is 7 bits Exp, SIG_BITS bits Significand.
    pub u32,
);

impl Price {
    /// Number of significant figures to which prices are truncated.
    pub const PRECISION: u32 = 5;

    /// Minimum value for significand if exp is greater than 0.
    pub const SIG_MIN: u32 = 10u32.pow(Self::PRECISION - 1);
    /// Maximum value for significand if exp is greater than 0.
    pub const SIG_MAX: u32 = 10u32.pow(Self::PRECISION) - 1;
    /// Number of bits in significand to place exp.
    pub const SIG_BITS: u32 = Self::SIG_MAX.ilog2() +1;
    /// Mask to get significand.
    pub const SIG_MASK: u32 = 0xFFFFFFFF >> (32 - Self::SIG_BITS);
    /// Maximum value for exp.
    pub const EXP_MAX: u32 = 25;
    /// Number of bits in exp.
    pub const EXP_BITS: u32 = Self::EXP_MAX.ilog2() +1;

    /// Price equal to Price::from(dec!("0.00000000001")).
    pub const MIN: Self = Self(403216u32);
    /// dec!("0.00000000001")
    pub const DECIMAL_MIN: Decimal = Decimal(I192::from_digits([10000000, 0, 0]));

    /// Price equal to Price::from(dec!("100000000000")).
    pub const MAX: Self = Self(3286800u32);
    /// dec!("100000000000")
    pub const DECIMAL_MAX: Decimal = Decimal(I192::from_digits([7886392056514347008, 5421010862, 0]));

    /// Create a new price.
    /// 
    /// # Arguments
    /// 
    /// * `exp` - Exponent.
    /// * `significand` - Significand.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new price.
    /// 
    /// # Requires
    /// 
    /// * `exp` and `significand` are in bounds.
    /// 
    pub fn new(exp: u32, significand: u32) -> Self {
        // Create price
        Self(exp << Self::SIG_BITS | significand)
    }

    /// Get exponent.
    /// 
    /// # Returns
    /// 
    /// * `u32` - Exponent.
    /// 
    pub fn get_exp(&self) -> u32 {
        self.0 >> Self::SIG_BITS
    }

    /// Get significand.
    /// 
    /// # Returns
    /// 
    /// * `u32` - Significand.
    /// 
    pub fn get_significand(&self) -> u32 {
        self.0 & Self::SIG_MASK
    }
}

// Implement Ord, PartialOrd, PartialEq, and Eq for Price
impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Price {}

impl From<Decimal> for Price {
    /// Convert a positive decimal to a price. Truncates decimal to PRECISION.
    /// 
    /// # Arguments
    /// 
    /// * `value` - Decimal to convert.
    /// 
    /// # Returns
    /// 
    /// * `Price` - The new price.
    /// 
    /// # Panics
    /// 
    /// * If value is not in range of [0.00000000001, 100000000000].
    /// 
    fn from(value: Decimal) -> Self {
        // Asserts that value is in bounds
        assert!(value.is_valid_price(), "{} is not in valid price range of {} to {}.", value, Self::DECIMAL_MIN, Self::DECIMAL_MAX);

        // Calculate approximate log10 of value
        let pos_2: u32 = I192::BITS - value.0.leading_zeros();
        let pos_10: u32 = pos_2 * 30103 / 100000;

        // Reduce value to PRECISION
        let (mut reduced, mut exp): (u32, u32) = if pos_10 > Self::PRECISION {
            let exp: u32 = pos_10 - Self::PRECISION;
            let divisor: I192 = I192::from(10u8).pow(exp);
            ((value.0 / divisor).to_u32().unwrap(), exp)
        } else {
            (value.0.to_u32().unwrap(), 0)
        };

        // Adjust value to fit if log was not exact
        if reduced > Self::SIG_MAX {
            reduced /= 10;
            exp += 1;
        }

        // Create price
        Self::new(exp, reduced)
    }
}

impl From<Price> for Decimal {
    /// Convert a price to a decimal.
    /// 
    /// # Arguments
    /// 
    /// * `value` - Price to convert.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new decimal.
    /// 
    /// # Requires
    /// 
    /// * `value` is a valid price.
    /// 
    fn from(value: Price) -> Self {
        let multiplier: I192 = I192::from(10u8).pow(value.get_exp());
        Decimal(I192::from(value.get_significand()) * multiplier)
    }
}

pub trait PriceRange {
    fn is_valid_price(&self) -> bool;
    fn round_to_price_range(&self) -> Self;
}

impl PriceRange for Decimal {
    fn is_valid_price(&self) -> bool {
        *self >= Price::DECIMAL_MIN && *self <= Price::DECIMAL_MAX
    }

    fn round_to_price_range(&self) -> Self {
        if *self < Price::DECIMAL_MIN {
            Price::DECIMAL_MIN
        } else if *self > Price::DECIMAL_MAX {
            Price::DECIMAL_MAX
        } else {
            *self
        }
    }
}
