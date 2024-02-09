#![allow(dead_code)]
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::token_validator::*;

#[test]
fn test_validate_token_basic_valid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);

    validate_token(token, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_divisibility_valid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), 6, vars.account_component);

    validate_token(token, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_divisibility_invalid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), 5, vars.account_component);

    validate_token(token, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Token is not divisible by at least 6 decimals.",
            )
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_not_fungible_invalid() {
    let mut vars = setup();

    let token = vars.test_runner.create_non_fungible_resource(vars.account_component);

    validate_token(token, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Token is not fungible.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_black_listed_invalid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);

    update_black_list(token, true, true, &mut vars).expect_commit_success();

    validate_token(token, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Token is blacklisted.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_black_listed_white_listed_invalid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);

    update_white_list(token, true, true, &mut vars).expect_commit_success();
    update_black_list(token, true, true, &mut vars).expect_commit_success();

    validate_token(token, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Token is blacklisted.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_restricted_recallable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => rule!(require(vars.admin_badge));
                    recaller_updater => None;
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to be recallable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_restricted_recallable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => AccessRule::DenyAll;
                    recaller_updater => None;
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_restricted_recall_updatable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => None;
                    recaller_updater => rule!(require(vars.admin_badge));
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to be recallable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_restricted_recall_updatable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => None;
                    recaller_updater => AccessRule::DenyAll;
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_restricted_recallable_whitelist_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => rule!(require(vars.admin_badge));
                    recaller_updater => None;
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(true, true, &mut vars).expect_commit_success();
    update_white_list(token_a, true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_not_restricted_recallable_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None, 
                recall_roles: recall_roles!(
                    recaller => rule!(require(vars.admin_badge));
                    recaller_updater => None;
                ), 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_recallable(false, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_restricted_freezable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => rule!(require(vars.admin_badge));
                    freezer_updater => None;
                ), 
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to be freezable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_restricted_freezable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => AccessRule::DenyAll;
                    freezer_updater => None;
                ), 
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_restricted_freeze_updatable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => None;
                    freezer_updater => rule!(require(vars.admin_badge));
                ), 
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to be freezable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_restricted_freeze_updatable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => None;
                    freezer_updater => AccessRule::DenyAll;
                ), 
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_restricted_freezable_whitelist_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => rule!(require(vars.admin_badge));
                    freezer_updater => None;
                ),
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(true, true, &mut vars).expect_commit_success();
    update_white_list(token_a, true, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_not_restricted_freezable_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: freeze_roles!(
                    freezer => rule!(require(vars.admin_badge));
                    freezer_updater => None;
                ), 
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    set_restrict_freezable(false, true, &mut vars).expect_commit_success();

    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_deposit_updatable_not_restricted_freezable_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: deposit_roles!(
                    depositor => AccessRule::AllowAll;
                    depositor_updater => rule!(require(vars.admin_badge));
                ),
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(false, true, &mut vars).expect_commit_success();
    
    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_deposit_updatable_restricted_freezable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: deposit_roles!(
                    depositor => AccessRule::AllowAll;
                    depositor_updater => rule!(require(vars.admin_badge));
                ),
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(true, true, &mut vars).expect_commit_success();
    
    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to have depositable updatable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_deposit_updatable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: deposit_roles!(
                    depositor => AccessRule::AllowAll;
                    depositor_updater => AccessRule::DenyAll;
                ),
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_deposit_updatable_restrict_freezable_whitelist_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: None, 
                deposit_roles: deposit_roles!(
                    depositor => AccessRule::AllowAll;
                    depositor_updater => rule!(require(vars.admin_badge));
                ),
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(true, true, &mut vars);
    update_white_list(token_a, true, true, &mut vars).expect_commit_success();
    
    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_withdraw_updatable_not_restrict_freezable_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: withdraw_roles!(
                    withdrawer => AccessRule::AllowAll;
                    withdrawer_updater => rule!(require(vars.admin_badge));
                ),
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(false, true, &mut vars);
    
    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_withdraw_updatable_restrict_freezable_invalid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: withdraw_roles!(
                    withdrawer => AccessRule::AllowAll;
                    withdrawer_updater => rule!(require(vars.admin_badge));
                ),
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(true, true, &mut vars);
    
    validate_token(token_a, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Only whitelisted tokens are allowed to have withdrawable updatable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_validate_token_withdraw_updatable_denyall_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: withdraw_roles!(
                    withdrawer => AccessRule::AllowAll;
                    withdrawer_updater => AccessRule::DenyAll;
                ),
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];
    
    validate_token(token_a, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_withdraw_updatable_restrict_freezable_whitelist_valid() {
    let mut vars = setup();

    let manifest = ManifestBuilder::new()
        .create_fungible_resource( 
            OwnerRole::None, 
            false, 
            DIVISIBILITY_MAXIMUM, 
            FungibleResourceRoles { 
                mint_roles: None, 
                burn_roles: None, 
                freeze_roles: None,
                recall_roles: None, 
                withdraw_roles: withdraw_roles!(
                    withdrawer => AccessRule::AllowAll;
                    withdrawer_updater => rule!(require(vars.admin_badge));
                ),
                deposit_roles: None,
            }, 
            metadata!(
                init {
                    "name" => "Token A", locked;
                }
            ), 
            Some(dec!(1)))
        .call_method(
            vars.admin_account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    let token_a = receipt.expect_commit_success().new_resource_addresses()[0];

    set_restrict_freezable(true, true, &mut vars);
    update_white_list(token_a, true, true, &mut vars).expect_commit_success();
    
    validate_token(token_a, &mut vars).expect_commit_success();
}
