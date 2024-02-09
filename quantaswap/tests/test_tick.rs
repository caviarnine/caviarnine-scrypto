use scrypto::prelude::*;

use quantaswap::tick::*;
use quantaswap::tick_index::*;

#[test]
fn test_bits() {
    let tick = Tick::MAX;
    let bits = u32::BITS - tick.0.leading_zeros();
    let bits_rounded = (bits + TickIndex::FACTOR_SHIFT - 1) / TickIndex::FACTOR_SHIFT * TickIndex::FACTOR_SHIFT;
    assert_eq!(bits_rounded, TickIndex::TICK_BITS_USED)
}

#[test]
fn test_one() {
    let tick: Tick = Tick(Tick::OFFSET);
    let decimal: Decimal = tick.into();
    assert_eq!(decimal, dec!("1"));
}

#[test]
fn test_positive() {
    let mut last = dec!(1);
    let num = Tick::MAX.0 - Tick::OFFSET;
    for i in 1..num {
        let tick: Tick = Tick(Tick::OFFSET + i);
        let decimal: Decimal = tick.into();
        let ratio = decimal / last;
        println!("{} {:?}", i, ratio);
        assert!(ratio == dec!("1.0005") || ratio == dec!("1.000499999999999999"));
        last = decimal;
    }
}

#[test]
fn test_negative() {
    let mut last = dec!(1);
    let num = Tick::OFFSET;
    for i in 1..num {
        let tick: Tick = Tick(Tick::OFFSET + i);
        let decimal: Decimal = tick.into();
        let ratio = decimal / last;
        println!("{} {:?}", i, ratio);
        assert!(ratio == dec!("1.0005") || ratio == dec!("1.000499999999999999"));
        last = decimal;
    }
}

#[test]
fn test_round_trip_not_exact() {
    let mut decimal = Decimal::from(Tick::MIN);
    let mul: Decimal = dec!("1.00077");

    while decimal <= Decimal::from(Tick::MAX) {
        let tick: Tick = decimal.into();
        let reverse: Decimal = tick.into();
        let tick2: Tick = reverse.into();
        
        println!("{:?}", tick);
        assert_eq!(tick, tick2);

        decimal = decimal * mul;
    }
}

#[test]
fn test_round_trip_exact() {
    for i in 0..Tick::MAX.0 +1 {
        let tick: Tick = Tick(i);
        let decimal: Decimal = tick.into();
        let tick2: Tick = decimal.into();
        let decimal2: Decimal = tick.into();

        println!("{:?}", tick);
        assert_eq!(tick, tick2);
        assert_eq!(decimal, decimal2);
    }
}

#[test]
fn test_round_trip_rounding() {
    for i in 1..Tick::MAX.0 +1 {
        let tick: Tick = Tick(i);
        let decimal: Decimal = Decimal::from(tick);

        let decimal1: Decimal = decimal * dec!("1.0002499");
        let decimal2: Decimal = decimal / dec!("1.0002499");
        let decimal3: Decimal = decimal * dec!("1.0002501");
        let decimal4: Decimal = decimal / dec!("1.0002501");

        let tick1: Tick = decimal1.into();
        let tick2: Tick = decimal2.into();
        let tick3: Tick = decimal3.into();
        let tick4: Tick = decimal4.into();

        println!("{:?}", tick);
        assert_eq!(tick, tick1);
        assert_eq!(tick, tick2);
        assert!(tick != tick3);
        assert!(tick != tick4);
    }
}

#[test]
fn test_min() {
    let decimal = Decimal::from(Tick::MIN);
    println!("{:?}", decimal);
    assert_eq!(decimal * decimal, dec!("0.000000000001892254"));
    
    let tick: Tick = dec!("0.000000000001892254").checked_sqrt().unwrap().into();
    assert_eq!(Tick::MIN, tick);
}

#[test]
fn test_max() {
    let decimal = Decimal::from(Tick::MAX);
    println!("{:?}", decimal);
    assert_eq!(decimal * decimal, dec!("528470197086.935858253558842035"));

    let tick: Tick = dec!("528470197086.935858253558842035").checked_sqrt().unwrap().into();
    assert_eq!(Tick::MAX, tick);
}

#[test]
fn test_ordering() {
    let price1: Tick = dec!(1).into();
    let price2: Tick = dec!(2).into();
    let price3: Tick = dec!("0.1").into();
    let price4: Tick = dec!("0.2").into();

    assert!(price1 < price2);
    assert!(price3 < price4);
    assert!(price3 < price1);
}

#[test]
fn test_tick_upper() {
    let tick: Tick = Tick(30000);
    let tick_upper: Tick = tick.tick_upper(10);
    assert_eq!(tick_upper, Tick(30010));
}

#[test]
fn test_tick_lower() {
    let tick: Tick = Tick(30000);
    let tick_lower: Tick = tick.tick_lower(10);
    assert_eq!(tick_lower, Tick(29990));
}

#[test]
fn test_round_up() {
    let tick: Tick = Tick(30000);
    let tick_round_up: Tick = tick.round_up(10);
    assert_eq!(tick_round_up, Tick(30000));

    let tick: Tick = Tick(30001);
    let tick_round_up: Tick = tick.round_up(10);
    assert_eq!(tick_round_up, Tick(30010));
}

#[test]
fn test_round_down() {
    let tick: Tick = Tick(30000);
    let tick_round_down: Tick = tick.round_down(10);
    assert_eq!(tick_round_down, Tick(30000));

    let tick: Tick = Tick(30009);
    let tick_round_down: Tick = tick.round_down(10);
    assert_eq!(tick_round_down, Tick(30000));
}

#[test]
fn test_is_valid() {
    let tick: Tick = Tick(30010);
    assert!(tick.is_valid(10));

    let tick: Tick = Tick(30001);
    assert!(!tick.is_valid(10));
}