// use ::lsu_pool::consts::*;
use ::lsu_pool::events::*;
use scrypto::{api::ObjectModuleId, prelude::*};

mod common;

pub use crate::common::lsu_pool;
pub use crate::common::lsu_token_validator;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_add_liquidity_tx_fees() {
    // ARRANGE
    let mut vars = setup();
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // initial add liquidity:
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(100));
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    // println!("receipt: {:?}", receipt);
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // add a bunch of validators
    for _i in 0..30 {
        let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(100));
        let _receipt = lsu_pool::add_liquidity_with_proof_receipt(
            lsu01_resource,
            dec!(10),
            credit_token,
            nft_id.clone(),
            &mut vars,
        );
    }

    // set up another
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT 1
    let fee_summary =
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars).fee_summary;

    println!(
        "1) fee_summary Cost: {} XRD",
        fee_summary.total_execution_cost_in_xrd
    );

    // ACT 2
    let fee_summary =
        lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars).fee_summary;

    println!(
        "2) fee_summary Cost: {} XRD",
        fee_summary.total_execution_cost_in_xrd
    );

    // ACT 3
    let fee_summary = lsu_pool::add_liquidity_with_proof_receipt(
        lsu01_resource,
        dec!(10),
        credit_token,
        nft_id.clone(),
        &mut vars,
    )
    .fee_summary;

    println!(
        "3) fee_summary (with credit) Cost: {} XRD",
        fee_summary.total_execution_cost_in_xrd
    );
}

#[test]
fn test_add_liquidity_receipt_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_add_liquidity_receipt_failure_not_lsu() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = token(&mut vars, dec!(1000));

    // ACT
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "Can only add LSU tokens as liquidity.");
}

#[test]
fn test_add_liquidity_receipt_failure_not_in_active_set() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_token_validator::update_active_set(lsu01_resource, false, &mut vars).expect_commit_success();

    // ACT
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "LSU must be for an active validator.");
}

#[test]
fn set_validator_max_before_fee_add_validators_below_max_no_reserve() {
    // ARRANGE
    let mut vars = setup();

    // set max validators = 3
    lsu_pool::set_validator_max_before_fee(3u32, &mut vars);

    // ACT - add 3 validators
    let temp1 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp1, dec!(10), &mut vars);

    let temp2 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp2, dec!(20), &mut vars);

    let temp3 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp3, dec!(30), &mut vars);

    // ASSERT
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp1, &mut vars), None);
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp2, &mut vars), None);
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp3, &mut vars), None);
}

#[test]
fn set_validator_max_before_fee_add_validators_above_max_reserve() {
    // ARRANGE
    let mut vars = setup();

    // set max validators = 3
    lsu_pool::set_validator_max_before_fee(3u32, &mut vars);

    // ACT 1 - add 3 validators
    let temp1 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp1, dec!(10), &mut vars);

    let temp2 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp2, dec!(20), &mut vars);

    let temp3 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp3, dec!(30), &mut vars);

    // ASSERT 1
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp1, &mut vars), None);
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp2, &mut vars), None);
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp3, &mut vars), None);

    // ACT 2 - add 1 more validator
    let temp4 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp4, dec!(40), &mut vars);

    // ASSERT 2
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp4, &mut vars), None);

    // ACT 3 - add 1 more validator
    let temp5 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp5, dec!(40), &mut vars);

    // ASSERT 3
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(temp5, &mut vars),
        Some(dec!(1))
    );

    // ACT 4 - add 1 more validator
    let temp6 = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp6, dec!(40), &mut vars);

    // ASSERT 4
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(temp6, &mut vars),
        Some(dec!(8))
    );
}

#[test]
fn set_validator_max_before_fee_add_validators_above_max_failure_not_enough_tokens_to_take() {
    // ARRANGE
    let mut vars = setup();

    // set max validators = 3
    lsu_pool::set_validator_max_before_fee(3u32, &mut vars);

    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);
    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);
    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);

    // these charge
    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);
    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);
    let temp = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(temp, dec!(100), &mut vars);

    // ASSERT
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(temp, &mut vars),
        Some(dec!(8)) // 2^3
    );

    // ASSERT
    let temp6 = validator::create_lsu_resource(&mut vars, dec!(1000));
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(temp6, dec!(26), &mut vars); // 3^3

    receipt.expect_commit_failure();
    assert_eq!(lsu_pool::get_reserve_vault_balance(temp6, &mut vars), None);
}

