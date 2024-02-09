use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

mod common;
pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_reserve_withdraw_zero() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::set_burn_percentage(&mut vars, fee_vaults_component, dec!(0));
    fee_vaults::reserve_deposit(&mut vars, fee_vaults_component, dec!(1));
    let reserve_amount0 = fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component);

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        dec!(0),
    );
    
    // ASSERT
    let reserve_amount1 = fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component);
    receipt.expect_commit_success();
    assert_eq!(reserve_amount0, reserve_amount1);
}

#[test]
fn test_reserve_withdraw_less() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::reserve_deposit(&mut vars, fee_vaults_component, dec!(2));

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        dec!(1),
    );
    
    // ASSERT
    let reserve_amount1 = fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component);
    receipt.expect_commit_success();
    assert_eq!(reserve_amount1, dec!(1));
}

#[test]
fn test_reserve_withdraw_more() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::reserve_deposit(&mut vars, fee_vaults_component, dec!(1));

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        dec!(2),
    );
    
    // ASSERT
    let reserve_amount1 = fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component);
    receipt.expect_commit_success();
    assert_eq!(reserve_amount1, dec!(0));
}

#[test]
fn test_reserve_withdraw_no_proof_invalid() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        false,
        swap_amount,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_reserve_withdraw_negative_invalid() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        dec!("-0.1"),
    );

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Amount must be positive.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_treasury_withdraw() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // Deposit tokens
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    fee_vaults::treasury_deposit(&mut vars, fee_vaults_component, resource_address, dec!(1));

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        resource_address,
    );

    // ASSERT
    let treasury_amount = fee_vaults::get_treasury_vault_amount(&mut vars, fee_vaults_component, resource_address);
    receipt.expect_commit_success();
    assert_eq!(treasury_amount, dec!(0));
}

#[test]
fn test_treasury_withdraw_twice() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // Deposit tokens
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    fee_vaults::treasury_deposit(&mut vars, fee_vaults_component, resource_address, dec!(1));

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        resource_address,
    );

    // ASSERT
    receipt.expect_commit_success();

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        resource_address,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_treasury_withdraw_no_proof_invalid() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        false,
        resource_address,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_treasury_withdraw_no_vault_invalid() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        resource_address,
    );

    // ASSERT
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                println!("msg: {}", msg);
                msg.contains("Vault not found.")
            },
            _ => false,
        }
    });
}

