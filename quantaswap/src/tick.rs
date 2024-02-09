use std::cmp::Ordering;
use std::convert::From;
use scrypto::prelude::*;

#[derive(ScryptoSbor, Copy, Clone, Debug)]
pub struct Tick(
    pub u32,
);

impl Tick {
    /// Value offset to make all numbers positive.
    pub const OFFSET: u32 = 27000;

    /// Minimum tick value.
    pub const MIN: Self = Self(0u32);
    /// Maximum tick value.
    pub const MAX: Self = Self(54000u32);
    /// Tick value of decimal 1.
    pub const ONE: Self = Self(Self::OFFSET);

    /// Check if tick is valid.
    pub fn is_valid(&self, bin_span: u32) -> bool {
        self.0 % bin_span == 0 &&
        self >= &Self::MIN &&
        self <= &Self::MAX
    }

    /// Get the next tick above.
    /// 
    /// # Arguments
    /// 
    /// * `bin_span` - The bin span.
    /// 
    /// # Returns
    /// 
    /// * `Tick` - The tick above.
    /// 
    pub fn tick_upper(&self, bin_span: u32) -> Self {
        Self(self.0 + bin_span)
    }

    /// Get the next tick below.
    /// 
    /// # Arguments
    /// 
    /// * `bin_span` - The bin span.
    /// 
    /// # Returns
    /// 
    /// * `Tick` - The tick below.
    /// 
    pub fn tick_lower(&self, bin_span: u32) -> Self {
        Self(self.0 - bin_span)
    }

    /// Round tick up to nearest multiple of bin span.
    /// 
    /// # Arguments
    /// 
    /// * `bin_span` - The bin span.
    /// 
    /// # Returns
    /// 
    /// * `Tick` - The rounded tick.
    /// 
    pub fn round_up(&self, bin_span: u32) -> Self {
        Self((self.0 + bin_span - 1) / bin_span * bin_span)
    }

    /// Round tick down to nearest multiple of bin span.
    /// 
    /// # Arguments
    /// 
    /// * `bin_span` - The bin span.
    /// 
    /// # Returns
    /// 
    /// * `Tick` - The rounded tick.
    /// 
    pub fn round_down(&self, bin_span: u32) -> Self {
        Self(self.0 / bin_span * bin_span)
    }
}

// Implement Ord, PartialOrd, PartialEq, and Eq for Tick
impl Ord for Tick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Tick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Tick {}


// Implement From and Into for Tick and Decimal
impl From<Decimal> for Tick {
    /// Convert a positive decimal into a tick.
    /// 
    /// # Arguments
    /// 
    /// * `value` - Decimal to convert.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new tick.
    /// 
    /// # Requires
    /// 
    /// * `value` - Is in range [0.000000000001892254, 528470197086.935858253558842035].
    /// 
    fn from(value: Decimal) -> Self {
        // Constants
        const BASE_ZEROS: i32 = Decimal::ONE.0.leading_zeros() as i32;
        const ONE: I192 = I192::from_digits([1000000000000000000, 0, 0]); // dec!(1)
        const OFFSET: I192 = I192::from_digits([0xe955ef0800000000, 0x58eb2576b807bc40, 0x4f]); // (Tick::OFFSET + 0.5) * 10^36
        const LOG_2_10005: I192 = I192::from_digits([0x2b821d1ea3ec788d, 0x4b, 0]); // 1 / log_2(10005)
        const LN_10005: I192 = I192::from_digits([0x72839102b6c3cce0, 0x6c, 0]); // 1 / ln(10005)
        const DIVISOR: I192 = I192::from_digits([0xb34b9f1000000000, 0xc097ce7bc90715, 0]); // 10^36

        // Get integer log base 2 of value
        let ilog_2: i32 = BASE_ZEROS - (value.0.leading_zeros() as i32);

        // Shift value near 1
        let remaining: I192 = if ilog_2 >= 0 {
            value.0 >> ilog_2 as u32
        } else {
            value.0 << -ilog_2 as u32
        };

        // Calculate log 1.0005 term from ilog_2
        let log_10005_0: I192 = Decimal::from(ilog_2).0 * LOG_2_10005;

        // Calculate closest tick
        if !remaining.is_zero() {
            // Calculate ln of remaining using taylor series
            const TWO: I192 = I192::from_digits([2, 0, 0]);
            let x: I192 = remaining; 
            let mut ln: I192 = I192::ZERO;
            let mut term: I192 = (x - ONE) * ONE / (x + ONE);
            let mut k: I192 = 1.into();
            let multiplier: I192 = term * term / ONE;
            
            // Calculate terms until term is zero
            term *= TWO;
            while !term.is_zero() {
                ln += term;
                let k2: I192 = k + TWO;
                term = term * multiplier / ONE * k / k2;
                k = k2;
            }

            // Calculate log 1.0005 term from ln
            let log_10005_1 = ln * LN_10005;

            // Combine terms
            let log_10005: I192 = log_10005_0 + log_10005_1;        

            // Round, convert, and return
            Self(((log_10005 + OFFSET) / DIVISOR).to_u32().unwrap())
        } else {
            // Round, convert, and return
            Self(((log_10005_0 + OFFSET) / DIVISOR).to_u32().unwrap())
        }
    }
}

