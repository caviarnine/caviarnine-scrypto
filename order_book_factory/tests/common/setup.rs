use scrypto::prelude::*;
use scrypto_unit::*;

use super::vars::*;
use super::fee_controller;
use super::fee_vaults;
use super::order_book;
use super::order_book_factory;
use super::token_validator;

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
    let amount_x = Decimal(I192::from(2).pow(152));
    let amount_y = Decimal(I192::from(2).pow(152));
    let floop_token = test_runner.create_freely_mintable_and_burnable_fungible_resource(OwnerRole::None, Some(dec!(1000)), DIVISIBILITY_MAXIMUM, admin_account_component);
    let token_x = test_runner.create_fungible_resource(amount_x, DIVISIBILITY_MAXIMUM, account_component);
    let token_y = test_runner.create_fungible_resource(amount_y, DIVISIBILITY_MAXIMUM, account_component);
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

    // Publish order book package
    let order_book_package = test_runner.compile_and_publish("../order_book");
    println!("order_book_package: {:?}", encoder.encode(order_book_package.to_vec().as_slice()));

    // Create order book
    let manifest = order_book::build_manifest(
        order_book_package, 
        rule!(require(admin_badge)),
        AccessRule::AllowAll,
        token_x, 
        token_y);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let order_book_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let order_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    // Publish token validator package
    let token_validator_package = test_runner.compile_and_publish("../token_validator");
    println!("token_validator_package: {:?}", encoder.encode(token_validator_package.to_vec().as_slice()));

    // Create token validator
    let manifest = token_validator::build_manifest(
        token_validator_package, 
        admin_badge);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

    // Publish order book factory package
    let order_book_factory_package = test_runner.compile_and_publish("../order_book_factory");
    println!("order_book_factory_package: {:?}", encoder.encode(order_book_factory_package.to_vec().as_slice()));

    // Create order book factory
    let manifest = order_book_factory::build_manifest(
        order_book_factory_package, 
        admin_badge,
        token_validator_component);
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let order_book_factory_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

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
        order_book_package,
        order_book_component,
        order_book_factory_package,
        order_book_factory_component,
        token_validator_package,
        token_validator_component,
        admin_badge,
        floop_token,
        token_x,
        amount_x,
        token_y,
        amount_y,
        order_receipt,
    }
}
