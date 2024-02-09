use scrypto::prelude::*;
mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_treasury_deposit_01() {
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
    let receipt = fee_vaults::general_deposit_receipt(
        &mut vars,
        new_fee_vaults_component,
        "treasury_deposit",
        resource_address,
        dec!(10),
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_treasury_deposit_02() {
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
    fee_vaults::treasury_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the treasury component
    let resource_amount = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount, dec!(10));
}

#[test]
fn test_treasury_deposit_03() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT 1
    fee_vaults::treasury_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // ACT 2
    fee_vaults::treasury_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(17),
    );

    // get the balance of the resource in the treasury component
    let resource_amount = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount, dec!(27));
}

#[test]
fn test_swap_vault_deposit_01() {
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
    let receipt = fee_vaults::general_deposit_receipt(
        &mut vars,
        new_fee_vaults_component,
        "swap_vault_deposit",
        resource_address,
        dec!(10),
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_swap_vault_deposit_02() {
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
    fee_vaults::swap_vault_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    let resource_amount = fee_vaults::get_swap_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount, dec!(10));
}

#[test]
fn test_swap_vault_deposit_03() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT 1
    fee_vaults::swap_vault_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // ACT 2
    fee_vaults::swap_vault_deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(17),
    );

    let resource_amount = fee_vaults::get_swap_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount, dec!(27));
}

#[test]
fn test_reserve_deposit() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // ACT
    fee_vaults::reserve_deposit(&mut vars, fee_vaults_component, dec!(1));
    
    // ASSERT
    assert_eq!(
        fee_vaults::get_reserve_amount(&mut vars, fee_vaults_component),
        dec!(1)
    );
}

#[test]
fn test_deposit_percentage_zero_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!(0));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let receipt = fee_vaults::general_deposit_receipt(
        &mut vars,
        new_fee_vaults_component,
        "deposit",
        resource_address,
        dec!(10),
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
#[should_panic]
fn test_deposit_percentage_zero_02() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!(0));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the treasury vault
    let _resource_amount_treasury = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );
}

#[test]
fn test_deposit_percentage_zero_03() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!(0));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get tge current epoch
    let current_epoch = vars.test_runner.get_current_epoch();

    // ACT
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the swap vault
    let resource_amount_swap =
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(resource_amount_swap, dec!(10));
    assert_eq!(last_swapped_epoch, current_epoch);
}

#[test]
fn test_deposit_percentage_zero_04() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!(0));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // get tge current epoch
    let current_epoch = vars.test_runner.get_current_epoch();
    println!("current_epoch: {:?}", current_epoch);

    // ACT 1
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // move epoch forward
    vars.test_runner.set_current_epoch(Epoch::of(10));

    // ACT 2
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the swap vault
    let resource_amount_swap =
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // get last swapped epoch
    let last_swapped_epoch =
        fee_vaults::get_last_swapped_epoch(&mut vars, new_fee_vaults_component, resource_address);

    // ASSERT
    assert_eq!(resource_amount_swap, dec!(20));
    assert_eq!(last_swapped_epoch, current_epoch);
}

#[test]
fn test_deposit_percentage_one_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!(1));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the swap vault
    let resource_amount_swap =
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // get resource amount in treasury vault
    let resource_amount_treasury = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount_swap, dec!(0));
    assert_eq!(resource_amount_treasury, dec!(10));
}

#[test]
fn test_deposit_percentage_nonzero_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("0.4"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get the balance of the resource in the swap vault
    let resource_amount_swap =
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, resource_address);

    // get resource amount in treasury vault
    let resource_amount_treasury = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // ASSERT
    assert_eq!(resource_amount_swap, dec!(6));
    assert_eq!(resource_amount_treasury, dec!(4));
}

#[test]
fn test_deposit_percentage_nonzero_03() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("0.4"));

    // use new floop resource
    let resource_address = vars.token_floop_new_resource_address;

    // get start balance
    let start_balance = vars.test_runner.get_component_balance(vars.account_component_address, resource_address);
    
    // ACT
    fee_vaults::deposit(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
        dec!(10),
    );

    // get resource amount in treasury vault
    let resource_amount_treasury = fee_vaults::get_treasury_vault_amount(
        &mut vars,
        new_fee_vaults_component,
        resource_address,
    );

    // get end balance
    let end_balance = vars.test_runner.get_component_balance(vars.account_component_address, resource_address);

    // ASSERT
    assert_eq!(start_balance, dec!(1000));
    assert_eq!(end_balance, dec!(990));
    assert_eq!(resource_amount_treasury, dec!(4));
}

#[test]
fn test_deposit_batch_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("0.4"));

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

    // ACT
    let receipt = fee_vaults::deposit_batch_reciept(
        &mut vars,
        new_fee_vaults_component,
        vec![
            (token_a, dec!(100)),
            (token_b, dec!(100)),
            (token_c, dec!(100)),
        ],
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_deposit_batch_02() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("0.4"));

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

    // ACT
    fee_vaults::deposit_batch(
        &mut vars,
        new_fee_vaults_component,
        vec![
            (token_a, dec!(10)),
            (token_b, dec!(100)),
            (token_c, dec!(200)),
        ],
    );

    // ASSERT
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, token_a),
        dec!(6)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, new_fee_vaults_component, token_a),
        dec!(4)
    );
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, token_b),
        dec!(60)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, new_fee_vaults_component, token_b),
        dec!(40)
    );
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, new_fee_vaults_component, token_c),
        dec!(120)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, new_fee_vaults_component, token_c),
        dec!(80)
    );
}

#[test]
fn test_deposit_divisibility_0() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.33333"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_NONE,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, fee_vaults_component, resource_address, dec!(10));

    // ASSERT
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(7)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(3)
    );
}

#[test]
fn test_deposit_divisibility_1() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.33333"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        1,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, fee_vaults_component, resource_address, dec!(10));

    // ASSERT
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(6.7)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(3.3)
    );
}

#[test]
fn test_deposit_divisibility_2() {
    // ARRANGE
    let mut vars = setup();
    let fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));
    fee_vaults::set_treasury_percentage(&mut vars, fee_vaults_component, dec!("0.33333"));

    // create resource
    let resource_address = vars.test_runner.create_fungible_resource(
        dec!(1000),
        2,
        vars.account_component_address,
    );

    // ACT
    fee_vaults::deposit(&mut vars, fee_vaults_component, resource_address, dec!(10));

    // ASSERT
    assert_eq!(
        fee_vaults::get_swap_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(6.67)
    );
    assert_eq!(
        fee_vaults::get_treasury_vault_amount(&mut vars, fee_vaults_component, resource_address),
        dec!(3.33)
    );
}
