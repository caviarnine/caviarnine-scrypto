#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::fee_controller;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_set_protocol_fee_default_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = fee_controller::set_method_with_u16_input_receipt(
        &mut vars,
        fee_controller_component,
        "set_protocol_fee_default",
        false,
        5u16,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_protocol_fee_default_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = fee_controller::set_method_with_u16_input_receipt(
        &mut vars,
        fee_controller_component,
        "set_protocol_fee_default",
        true,
        5u16,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_protocol_fee_default_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    fee_controller::set_protocol_fee_default(&mut vars, fee_controller_component, 500u16);
    let protocol_fee_default =
        fee_controller::get_protocol_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(protocol_fee_default, Decimal::from_str("0.0005").unwrap());
}

#[test]
#[should_panic]
fn test_set_protocol_fee_default_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT - ASSERT
    fee_controller::set_protocol_fee_default(&mut vars, fee_controller_component, 10001u16);
}

#[test]
fn test_set_protocol_fee_default_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    fee_controller::set_protocol_fee_default(&mut vars, fee_controller_component, 9999u16);

    let protocol_fee_default =
        fee_controller::get_protocol_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(protocol_fee_default, Decimal::from_str("0.009999").unwrap());
}

#[test]
fn test_set_liquidity_fee_default_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = fee_controller::set_method_with_u16_input_receipt(
        &mut vars,
        fee_controller_component,
        "set_liquidity_fee_default",
        false,
        5u16,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_liquidity_fee_default_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = fee_controller::set_method_with_u16_input_receipt(
        &mut vars,
        fee_controller_component,
        "set_liquidity_fee_default",
        true,
        5u16,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_liquidity_fee_default_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    fee_controller::set_liquidity_fee_default(&mut vars, fee_controller_component, 500u16);
    let liquidity_fee_default =
        fee_controller::get_liquidity_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(liquidity_fee_default, Decimal::from_str("0.0005").unwrap());
}

#[test]
#[should_panic]
fn test_set_liquidity_fee_default_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT - ASSERT
    fee_controller::set_liquidity_fee_default(&mut vars, fee_controller_component, 50001u16);
}

#[test]
fn test_set_liquidity_fee_default_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    fee_controller::set_liquidity_fee_default(&mut vars, fee_controller_component, 49999u16);

    let liquidity_fee_default =
        fee_controller::get_liquidity_fee_default(&mut vars, fee_controller_component);

    // ASSERT
    assert_eq!(
        liquidity_fee_default,
        Decimal::from_str("0.049999").unwrap()
    );
}

#[test]
fn test_set_protocol_fee_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let some_random_package_address = vars.fee_controller_package_address;

    // ACT
    let receipt = fee_controller::set_protocol_fee_receipt_with_proof(
        &mut vars,
        fee_controller_component,
        true,
        some_random_package_address,
        5u16,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_protocol_fee_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let some_random_package_address = vars.fee_controller_package_address;

    // ACT
    let receipt = fee_controller::set_protocol_fee_receipt_with_proof(
        &mut vars,
        fee_controller_component,
        false,
        some_random_package_address,
        5u16,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_protocol_fee_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let some_random_package_address = vars.fee_controller_package_address;

    // ACT
    fee_controller::set_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
        500u16,
    );
    let protocol_fee = fee_controller::get_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
    );

    // ASSERT
    assert_eq!(protocol_fee, Decimal::from_str("0.0005").unwrap());
}

#[test]
#[should_panic]
fn test_set_protocol_fee_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let some_random_package_address = vars.fee_controller_package_address;

    // ACT - ASSERT
    fee_controller::set_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
        10001u16,
    );
}

#[test]
fn test_set_protocol_fee_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    let some_random_package_address = vars.fee_controller_package_address;

    // ACT
    fee_controller::set_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
        9999u16,
    );
    let protocol_fee = fee_controller::get_protocol_fee(
        &mut vars,
        fee_controller_component,
        some_random_package_address,
    );

    // ASSERT
    assert_eq!(protocol_fee, Decimal::from_str("0.009999").unwrap());
}

#[test]
fn test_set_liquidity_fee_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set up tokens
    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let some_random_addresses = vec![address01, address02];

    // ACT
    let receipt = fee_controller::set_liquidity_fee_receipt_with_proof(
        &mut vars,
        fee_controller_component,
        true,
        some_random_addresses,
        5u16,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_liquidity_fee_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set up tokens
    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let some_random_addresses = vec![address01, address02];

    // ACT
    let receipt = fee_controller::set_liquidity_fee_receipt_with_proof(
        &mut vars,
        fee_controller_component,
        false,
        some_random_addresses,
        5u16,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_liquidity_fee_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set up tokens
    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_controller::set_liquidity_fee(
        &mut vars,
        fee_controller_component,
        vec![address01, address02],
        500u16,
    );
    let liquidity_fee = fee_controller::get_liquidity_fee(
        &mut vars,
        fee_controller_component,
        vec![address01, address02],
    );

    // ASSERT
    assert_eq!(liquidity_fee, Decimal::from_str("0.0005").unwrap());
}

#[test]
#[should_panic]
fn test_set_liquidity_fee_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set up tokens
    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT - ASSERT
    fee_controller::set_liquidity_fee(
        &mut vars,
        fee_controller_component,
        vec![address01, address02],
        50001u16,
    );
}

#[test]
fn test_set_liquidity_fee_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // set up tokens
    // set some random addresses
    let address01 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let address02 = vars.test_runner.create_fungible_resource(
        dec!(10),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_controller::set_liquidity_fee(
        &mut vars,
        fee_controller_component,
        vec![address01, address02],
        49999u16,
    );
    let liquidity_fee = fee_controller::get_liquidity_fee(
        &mut vars,
        fee_controller_component,
        vec![address01, address02],
    );

    // ASSERT
    assert_eq!(liquidity_fee, Decimal::from_str("0.049999").unwrap());
}
