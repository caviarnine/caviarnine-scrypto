use scrypto::prelude::*;

use order_book::price::*;
use order_book::price_index::*;

#[test]
fn test_amount_extremes() {
    let max_price_dec: Decimal = Price::DECIMAL_MAX;
    let min_price_dec: Decimal = Price::DECIMAL_MIN;
    
    let amount = Decimal(I192::from(2).pow(152));
    let max_amount = amount.checked_mul(max_price_dec);
    let min_amount = amount.checked_div(min_price_dec);

    assert!(max_amount.is_some());
    assert!(min_amount.is_some());
}

#[test]
fn test_bits() {
    let price = Price::MAX;
    let bits = u32::BITS - price.0.leading_zeros();
    let bits_rounded = (bits + PriceIndex::FACTOR_SHIFT - 1) / PriceIndex::FACTOR_SHIFT * PriceIndex::FACTOR_SHIFT;
    assert_eq!(bits_rounded, PriceIndex::PRICE_BITS_USED)
}

#[test]
fn test_price_ord() {
    let price1 = Price::new(3, 30000);
    let price2 = Price::new(3, 50000);
    let price3 = Price::new(3, 25000);
    let price4 = Price::new(4, 30000);
    let price5 = Price::new(2, 30000);

    assert!(price1 < price2);
    assert!(price2 > price3);
    assert!(price1 > price3);
    assert!(price1 < price4);
    assert!(price1 > price5);
}

#[test]
fn test_price_from_decimal() {
    let decimal = dec!("1.23456");
    let price: Price = decimal.into();

    assert_eq!(price.get_exp(), 14);
    assert_eq!(price.get_significand(), 12345);
}

#[test]
fn test_price_to_decimal() {
    let price = Price::new(1, 12345);

    let decimal: Decimal = price.into();
    assert_eq!(decimal, dec!("0.00000000000012345"));
}

#[test]
fn test_price_round_trip() {
    let original_decimal = dec!("1.23456");
    let price: Price = original_decimal.into();
    let new_decimal: Decimal = price.into();

    assert_eq!(new_decimal, dec!("1.23450"));
}

#[test]
fn test_price_to_decimal_round_trip() {
    let price1 = Price::new(3, 99999);
    let price2 = Price::new(4, 10000);

    let price: Decimal = price1.into();
    let price_struct: Price = price1;
    assert_eq!(Decimal::from(Price::from(price)), price);
    assert_eq!(Price::from(price), price_struct);

    let price: Decimal = price2.into();
    let price_struct: Price = price2;
    assert_eq!(Decimal::from(Price::from(price)), price);
    assert_eq!(Price::from(price), price_struct);
}

#[test]
fn test_decimal_to_price() {
    let mut adder: Decimal = dec!("0.000000000000001");
    let mut price: Decimal = Price::DECIMAL_MIN;
    let mut count: u32 = 0;
    let last_price_struct: Price = price.into();

    while price <= Price::DECIMAL_MAX {
        let price_struct: Price = price.into();
        assert!(price_struct.get_significand() >= Price::SIG_MIN);
        assert!(price_struct.get_significand() <= Price::SIG_MAX);

        let reversed: Decimal = price_struct.into();
        assert_eq!(reversed, price);
        assert!(price_struct >= last_price_struct);

        if count > 0 && count % 90000 == 0 {
            adder = adder * dec!(10);
        }
        price = price + adder;
        count += 1;
    }
}

#[test]
fn test_decimal_to_price_rounding() {
    let mut adder0: Decimal = dec!("0.000000000000001");
    let mut adder1: Decimal = dec!(0);
    let mut price_rounded: Decimal = Price::DECIMAL_MIN;
    let mut price: Decimal = Price::DECIMAL_MIN;
    let mut count: u32 = 0;

    while price <= Price::DECIMAL_MAX {
        let price_struct: Price = price.into();

        let reversed: Decimal = price_struct.into();
        assert_eq!(reversed, price_rounded);

        if count > 0 && count % 90000 == 0 {
            adder1 = adder0;
            adder0 = adder0 * dec!(10);
        }
        price_rounded = price_rounded + adder0;
        price = price_rounded + adder1;
        count += 1;
    }
}

#[test]
fn test_precision() {
    assert_eq!(Price::from(dec!(1)).get_significand(), 10u32.pow(Price::PRECISION -1));
}

#[test]
fn test_min() {
    assert_eq!(Price::DECIMAL_MIN, dec!("0.00000000001"));
    assert_eq!(Price::MIN, Price::from(Price::DECIMAL_MIN));
}

#[test]
fn test_max() {
    assert_eq!(Price::DECIMAL_MAX, dec!("100000000000"));
    assert_eq!(Price::MAX, Price::from(Price::DECIMAL_MAX));
    assert_eq!(Price::EXP_MAX, Price::MAX.get_exp());
}

#[test]
fn round_to_price_range_no_round() {
    let price = dec!("1.23456");
    let rounded = price.round_to_price_range();
    assert_eq!(price, rounded);
}

#[test]
fn round_to_price_range_round_up() {
    let price = Decimal::MIN;
    let rounded = price.round_to_price_range();
    assert_eq!(rounded, Price::DECIMAL_MIN);
}

#[test]
fn round_to_price_range_round_up_zero() {
    let price = Decimal::ZERO;
    let rounded = price.round_to_price_range();
    assert_eq!(rounded, Price::DECIMAL_MIN);
}

#[test]
fn round_to_price_range_round_down() {
    let price = Decimal::MAX;
    let rounded = price.round_to_price_range();
    assert_eq!(rounded, Price::DECIMAL_MAX);
}

#[test]
#[should_panic]
fn test_too_small_invalid() {
    let small: Decimal = Price::DECIMAL_MIN - dec!("0.000000000000000001");
    assert!(!small.is_valid_price());
    let _price: Price = small.into();
}

#[test]
#[should_panic]
fn test_too_large_invalid() {
    let large: Decimal = Price::DECIMAL_MAX + dec!("0.000000000000000001");
    assert!(!large.is_valid_price());
    let _price: Price = large.into();
}