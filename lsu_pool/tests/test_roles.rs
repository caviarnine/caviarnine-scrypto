// use ::lsu_pool::consts::*;
use scrypto::prelude::*;
mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_role_user_without_proof_failure() {
    // ARRANGE
    let mut vars = setup();

    // ACT change role
    let receipt = lsu_pool::update_role_rule_receipt(
        "user".to_string(),
        AccessRule::DenyAll,
        false,
        &mut vars,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_role_user_with_proof_success() {
    // ARRANGE
    let mut vars = setup();

    // ACT change role
    let receipt = lsu_pool::update_role_rule_receipt(
        "user".to_string(),
        AccessRule::DenyAll,
        true,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_change_user_role_rule_deny_failure_to_add_liquidity() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT 1
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ASSERT 1
    receipt.expect_commit_success();

    // ACT change role - DenyAll
    lsu_pool::update_role_rule_receipt("user".to_string(), AccessRule::DenyAll, true, &mut vars)
        .expect_commit_success();

    // ACT - add liquidity 2)
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ASSERT 2
    receipt.expect_auth_failure();

    // ACT change role AllowAll
    lsu_pool::update_role_rule_receipt("user".to_string(), AccessRule::AllowAll, true, &mut vars)
        .expect_commit_success();

    // ACT - add liquidity 3)
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ASSERT 3
    receipt.expect_commit_success();
}

#[test]
fn test_change_user_role_rule_deny_failure_to_swap() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(100), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(200), &mut vars);

    // ACT - SWAP 1)
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    // ASSERT 1
    receipt.expect_commit_success();

    // ACT change role - DenyAll
    lsu_pool::update_role_rule_receipt("user".to_string(), AccessRule::DenyAll, true, &mut vars)
        .expect_commit_success();

    // ACT - SWAP 2)
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    // ASSERT 2
    receipt.expect_auth_failure();

    // ACT change role AllowAll
    lsu_pool::update_role_rule_receipt("user".to_string(), AccessRule::AllowAll, true, &mut vars)
        .expect_commit_success();

    // ACT - add liquidity 3)
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    // ASSERT 3
    receipt.expect_commit_success();
}

#[test]
fn test_change_user_role_rule_deny_success_to_remove_liquidity() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(100), &mut vars);

    // ACT 1 - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    // ASSERT 1
    receipt.expect_commit_success();

    // ACT change role - DenyAll
    lsu_pool::update_role_rule_receipt("user".to_string(), AccessRule::DenyAll, true, &mut vars)
        .expect_commit_success();

    // ACT 2 - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    // ASSERT 2
    receipt.expect_commit_success();
}
