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
fn test_swap_tx_fees() {
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

    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let fee_summary =
        lsu_pool::swap_receipt(lsu01_resource, dec!(3), lsu02_resource, &mut vars).fee_summary;

    println!(
        " fee_summary Cost: {} XRD",
        fee_summary.total_execution_cost_in_xrd
    );
}

#[test]
fn test_swap_receipt() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_swap() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT - ASSEERT
    lsu_pool::swap(lsu01_resource, dec!(3), lsu02_resource, &mut vars);
}

#[test]
fn test_swap_receipt_receiving_token_not_lsu_failure() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let token_a = token(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let receipt = lsu_pool::swap_receipt(token_a, dec!(3), lsu02_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "Can only swap LSU tokens.");
}

#[test]
fn test_swap_receipt_receiving_token_not_in_active_set() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // remove from active set
    lsu_token_validator::update_active_set(lsu02_resource, false, &mut vars).expect_commit_success();

    // ACT
    let receipt = lsu_pool::swap_receipt(lsu02_resource, dec!(3), lsu02_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "LSU must be for an active validator.");
}

#[test]
fn test_swap_receipt_paying_token_not_lsu_failure() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let token_a = token(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(3), token_a, &mut vars);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "Can only swap for LSU tokens.");
}

#[test]
fn test_swap_receipt_empty_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(0), lsu02_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_swap_receipt_same_token_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(5), lsu01_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_swap_vault_changes() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT
    lsu_pool::swap(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT
    assert_eq!(vault_balance_lsu01, dec!(13));
    assert_eq!(vault_balance_lsu02, dec!(17));
}

#[test]
fn test_swap_vault_changes_bigger_than_paying_vault() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT - since price is 1:1
    lsu_pool::swap(lsu01_resource, dec!(21), lsu02_resource, &mut vars);

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT
    assert_eq!(vault_balance_lsu01, dec!(30));
    assert_eq!(vault_balance_lsu02, dec!(0));
}

#[test]
fn test_swap_vault_changes_reserve_fee_only() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    let reserve_fee = dec!("0.0050");
    lsu_pool::set_reserve_fee(reserve_fee, &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(21), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(37), &mut vars);

    // get wallet balance before of lsu02
    let balance_before_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // ACT - since price is 1:1
    let sell_amount = dec!(3);
    let receipt = lsu_pool::swap_receipt(lsu01_resource, sell_amount, lsu02_resource, &mut vars);
    
    // ASSERT
    receipt.expect_commit_success();

    // get wallet balance after of lsu02
    let balance_after_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // get swap changes for the wallet
    let wallet_balance_change_lsu02 = balance_after_lsu02 - balance_before_lsu02;

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();
    let reserve_balance_lsu01 = lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars);
    let reserve_balance_lsu02 = lsu_pool::get_reserve_vault_balance(lsu02_resource, &mut vars);

    // get price
    let price_lsu01_lsu02 = lsu_pool::get_price(lsu01_resource, lsu02_resource, &mut vars).unwrap();
    let buy_amount = sell_amount * price_lsu01_lsu02;
    let buy_amount_after_fee = buy_amount * (Decimal::ONE - reserve_fee);

    // ASSERT
    assert_eq!(wallet_balance_change_lsu02, buy_amount_after_fee);
    assert_eq!(reserve_balance_lsu01, None);
    assert_eq!(reserve_balance_lsu02, Some(reserve_fee * buy_amount));
    assert_eq!(vault_balance_lsu01, dec!(21) + sell_amount);
    assert_eq!(vault_balance_lsu02, dec!(37) - buy_amount);
}

