use scrypto::prelude::*;

mod common;

pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::token_bridge;
pub use crate::common::vars::Vars;

#[test]
fn test_initialization_success_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_token = vars.token_floop_old;

    // ACT
    let receipt = token_bridge::new_token_bridge_manifest_receipt(
        vars.admin_badge_resource_address,
        old_resource_token,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initialization_success_02() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_token = vars.token_floop_old;

    // ACT
    let receipt = token_bridge::new_token_bridge_manifest_receipt(
        vars.admin_badge_resource_address,
        old_resource_token,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initialization_failure_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_token = vars.test_runner.create_fungible_resource(
        dec!(0),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let receipt = token_bridge::new_token_bridge_manifest_receipt(
        vars.admin_badge_resource_address,
        old_resource_token,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_failure();
}

#[test]
fn test_initialization_failure_02() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_token = vars
        .test_runner
        .create_non_fungible_resource(vars.account_component_address);

    // ACT
    let receipt = token_bridge::new_token_bridge_manifest_receipt(
        vars.admin_badge_resource_address,
        old_resource_token,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_failure();
}
