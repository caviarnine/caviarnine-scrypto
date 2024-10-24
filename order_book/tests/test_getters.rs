#![allow(dead_code)]
use order_book::price::Price;
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;

#[test]
fn test_get_fee_controller_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_fee_controller_address(&mut vars), 
        vars.fee_controller_component
    );
}

#[test]
fn test_get_fee_vaults_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_fee_vaults_address(&mut vars), 
        vars.fee_vaults_component
    );
}

#[test]
fn test_get_token_x_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_token_x_address(&mut vars), 
        vars.token_x
    );
}

#[test]
fn test_get_token_y_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_token_y_address(&mut vars), 
        vars.token_y
    );
}

#[test]
fn test_get_order_receipt_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_receipt_address(&mut vars), 
        vars.order_receipt
    );
}

#[test]
fn test_get_amount_x_basic() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_amount_x_after_limit() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(0)
    );

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(1)
    );
}

#[test]
fn test_get_amount_x_after_market() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(0)
    );

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(1)
    );

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_amount_y_basic() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_amount_y_after_limit() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(0)
    );

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(1)
    );
}

#[test]
fn test_get_amount_y_after_market() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(0)
    );

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(1)
    );

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_price_start() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_last_price(&mut vars), 
        Price::DECIMAL_MIN
    );
}

#[test]
fn test_get_price_after_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_last_price(&mut vars), 
        Price::DECIMAL_MIN
    );
}

#[test]
fn test_get_price_after_market_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_last_price(&mut vars), 
        dec!(1)
    );
}

#[test]
fn test_get_price_after_market_y_and_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_last_price(&mut vars), 
        dec!(1)
    );

    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_last_price(&mut vars), 
        dec!(1)
    );

    market_order(vars.token_x, dec!("0.1"), None, &mut vars).expect_commit_success();

    assert_eq!(
        get_last_price(&mut vars), 
        dec!(2)
    );
}

#[test]
fn test_get_current_ask_price_start() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_current_ask_price(&mut vars), 
        None
    );
}

#[test]
fn test_get_current_ask_price_after_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );
}

#[test]
fn test_get_current_ask_price_after_worse_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );
}

#[test]
fn test_get_current_ask_price_after_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );

    limit_order(vars.token_x, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!("0.5"))
    );
}

#[test]
fn test_get_current_ask_price_after_market() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(2))
    );

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );

    market_order(vars.token_y, dec!(1), None, &mut vars);

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(2))
    );
}

#[test]
fn test_get_current_ask_price_truncate_price_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(213123), dec!("43213.213123"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!("43213"))
    );
}

#[test]
fn test_get_current_ask_price_truncate_price_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(213123), dec!("43213.213123"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!("43213"))
    );

    limit_order(vars.token_x, dec!("0.12"), dec!("0.9874323"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!("0.98743"))
    );
}

#[test]
fn test_get_current_ask_price_after_limit_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );
}

#[test]
fn test_get_current_bid_price_start() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_current_bid_price(&mut vars), 
        None
    );
}

#[test]
fn test_get_current_bid_price_after_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );
}

#[test]
fn test_get_current_bid_price_after_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );

    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(2))
    );
}

#[test]
fn test_get_current_bid_price_after_worse_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );

    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );
}

#[test]
fn test_get_current_bid_price_after_market() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!("0.5"))
    );

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );

    market_order(vars.token_x, dec!(1), None, &mut vars);

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!("0.5"))
    );
}

#[test]
fn test_get_current_bid_price_truncate_price_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(213123), dec!("21321.98743"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(21321))
    );
}

#[test]
fn test_get_current_bid_price_truncate_price_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(213123), dec!("0.9874323"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!("0.98743"))
    );

    limit_order(vars.token_y, dec!("0.12312"), dec!("21378.123"), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(21378))
    );
}

#[test]
fn test_get_current_bid_price_after_limit_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );
}

