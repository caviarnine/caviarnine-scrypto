use scrypto::prelude::*;

mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn get_validator_counter() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let validator_counter = lsu_pool::get_validator_counter(&mut vars);

    // ASSERT
    assert_eq!(validator_counter, 0);
}

#[test]
fn get_validator_pointer() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let validator_pointer = lsu_pool::get_validator_pointer(&mut vars);

    // ASSERT
    assert_eq!(validator_pointer, 0);
}

#[test]
fn get_validator_address_map_one() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT
    let resource = lsu_pool::get_validator_address_map(0, &mut vars);

    // ASSERT
    assert_eq!(resource, lsu01_resource);
}

#[test]
fn get_validator_address_map_two() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu02_resource, dec!(10), &mut vars);

    // ACT
    let resource = lsu_pool::get_validator_address_map(1, &mut vars);

    // ASSERT
    assert_eq!(resource, lsu02_resource);
    assert_eq!(lsu_pool::get_validator_counter(&mut vars), 2);
}

#[test]
fn get_validator_address_test_from_swap() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(100), &mut vars);

    // ACT
    lsu_pool::swap(lsu02_resource, dec!(10), lsu01_resource, &mut vars);

    // ASSERT
    assert_eq!(
        lsu_pool::get_validator_address_map(0, &mut vars),
        lsu01_resource
    );
    assert_eq!(
        lsu_pool::get_validator_address_map(1, &mut vars),
        lsu02_resource
    );
    assert_eq!(lsu_pool::get_validator_counter(&mut vars), 2);
}

#[test]
fn update_multiple_validator_prices_success_01() {
    // ARRANGE
    let mut vars = setup();

    // add liquidity in lots of tokens
    for _ in 0..1 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    }

    // ACT - ASSERT
    lsu_pool::update_multiple_validator_prices(3, &mut vars);
}

#[test]
fn update_multiple_validator_prices_success_10() {
    // ARRANGE
    let mut vars = setup();

    // add liquidity in lots of tokens
    for _ in 0..10 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    }

    // ACT - ASSERT
    lsu_pool::update_multiple_validator_prices(3, &mut vars);
}

#[test]
fn update_multiple_validator_prices_check_pointer_and_count() {
    // ARRANGE
    let mut vars = setup();

    // ACT - add liquidity in lots of tokens
    for _ in 0..10 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    }

    // ASSERT
    let count = lsu_pool::get_validator_counter(&mut vars);
    assert_eq!(count, 10);
    
    // ACT
    let pointer0 = lsu_pool::get_validator_pointer(&mut vars);
    lsu_pool::update_multiple_validator_prices(3, &mut vars);
    
    // ASSERT
    let pointer1 = lsu_pool::get_validator_pointer(&mut vars);
    let pointer1_expected = (pointer0 + 3) % count;
    assert_eq!(pointer1, pointer1_expected);
}

#[test]
fn update_multiple_validator_prices_check_pointer_and_count_multiple() {
    // ARRANGE
    let mut vars = setup();

    // add liquidity in lots of tokens
    for _ in 0..30 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(100));
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    }

    // ASSERT
    let count = lsu_pool::get_validator_counter(&mut vars);
    assert_eq!(count, 30);

    // ACT 1
    let pointer0 = lsu_pool::get_validator_pointer(&mut vars);
    lsu_pool::update_multiple_validator_prices(3, &mut vars);
    
    // ASSERT 1
    let pointer1 = lsu_pool::get_validator_pointer(&mut vars);
    let pointer1_expected = (pointer0 + 3) % count;
    assert_eq!(pointer1, pointer1_expected);

    // ACT 2
    let pointer0 = lsu_pool::get_validator_pointer(&mut vars);
    lsu_pool::update_multiple_validator_prices(12, &mut vars);
    
    // ASSERT 2
    let pointer1 = lsu_pool::get_validator_pointer(&mut vars);
    let pointer1_expected = (pointer0 + 12) % count;
    assert_eq!(pointer1, pointer1_expected);

    // ACT 3
    let pointer0 = lsu_pool::get_validator_pointer(&mut vars);
    lsu_pool::update_multiple_validator_prices(20, &mut vars);
    
    // ASSERT 3
    let pointer1 = lsu_pool::get_validator_pointer(&mut vars);
    let pointer1_expected = (pointer0 + 20) % count;
    assert_eq!(pointer1, pointer1_expected);
}

#[test]

fn update_multiple_validator_prices_tx_fees() {
    // ARRANGE
    let mut vars = setup();

    // add liquidity in lots of tokens
    for _ in 0..30 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(100));
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    }

    for i in [1, 5, 10, 20] {
        // ACT
        let fee_summary =
            lsu_pool::get_method_with_u32_receipt(i, "update_multiple_validator_prices", &mut vars)
                .fee_summary
                .clone();

        println!(
            "fee_summary for number updates: {}, Cost: {} XRD",
            i, fee_summary.total_execution_cost_in_xrd
        );
    }

    // ASSERT
}
