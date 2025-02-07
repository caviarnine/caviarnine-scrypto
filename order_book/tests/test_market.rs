#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

use order_book::price::Price;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;

#[test]
fn test_market_basic_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_basic_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);
    
    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_market_empty_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_empty_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_more_than_book_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(10), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_more_than_book_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(10), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_market_stop_price_trigger_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(10), Some(dec!(1)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_market_stop_price_trigger_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(10), Some(dec!(1)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_market_stop_price_not_trigger_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(10), Some(dec!("0.4")), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(3), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(3)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_stop_price_not_trigger_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(10), Some(dec!(3)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(3), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(3)
    );
}

#[test]
fn test_market_stop_price_min_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(10), Some(Decimal::from(Price::MIN)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(3), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(3)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_stop_price_min_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(10), Some(Decimal::from(Price::MIN)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(2), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(2)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_stop_price_max_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(10), Some(Decimal::from(Price::MAX)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(2), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(2)
    );
}

#[test]
fn test_market_stop_price_max_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(10), Some(Decimal::from(Price::MAX)), &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(3), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(3)
    );
}

#[test]
fn test_market_large_gap_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(100000000000), dec!(100000000000), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(2), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(2), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(2)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_large_gap_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(100000000000), dec!("0.00000000001"), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(2), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(2), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(2)
    );
}

#[test]
fn test_market_many_in_limit_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    for _ in 0..10 {
        let price: Decimal = dec!(1);
        let amount: Decimal = dec!(1);

        limit_order(vars.token_y, amount, price, &mut vars).expect_commit_success();
    }

    let price = dec!(1);
    let mut amount_limit = dec!(10);

    let limits = get_bid_limits(None, None, None, &mut vars);
    assert_eq!(limits.len(), 1);
    assert_eq!(
        limits[0], 
        (price, amount_limit)
    );

    market_order(vars.token_x, dec!("2.5"), None, &mut vars).expect_commit_success();

    amount_limit = amount_limit - dec!("2.5");
    assert_eq!(
        get_bid_limits(None, None, None, &mut vars)[0], 
        (price, amount_limit)
    );

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!("0.5")),
        _ => panic!()
    }

    market_order(vars.token_x, dec!(8), None, &mut vars).expect_commit_success();
    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );
}

#[test]
fn test_market_many_in_limit_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    for _ in 0..10 {
        let price: Decimal = dec!(1);
        let amount: Decimal = dec!(1);

        limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();
    }

    let price = dec!(1);
    let mut amount_limit = dec!(10);

    let limits = get_ask_limits(None, None, None, &mut vars);
    assert_eq!(limits.len(), 1);
    assert_eq!(
        limits[0], 
        (price, amount_limit)
    );

    market_order(vars.token_y, dec!("2.5"), None, &mut vars).expect_commit_success();

    amount_limit = amount_limit - dec!("2.5");
    assert_eq!(
        get_ask_limits(None, None, None, &mut vars)[0], 
        (price, amount_limit)
    );

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!("0.5")),
        _ => panic!()
    }

    market_order(vars.token_y, dec!(8), None, &mut vars).expect_commit_success();
    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );
}

#[test]
fn test_market_multiple_limits_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    for i in 0..10 {
        let price: Decimal = (1+i).into();
        let amount: Decimal = (1+i).into();

        limit_order(vars.token_y, amount, price, &mut vars).expect_commit_success();
    }

    assert_eq!(get_bid_limits(None, None, None, &mut vars).len(), 10);

    market_order(vars.token_x, dec!("2.5"), None, &mut vars).expect_commit_success();

    let limits = get_bid_limits(None, None, None, &mut vars);
    assert_eq!(limits.len(), 8);
    assert_eq!(
        limits[0], 
        (dec!(8), dec!("0.5"))
    );

    match get_order_status(10, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(9, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(8, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!("0.5")),
        _ => panic!()
    }

    market_order(vars.token_x, dec!(8), None, &mut vars).expect_commit_success();
    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );
}

#[test]
fn test_market_multiple_limits_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    for i in 0..10 {
        let price: Decimal = (1+i).into();
        let amount: Decimal = 1.into();

        limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();
    }

    assert_eq!(get_ask_limits(None, None, None, &mut vars).len(), 10);

    market_order(vars.token_y, dec!("4.5"), None, &mut vars).expect_commit_success();

    let limits = get_ask_limits(None, None, None, &mut vars);
    assert_eq!(limits.len(), 8);
    assert_eq!(
        limits[0], 
        (dec!(3), dec!("0.5"))
    );

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!("0.5")),
        _ => panic!()
    }

    market_order(vars.token_y, dec!("50.5"), None, &mut vars).expect_commit_success();
    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );
}

