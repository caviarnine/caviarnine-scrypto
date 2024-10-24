use ::lsu_pool::consts::*;
use ::lsu_pool::events::*;
use scrypto::{api::ObjectModuleId, prelude::*};
// use scrypto::prelude::*;

mod common;

pub use crate::common::lsu_pool;
use crate::common::lsu_token_validator;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_set_token_validator_with_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_token_validator";

    // Create lsu token validator
    let manifest = lsu_token_validator::build_manifest(vars.token_validator_package_address, vars.admin_badge_resource_address);
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component_address = receipt.expect_commit(true).new_component_addresses()[0];

    // ACT
    let receipt = lsu_pool::set_method_with_component_address(token_validator_component_address, true, method_name, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_token_validator_without_proof_receipt_failure() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_token_validator";

    // Create lsu token validator
    let manifest = lsu_token_validator::build_manifest(vars.token_validator_package_address, vars.admin_badge_resource_address);
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component_address = receipt.expect_commit(true).new_component_addresses()[0];

    // ACT
    let receipt = lsu_pool::set_method_with_component_address(token_validator_component_address, false, method_name, &mut vars);

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_token_validator_event() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_token_validator";

    // Create lsu token validator
    let manifest = lsu_token_validator::build_manifest(vars.token_validator_package_address, vars.admin_badge_resource_address);
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component_address = receipt.expect_commit(true).new_component_addresses()[0];

    // ACT
    let receipt = lsu_pool::set_method_with_component_address(token_validator_component_address, true, method_name, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SetTokenValidatorEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SetTokenValidatorEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();

    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1,
        )
    );

    // ASSERT data in event
    assert_eq!(decode_data.token_validator_component_address, token_validator_component_address);
}

// tests for protocol_fee
#[test]
fn test_set_protocol_fee_with_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_protocol_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), true, method_name, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_protocol_fee_without_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_protocol_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), false, method_name, &mut vars);

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_protocol_fee_with_proof_success() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::set_protocol_fee(dec!("0.005"), &mut vars);
}

#[test]
fn test_set_protocol_fee_limits() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_protocol_fee";
    let delta = dec!("0.000001");

    // ASSERT - success
    lsu_pool::set_method_with_decimal(dec!(0), true, method_name, &mut vars)
        .expect_commit_success();
    lsu_pool::set_method_with_decimal(PROTOCOL_FEE_MAX, true, method_name, &mut vars)
        .expect_commit_success();

    // ASSERT - failure
    lsu_pool::set_method_with_decimal(PROTOCOL_FEE_MAX + delta, true, method_name, &mut vars)
        .expect_commit_failure();
    lsu_pool::set_method_with_decimal(-delta, true, method_name, &mut vars).expect_commit_failure();
}

#[test]
fn test_emit_protocol_fee() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_protocol_fee";
    let new_fee = dec!("0.0050");

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(new_fee, true, method_name, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SetProtocolFeeEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SetProtocolFeeEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();

    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1,
        )
    );

    // ASSERT data in event
    assert_eq!(decode_data.protocol_fee, new_fee);
}

// test for liquidity fee
#[test]
fn test_set_liquidity_fee_with_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_liquidity_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), true, method_name, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_liquidity_fee_without_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_liquidity_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), false, method_name, &mut vars);

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_liquidity_fee_with_proof_success() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::set_liquidity_fee(dec!("0.005"), &mut vars);
}

#[test]
fn test_set_liquidity_fee_limits() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_liquidity_fee";
    let delta = dec!("0.000001");

    // ASSERT - success
    lsu_pool::set_method_with_decimal(dec!(0), true, method_name, &mut vars)
        .expect_commit_success();
    lsu_pool::set_method_with_decimal(LIQUIDITY_FEE_MAX, true, method_name, &mut vars)
        .expect_commit_success();

    // ASSERT - failure
    lsu_pool::set_method_with_decimal(LIQUIDITY_FEE_MAX + delta, true, method_name, &mut vars)
        .expect_commit_failure();
    lsu_pool::set_method_with_decimal(-delta, true, method_name, &mut vars).expect_commit_failure();
}

#[test]
fn test_emit_liquidity_fee() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_liquidity_fee";
    let new_fee = dec!("0.0050");

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(new_fee, true, method_name, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SetLiquidityFeeEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SetLiquidityFeeEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();
    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1,
        )
    );

    // ASSERT data in event
    assert_eq!(decode_data.liquidity_fee, new_fee);
}

// test for reserve fee
#[test]
fn test_set_reserve_fee_with_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_reserve_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), true, method_name, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_reserve_fee_without_proof_receipt_failure() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_reserve_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(dec!("0.005"), false, method_name, &mut vars);

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_reserve_fee_with_proof_success() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::set_reserve_fee(dec!("0.005"), &mut vars);
}

#[test]
fn test_set_reserve_fee_limits() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_reserve_fee";
    let delta = dec!("0.000001");

    // ASSERT - success
    lsu_pool::set_method_with_decimal(dec!(0), true, method_name, &mut vars)
        .expect_commit_success();
    lsu_pool::set_method_with_decimal(RESERVE_FEE_MAX, true, method_name, &mut vars)
        .expect_commit_success();

    // ASSERT - failure
    lsu_pool::set_method_with_decimal(RESERVE_FEE_MAX + delta, true, method_name, &mut vars)
        .expect_commit_failure();
    lsu_pool::set_method_with_decimal(-delta, true, method_name, &mut vars).expect_commit_failure();
}

#[test]
fn test_emit_reserve_fee() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_reserve_fee";
    let new_fee = dec!("0.0050");

    // ACT
    let receipt = lsu_pool::set_method_with_decimal(new_fee, true, method_name, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SetReserveFeeEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SetReserveFeeEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();
    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1,
        )
    );

    // ASSERT data in event
    assert_eq!(decode_data.reserve_fee, new_fee);
}

#[test]
pub fn take_from_reserve_vaults_receipt_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero except reserve fee
    lsu_pool::set_reserve_fee(dec!("0.01"), &mut vars);

    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars),
        None
    );

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(100), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // remove all liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(100),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // check reserve vault balance
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap(),
        dec!(1)
    );

    // ACT
    let receipt = lsu_pool::take_from_reserve_vaults_receipt(lsu01_resource, true, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap(),
        dec!(0)
    );
}

#[test]
pub fn take_from_reserve_vaults_receipt_failure() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero except reserve fee
    lsu_pool::set_reserve_fee(dec!("0.01"), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(100), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // remove all liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(100),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // check reserve vault balance
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap(),
        dec!(1)
    );

    // ACT
    let receipt = lsu_pool::take_from_reserve_vaults_receipt(lsu01_resource, false, &mut vars);

    // ASSERT
    receipt.expect_commit_failure();
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap(),
        dec!(1)
    );
}

#[test]
fn set_validator_max_before_fee_no_proof_failure() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_validator_max_before_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_u32(10u32, false, method_name, &mut vars);

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn set_validator_max_before_fee_with_proof_success() {
    // ARRANGE
    let mut vars = setup();

    let method_name = "set_validator_max_before_fee";

    // ACT
    let receipt = lsu_pool::set_method_with_u32(10u32, true, method_name, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn set_validator_max_before_fee_success() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::set_validator_max_before_fee(10u32, &mut vars);
}
