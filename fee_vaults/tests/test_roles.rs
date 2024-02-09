#![allow(dead_code)]
use scrypto::prelude::*;

mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_set_owner_role_rule_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let receipt = fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::AllowAll,
        false,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_owner_role_rule_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let receipt = fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::AllowAll,
        true,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT 1
    let receipt = fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::DenyAll,
        true,
    );

    // ASSERT 1
    receipt.expect_commit_success();
}

#[test]
fn test_set_owner_role_rule_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::AllowAll,
        true,
    )
    .expect_commit_success();

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
fn test_set_owner_role_rule_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::DenyAll,
        true,
    )
    .expect_commit_success();

    // ACT
    let receipt = fee_vaults::set_method_with_decimal_input_receipt(
        &mut vars,
        fee_vaults_component,
        "set_treasury_percentage",
        true,
        dec!("0.1"),
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_set_owner_role_rule_06() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT 1
    fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::DenyAll,
        true,
    )
    .expect_commit_success();

    // ASSERT 1
    fee_vaults::set_method_with_decimal_input_receipt(
        &mut vars,
        fee_vaults_component,
        "set_treasury_percentage",
        true,
        dec!("0.1"),
    )
    .expect_auth_failure();

    // ASSERT 2
    fee_vaults::set_owner_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        AccessRule::AllowAll,
        true,
    )
    .expect_auth_failure();
}

#[test]
fn test_update_reserve_manager_role_rule_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        dec!("0.1"),
    );
    let role = "reserve_manager".to_string();

    // ACT
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::AllowAll,
        false,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_update_reserve_manager_role_rule_02() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::set_burn_percentage(&mut vars, fee_vaults_component, dec!(0));

    // create tokens
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit(
        &mut vars,
        fee_vaults_component,
        token_a, 
        dec!(50),
    );
    fee_vaults::swap(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        swap_amount,
        token_a,
    );
    // ACT
    let role = "reserve_manager".to_string();
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::DenyAll,
        true,
    );

    // ASSERT
    receipt.expect_commit_success();

    // ACT
    let receipt = fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        swap_amount,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_reserve_manager_role_rule_03() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::set_burn_percentage(&mut vars, fee_vaults_component, dec!(0));

    // create tokens
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit(
        &mut vars,
        fee_vaults_component,
        token_a, 
        dec!(50),
    );
    fee_vaults::swap(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        swap_amount,
        token_a,
    );

    // ACT
    fee_vaults::reserve_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        swap_amount,
    );

    // ASSERT
    let amount = vars.test_runner.get_component_balance(vars.admin_account_component_address, token_floop_new_resource_address);

    assert_eq!(
        amount,
        swap_amount
    );
}

#[test]
fn test_update_treasury_manager_role_rule_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        dec!("0.1"),
    );
    let role = "treasury_manager".to_string();

    // ACT
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::AllowAll,
        false,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_update_treasury_manager_role_rule_02() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!(1));

    // create tokens
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit tokens into the fee_vaults
    fee_vaults::deposit(
        &mut vars,
        fee_vaults_component,
        token_a, 
        dec!(50),
    );

    // ACT
    let role = "treasury_manager".to_string();
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::DenyAll,
        true,
    );

    // ASSERT
    receipt.expect_commit_success();

    // ACT
    let receipt = fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        token_a,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_treasury_manager_role_rule_03() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(
        &mut vars,
        swap_amount,
    );
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!(1));

    // create tokens
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit tokens into the fee_vaults
    fee_vaults::deposit(
        &mut vars,
        fee_vaults_component,
        token_a, 
        dec!(50),
    );

    // ACT
    fee_vaults::treasury_withdraw_with_proof_receipt(
        &mut vars,
        fee_vaults_component,
        true,
        token_a,
    );

    // ASSERT
    let amount = vars.test_runner.get_component_balance(vars.admin_account_component_address, token_a);

    assert_eq!(
        amount,
        dec!(50)
    );
}

#[test]
fn test_update_user_role_rule_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let role = "user".to_string();

    // ACT
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::AllowAll,
        false,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_update_user_role_rule_02() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let role = "user".to_string();

    // ACT
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::AllowAll,
        true,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_update_user_role_rule_03() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let role = "user".to_string();

    // ACT
    let receipt = fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::DenyAll,
        true,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_update_user_role_rule_04() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let role = "user".to_string();
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // switch on access to user to swap
    fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::AllowAll,
        true,
    )
    .expect_commit_success();

    // set up swapping
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit liquidity
    fee_vaults::deposit(&mut vars, fee_vaults_component, token_a, dec!(100));

    // ACT
    let receipt = fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(1),
        token_a,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_update_user_role_rule_05() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let role = "user".to_string();
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // switch off access to user to swap
    fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        role,
        AccessRule::DenyAll,
        true,
    )
    .expect_commit_success();

    // set up swapping
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit liquidity
    fee_vaults::deposit(&mut vars, fee_vaults_component, token_a, dec!(100));

    // ACT
    let receipt = fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(1),
        token_a,
    );

    // ASSERT
    receipt.expect_auth_failure();
}

#[test]
fn test_update_user_role_rule_06() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // switch off access to user to swap
    fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        "user".to_string(),
        AccessRule::DenyAll,
        true,
    )
    .expect_commit_success();

    // set up swapping
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit liquidity
    fee_vaults::deposit(&mut vars, fee_vaults_component, token_a, dec!(100));

    // ACT
    fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(1),
        token_a,
    )
    .expect_auth_failure();

    // switch on access to user to swap
    fee_vaults::update_role_rule_receipt(
        &mut vars,
        fee_vaults_component,
        "user".to_string(),
        AccessRule::AllowAll,
        true,
    )
    .expect_commit_success();

    // ASSERT
    fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(1),
        token_a,
    )
    .expect_commit_success();
}
