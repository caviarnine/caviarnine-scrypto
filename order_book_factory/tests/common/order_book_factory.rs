use scrypto::{prelude::*, api::ObjectModuleId};
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::*;

pub fn build_manifest(
    order_book_factory_package: PackageAddress,
    admin_badge: ResourceAddress,
    token_validator_component: ComponentAddress,
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            order_book_factory_package,
            "OrderBookFactory",
            "new",
            manifest_args!(admin_badge, token_validator_component))
        .build()
}

pub fn set_owner_rule(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_owner_role(vars.order_book_factory_component, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_owner_role(vars.order_book_factory_component, rule)
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

pub fn set_role_rule(role: String, rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_role(vars.order_book_factory_component, ObjectModuleId::Main, RoleKey { key: role }, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_role(vars.order_book_factory_component, ObjectModuleId::Main, RoleKey { key: role }, rule)
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET ROLE RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_owner_rule_default(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.order_book_factory_component,
                "set_owner_rule_default",
                manifest_args!(rule))
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .call_method(
                vars.order_book_factory_component,
                "set_owner_rule_default",
                manifest_args!(rule))
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET OWNER RULE DEFAULT\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_user_rule_default(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.order_book_factory_component,
                "set_user_rule_default",
                manifest_args!(rule))
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .call_method(
                vars.order_book_factory_component,
                "set_user_rule_default",
                manifest_args!(rule))
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET USER RULE DEFAULT\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_token_validator(token_validator_address: ComponentAddress, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .call_method(
                vars.order_book_factory_component,
                "set_token_validator",
                manifest_args!(token_validator_address))
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .call_method(
                vars.order_book_factory_component,
                "set_token_validator",
                manifest_args!(token_validator_address))
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    print!("{:?}", receipt);

    receipt
}

pub fn get_owner_rule_default(vars: &mut Vars) -> AccessRule {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_owner_rule_default",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<AccessRule>(1)
}

pub fn get_user_rule_default(vars: &mut Vars) -> AccessRule {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_user_rule_default",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<AccessRule>(1)
}

pub fn get_token_validator_address(vars: &mut Vars) -> ComponentAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_token_validator_address",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<ComponentAddress>(1)
}

pub fn get_order_book_count(vars: &mut Vars) -> u64 {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_order_book_count",
            manifest_args!())
        .build();
        
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<u64>(1)
}

pub fn get_order_books(start: Option<u64>, end: Option<u64>, vars: &mut Vars) -> Vec<ComponentAddress> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_order_books",
            manifest_args!(start, end))
        .build();
        
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<Vec<ComponentAddress>>(1)
}

pub fn get_order_book_pair(order_book_address: ComponentAddress, vars: &mut Vars) -> Option<(ResourceAddress, ResourceAddress)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_order_book_pair",
            manifest_args!(order_book_address))
        .build();
        
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<Option<(ResourceAddress, ResourceAddress)>>(1)
}

pub fn get_order_books_by_pair(token_x_address: ResourceAddress, token_y_address: ResourceAddress, start: Option<u64>, end: Option<u64>, vars: &mut Vars) -> Vec<ComponentAddress> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "get_order_books_by_pair",
            manifest_args!(token_x_address, token_y_address, start, end))
        .build();
        
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    print!("{:?}", receipt);

    receipt.expect_commit_success().output::<Vec<ComponentAddress>>(1)
}

pub fn new_order_book(token_x: ResourceAddress, token_y: ResourceAddress, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_factory_component,
            "new_order_book",
            manifest_args!(token_x, token_y, None::<ManifestAddressReservation>))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    print!("{:?}", receipt);

    receipt
}