impl From<Tick> for Decimal {
    /// Convert a tick to a decimal.
    /// 
    /// # Arguments
    /// 
    /// * `value` - Tick to convert.
    /// 
    /// # Returns
    /// 
    /// * `Self` - The new decimal.
    /// 
    /// # Requires
    /// 
    /// * `value` - Is a valid tick.
    /// 
    fn from(value: Tick) -> Self {
        // Determine if positive or negative
        let (exp, positive) = if value.0 >= Tick::OFFSET {
            (value.0 - Tick::OFFSET, true)
        } else {
            ((value.0 as i32 - Tick::OFFSET as i32).unsigned_abs(), false)
        };
    
        // Calculate decimal from each bit of the exponent
        let mut result = if exp & 0x1 != 0 { 
            I384::from_digits([0xced916872b020c4a, 0x20c49ba5e353f7, 1, 0, 0, 0]) 
        } else { 
            I384::from_digits([0, 0, 1, 0, 0, 0]) 
        };
        if exp & 0x2 != 0 { result = (result * I384::from_digits([0xffda4052d666aa, 0x418d6909aed56b, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x4 != 0 { result = (result * I384::from_digits([0xeb36e9b6781557c0, 0x832b9b30d17561, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x8 != 0 { result = (result * I384::from_digits([0x7d06ce9cffd53266, 0x1069a6c09e4619c, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x10 != 0 { result = (result * I384::from_digits([0xe4e204399c1a6a84, 0x20e423886132c2f, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x20 != 0 { result = (result * I384::from_digits([0x2ff8b3e6fc5af4e4, 0x420be453d8e80c6, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x40 != 0 { result = (result * I384::from_digits([0x87c1fe56c0f43853, 0x85286acc3c3b34c, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x80 != 0 { result = (result * I384::from_digits([0x3967da909b785eed, 0x10ea505ee14a5cb5, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x100 != 0 { result = (result * I384::from_digits([0x481ac7d634db64ed, 0x22f2c140a56d50eb, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x200 != 0 { result = (result * I384::from_digits([0x1c250bcdc53b7f7, 0x4aaae40866e6f7c4, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x400 != 0 { result = (result * I384::from_digits([0x65fccb6eb437bcb2, 0xab1d05f94f931918, 0x1, 0, 0, 0])) >> 128 }
        if exp & 0x800 != 0 { result = (result * I384::from_digits([0xb79e4d5e8d69c09e, 0xc899d5380a146cf2, 0x2, 0, 0, 0])) >> 128 }
        if exp & 0x1000 != 0 { result = (result * I384::from_digits([0xbe2133785a45385b, 0xbf980e7846a3f77a, 0x7, 0, 0, 0])) >> 128 }
        if exp & 0x2000 != 0 { result = (result * I384::from_digits([0xab8fa709bbaf5821, 0x9b50a7c860576f1, 0x3c, 0, 0, 0])) >> 128 }
        if exp & 0x4000 != 0 { result = (result * I384::from_digits([0x23e2d39925ac624a, 0x8d3b2523687f9e6a, 0xe14, 0, 0, 0])) >> 128 }
    
        // If negative calculate inverse
        if !positive {
            result = I384::from_digits([0, 0, 0, 0, 1, 0]) / result;
        }

        // Divide to go from base 2^128 to base 10^18
        result /= I384::from_digits([0x725dd1d243aba0e7, 0x12, 0, 0, 0, 0]);
    
        // Convert and return
        Decimal(result.try_into().unwrap())
    }
}
