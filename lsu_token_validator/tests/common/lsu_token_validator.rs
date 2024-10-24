use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::*;

pub fn build_manifest(
    token_validator_package: PackageAddress,
    admin_badge: ResourceAddress,
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            token_validator_package,
            "LsuTokenValidator",
            "new",
            manifest_args!(admin_badge))
        .build()
}

pub fn set_owner_rule(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_owner_role(vars.lsu_token_validator_component, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_owner_role(vars.lsu_token_validator_component, rule)
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET OWNER RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn update_active_set(resource_address: ResourceAddress, contain: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.lsu_token_validator_component,
                "update_active_set",
                manifest_args!(resource_address, contain))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.lsu_token_validator_component,
            "update_active_set",
            manifest_args!(resource_address, contain))
        .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nUPDATE ACTIVE SET\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_require_active(require_active: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.lsu_token_validator_component,
                "set_require_active",
                manifest_args!(require_active))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.lsu_token_validator_component,
            "set_require_active",
            manifest_args!(require_active))
        .build()
    };
    
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nSET REQUIRE ACTIVE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn get_in_active_set(resource_address: ResourceAddress, vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_token_validator_component,
            "get_in_active_set",
            manifest_args!(resource_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET IN ACTIVE SET\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_require_active(vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_token_validator_component,
            "get_require_active",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET REQUIRE ACTIVE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_is_lsu_token(resource_address: ResourceAddress, vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_token_validator_component,
            "get_is_lsu_token",
            manifest_args!(resource_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET IS LSU TOKEN\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn validate_token(resource_address: ResourceAddress, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_token_validator_component,
            "validate_token",
            manifest_args!(resource_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nVALIDATE TOKEN\n");
    println!("{:?}", receipt);
    receipt
}
