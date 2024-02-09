use scrypto::prelude::*;
use scrypto_unit::*;

use super::vars::*;
use super::fee_controller;
use super::fee_vaults;
use super::quantaswap;

pub fn setup() -> Vars {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create accounts
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
    let (admin_public_key, _private_key, admin_account_component) = test_runner.new_allocated_account();

    // Publish prerequisite packages
    let encoder = AddressBech32Encoder::for_simulator();
    println!("Compiling and publishing packages...");
    let fee_controller_package = test_runner.compile_and_publish("../fee_controller");
    println!("fee_controller_package: {:?}", encoder.encode(fee_controller_package.to_vec().as_slice()));
    let fee_vaults_package = test_runner.compile_and_publish("../fee_vaults");
    println!("fee_vaults_package: {:?}", encoder.encode(fee_vaults_package.to_vec().as_slice()));

    // Create tokens
    let amount_x = Decimal(I192::from(2).pow(152)).checked_floor().unwrap();
    let amount_y = Decimal(I192::from(2).pow(152)).checked_floor().unwrap();
    let divisibility_x = 18;
    let divisibility_y = 18;
    let floop_token = test_runner.create_freely_mintable_and_burnable_fungible_resource(OwnerRole::None, Some(dec!(1000)), DIVISIBILITY_MAXIMUM, admin_account_component);
    let token_x = test_runner.create_fungible_resource(amount_x, divisibility_x, account_component);
    let token_y = test_runner.create_fungible_resource(amount_y, divisibility_y, account_component);
    let admin_badge = test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, admin_account_component);

    // Create fee controller
    let manifest = fee_controller::build_manifest( 
        fee_controller_package, 
        admin_badge);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let fee_controller_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    println!("fee_controller_component: {:?}", encoder.encode(fee_controller_component.to_vec().as_slice()));

    // Create fee vaults
    let manifest = fee_vaults::build_manifest(
        fee_vaults_package, 
        admin_badge,
        floop_token, 
        dec!(1));
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let fee_vaults_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    println!("fee_vaults_component: {:?}", encoder.encode(fee_vaults_component.to_vec().as_slice()));

    // Publish quantaswap package
    let quantaswap_package = test_runner.compile_and_publish("../quantaswap");
    println!("quantaswap_package: {:?}", encoder.encode(quantaswap_package.to_vec().as_slice()));

    // Create quantaswap
    let bin_span = 20;
    let manifest = quantaswap::build_manifest(
        quantaswap_package, 
        rule!(require(admin_badge)),
        AccessRule::AllowAll,
        token_x, 
        token_y, 
        bin_span);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    Vars {
        test_runner,
        public_key,
        admin_public_key,
        account_component,
        admin_account_component,
        fee_controller_package,
        fee_controller_component,
        fee_vaults_package,
        fee_vaults_component,
        quantaswap_package,
        quantaswap_component,
        admin_badge,
        floop_token,
        token_x,
        amount_x,
        divisibility_x,
        token_y,
        amount_y,
        divisibility_y,
        liquidity_receipt,
        bin_span,
    }
}
