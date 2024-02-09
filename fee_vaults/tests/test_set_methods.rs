use scrypto::prelude::*;

mod common;
pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_get_treasury_percentage_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let receipt = fee_vaults::set_method_with_decimal_input_receipt(
        &mut vars,
        fee_vaults_component,
        "set_treasury_percentage",
        false,
        dec!("0.1"),
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_get_treasury_percentage_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let receipt = fee_vaults::set_method_with_decimal_input_receipt(
        &mut vars,
        fee_vaults_component,
        "set_treasury_percentage",
        true,
        dec!("0.1"),
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_get_treasury_percentage_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.1"));
    let treasury_percentage = fee_vaults::get_treasury_percentage(&mut vars, fee_vaults_component);

    // ASSERT
    assert_eq!(treasury_percentage, dec!("0.1"));
}

#[test]
#[should_panic]
fn test_get_treasury_percentage_boundary_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT - ASSERT
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("1.00001"));
}

#[test]
fn test_get_treasury_percentage_boundary_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT - ASSERT
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.999999"));
}

#[test]
#[should_panic]
fn test_get_treasury_percentage_boundary_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT - ASSERT
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("-0.000001"));
}

#[test]
fn test_get_treasury_percentage_boundary_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT - ASSERT
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.0000001"));
}

#[test]
fn test_set_swap_amount_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let swap_vault_amount = dec!("0.5");

    // ACT
    let receipt = fee_vaults::set_method_with_decimal_input_receipt(
        &mut vars,
        fee_vaults_component,
        "set_swap_amount",
        false,
        swap_vault_amount,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_swap_amount_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let swap_vault_amount = dec!("0.5");

    // ACT
    fee_vaults::set_swap_amount(&mut vars, fee_vaults_component, swap_vault_amount);
    let swap_amount = fee_vaults::get_swap_amount(&mut vars, fee_vaults_component);

    // ASSERT
    assert_eq!(swap_amount, swap_vault_amount);
}

#[test]
#[should_panic]
fn test_set_swap_amount_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let swap_vault_amount = dec!("-0.5");

    // ACT - ASSERT
    fee_vaults::set_swap_amount(&mut vars, fee_vaults_component, swap_vault_amount);
}

#[test]
fn test_set_max_epochs_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let max_epochs = 1000u64;

    // ACT
    let receipt = fee_vaults::set_max_epochs_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        false,
        max_epochs,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_max_epochs_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let max_epochs = 1000u64;

    // ACT
    let receipt = fee_vaults::set_max_epochs_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        max_epochs,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_max_epochs_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let update_max_epochs = 1000u64;

    // ACT
    fee_vaults::set_max_epochs(&mut vars, fee_vaults_component, update_max_epochs);
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, fee_vaults_component);

    // ASSERT
    assert_eq!(update_max_epochs, max_epochs);
}
