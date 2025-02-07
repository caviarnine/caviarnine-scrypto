use scrypto::prelude::*;

mod common;

pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::token_bridge;
pub use crate::common::vars::Vars;

#[test]
fn test_get_new_token_address_success() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let receipt = token_bridge::get_method_with_no_input_receipt(
        token_bridge_component,
        "get_new_token_address",
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_get_new_token_address_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let new_token_address = token_bridge::get_new_token_address(token_bridge_component, &mut vars);

    // ASSERT
    assert_ne!(old_resource_address, new_token_address);
    assert_ne!(old_resource_address, vars.token_caviar_old);
}

#[test]
fn test_get_old_token_address_success() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let receipt = token_bridge::get_method_with_no_input_receipt(
        token_bridge_component,
        "get_old_token_address",
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_get_old_token_address_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let old_token_address = token_bridge::get_old_token_address(token_bridge_component, &mut vars);

    // ASSERT
    assert_eq!(old_resource_address, old_token_address);
    assert_ne!(old_resource_address, vars.token_caviar_old);
}

#[test]
fn test_old_tokens_vault_amount_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let old_tokens_vault_amount =
        token_bridge::get_old_tokens_amount(token_bridge_component, &mut vars);

    // ASSERT
    assert_eq!(dec!(0), old_tokens_vault_amount);
}

#[test]
fn test_bridge_tokens_vault_amount_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon FLOOP".to_string(),
        "FLOOP".to_string(),
        "FLOOP the only token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let bridge_tokens_vault_amount =
        token_bridge::get_new_tokens_amount(token_bridge_component, &mut vars);

    // ASSERT
    assert_eq!(dec!(1000), bridge_tokens_vault_amount);
}

#[test]
fn test_bridge_tokens_vault_amount_02() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_caviar_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let bridge_tokens_vault_amount =
        token_bridge::get_new_tokens_amount(token_bridge_component, &mut vars);

    // ASSERT
    assert_eq!(dec!(1000000000), bridge_tokens_vault_amount);
}

#[test]
fn test_bridge_success_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_caviar_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let receipt = token_bridge::bridge_receipt(
        token_bridge_component,
        old_resource_address,
        dec!(17),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_bridge_failure_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_caviar_old;
    let wrong_resource_address = vars.token_floop_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // ACT
    let receipt = token_bridge::bridge_receipt(
        token_bridge_component,
        wrong_resource_address,
        dec!(17),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_failure();
}

#[test]
fn test_bridge_balances_01() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_caviar_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // get bridge token resource address
    let new_token_address = token_bridge::get_new_token_address(token_bridge_component, &mut vars);

    // get wallet balance amounts before
    let balance_amounts_before: HashMap<ResourceAddress, Decimal> = vars
        .test_runner
        .get_component_resources(vars.account_component_address)
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();

    // get vault balance amounts before
    let bridge_tokens_vault_amount_before =
        token_bridge::get_new_tokens_amount(token_bridge_component, &mut vars);
    let old_tokens_vault_amount_before =
        token_bridge::get_old_tokens_amount(token_bridge_component, &mut vars);

    // ACT
    let amount = dec!(17);
    token_bridge::bridge(
        token_bridge_component,
        old_resource_address,
        amount,
        &mut vars,
    );

    // get wallet balance amounts after
    let balance_amounts_after: HashMap<ResourceAddress, Decimal> = vars
        .test_runner
        .get_component_resources(vars.account_component_address)
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();
    // get vault balance amounts after
    let bridge_tokens_vault_amount_after =
        token_bridge::get_new_tokens_amount(token_bridge_component, &mut vars);
    let old_tokens_vault_amount_after =
        token_bridge::get_old_tokens_amount(token_bridge_component, &mut vars);

    // ASSERT - wallet old_resource_address balances have changed by amount
    assert_eq!(
        *balance_amounts_before.get(&old_resource_address).unwrap_or(&Decimal::ZERO) - amount,
        *balance_amounts_after.get(&old_resource_address).unwrap_or(&Decimal::ZERO),
    );

    // ASSERT - wallet new_token_address balances have changed by amount
    assert_eq!(
        *balance_amounts_before.get(&new_token_address).unwrap_or(&Decimal::ZERO) + amount,
        *balance_amounts_after.get(&new_token_address).unwrap_or(&Decimal::ZERO),
    );

    // ASSERT - vault old_resource_address balances have changed by amount
    assert_eq!(
        old_tokens_vault_amount_before + amount,
        old_tokens_vault_amount_after
    );

    // ASSERT - vault new_token_address balances have changed by amount
    assert_eq!(
        bridge_tokens_vault_amount_before - amount,
        bridge_tokens_vault_amount_after
    );
}

#[test]
fn test_bridged_tokens_burnable() {
    // ARRANGE
    let mut vars = setup();
    let old_resource_address = vars.token_caviar_old;
    let token_bridge_component = token_bridge::new_token_bridge_manifest(
        vars.admin_badge_resource_address,
        old_resource_address,
        "Babylon CAVIAR".to_string(),
        "CAVIAR".to_string(),
        "CAVIAR the only other token you need!".to_string(),
        &mut vars,
    );

    // bridge tokens
    token_bridge::bridge(
        token_bridge_component,
        old_resource_address,
        dec!(17),
        &mut vars,
    );

    // get bridge token resource address
    let new_token_address = token_bridge::get_new_token_address(token_bridge_component, &mut vars);

    // ACT
    let receipt = token_bridge::burn_token(new_token_address, dec!(5), &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}
