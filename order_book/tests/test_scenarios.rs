#![allow(dead_code)]
use rand::Rng;
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;

#[test]
fn limit_rand() {
    let mut vars: Vars = setup();
    let mut rng = rand::thread_rng();
    
    let mut input_limits: BTreeMap<Decimal, Decimal> = BTreeMap::new();
    for _ in 0..100 {
        let price: Decimal = rng.gen_range(1..=1000).into();
        let amount: Decimal = rng.gen_range(1..=1000).into();

        if let Some(limit_amount) = input_limits.get_mut(&price) {
            *limit_amount += amount;
        } else {
            input_limits.insert(price, amount);
        }

        limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();

        vars.amount_x = vars.amount_x - amount;
        assert_balance(vars.token_x, vars.amount_x, &mut vars);
    }

    let limits = get_ask_limits(None, None, None, &mut vars);
    for limit in limits {
        assert!(input_limits.contains_key(&limit.0));
        assert_eq!(*input_limits.get(&limit.0).unwrap(), limit.1);
    }
}

#[test]
fn limit_rand_market_claim() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let mut rng = rand::thread_rng();
    
    for _ in 0..100 {
        let price: Decimal = rng.gen_range(1..=1000).into();
        let amount: Decimal = rng.gen_range(1..=1000).into();

        limit_order(vars.token_x, amount, price, &mut vars);
    }

    market_order(vars.token_y, dec!(300), None, &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1000000), None, &mut vars).expect_commit_success();

    for i in 0..100 {
        let ids = BTreeSet::from([NonFungibleLocalId::integer(i + 1)]);
        let result = claim_orders(ids, &mut vars);
        print!("{:?}", result);
    }
}

#[test]
fn test_market_big_limit_x() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    for _ in 0..10 {
        let price: Decimal = 10.into();
        let amount: Decimal = 10.into();

        limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();
    }

    let price = dec!(10);
    let mut amount_limit = dec!(100);

    let limits = get_ask_limits(None, Some(price), None, &mut vars);
    assert_eq!(limits.len(), 1);
    assert_eq!(limits[0], (price, amount_limit));

    market_order(vars.token_y, dec!(20), Some(price - dec!(1)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(price), None, &mut vars);
    assert_eq!(limits[0], (price, amount_limit));

    market_order(vars.token_y, dec!(200), Some(price), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(price), None, &mut vars);
    amount_limit -= dec!(20) * (dec!(1) - fee);
    assert_eq!(limits[0], (price, amount_limit));

    market_order(vars.token_y, dec!(1000), Some(price + dec!(1)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(price), None, &mut vars);
    assert_eq!(limits.len(), 0);
}

#[test]
fn test_market_multi_limit_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    for i in 0..10 {
        let price: Decimal = 10u32.pow(i).into();
        let amount: Decimal = 10.into();

        limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();
    }

    let limits = get_ask_limits(None, Some(dec!(1000000000)), None, &mut vars);
    assert_eq!(limits.len(), 10);

    market_order(vars.token_y, dec!(210), Some(dec!(100)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(dec!(1000000000)), None, &mut vars);
    assert_eq!(limits.len(), 8);
    assert_eq!(limits[0], (dec!(100), dec!(9)));

    market_order(vars.token_y, dec!(900), Some(dec!(99)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(dec!(1000000000)), None, &mut vars);
    assert_eq!(limits.len(), 8);
    assert_eq!(limits[0], (dec!(100), dec!(9)));

    market_order(vars.token_y, dec!(100000), Some(dec!(100)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(dec!(1000000000)), None, &mut vars);
    assert_eq!(limits.len(), 7);
    assert_eq!(limits[0], (dec!(1000), dec!(10)));

    market_order(vars.token_y, dec!(100000000000), Some(dec!(1000000000)), &mut vars).expect_commit_success();
    let limits = get_ask_limits(None, Some(dec!(1000000000)), None, &mut vars);
    assert_eq!(limits.len(), 0);
}

#[test]
fn test_scenarios_1() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(100.000000), dec!(4.225100), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1000.000000), dec!(1.000000), &mut vars).expect_commit_success(); 
    limit_order(vars.token_y, dec!(2000.000000), dec!(2.500000), &mut vars).expect_commit_success(); 
    limit_order(vars.token_y, dec!(1538.461538), dec!(3.250000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(3333.333333), dec!(1.500000), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(422.510000), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1538.461538), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(437.734264), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(999.700000), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(562.565736), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(437.134264), None, &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(5000.000000), dec!(8.000000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(666.666667), dec!(1.500000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(3334.444815), dec!(1.499500), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1.000000), dec!(5.000000), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(100.000000), dec!(2.500000), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(250.000000), None, &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(5.000000), None, &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(156.458430), None, &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(10000.000000), dec!(2.500000), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1.000000), dec!(20.000000), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(299.916644), None, &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(0.999700), None, &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(9)]), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(16000.000000), dec!(3.025800), &mut vars).expect_commit_success();

    limit_order(vars.token_x, dec!(100.000000), dec!(3.000000), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(100.000000), dec!(2.510000), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(251.000000), None, &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(15)]), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(999.700000), None, &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(2.000000), dec!(3.500000), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(16)]), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(13)]), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(15000.000000), dec!(5.000000), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(2.000000), dec!(2.709400), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(4761.904762), dec!(1.050000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(4901.960784), dec!(1.020000), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(5.137237), None, &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(0.100000), dec!(2.709400), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(19)]), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(20)]), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(116.472000), dec!(2.500000), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(11)]), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(10874.229570), dec!(2.000000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(10000.000000), dec!(1.000000), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(24)]), &mut vars).expect_commit_success();

    println!("amount x: {}, amount y: {}", get_amount_x(&mut vars), get_amount_y(&mut vars));
    println!("ask: {:?}, bid: {:?}", get_current_ask_price(&mut vars), get_current_bid_price(&mut vars));
    let receipt = market_order(vars.token_x, dec!(91.328660), None, &mut vars);
    println!("amount x: {}, amount y: {}", get_amount_x(&mut vars), get_amount_y(&mut vars));
    println!("ask: {:?}, bid: {:?}", get_current_ask_price(&mut vars), get_current_bid_price(&mut vars));
    println!("receipt: {:?}", receipt);

    market_order(vars.token_x, dec!(25.143340), None, &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(100.000000), dec!(2.676000), &mut vars).expect_commit_success();
    claim_orders(BTreeSet::from([NonFungibleLocalId::integer(25)]), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1000.000000), dec!(1.732000), &mut vars).expect_commit_success();
    println!("ASK {:?}", get_ask_limits(None, None, None, &mut vars));
    println!("BID {:?}", get_bid_limits(None, None, None, &mut vars));

    //----------------------------------------------------------------
}
