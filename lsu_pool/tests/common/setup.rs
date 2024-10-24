use scrypto::prelude::*;
use scrypto_unit::*;

use super::vars::*;

use crate::common::lsu_token_validator;
use crate::common::fee_controller;
use crate::common::fee_vaults;
use crate::common::lsu_pool;

pub fn token(vars: &mut Vars, amount: Decimal) -> ResourceAddress {
    vars.test_runner.create_fungible_resource(
        amount,
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    )
}

pub fn setup() -> Vars {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create accounts
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
    let (admin_public_key, _private_key, admin_account_component) =
        test_runner.new_allocated_account();

    // Publish prerequisite packages
    let encoder = AddressBech32Encoder::for_simulator();
    println!("Compiling and publishing packages...");
    let fee_controller_package = test_runner.compile_and_publish("../fee_controller");
    println!(
        "fee_controller_package: {:?}",
        encoder.encode(fee_controller_package.to_vec().as_slice())
    );
    let fee_vaults_package = test_runner.compile_and_publish("../fee_vaults");
    println!(
        "fee_vaults_package: {:?}",
        encoder.encode(fee_vaults_package.to_vec().as_slice())
    );

    // Create tokens
    let floop_token = test_runner.create_freely_mintable_and_burnable_fungible_resource(OwnerRole::None, Some(dec!(1000)), DIVISIBILITY_MAXIMUM, admin_account_component);
    let admin_badge =
        test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, admin_account_component);

    // Create fee controller
    let manifest = fee_controller::build_manifest(fee_controller_package, admin_badge);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let fee_controller_component = receipt.expect_commit(true).new_component_addresses()[0];
    println!(
        "fee_controller_component: {:?}",
        encoder.encode(fee_controller_component.to_vec().as_slice())
    );

    // Create fee vaults
    let manifest =
        fee_vaults::build_manifest(fee_vaults_package, admin_badge, floop_token, dec!(1));
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let fee_vaults_component = receipt.expect_commit(true).new_component_addresses()[0];
    println!(
        "fee_vaults_component: {:?}",
        encoder.encode(fee_vaults_component.to_vec().as_slice())
    );
    
    // Publish lsu token validator package
    let lsu_token_validator_package = test_runner.compile_and_publish("../lsu_token_validator");
    println!(
        "lsu_token_validator_package: {:?}",
        encoder.encode(lsu_token_validator_package.to_vec().as_slice())
    );

    // Create lsu token validator
    let manifest = lsu_token_validator::build_manifest(lsu_token_validator_package, admin_badge);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component = receipt.expect_commit(true).new_component_addresses()[0];

    // Publish lsu pool package
    let lsu_pool_package = test_runner.compile_and_publish("../lsu_pool");
    println!(
        "lsu_pool_package: {:?}",
        encoder.encode(lsu_pool_package.to_vec().as_slice())
    );

    // Create lsu pools
    let manifest = lsu_pool::build_manifest(
        lsu_pool_package,
        admin_badge, 
        token_validator_component);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let lsu_pool_component = receipt.expect_commit(true).new_component_addresses()[0];
    let _lsu_lp_token = receipt.expect_commit(true).new_resource_addresses()[0];

    Vars {
        test_runner,
        public_key,
        account_component_address: account_component,
        admin_public_key,
        admin_account_component_address: admin_account_component,
        admin_badge_resource_address: admin_badge,
        fee_vaults_component_address: fee_vaults_component,
        token_validator_package_address: lsu_token_validator_package,
        token_validator_component_address: token_validator_component,
        lsu_pool_package_address: lsu_pool_package,
        lsu_pool_component_address: lsu_pool_component,
    }
}
