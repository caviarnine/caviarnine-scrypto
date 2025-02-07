use scrypto::prelude::*;

mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_validator_basic_01() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    // set up a validator + stake to it and get the lsu tokens
    let lsu_resource_address_01 = validator::create_lsu_resource(&mut vars, dec!(100));
    let lsu_resource_address_02 = validator::create_lsu_resource(&mut vars, dec!(100));

    // ASSERT
    assert_ne!(lsu_resource_address_01, lsu_resource_address_02);
}

#[test]
fn test_is_lsu_token_true() {
    // ARRANGE
    let mut vars = setup();
    let lsu_resource_address_01 = validator::create_lsu_resource(&mut vars, dec!(100));

    // ASSERT
    assert!(lsu_pool::is_lsu_token(lsu_resource_address_01, &mut vars));
}

#[test]
fn test_is_lsu_token_fake_simple_false() {
    // ARRANGE
    let mut vars = setup();
    let fake_lsu = token(&mut vars, dec!(100));

    // ASSERT
    assert!(!lsu_pool::is_lsu_token(fake_lsu, &mut vars));
}

#[test]
fn test_is_lsu_token_fake_complex_false() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    let fake_lsu =
        validator::fake_lsu_with_meta_data(&mut vars, dec!(100), validator_component_address);

    // ASSERT
    assert!(!lsu_pool::is_lsu_token(fake_lsu, &mut vars));
}

#[test]
fn test_is_validator_01() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    // ASSERT
    assert!(lsu_pool::is_validator(
        validator_component_address,
        &mut vars
    ));
}

#[test]
fn test_is_not_a_validator_01() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let is_not_validator = lsu_pool::is_validator(vars.lsu_pool_component_address, &mut vars);

    // ASSERT
    assert!(!is_not_validator);
}

#[test]
fn test_get_validator_address_some() {
    // ARRANGE
    let mut vars = setup();

    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    // register validator
    validator::validator_update_accept_delegated_stake(
        &mut vars,
        validator_component_address,
        true,
    );

    // stake validator
    let lsu_resource_address_01 =
        validator::stake_validator(&mut vars, validator_component_address, dec!(100));

    // ACT
    let temp = lsu_pool::get_validator_address(lsu_resource_address_01, &mut vars);

    // ASSERT
    assert_eq!(temp, Some(validator_component_address));
}

#[test]
fn test_get_validator_address_none() {
    // ARRANGE
    let mut vars = setup();

    // fake lsu
    let fake_lsu = token(&mut vars, dec!(100));

    // ACT
    let temp = lsu_pool::get_validator_address(fake_lsu, &mut vars);

    // ASSERT
    assert_eq!(temp, None);
}

#[test]
fn test_get_validator_price_lsu_xrd_some() {
    // ARRANGE
    let mut vars = setup();

    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    // register validator
    validator::validator_update_accept_delegated_stake(
        &mut vars,
        validator_component_address,
        true,
    );

    // stake validator
    let lsu_resource_address_01 =
        validator::stake_validator(&mut vars, validator_component_address, dec!(100));

    // ACT
    let price_lsu_xrd = lsu_pool::get_validator_price_lsu_xrd(lsu_resource_address_01, &mut vars);

    // ASSERT
    assert_eq!(price_lsu_xrd, Some(dec!(1)));
}

#[test]
fn test_get_validator_price_lsu_xrd_none() {
    // ARRANGE
    let mut vars = setup();

    // fake lsu
    let fake_lsu = token(&mut vars, dec!(100));

    // ACT
    let temp = lsu_pool::get_validator_price_lsu_xrd(fake_lsu, &mut vars);

    // ASSERT
    assert_eq!(temp, None);
}

#[test]
#[ignore = "getting price isn't working"]
fn test_get_validator_price_lsu_xrd_change_price_through_emissions() {
    // ARRANGE
    let mut vars = setup();

    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    // register validator
    validator::validator_update_accept_delegated_stake(
        &mut vars,
        validator_component_address,
        true,
    );

    // stake validator
    let lsu_resource_address_01 =
        validator::stake_validator(&mut vars, validator_component_address, dec!(100));

    let current_epoch = vars.test_runner.get_current_epoch();
    println!("current_epoch: {:?}", current_epoch);

    vars.test_runner.set_current_epoch(Epoch::of(3));

    // ACT
    let price_lsu_xrd = lsu_pool::get_validator_price_lsu_xrd(lsu_resource_address_01, &mut vars);

    // ASSERT
    assert_eq!(price_lsu_xrd, Some(dec!(1)));
}
