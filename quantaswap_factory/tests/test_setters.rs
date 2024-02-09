#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap_factory::*;
pub use crate::common::quantaswap_factory;
pub use crate::common::fee_vaults;
pub use crate::common::fee_controller;
pub use crate::common::token_validator;
pub use crate::common::quantaswap;

#[test]
fn test_set_owner_rule_default_valid() {
    let mut vars: Vars = setup();

    set_owner_rule_default(AccessRule::DenyAll, true, &mut vars).expect_commit_success();

    assert_eq!(
        get_owner_rule_default(&mut vars),
        AccessRule::DenyAll
    );
}

#[test]
fn test_set_owner_rule_default_without_admin_invalid() {
    let mut vars: Vars = setup();

    set_owner_rule_default(AccessRule::DenyAll, false, &mut vars).expect_auth_failure();
}

#[test]
fn test_set_user_rule_default_valid() {
    let mut vars: Vars = setup();

    set_user_rule_default(AccessRule::DenyAll, true, &mut vars).expect_commit_success();

    assert_eq!(
        get_user_rule_default(&mut vars),
        AccessRule::DenyAll
    );
}

#[test]
fn test_set_user_rule_default_without_admin_invalid() {
    let mut vars: Vars = setup();

    set_user_rule_default(AccessRule::DenyAll, false, &mut vars).expect_auth_failure();
}

#[test]
fn test_set_token_validator_valid() {
    let mut vars: Vars = setup();

    let manifest = token_validator::build_manifest(
        vars.token_validator_package, 
        vars.admin_badge);
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

    set_token_validator(token_validator_component, true, &mut vars).expect_commit_success();

    assert_eq!(
        get_token_validator_address(&mut vars),
        token_validator_component
    );
}

#[test]
fn test_set_token_validator_without_admin_invalid() {
    let mut vars: Vars = setup();

    let manifest = token_validator::build_manifest(
        vars.token_validator_package, 
        vars.admin_badge);
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

    set_token_validator(token_validator_component, false, &mut vars).expect_auth_failure();
}