#[test]
fn test_market_after_cancel_claim_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(2), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(2), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(2)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    match get_order_status(3, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_after_cancel_claim_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(2), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(2), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(2)
    );

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    match get_order_status(3, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_after_active_claim_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!("0.5"), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_after_active_claim_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!("0.5"), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_after_filled_claim_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_after_filled_claim_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!()
    };
}

#[test]
fn test_market_fee_not_filled_x() {
    let mut vars: Vars = setup();

    market_order(vars.token_x, dec!(1), None, &mut vars);

    assert_balance(vars.token_x, vars.amount_x, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_fee_not_filled_y() {
    let mut vars: Vars = setup();

    market_order(vars.token_y, dec!(1), None, &mut vars);

    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_market_fee_partially_filled_x() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    limit_order(vars.token_y, dec!(2), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    let amount_fee = dec!(1) * fee;

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1) - amount_fee, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1) - amount_fee
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1) + amount_fee
    );
}

#[test]
fn test_market_fee_partially_filled_y() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    limit_order(vars.token_x, dec!(2), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    let amount_fee: Decimal = dec!(1) * fee;

    assert_balance(vars.token_x, vars.amount_x - dec!(1) - amount_fee, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1) + amount_fee
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1) - amount_fee
    );
}

#[test]
fn test_market_small_price_with_fee_x() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let price = Decimal::from(Price::MIN);
    let amount_y = dec!("0.00001");
    let amount_x = round_up(amount_y / price, vars.divisibility_x);
    limit_order(vars.token_y, amount_y, price, &mut vars).expect_commit_success();

    market_order(vars.token_x, amount_x, None, &mut vars).expect_commit_success();

    let amount_fee_x: Decimal = round_up(amount_x * fee, vars.divisibility_x);
    let amount_fee_y: Decimal = round_up(amount_fee_x * price, vars.divisibility_y);

    assert_balance(vars.token_x, vars.amount_x - amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_fee_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        amount_x - amount_fee_x
    );

    assert_eq!(
        get_amount_y(&mut vars),
        amount_fee_y
    );
}

#[test]
fn test_market_large_price_with_fee_x() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let price = Price::DECIMAL_MAX;
    let amount_y = dec!(100000);
    let amount_x = round_up(amount_y / price, vars.divisibility_x);
    limit_order(vars.token_y, amount_y, price, &mut vars).expect_commit_success();

    market_order(vars.token_x, amount_x, None, &mut vars).expect_commit_success();

    let amount_fee_x: Decimal = round_up(amount_x * fee, vars.divisibility_x);
    let amount_fee_y: Decimal = round_up(amount_fee_x * price, vars.divisibility_y);

    assert_balance(vars.token_x, vars.amount_x - amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_fee_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        amount_x - amount_fee_x
    );

    assert_eq!(
        get_amount_y(&mut vars),
        amount_fee_y
    );
}

#[test]
fn test_market_small_price_with_fee_y() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let price = Price::DECIMAL_MIN;
    let amount_x = dec!(100000);
    let amount_y = round_up(amount_x * price, vars.divisibility_y);
    limit_order(vars.token_x, amount_x, price, &mut vars).expect_commit_success();

    market_order(vars.token_y, amount_y, None, &mut vars).expect_commit_success();

    let amount_fee_y: Decimal = round_up(amount_y * fee, vars.divisibility_y);
    let amount_fee_x: Decimal = round_up(amount_fee_y / price, vars.divisibility_x);

    assert_balance(vars.token_x, vars.amount_x - amount_fee_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        amount_fee_x
    );

    assert_eq!(
        get_amount_y(&mut vars),
        amount_y - amount_fee_y
    );
}

#[test]
fn test_market_large_price_with_fee_y() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let price = Decimal::from(Price::MAX);
    let amount_x = dec!("0.00001");
    let amount_y = round_up(amount_x * price, vars.divisibility_y);
    limit_order(vars.token_x, amount_x, price, &mut vars).expect_commit_success();

    market_order(vars.token_y, amount_y, None, &mut vars).expect_commit_success();

    let amount_fee_y: Decimal = round_up(amount_y * fee, vars.divisibility_y);
    let amount_fee_x: Decimal = round_up(amount_fee_y / price, vars.divisibility_x);

    assert_balance(vars.token_x, vars.amount_x - amount_fee_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        amount_fee_x
    );

    assert_eq!(
        get_amount_y(&mut vars),
        amount_y - amount_fee_y
    );
}

#[test]
fn test_market_stop_price_negative_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_x, dec!(10), Some(dec!("-1")), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_market_stop_price_negative_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    market_order(vars.token_y, dec!(10), Some(dec!("-1")), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1)), (dec!(2), dec!(1))]
    );
}

#[test]
fn test_market_amount_zero_invalid_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(0), None, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Order size must be greater than zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_market_amount_zero_invalid_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(0), None, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Order size must be greater than zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_market_other_token_invalid() {
    let mut vars: Vars = setup();

    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(token_a, dec!(1), None, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid token address.")
            },
            _ => false,
        }
    });
}
