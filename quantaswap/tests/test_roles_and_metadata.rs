#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError;
use radix_engine::errors::SystemError;
use transaction::prelude::ManifestBuilder;

use quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;

#[test]
fn test_set_owner_role_rule_pool() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.quantaswap_component, 
        AccessRule::DenyAll, 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_pool_twice() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.quantaswap_component, 
        rule!(require(vars.token_x)), 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();

    set_owner_rule(
        vars.quantaswap_component, 
        rule!(require(vars.admin_badge)), 
        vars.token_x,
        vars.account_component, 
        vars.public_key, 
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_pool_bad_proof() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.quantaswap_component, 
        AccessRule::DenyAll, 
        vars.token_x, 
        vars.account_component, 
        vars.public_key, 
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_set_owner_role_rule_liquidity_receipt() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.liquidity_receipt, 
        AccessRule::DenyAll, 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_liquidity_receipt_twice() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.liquidity_receipt, 
        rule!(require(vars.token_x)), 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();

    set_owner_rule(
        vars.liquidity_receipt, 
        rule!(require(vars.admin_badge)), 
        vars.token_x,
        vars.account_component, 
        vars.public_key, 
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_liquidity_receipt_bad_proof() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.liquidity_receipt, 
        AccessRule::DenyAll, 
        vars.token_x, 
        vars.account_component, 
        vars.public_key, 
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_set_user_role_rule() {
    let mut vars: Vars = setup();

    set_role_rule(
        vars.quantaswap_component, 
        "user", 
        AccessRule::DenyAll, 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_user_role_rule_twice() {
    let mut vars: Vars = setup();

    set_role_rule(
        vars.quantaswap_component,
        "user",
        AccessRule::DenyAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    set_role_rule(
        vars.quantaswap_component,
        "user",
        AccessRule::AllowAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_user_role_rule_bad_proof() {
    let mut vars: Vars = setup();

    set_role_rule(
        vars.quantaswap_component, 
        "user", 
        AccessRule::DenyAll, 
        vars.token_x, 
        vars.account_component, 
        vars.public_key, 
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_user_role_methods_deny_all() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    set_role_rule(
        vars.quantaswap_component, 
        "user", 
        AccessRule::DenyAll, 
        vars.admin_badge, 
        vars.admin_account_component, 
        vars.admin_public_key, 
        &mut vars,
    ).expect_commit_success();

    // Mint liquidity receipt
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.quantaswap_component,
        "mint_liquidity_receipt",
        manifest_args!(),
    )
    .call_method(
        vars.account_component,
        "deposit_batch",
        manifest_args!(ManifestExpression::EntireWorktop))
    .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    receipt.expect_auth_failure();

    add_liquidity_to_receipt(id, dec!(1), dec!(1), vec![(Tick::ONE.0, dec!(1), dec!(1))], &mut vars).expect_auth_failure();
    swap(vars.token_x, dec!(1), &mut vars).expect_auth_failure();
}


#[test]
fn test_get_metadata_pool() {
    let mut vars: Vars = setup();

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "token_x"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_x)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "token_y"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_y)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "liquidity_receipt"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.liquidity_receipt)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "name"),
        Some(MetadataValue::String(format!("Pool {}/{}", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "description"),
        Some(MetadataValue::String(format!("Pool for pair {}/{}.", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["defi".into(), "dex".into(), "amm".into(), "pool".into()]))
    );
}

#[test]
fn test_get_metadata_liquidity_receipt() {
    let mut vars: Vars = setup();

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "component"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.quantaswap_component)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "package"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.quantaswap_package)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "token_x"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_x)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "token_y"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_y)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "name"),
        Some(MetadataValue::String(format!("Liquidity Receipt {}/{}", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "description"),
        Some(MetadataValue::String(format!("Used to store liquidity positions for pair {}/{}.", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["defi".into(), "dex".into(), "amm".into(), "LP token".into(), "receipt".into()]))
    );
}

#[test]
fn test_set_metadata_pool() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "test_key",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "test_key"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_pool_name_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "name",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "name"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_pool_description_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "description",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "description"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_pool_tags_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "tags",
        vec!["test_value"],
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.quantaswap_component.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["test_value".into()]))
    );
}

#[test]
fn test_set_metadata_pool_token_x_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "token_x",
        GlobalAddress::from(vars.token_x),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_pool_token_y_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "token_y",
        GlobalAddress::from(vars.token_y),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_pool_liquidity_receipt_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "liquidity_receipt",
        GlobalAddress::from(vars.liquidity_receipt),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_pool_bad_proof() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.quantaswap_component,
        "test_key",
        "test_value",
        vars.token_x,
        vars.account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_set_metadata_liquidity_receipt() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "test_key",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "test_key"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_liquidity_receipt_name_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "name",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "name"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_liquidity_receipt_description_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "description",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "description"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_liquidity_receipt_tags_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "tags",
        vec!["test_value"],
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.liquidity_receipt.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["test_value".into()]))
    );
}

#[test]
fn test_set_metadata_liquidity_receipt_component_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "component",
        GlobalAddress::from(vars.quantaswap_component),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_liquidity_receipt_package_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "package",
        GlobalAddress::from(vars.quantaswap_package),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_liquidity_receipt_token_x_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "token_x",
        GlobalAddress::from(vars.token_x),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_liquidity_receipt_token_y_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "token_y",
        GlobalAddress::from(vars.token_y),
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_specific_failure(|err| {
        match err {
            RuntimeError::SystemError(err) => *err == SystemError::KeyValueEntryLocked,
            _ => false,
        }
    });
}

#[test]
fn test_set_metadata_liquidity_receipt_bad_proof() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.liquidity_receipt,
        "test_key",
        "test_value",
        vars.token_x,
        vars.account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_burn_receipt_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let ids = BTreeSet::from([id]);
    let manifest = transaction::builder::ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.liquidity_receipt, ids)
        .burn_all_from_worktop(vars.liquidity_receipt)
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    receipt.expect_auth_failure();
}