#[test]
fn test_balance_changes_new_receipt_new_liquidity_tokens() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get liquidity token supply
    let liquidity_token_total_supply = lsu_pool::get_liquidity_token_total_supply(&mut vars);

    // ASSERT
    receipt.expect_commit_success();
    assert_eq!(liquidity_token_total_supply, dec!(10));
    assert_balance(credit_token, dec!(1), &mut vars);
    assert_balance(liquidity_token, dec!(10), &mut vars);
}

#[test]
fn test_new_receipt_updated_data_correct() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // extract the nft id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT get nft data
    let nft_data = lsu_pool::get_nft_data(nft_id, &mut vars);

    // ASSERT
    assert!(nft_data.contains_key(&lsu01_resource));
    assert_eq!(nft_data.get(&lsu01_resource).unwrap(), &dec!(10));
}

#[test]
fn test_add_liquidity_check_vault() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get vault liquidity in lsu 01
    let vault_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();

    // ASSERT
    assert_eq!(vault_lsu01, dec!(10));
}

#[test]
fn test_add_liquidity_check_reserve_vault() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get vault liquidity in lsu 01
    let reserve_vault_lsu01 = lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars);

    // ASSERT
    assert!(reserve_vault_lsu01.is_none());
}

#[test]
fn test_add_liquidity_with_proof_receipt_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity (first time)
    let receipt1 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id from the receipt
    let nft_id = extract_nftlocalid(receipt1, credit_token);

    // ACT - add liquidity (second time) this time pass proof
    let receipt = lsu_pool::add_liquidity_with_proof_receipt(
        lsu01_resource,
        dec!(11),
        credit_token,
        nft_id.clone(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[ignore = "unsure how to test this"]
#[test]
fn test_add_liquidity_with_wrong_proof_receipt_failure() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity (first time)
    let receipt1 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id from the receipt
    let nft_id = extract_nftlocalid(receipt1, credit_token);

    // ACT - add liquidity (second time) this time pass proof
    let receipt = lsu_pool::add_liquidity_with_proof_receipt(
        lsu01_resource,
        dec!(11),
        credit_token,
        nft_id.clone(),
        &mut vars,
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
}

#[test]
fn test_credit_same_resource_twice() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity (first time)
    let receipt1 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id from the receipt
    let nft_id = extract_nftlocalid(receipt1, credit_token);

    // ACT - add liquidity (second time) this time pass proof
    lsu_pool::add_liquidity_with_proof_receipt(
        lsu01_resource,
        dec!(11),
        credit_token,
        nft_id.clone(),
        &mut vars,
    )
    .expect_commit_success();

    // ACT get nft data
    let nft_data = lsu_pool::get_nft_data(nft_id, &mut vars);

    // ASSERT
    assert_eq!(nft_data.get(&lsu01_resource).unwrap(), &dec!(18));
}

#[test]
fn test_credit_two_resources() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity (first time)
    let receipt1 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id from the receipt
    let nft_id = extract_nftlocalid(receipt1, credit_token);

    // ACT - add liquidity (second time) this time pass proof
    lsu_pool::add_liquidity_with_proof_receipt(
        lsu02_resource,
        dec!(11),
        credit_token,
        nft_id.clone(),
        &mut vars,
    )
    .expect_commit_success();

    // ACT get nft data
    let nft_data = lsu_pool::get_nft_data(nft_id, &mut vars);

    // ASSERT
    assert_eq!(nft_data.get(&lsu01_resource).unwrap(), &dec!(7));
    assert_eq!(nft_data.get(&lsu02_resource).unwrap(), &dec!(11));
}

#[test]
fn test_add_liquidity_emit() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();
    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1,
        )
    );

    // ASSERT data in event
    // thes are positive because we are ADDING liquidity
    assert_eq!(decode_data.resource_address, lsu01_resource);
    assert_eq!(decode_data.amount, dec!(10));
    assert_eq!(decode_data.liquidity_token_amount_change, dec!(10));
}
