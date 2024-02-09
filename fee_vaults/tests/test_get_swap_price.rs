use scrypto::prelude::*;
mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_get_swap_price_zero_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // get the max epochs
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // deposit some tokens
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(last_swapped_epoch, current_epoch);

    // move epoch forward
    vars.test_runner
        .set_current_epoch(Epoch::of(current_epoch.number() + max_epochs + 1u64));

    // get epoch after shited
    let current_epoch_after_shift = vars.test_runner.get_current_epoch();

    // ACT
    let swap_price =
        fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);

    // println!("current_epoch: {:?}", current_epoch);
    // println!("last_swapped_epoch: {:?}", last_swapped_epoch);
    // println!("current_epoch_after_shift: {:?}", current_epoch_after_shift);
    // println!("max_epochs: {}", max_epochs);
    // println!("swap_price: {}", swap_price);

    // ASSERT
    assert!(current_epoch_after_shift.number() > current_epoch.number() + max_epochs);
    assert_eq!(swap_price, Decimal::ZERO);
}

#[test]
fn test_get_swap_price_max_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // get the max epochs
    // let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // deposit some tokens
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // ACT
    let swap_price =
        fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(last_swapped_epoch, current_epoch);
    assert_eq!(swap_price, swap_amount);
}

#[test]
fn test_get_swap_price_between_01() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get the max epochs
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // deposit some tokens
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // define a new Epoch and move epoch forward
    let new_epoch = Epoch::of(current_epoch.number() + 10u64);
    vars.test_runner.set_current_epoch(new_epoch);

    // ACT
    let swap_price =
        fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);

    let epoch_diff = Decimal::try_from(new_epoch.number() - last_swapped_epoch.number()).unwrap();
    let max_epochs = Decimal::try_from(max_epochs).unwrap();
    let ratio = Decimal::ONE - (epoch_diff / max_epochs);

    println!("ratio: {}", ratio);

    // ASSERT
    assert_eq!(last_swapped_epoch, current_epoch);
    assert_eq!(swap_price, swap_amount * ratio);
}

#[test]
fn test_get_swap_price_between_02() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get the max epochs
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // deposit some tokens
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // define a new Epoch and move epoch forward
    let new_epoch = Epoch::of(current_epoch.number() + 50u64);
    vars.test_runner.set_current_epoch(new_epoch);

    // ACT
    let swap_price = fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);
    
    let epoch_diff = Decimal::try_from(new_epoch.number() - last_swapped_epoch.number()).unwrap();
    let max_epochs = Decimal::try_from(max_epochs).unwrap();
    let ratio = Decimal::ONE - (epoch_diff / max_epochs);

    println!("ratio: {}", ratio);

    // ASSERT
    assert_eq!(last_swapped_epoch, current_epoch);
    assert_eq!(swap_price, swap_amount * ratio);
}

#[test]
fn test_get_swap_price_between_03() {
    // ARRANGE
    let mut vars = setup();
    let swap_amount = dec!("0.1");
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, swap_amount);

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get the max epochs
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // get the current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // deposit some tokens
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // define a new Epoch and move epoch forward
    let new_epoch = Epoch::of(current_epoch.number() + 350u64);
    vars.test_runner.set_current_epoch(new_epoch);

    // ACT
    let swap_price =
        fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);

    let epoch_diff = Decimal::try_from(new_epoch.number() - last_swapped_epoch.number()).unwrap();
    let max_epochs = Decimal::try_from(max_epochs).unwrap();
    let ratio = Decimal::ONE - (epoch_diff / max_epochs);

    println!("ratio: {}", ratio);

    // ASSERT
    assert_eq!(last_swapped_epoch, current_epoch);
    assert_eq!(swap_price, swap_amount * ratio);
}