#[test]
fn test_get_ask_limits_start() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_ask_limits_after_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_after_worse_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1)), (dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_after_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!("0.5"), dec!(1)), (dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_start_price() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(Some(dec!(1)), None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_stop_price() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, Some(dec!(1)), None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_stop_price_inbetween() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, Some(dec!("1.5")), None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_number() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, Some(1), &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_start_price_below() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(Some(dec!("0.5")), None, None, &mut vars),
        vec![(dec!(1), dec!(1)), (dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_start_price_inbetween() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(Some(dec!("1.5")), None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_start_price_above() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(Some(dec!(3)), None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_ask_limits_after_limit_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_start_price_negative() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(Some(dec!("-1")), None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );
}

#[test]
fn test_get_ask_limits_stop_price_negative() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, Some(dec!("-1")), None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_bid_limits_start() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_bid_limits_after_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_after_better_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(2), dec!("0.5")), (dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_after_worse_limit() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!("0.5"), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1)), (dec!("0.5"), dec!(2))]
    );
}

#[test]
fn test_get_bid_limits_start_price() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(Some(dec!(2)), None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_stop_price() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, Some(dec!(2)), None, &mut vars),
        vec![(dec!(2), dec!("0.5"))]
    );
}

#[test]
fn test_get_bid_limits_stop_price_inbetween() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, Some(dec!("1.5")), None, &mut vars),
        vec![(dec!(2), dec!("0.5"))]
    );
}

#[test]
fn test_get_bid_limits_number() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, Some(1), &mut vars),
        vec![(dec!(2), dec!("0.5"))]
    );
}

#[test]
fn test_get_bid_limits_start_price_above() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(Some(dec!("2.5")), None, None, &mut vars),
        vec![(dec!(2), dec!("0.5")), (dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_start_price_inbetween() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(Some(dec!("1.5")), None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_start_price_below() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(Some(dec!("0.5")), None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_bid_limits_after_limit_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_bid_limits_start_price_negative() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(Some(dec!("-1")), None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_bid_limits_stop_price_negative() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, Some(dec!("-1")), None, &mut vars),
        vec![(dec!(2), dec!("0.5"))]
    );
}

#[test]
fn test_get_order_status_id_0() {
    let mut vars: Vars = setup();

    match get_order_status(0, &mut vars) {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_after_limit_x_at_nonce() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_after_limit_y_at_nonce() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_after_limit_x_above_nonce() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(3, &mut vars) {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_after_limit_y_above_nonce() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(3, &mut vars) {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}


#[test]
fn test_get_order_status_invalid_id_type() {
    let mut vars: Vars = setup();

    let id = NonFungibleLocalId::string("1").unwrap();
    let manifest = transaction::prelude::ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_order_status",
            manifest_args!(id),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    
    let order_status = receipt.expect_commit_success().output::<OrderStatus>(1);
    
    match order_status {
        OrderStatus::Invalid => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_cancel_claimed_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_cancel_claimed_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_claim_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(2), None, &mut vars).expect_commit_success();
    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();


    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_claim_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(2), None, &mut vars).expect_commit_success();
    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_two_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_two_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_y_and_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(2), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_x_and_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(2), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_part_filled_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(2), dec!(2), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(2), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(1));
            assert_eq!(order_data.amount_total, dec!(2));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_open_part_filled_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!("0.5"), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!("0.5"));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(1));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(1));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_first_open_second_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(1));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_status_filled_first_open_second_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(1));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}

#[test]
fn test_get_order_statuses_empty() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_statuses(vec![], &mut vars).len(),
        0
    );
}

#[test]
fn test_get_order_statuses_several() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let order_statuses = get_order_statuses(vec![1, 2, 3, 4], &mut vars);

    match order_statuses[0] {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match order_statuses[1] {
        OrderStatus::Open(order_data) => {
            assert!(order_data.is_ask);
            assert_eq!(order_data.price, dec!(2));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match order_statuses[2] {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }

    match order_statuses[3] {
        OrderStatus::Open(order_data) => {
            assert!(!order_data.is_ask);
            assert_eq!(order_data.price, dec!(1));
            assert_eq!(order_data.amount_filled, dec!(0));
            assert_eq!(order_data.amount_total, dec!(1));
        },
        _ => panic!()
    }
}
