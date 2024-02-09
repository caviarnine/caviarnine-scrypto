use scrypto::prelude::*;
mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_get_burn_percentage_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let burn_percentage =
        fee_vaults::get_burn_percentage(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(burn_percentage, Decimal::ONE);
}

#[test]
fn test_get_treasury_percentage_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let treasury_percentage =
        fee_vaults::get_treasury_percentage(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(treasury_percentage, Decimal::ZERO);
}

#[test]
fn test_get_swap_amount_01() {
    // ARRANGE
    let mut vars = setup();
    let initialize_swap_amount = dec!("0.1");
    let new_fee_vaults_component =
        fee_vaults::new_fee_vaults_manifest(&mut vars, initialize_swap_amount);

    // ACT
    let swap_amount = fee_vaults::get_swap_amount(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(initialize_swap_amount, swap_amount);
}

#[test]
fn test_get_swap_amount_02() {
    // ARRANGE
    let mut vars = setup();
    let initialize_swap_amount = dec!("0.5");
    let new_fee_vaults_component =
        fee_vaults::new_fee_vaults_manifest(&mut vars, initialize_swap_amount);

    // ACT
    let swap_amount = fee_vaults::get_swap_amount(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(initialize_swap_amount, swap_amount);
}

#[test]
fn test_get_max_epochs_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let max_epochs = fee_vaults::get_max_epochs(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(max_epochs, 10000u64);
}

#[test]
#[should_panic]
fn test_get_swap_vault_amount_panic() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT - ASSERT
    let _swap_vault_amount =
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);
}

#[test]
fn test_get_swap_vault_amount() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, new_fee_vaults_component, resource_address, dec!(100));
    let swap_vault_amount = fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(swap_vault_amount, dec!(100));
}

#[test]
fn test_get_reserve_amount() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    let reserve_amount = fee_vaults::get_reserve_amount(&mut vars, new_fee_vaults_component);

    // ASSERT
    assert_eq!(reserve_amount, Decimal::ZERO);
}

#[test]
#[should_panic]
fn test_get_treasury_vault_amount_panic() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT - ASSERT
    let _treasury_vault_amount = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );
}

#[test]
fn test_get_treasury_vault_amount() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, new_fee_vaults_component, resource_address, dec!(100));
    let treasury_vault_amount = fee_vaults::get_treasury_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(treasury_vault_amount, dec!(100));
}

#[test]
#[should_panic]
fn test_get_swap_price_panic() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT - ASSERT
    let _swap_price =
        fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);
}

#[test]
fn test_get_swap_price() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, new_fee_vaults_component, resource_address, dec!(100));
    let swap_price = fee_vaults::get_swap_price(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(swap_price, dec!("0.1"));
}

#[test]
#[should_panic]
fn test_get_last_swapped_epoch_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT - ASSERT
    let _last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);
}

#[test]
fn test_get_last_swapped_epoch_02() {
        // ARRANGE
        let mut vars = setup();
        let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    
        // create resource
        let resource_address = vars.test_runner.create_fungible_resource(
            dec!(1000),
            DIVISIBILITY_MAXIMUM,
            vars.account_component_address,
        );
    
        // ACT
        fee_vaults::deposit(&mut vars, new_fee_vaults_component, resource_address, dec!(100));
        let last_swapped_epoch = fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);
    
        // ASSERT
        assert_eq!(last_swapped_epoch, vars.test_runner.get_current_epoch());
}
