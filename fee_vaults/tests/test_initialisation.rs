#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;
use transaction::prelude::ManifestBuilder;

mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_initialization_success_01() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!("0.1"));

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initialization_success_02() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!("0"));

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Swap amount must be greater than zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_initialization_burnable_valid_0() {
    // ARRANGE
    let mut vars = setup();
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::AllowAll;
                    burner_updater => AccessRule::DenyAll;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    vars.token_floop_new_resource_address = receipt.expect_commit_success().new_resource_addresses()[0];

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!(1));

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initialization_burnable_valid_1() {
    // ARRANGE
    let mut vars = setup();
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::AllowAll;
                    burner_updater => None;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    vars.token_floop_new_resource_address = receipt.expect_commit_success().new_resource_addresses()[0];

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!(1));

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initialization_not_maximum_divisible_invalid() {
    // ARRANGE
    let mut vars = setup();
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            17,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::AllowAll;
                    burner_updater => AccessRule::DenyAll;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    vars.token_floop_new_resource_address = receipt.expect_commit_success().new_resource_addresses()[0];

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!(1));

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Swap token must be maximum divisible.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_initialization_not_burnable_invalid() {
    // ARRANGE
    let mut vars = setup();
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::DenyAll;
                    burner_updater => AccessRule::DenyAll;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    vars.token_floop_new_resource_address = receipt.expect_commit_success().new_resource_addresses()[0];

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!(1));

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Swap token must be burnable.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_initialization_updatable_burnable_invalid() {
    // ARRANGE
    let mut vars = setup();
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::DenyAll;
                    burner_updater => AccessRule::AllowAll;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    vars.token_floop_new_resource_address = receipt.expect_commit_success().new_resource_addresses()[0];

    // ACT
    let receipt = fee_vaults::new_fee_vaults_manifest_receipt(&mut vars, dec!(1));

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Swap token must be burnable.")
            },
            _ => false,
        }
    });
}