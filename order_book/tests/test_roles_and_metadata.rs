#![allow(dead_code)]
use radix_engine::errors::RuntimeError;
use radix_engine::errors::SystemError;
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;

#[test]
fn test_set_owner_role_rule_order_book() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_book_component,
        AccessRule::DenyAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_order_book_twice() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_book_component,
        rule!(require(vars.token_x)), 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    set_owner_rule(
        vars.order_book_component,
        rule!(require(vars.admin_badge)), 
        vars.token_x,
        vars.account_component,
        vars.public_key,
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_order_book_bad_proof() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_book_component,
        AccessRule::DenyAll, 
        vars.token_x,
        vars.account_component,
        vars.public_key,
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_set_owner_role_rule_order_receipt() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_receipt,
        AccessRule::DenyAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_order_receipt_twice() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_receipt,
        rule!(require(vars.token_x)), 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    set_owner_rule(
        vars.order_receipt,
        rule!(require(vars.admin_badge)), 
        vars.token_x,
        vars.account_component,
        vars.public_key,
        &mut vars,
    ).expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_order_receipt_bad_proof() {
    let mut vars: Vars = setup();

    set_owner_rule(
        vars.order_receipt,
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
        vars.order_book_component,
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
        vars.order_book_component,
        "user",
        AccessRule::DenyAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    set_role_rule(
        vars.order_book_component,
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
        vars.order_book_component,
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

    set_role_rule(
        vars.order_book_component,
        "user".to_string(),
        AccessRule::DenyAll, 
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_auth_failure();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_auth_failure();
}

#[test]
fn test_get_metadata_order_book() {
    let mut vars: Vars = setup();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "token_x"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_x)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "token_y"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_y)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "order_receipt"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.order_receipt)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "name"),
        Some(MetadataValue::String(format!("Order Book {}/{}", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "description"),
        Some(MetadataValue::String(format!("Order book for pair {}/{}.", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["defi".into(), "dex".into(), "order book".into()]))
    );
}

#[test]
fn test_get_metadata_order_receipt() {
    let mut vars: Vars = setup();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "component"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.order_book_component)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "package"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.order_book_package)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "token_x"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_x)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "token_y"),
        Some(MetadataValue::GlobalAddress(GlobalAddress::from(vars.token_y)))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "name"),
        Some(MetadataValue::String(format!("Order Receipt {}/{}", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "description"),
        Some(MetadataValue::String(format!("Used to claim tokens from a limit order for pair {}/{}.", "", "")))
    );

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["defi".into(), "dex".into(), "order book".into(), "receipt".into()]))
    );
}

#[test]
fn test_set_metadata_order_book() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "test_key",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "test_key"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_book_name_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "name",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "name"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_book_description_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "description",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "description"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_book_tags_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "tags",
        vec!["test_value"],
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_book_component.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["test_value".into()]))
    );
}

#[test]
fn test_set_metadata_order_book_token_x_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
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
fn test_set_metadata_order_book_token_y_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
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
fn test_set_metadata_order_book_order_receipt_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "order_receipt",
        GlobalAddress::from(vars.order_receipt),
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
fn test_set_metadata_order_book_bad_proof() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_book_component,
        "test_key",
        "test_value",
        vars.token_x,
        vars.account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_auth_failure();
}

#[test]
fn test_set_metadata_order_receipt() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "test_key",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "test_key"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_receipt_name_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "name",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "name"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_receipt_description_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "description",
        "test_value",
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "description"),
        Some(MetadataValue::String("test_value".into()))
    );
}

#[test]
fn test_set_metadata_order_receipt_tags_updatable() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "tags",
        vec!["test_value"],
        vars.admin_badge,
        vars.admin_account_component,
        vars.admin_public_key,
        &mut vars,
    ).expect_commit_success();
    
    assert_eq!(
        vars.test_runner.get_metadata(vars.order_receipt.into(), "tags"),
        Some(MetadataValue::StringArray(vec!["test_value".into()]))
    );
}

#[test]
fn test_set_metadata_order_receipt_component_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "component",
        GlobalAddress::from(vars.order_book_component),
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
fn test_set_metadata_order_receipt_package_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
        "package",
        GlobalAddress::from(vars.order_book_package),
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
fn test_set_metadata_order_receipt_token_x_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
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
fn test_set_metadata_order_receipt_token_y_locked() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
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
fn test_set_metadata_order_receipt_bad_proof() {
    let mut vars: Vars = setup();

    set_metadata(
        vars.order_receipt,
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

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    let manifest = transaction::builder::ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.order_receipt, ids)
        .burn_all_from_worktop(vars.order_receipt)
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    receipt.expect_auth_failure();
}
