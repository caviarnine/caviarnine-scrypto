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
            "TokenValidator",
            "new",
            manifest_args!(admin_badge))
        .build()
}

pub fn set_owner_rule(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_owner_role(vars.token_validator_component, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_owner_role(vars.token_validator_component, rule)
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

pub fn update_white_list(token_address: ResourceAddress, contain: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.token_validator_component,
                "update_white_list",
                manifest_args!(token_address, contain))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.token_validator_component,
            "update_white_list",
            manifest_args!(token_address, contain))
        .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nUPDATE WHITE LIST\n");
    println!("{:?}", receipt);
    receipt
}

pub fn update_black_list(token_address: ResourceAddress, contain: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.token_validator_component,
                "update_black_list",
                manifest_args!(token_address, contain))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.token_validator_component,
            "update_black_list",
            manifest_args!(token_address, contain))
        .build()
    };
    
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nUPDATE BLACK LIST\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_restrict_recallable(restrict: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.token_validator_component,
                "set_restrict_recallable",
                manifest_args!(restrict))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.token_validator_component,
            "set_restrict_recallable",
            manifest_args!(restrict))
        .build()
    };
    
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nSET RESTRICT RECALLABLE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_restrict_freezable(restrict: bool, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.token_validator_component,
                "set_restrict_freezable",
                manifest_args!(restrict))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.token_validator_component,
            "set_restrict_freezable",
            manifest_args!(restrict))
        .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nSET RESTRICT FREEZABLE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_minimum_divisibility(minimum_divisibility: u8, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.token_validator_component,
                "set_minimum_divisibility",
                manifest_args!(minimum_divisibility))
            .build()
    } else {
        ManifestBuilder::new()
        // .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.token_validator_component,
            "set_minimum_divisibility",
            manifest_args!(minimum_divisibility))
        .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)]
    );

    println!("\nSET MINIMUM DIVISIBILITY\n");
    println!("{:?}", receipt);
    receipt
}

pub fn get_white_listed(token_address: ResourceAddress, vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "get_white_listed",
            manifest_args!(token_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET WHITE LISTED\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_black_listed(token_address: ResourceAddress, vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "get_black_listed",
            manifest_args!(token_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET BLACK LISTED\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_restrict_recallable(vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "get_restrict_recallable",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET RESTRICT RECALLABLE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_restrict_freezable(vars: &mut Vars) -> bool {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "get_restrict_freezable",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET RESTRICT FREEZABLE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<bool>(1)
}

pub fn get_minimum_divisibility(vars: &mut Vars) -> u8 {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "get_minimum_divisibility",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nGET MINIMUM DIVISIBILITY\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<u8>(1)
}

pub fn validate_token(token_address: ResourceAddress, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.token_validator_component,
            "validate_token",
            manifest_args!(token_address))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest, 
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );

    println!("\nVALIDATE TOKEN\n");
    println!("{:?}", receipt);
    receipt
}
