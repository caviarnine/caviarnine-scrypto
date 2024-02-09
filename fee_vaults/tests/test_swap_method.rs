#![allow(dead_code)]
use radix_engine::errors::ApplicationError::PanicMessage;
use radix_engine::errors::RuntimeError::ApplicationError;
use scrypto::prelude::*;

mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_swap_invalid_tokens_01() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage = 0
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.4"));

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // deposit liquidity
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // ACT
    let receipt =
        fee_vaults::swap_receipt(&mut vars, fee_vaults_component, token_a, dec!(10), token_a);

    // ASSERT
    receipt.expect_specific_failure(|err| match err {
        ApplicationError(PanicMessage(msg)) => msg.contains("Invalid tokens for swapping."),
        _ => false,
    });
}

#[test]
fn test_swap_valid_tokens_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // set treasury percentage = 0
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.4"));

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit liquidity
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // ACT
    let receipt = fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(2),
        token_a,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_swap_invalid_amount_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // ACT
    let receipt = fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        swap_amount - dec!("0.00001"),
        token_a,
    );

    // ASSERT
    receipt.expect_specific_failure(|err| match err {
        ApplicationError(PanicMessage(msg)) => {
            println!("{}", msg);
            msg.contains("Not enough tokens.")
        }
        _ => false,
    });
}

#[test]
fn test_swap_valid_tokens_02() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // set treasury percentage = 0
    let percentage_for_swap_vaults = dec!("0.6");
    fee_vaults::set_treasury_percentage(
        &mut vars,
        fee_vaults_component,
        dec!(1) - percentage_for_swap_vaults,
    );

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // get epoch for resource token_a
    let epoch = fee_vaults::get_last_swapped_epoch(&mut vars, fee_vaults_component, token_a);

    // ASSERT
    assert_eq!(epoch, current_epoch);

    // move epoch forward
    // vars.test_runner.set_current_epoch(Epoch::of(10));

    // get swap vault amounts for token_a
    let swap_vault_amount_a_after_deposit =
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, token_a);

    // ASSERT
    assert_eq!(
        swap_vault_amount_a_after_deposit,
        dec!(50) * percentage_for_swap_vaults
    );

    // get start balances for the wallet
    let start_balance_floop = vars.test_runner.get_component_balance(
        vars.account_component_address,
        token_floop_new_resource_address,
    );

    let start_balance_a = vars
        .test_runner
        .get_component_balance(vars.account_component_address, token_a);

    let start_balance_b = vars
        .test_runner
        .get_component_balance(vars.account_component_address, token_b);

    // ACT
    fee_vaults::swap(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(2),
        token_a,
    );

    // get epoch for resource token_a
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, fee_vaults_component, token_a);

    // ASSERT epoch
    // assert_eq!(last_swapped_epoch, Epoch::of(10));
    assert_eq!(last_swapped_epoch, current_epoch);

    // get end balances
    let end_balance_floop = vars.test_runner.get_component_balance(
        vars.account_component_address,
        token_floop_new_resource_address,
    );

    let end_balance_a = vars
        .test_runner
        .get_component_balance(vars.account_component_address, token_a);

    let end_balance_b = vars
        .test_runner
        .get_component_balance(vars.account_component_address, token_b);

    let swap_vault_amount_a =
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, token_a);
    let swap_vault_amount_b =
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, token_b);

    // ASSERT
    assert_eq!(start_balance_floop - swap_amount, end_balance_floop);
    assert_eq!(
        start_balance_a + dec!(50) * percentage_for_swap_vaults,
        end_balance_a
    );
    assert_eq!(start_balance_b, end_balance_b);
    assert_eq!(swap_vault_amount_a, dec!(0));
    assert_eq!(
        swap_vault_amount_b,
        dec!(500) * percentage_for_swap_vaults
    );
}

#[test]
fn test_swap_vault_not_found_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_c = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // ACT
    let receipt = fee_vaults::swap_receipt(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(2),
        token_c,
    );

    // ASSERT
    receipt.expect_specific_failure(|err| match err {
        ApplicationError(PanicMessage(msg)) => msg.contains("Swap vault not found."),
        _ => false,
    });
}

#[test]
fn test_swap_vault_epoch_update_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // get current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // create 2 resources and deposit some of each of them
    // use new floop resource
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // new floop resource
    let token_floop_new_resource_address = vars.token_floop_new_resource_address;

    // deposit tokens into the fee_vaults
    fee_vaults::deposit_batch(
        &mut vars,
        fee_vaults_component,
        vec![(token_a, dec!(50)), (token_b, dec!(500))],
    );

    // move epoch forward
    vars.test_runner.set_current_epoch(Epoch::of(10));

    // ACT
    fee_vaults::swap(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        dec!(2),
        token_b,
    );

    // get epoch for resource token_a
    let last_swapped_epoch_a =
        fee_vaults::get_last_swapped_epoch(&mut vars, fee_vaults_component, token_a);

    // get epoch for resource token_a
    let last_swapped_epoch_b =
        fee_vaults::get_last_swapped_epoch(&mut vars, fee_vaults_component, token_b);

    // ASSERT
    assert_eq!(last_swapped_epoch_a, current_epoch);
    assert_eq!(last_swapped_epoch_b, Epoch::of(10));
}

#[test]
fn test_swap_reserve() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);
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
    fee_vaults::deposit(&mut vars, fee_vaults_component, token_a, dec!(50));

    // ACT
    fee_vaults::swap(
        &mut vars,
        fee_vaults_component,
        token_floop_new_resource_address,
        swap_amount,
        token_a,
    );
    let reserve_amount = fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component);

    // ASSERT
    assert_eq!(reserve_amount, swap_amount);
}
