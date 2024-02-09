#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::fee_controller;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_get_protocol_fee_default_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let protocol_fee_default =
        fee_controller::get_protocol_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(protocol_fee_default, Decimal::from_str("0.0003").unwrap());
}

#[test]
fn test_get_liquidity_fee_default_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let liquidity_fee_default =
        fee_controller::get_liquidity_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(liquidity_fee_default, Decimal::from_str("0.0030").unwrap());
}

#[test]
fn test_get_protocol_fee_01() {
    // ARRANGE
    let mut vars = setup();
    let some_random_package_address = vars.fee_controller_package_address;
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let protocol_fee_default =
        fee_controller::get_protocol_fee_default(&mut vars, fee_controller_component);

    // ACT
    let protocol_fee = fee_controller::get_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
    );

    // ASSERT
    assert_eq!(protocol_fee, protocol_fee_default);
}

#[test]
fn test_get_liquidity_fee_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(0),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(0),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let some_random_addresses = vec![address01, address02];

    // get default liquidity fee and protocol fee
    let liquidity_fee_default =
        fee_controller::get_liquidity_fee_default(&mut vars, fee_controller_component);

    // ACT
    let liquidity_fee = fee_controller::get_liquidity_fee(
        &mut vars,
        fee_controller_component,
        some_random_addresses,
    );

    // ASSERT
    assert_eq!(liquidity_fee, liquidity_fee_default);
}

#[test]
fn test_get_fees_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(0),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(0),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let some_random_addresses = vec![address01, address02];

    // random package address
    let some_random_package_address = vars.fee_controller_package_address;

    // get default liquidity fee and protocol fee
    let liquidity_fee_default =
        fee_controller::get_liquidity_fee_default(&mut vars, fee_controller_component);
    let protocol_fee_default =
        fee_controller::get_protocol_fee_default(&mut vars, fee_controller_component);

    // ACT
    let (protocol_fee, liquidity_fee) = fee_controller::get_fees(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
        some_random_addresses,
    );

    // ASSERT
    assert_eq!(protocol_fee, protocol_fee_default);
    assert_eq!(liquidity_fee, liquidity_fee_default);
}
