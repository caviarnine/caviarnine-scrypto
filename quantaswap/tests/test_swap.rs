#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

use ::quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;

#[test]
fn test_swap_no_liquidity_x() {
    let mut vars = setup();

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );
    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_swap_no_liquidity_y() {
    let mut vars = setup();

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );
    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_swap_x() { //1.22641423624
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -change_balance_x * price_mid;

    assert!(new_price < price);
    assert_eq!(change_balance_x, -amount);
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -change_balance_y / price_mid;

    assert!(new_price > price);
    assert_eq!(change_balance_y, -amount);
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_return_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price0 = get_price(&mut vars).unwrap();
    let balance_x0 = get_balance(vars.token_x, &mut vars);
    let balance_y0 = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let balance_y1 = get_balance(vars.token_y, &mut vars);
    let change_balance_y1 = balance_y1 - balance_y0;

    swap(vars.token_y, change_balance_y1, &mut vars).expect_commit_success();

    let price2 = get_price(&mut vars).unwrap();
    let balance_x2 = get_balance(vars.token_x, &mut vars);
    let balance_y2 = get_balance(vars.token_y, &mut vars);

    assert_eq!(balance_y0, balance_y2);
    assert_within_error_margin(price0, price2, dec!("0.0000001"));
    assert_within_error_margin(balance_x0, balance_x2,  dec!("0.0000001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_return_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);
    
    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
    
    let price0 = get_price(&mut vars).unwrap();
    let balance_x0 = get_balance(vars.token_x, &mut vars);
    let balance_y0 = get_balance(vars.token_y, &mut vars);
    
    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();
    
    let balance_x1 = get_balance(vars.token_x, &mut vars);
    let change_balance_x1 = balance_x1 - balance_x0;
    
    swap(vars.token_x, change_balance_x1, &mut vars).expect_commit_success();
    
    let price2 = get_price(&mut vars).unwrap();
    let balance_x2 = get_balance(vars.token_x, &mut vars);
    let balance_y2 = get_balance(vars.token_y, &mut vars);
    
    assert_eq!(balance_x0, balance_x2);
    assert_within_error_margin(price0, price2, dec!("0.0000001"));
    assert_within_error_margin(balance_y0, balance_y2,  dec!("0.0000001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_zero_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(0);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(new_price, price);
    assert_eq!(change_balance_x, dec!(0));
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_zero_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(0);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(new_price, price);
    assert_eq!(change_balance_x, dec!(0));
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_no_x_liquidity_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(0);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price < price);
    assert_eq!(change_balance_x, -amount);
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_no_y_liquidity_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price > price);
    assert_eq!(change_balance_y, -amount);
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_no_y_liquidity_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(5);
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(change_balance_x, dec!(0)); 
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_no_x_liquidity_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(0);
    let amount_y0 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(change_balance_x, dec!(0));
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_low_amount_no_y_liquidity_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MIN.0;
    let amount_x0 = dec!("10000000000000000");
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(change_balance_x, dec!(0));
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_low_amount_no_x_liquidity_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MAX.0;
    let amount_x0 = dec!(0);
    let amount_y0 = dec!("10000000000000000");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(change_balance_x, dec!(0));
    assert_eq!(change_balance_y, dec!(0));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_low_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_low_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_y_low_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!(0);
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_x_low_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_high_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);
    
    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_high_liquidity_high_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);
    
    assert!(new_price <= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_low_liquidity_low_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!(0);
    let amount_y = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_low_liquidity_high_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!(0);
    let amount_y = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = Decimal((price.0 * new_price.0).sqrt());
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price <= price);
    assert!(change_balance_y > dec!(0));
    assert_within_error_margin(-change_balance_x, -change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_low_liquidity_low_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);

    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_low_liquidity_high_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y: Decimal = get_balance(vars.token_y, &mut vars);

    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(-change_balance_y, -change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_low_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_low_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_y_low_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!(0);
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_y_low_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_high_amount_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -round_up(change_balance_x * price_mid, vars.divisibility_y);

    assert!(new_price <= price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_high_liquidity_high_amount_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("10000000000000000");
    let amount_y = dec!("10000000000000000");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!("10000000000000000");
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -round_up(change_balance_y / price_mid, vars.divisibility_x);

    assert!(new_price >= price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_low_liquidity_low_amount_no_locking_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MAX.0;
    let amount_x0 = dec!(0);
    let amount_y0 = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_low_liquidity_low_amount_no_locking_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MIN.0;
    let amount_x0 = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_high_tick_low_liquidity_high_amount_no_locking_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MAX.0;
    let amount_x0 = dec!(0);
    let amount_y0 = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = dec!("10000000000000000");
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!("1");
    let amount_y1 = dec!("1");
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_low_tick_low_liquidity_high_amount_no_locking_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::MIN.0;
    let amount_x0 = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = dec!("10000000000000000");
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(5);
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(0);
    let amount_y0 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(5);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_no_next_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(5);
    let amount_y0 = dec!("0.1");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_no_next_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!("0.1");
    let amount_y0 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_gap_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(5);
    let amount_y0 = dec!("0.1");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span * 200;
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_gap_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!("0.1");
    let amount_y0 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span * 200;
    let amount_x1 = dec!(5);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_many_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..5 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions.clone(), &mut vars).expect_commit_success();

    let amount = dec!(5);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(positions[4].0)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_many_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..5 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions.clone(), &mut vars).expect_commit_success();

    let amount = dec!(5);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(positions[4].0)
    );
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_return_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(5);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let price0 = get_price(&mut vars).unwrap();
    let balance_x0 = get_balance(vars.token_x, &mut vars);
    let balance_y0 = get_balance(vars.token_y, &mut vars);

    let amount = dec!(2);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let balance_y1 = get_balance(vars.token_y, &mut vars);
    let change_balance_y1 = balance_y1 - balance_y0;

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );

    swap(vars.token_y, change_balance_y1, &mut vars).expect_commit_success();

    let price2 = get_price(&mut vars).unwrap();
    let balance_x2 = get_balance(vars.token_x, &mut vars);
    let balance_y2 = get_balance(vars.token_y, &mut vars);

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );

    assert_eq!(balance_y0, balance_y2);
    assert_within_error_margin(price0, price2, dec!("0.00001"));
    assert_within_error_margin(balance_x0, balance_x2, dec!("0.00001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_cross_return_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!(1);
    let amount_y0 = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(5);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let price0 = get_price(&mut vars).unwrap();
    let balance_x0 = get_balance(vars.token_x, &mut vars);
    let balance_y0 = get_balance(vars.token_y, &mut vars);

    let amount = dec!(2);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let balance_x1 = get_balance(vars.token_x, &mut vars);
    let change_balance_x1 = balance_x1 - balance_x0;

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );

    swap(vars.token_x, change_balance_x1, &mut vars).expect_commit_success();

    let price2 = get_price(&mut vars).unwrap();
    let balance_x2 = get_balance(vars.token_x, &mut vars);
    let balance_y2 = get_balance(vars.token_y, &mut vars);

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );

    assert_eq!(balance_x0, balance_x2);
    assert_within_error_margin(price0, price2, dec!("0.00001"));
    assert_within_error_margin(balance_y0, balance_y2, dec!("0.00001"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_protocol_fee_x() {
    let mut vars = setup();
    fee_controller::set_liquidity_fee_default_zero(&mut vars);
    let protocol_fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -change_balance_x * price_mid * (dec!(1) - protocol_fee);

    assert!(new_price < price);
    assert_eq!(change_balance_x, -amount);
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_protocol_fee_y() {
    let mut vars = setup();
    fee_controller::set_liquidity_fee_default_zero(&mut vars);
    let protocol_fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -change_balance_y / price_mid * (dec!(1) - protocol_fee);

    assert!(new_price > price);
    assert_eq!(change_balance_y, -amount);
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_protocol_fee_part_x() {
    let mut vars = setup();
    fee_controller::set_liquidity_fee_default_zero(&mut vars);
    let protocol_fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -change_balance_x * price_mid * (dec!(1) - protocol_fee);

    assert!(new_price < price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

fn test_swap_protocol_fee_part_y() {
    let mut vars = setup();
    fee_controller::set_liquidity_fee_default_zero(&mut vars);
    let protocol_fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -change_balance_y / price_mid * (dec!(1) - protocol_fee);

    assert!(new_price > price);
    assert_eq!(change_balance_y, -amount);
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_liquidity_fee_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let liquidity_fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -change_balance_x * price_mid * (dec!(1) - liquidity_fee);

    assert!(new_price < price);
    assert_eq!(change_balance_x, -amount);
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_liquidity_fee_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let liquidity_fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -change_balance_y / price_mid * (dec!(1) - liquidity_fee);

    assert!(new_price > price);
    assert_eq!(change_balance_y, -amount);
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_liquidity_fee_part_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let liquidity_fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);
    swap(vars.token_x, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_y = -change_balance_x * price_mid * (dec!(1) - liquidity_fee);

    assert!(new_price < price);
    assert!(change_balance_x < dec!(0));
    assert_within_error_margin(change_balance_y, change_expected_y, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_liquidity_fee_part_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let liquidity_fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let price = get_price(&mut vars).unwrap();
    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);
    swap(vars.token_y, amount, &mut vars).expect_commit_success();

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;
    let price_mid = (price * new_price).checked_sqrt().unwrap();
    let change_expected_x = -change_balance_y / price_mid * (dec!(1) - liquidity_fee);

    assert!(new_price > price);
    assert!(change_balance_y < dec!(0));
    assert_within_error_margin(change_balance_x, change_expected_x, dec!("0.01"));
    assert_amount_sums(&mut vars);
}

#[test]
fn test_swap_invalid_token() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let token = vars.test_runner.create_fungible_resource(dec!(1000), DIVISIBILITY_MAXIMUM, vars.account_component);

    swap(token, dec!(1), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid token address.")
            },
            _ => false,
        }
    });
}
