#![allow(dead_code)]
use scrypto::prelude::*;
mod common;

pub use common::misc::*;
pub use crate::common::fee_controller;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_set_owner_role_rule() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = set_owner_rule(
        fee_controller_component,
        AccessRule::DenyAll,
        vars.admin_badge_resource_address,
        vars.admin_account_component_address,
        vars.admin_public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_twice() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    let token_x = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, vars.account_component_address);

    // ACT
    let receipt = set_owner_rule(
        fee_controller_component,
        rule!(require(token_x)),
        vars.admin_badge_resource_address,
        vars.admin_account_component_address,
        vars.admin_public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();

    // ACT
    let receipt = set_owner_rule(
        fee_controller_component,
        rule!(require(vars.admin_badge_resource_address)),
        token_x,
        vars.account_component_address,
        vars.public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_bad_proof() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    let token_x = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, vars.account_component_address);

    // ACT
    let receipt = set_owner_rule(
        fee_controller_component,
        AccessRule::DenyAll,
        token_x,
        vars.account_component_address,
        vars.public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_fee_manager_role_rule() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = set_role_rule(
        fee_controller_component,
        "fee_manager",
        AccessRule::DenyAll,
        vars.admin_badge_resource_address,
        vars.admin_account_component_address,
        vars.admin_public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_fee_manager_role_rule_twice() {
        // ARRANGE
        let mut vars = setup();
        let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);
    
        // ACT
        let receipt = set_role_rule(
            fee_controller_component,
            "fee_manager",
            AccessRule::DenyAll,
            vars.admin_badge_resource_address,
            vars.admin_account_component_address,
            vars.admin_public_key,
            &mut vars,
        );
    
        // ASSERT
        receipt.expect_commit_success();

        // ACT
        let receipt = set_role_rule(
            fee_controller_component,
            "fee_manager",
            rule!(require(vars.admin_badge_resource_address)),
            vars.admin_badge_resource_address,
            vars.admin_account_component_address,
            vars.admin_public_key,
            &mut vars,
        );
    
        // ASSERT
        receipt.expect_commit_success();
}

#[test]
fn test_set_fee_manager_role_rule_bad_proof() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    let token_x = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, vars.account_component_address);

    // ACT
    let receipt = set_role_rule(
        fee_controller_component,
        "fee_manager",
        AccessRule::DenyAll,
        token_x,
        vars.account_component_address,
        vars.public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_metadata() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = set_metadata(
        fee_controller_component,
        "test_key",
        "test_value",
        vars.admin_badge_resource_address,
        vars.admin_account_component_address,
        vars.admin_public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_metadata_name() {
    // ARRANGE
    let mut vars = setup();
    let fee_controller_component = fee_controller::new_fee_controller_manifest(&mut vars);

    // ACT
    let receipt = set_metadata(
        fee_controller_component,
        "name",
        "test_name",
        vars.admin_badge_resource_address,
        vars.admin_account_component_address,
        vars.admin_public_key,
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}