#[test]
fn test_swap_vault_changes_protocol_fee_only() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    let protocol_fee = dec!("0.0050");
    lsu_pool::set_protocol_fee(protocol_fee, &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(21), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(37), &mut vars);

    // get wallet balance before of lsu02
    let balance_before_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // ACT - since price is 1:1
    let sell_amount = dec!(3);
    let receipt = lsu_pool::swap_receipt(lsu01_resource, sell_amount, lsu02_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_success();

    // get wallet balance after of lsu02
    let balance_after_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // get swap changes for the wallet
    let wallet_balance_change_lsu02 = balance_after_lsu02 - balance_before_lsu02;

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();
    let reserve_balance_lsu01 = lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars);
    let reserve_balance_lsu02 = lsu_pool::get_reserve_vault_balance(lsu02_resource, &mut vars);

    // get price
    let price_lsu01_lsu02 = lsu_pool::get_price(lsu01_resource, lsu02_resource, &mut vars).unwrap();
    let buy_amount = sell_amount * price_lsu01_lsu02;
    let buy_amount_after_fee = buy_amount * (Decimal::ONE - protocol_fee);

    // ASSERT
    // the protocol fee has been sent to the FEE_VAULTS so that's why it doesn't add up
    assert_eq!(wallet_balance_change_lsu02, buy_amount_after_fee);
    assert_eq!(reserve_balance_lsu01, None);
    assert_eq!(reserve_balance_lsu02, None);
    assert_eq!(vault_balance_lsu01, dec!(21) + sell_amount);
    assert_eq!(vault_balance_lsu02, dec!(37) - buy_amount);
}

#[test]
fn test_swap_vault_changes_liquidity_fee_only() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    let liquidity_fee = dec!("0.0050");
    lsu_pool::set_liquidity_fee(liquidity_fee, &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(21), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(37), &mut vars);

    // get wallet balance before of lsu02
    let balance_before_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // ACT - since price is 1:1
    let sell_amount = dec!(3);
    let receipt = lsu_pool::swap_receipt(lsu01_resource, sell_amount, lsu02_resource, &mut vars);

    // ASSERT
    receipt.expect_commit_success();

    // get wallet balance after of lsu02
    let balance_after_lsu02 = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu02_resource);

    // get swap changes for the wallet
    let wallet_balance_change_lsu02 = balance_after_lsu02 - balance_before_lsu02;

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();
    let reserve_balance_lsu01 = lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars);
    let reserve_balance_lsu02 = lsu_pool::get_reserve_vault_balance(lsu02_resource, &mut vars);

    // get price
    let price_lsu01_lsu02 = lsu_pool::get_price(lsu01_resource, lsu02_resource, &mut vars).unwrap();
    let buy_amount = sell_amount * price_lsu01_lsu02;
    let buy_amount_after_fee = buy_amount * (Decimal::ONE - liquidity_fee);

    // ASSERT
    assert_eq!(wallet_balance_change_lsu02, buy_amount_after_fee);
    assert_eq!(reserve_balance_lsu01, None);
    assert_eq!(reserve_balance_lsu02, None);
    assert_eq!(vault_balance_lsu01, dec!(21) + sell_amount);
    assert_eq!(vault_balance_lsu02, dec!(37) - buy_amount_after_fee);
}

// TODO: test where validator price is different

#[test]
fn test_swap_emit_no_fees() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT - since price is 1:1
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(21), lsu02_resource, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SwapEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SwapEvent>(&event_data).unwrap();

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
    assert_eq!(decode_data.user_sell_resource_address, lsu01_resource);
    assert_eq!(decode_data.user_sell_amount, dec!(20));
    assert_eq!(decode_data.user_buy_resource_address, lsu02_resource);
    assert_eq!(decode_data.user_buy_amount, dec!(20));
}

#[test]
fn test_swap_emit_with_fees() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero for simplicity
    lsu_pool::set_protocol_fee(dec!("0.0001"), &mut vars);
    lsu_pool::set_liquidity_fee(dec!("0.0002"), &mut vars);
    lsu_pool::set_reserve_fee(dec!("0.0005"), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof(lsu02_resource, dec!(20), &mut vars);

    // ACT - since price is 1:1
    let receipt = lsu_pool::swap_receipt(lsu01_resource, dec!(21), lsu02_resource, &mut vars);

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<SwapEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<SwapEvent>(&event_data).unwrap();

    let eti1 = event_type_identifier.1.clone();
    // ASSERT simplex_factory_component
    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(
                *vars.lsu_pool_component_address.as_node_id(),
                ObjectModuleId::Main,
            ),
            eti1
        )
    );

    // ASSERT data in event
    assert_eq!(decode_data.user_sell_resource_address, lsu01_resource);
    assert_eq!(decode_data.user_sell_amount, dec!(20));
    assert_eq!(decode_data.user_buy_resource_address, lsu02_resource);
    assert_eq!(decode_data.user_buy_amount, dec!(20) * dec!("0.9992"));
}
