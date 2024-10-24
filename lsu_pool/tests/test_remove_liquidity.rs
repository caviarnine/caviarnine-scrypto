// use ::lsu_pool::consts::*;
use ::lsu_pool::events::*;
use scrypto::{api::ObjectModuleId, prelude::*};

mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_remove_liquidity_tx_fees() {
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

        // println!("receipt({}): {:?}", i, receipt);
    }

    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT - remove liquidity
    let fee_summary = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    )
    .fee_summary;

    println!(
        "1) fee_summary Cost: {} XRD",
        fee_summary.total_execution_cost_in_xrd
    );
}

#[test]
fn test_remove_liquidity_receipt_success() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_remove_liquidity_receipt_failure_wrong_tokens() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let fake_liquidity_token = token(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        fake_liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "Invalid liquidity provider tokens.");
}

#[test]
fn test_remove_liquidity_empty_liquidity_tokens() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ACT - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(0),
        lsu01_resource,
        &mut vars,
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_remove_liquidity_receipt_failure_none_of_payout_tokens() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(5),
        lsu02_resource,
        &mut vars,
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "The pool doesn't have any of this token.");
}

#[test]
fn test_remove_liquidity_receipt_failure_none_of_payout_tokens_even_if_non_lsu() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let token_a = token(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT - remove liquidity
    let receipt =
        lsu_pool::remove_liquidity_no_proof_receipt(liquidity_token, dec!(5), token_a, &mut vars);

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_failure();
    assert_contains_message(receipt, "The pool doesn't have any of this token.");
}

#[test]
fn test_remove_liquidity_multiple_times_no_fees() {
    // ARRANGE
    let mut vars = setup();

    // set fees to zero
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);

    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ACT 1 - add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof_receipt(lsu02_resource, dec!(90), &mut vars);

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 1
    assert_eq!(lsu01_vault_balance, dec!(10));
    assert_eq!(lsu02_vault_balance, dec!(90));

    // ACT 2 - remove ALL the LSU01 liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(15),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 2
    assert_eq!(lsu01_vault_balance, dec!(0));
    assert_eq!(lsu02_vault_balance, dec!(90));

    // ACT 3 - remove more liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 2
    assert_eq!(lsu01_vault_balance, dec!(0));
    assert_eq!(lsu02_vault_balance, dec!(90));
}

#[test]
fn test_remove_liquidity_multiple_times_with_fees() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get protocol, reserve and liquidity fee
    let _protocol_fee = lsu_pool::get_protocol_fee(&mut vars);
    let _reserve_fee = lsu_pool::get_reserve_fee(&mut vars);
    let liquidity_fee = lsu_pool::get_liquidity_fee(&mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ACT 1 - add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof_receipt(lsu02_resource, dec!(90), &mut vars);

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 1
    assert_eq!(lsu01_vault_balance, dec!(10));
    assert_eq!(lsu02_vault_balance, dec!(90));

    // ACT 2 - remove ALL the LSU01 liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(15),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 2
    assert_eq!(lsu01_vault_balance, dec!(10) * liquidity_fee);
    assert_eq!(lsu02_vault_balance, dec!(90));

    // ACT 3 - remove more liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // get vault balances of the 2 lsu resources and liquidity tokens
    let lsu01_vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let lsu02_vault_balance = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 2
    assert_eq!(
        lsu01_vault_balance,
        dec!(10) * liquidity_fee * liquidity_fee
    );
    assert_eq!(lsu02_vault_balance, dec!(90));
}

#[test]
fn test_remove_liquidity_liquidity_token_supply() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ASSERT - liquidity token supply is zero
    assert_eq!(
        lsu_pool::get_liquidity_token_total_supply(&mut vars),
        dec!(0)
    );

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ASSERT - liquidity token supply is zero
    assert_eq!(
        lsu_pool::get_liquidity_token_total_supply(&mut vars),
        dec!(10)
    );

    // ACT - remove liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(9),
        lsu01_resource,
        &mut vars,
    );

    // ASSERT - liquidity token supply is zero
    assert_eq!(
        lsu_pool::get_liquidity_token_total_supply(&mut vars),
        dec!(1)
    );

    println!("receipt: {:?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_remove_liquidity_charged_full_reserve_fee() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero except reserve fee
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(dec!("0.01"), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ACT - remove all liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // get reserve fvault balance
    let reserve_vault_lsu01 =
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap();

    // ASSERT
    assert_eq!(reserve_vault_lsu01, dec!("0.01") * dec!(10));
}

#[test]
fn test_remove_liquidity_charged_full_protocol_fee() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero except protocol fee
    lsu_pool::set_protocol_fee(dec!("0.0050"), &mut vars);
    lsu_pool::set_reserve_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    let balance_before = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu01_resource);

    // ACT - remove all liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    let balance_after = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu01_resource);

    let balance_change = balance_after - balance_before;

    // ASSERT
    receipt.expect_commit_success();
    assert!(lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).is_none());
    assert_eq!(balance_change, dec!(10) * (Decimal::ONE - dec!("0.0050")));
}

#[test]
fn test_remove_liquidity_with_liquidity_fee_emit() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // ACT - remove all liquidity
    let receipt = lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(10),
        lsu01_resource,
        &mut vars,
    );

    // get events
    let events = receipt.expect_commit(true).application_events.clone();

    // step through each event
    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner
                .is_event_name_equal::<RemoveLiquidityEvent>(event_type_identifier)
        })
        .expect("Event not found");

    // decode the data
    let decode_data = scrypto_decode::<RemoveLiquidityEvent>(&event_data).unwrap();

    // ASSERT simplex_factory_component
    let eti1 = event_type_identifier.1.clone();
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

    // get liquidity fee
    let liquidity_fee = lsu_pool::get_liquidity_fee(&mut vars);

    // ASSERT data in event
    // thes are positive because we are ADDING liquidity
    assert_eq!(decode_data.resource_address, lsu01_resource);
    assert_eq!(
        decode_data.amount,
        -dec!(10) * (Decimal::ONE - liquidity_fee)
    );
    assert_eq!(decode_data.liquidity_token_amount_change, -dec!(10));
}

// #####################################################################
// #####################################################################
// #######################   RUNFUND RECEIPT  ##########################
// #####################################################################
// #####################################################################

#[test]
fn test_remove_liquidity_with_credit_receipt() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity and credit token addresses
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id and data from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT remove liquidity with proof
    let receipt = lsu_pool::remove_liquidity_with_proof_receipt(
        liquidity_token,
        dec!(7),
        lsu01_resource,
        credit_token,
        nft_id.clone(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_full_discount_from_credit_receipt() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity and credit token addresses
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id and data from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);
    let nft_data = lsu_pool::get_nft_data(nft_id.clone(), &mut vars);

    // ASSERT 1
    assert_eq!(nft_data[&lsu01_resource], dec!(7));

    // ACT remove liquidity with proof
    let receipt = lsu_pool::remove_liquidity_with_proof_receipt(
        liquidity_token,
        dec!(7),
        lsu01_resource,
        credit_token,
        nft_id.clone(),
        &mut vars,
    );

    println!("{:#?}", receipt);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_partial_discount_from_credit_receipt_reserve_fee_only() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // // set fees to 0
    let reserve_fee = dec!("0.0050");
    lsu_pool::set_protocol_fee(dec!(0), &mut vars);
    lsu_pool::set_liquidity_fee(dec!(0), &mut vars);
    lsu_pool::set_reserve_fee(reserve_fee, &mut vars);

    // get liquidity and credit token addresses
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);
    lsu_pool::add_liquidity_no_proof_receipt(lsu02_resource, dec!(11), &mut vars);

    // now user has 18 liquidity tokens (7 from lsu01 and 11 from lsu02)

    // extract the nft id and data from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);
    let nft_data = lsu_pool::get_nft_data(nft_id.clone(), &mut vars);

    // ASSERT 1 - the user credit balances are 7 and 11
    assert_eq!(nft_data[&lsu01_resource], dec!(7));
    // assert_eq!(nft_data[&lsu02_resource], dec!(11)); // don;t test that becasue we didn't pass the proof

    // now swap and add more LSU1 for LSU2
    lsu_pool::swap(lsu01_resource, dec!(3), lsu02_resource, &mut vars);

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();
    let reserve_balance_lsu02 =
        lsu_pool::get_reserve_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 2 - the vault balances are 7 + 3 = 10 and 11 - 3 = 8, user received 3(1-reserve_fee) LSU2 tokens
    assert_eq!(vault_balance_lsu01, dec!(10));
    assert_eq!(vault_balance_lsu02, dec!(8));
    assert_eq!(reserve_balance_lsu02, dec!(3) * reserve_fee);

    // get balance before
    let balance_before = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu01_resource);

    // ACT remove liquidity with proof
    // user has 18 liquidity tokens
    // user takes 9 liquidity tokens (50% of the pool) and asks for LSU01
    // Will get 9 LSU01 tokens, 7 will be free and 2 will be charged
    let receipt = lsu_pool::remove_liquidity_with_proof_receipt(
        liquidity_token,
        dec!(9),
        lsu01_resource,
        credit_token,
        nft_id.clone(),
        &mut vars,
    );

    // get balance after
    let balance_after = vars
        .test_runner
        .get_component_balance(vars.account_component_address, lsu01_resource);

    // get wallet change
    let wallet_balance_change_lsu01 = balance_after - balance_before;

    // get vault balances
    let vault_balance_lsu01 = lsu_pool::get_vault_balance(lsu01_resource, &mut vars).unwrap();
    let vault_balance_lsu02 = lsu_pool::get_vault_balance(lsu02_resource, &mut vars).unwrap();
    let reserve_balance_lsu01 =
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars).unwrap();
    let reserve_balance_lsu02 =
        lsu_pool::get_reserve_vault_balance(lsu02_resource, &mut vars).unwrap();

    // ASSERT 3
    assert_eq!(
        wallet_balance_change_lsu01,
        dec!(7) + dec!(2) * (Decimal::ONE - reserve_fee)
    );
    assert_eq!(vault_balance_lsu01, dec!(1));
    assert_eq!(vault_balance_lsu02, dec!(8));
    assert_eq!(reserve_balance_lsu01, dec!(2) * reserve_fee);
    assert_eq!(reserve_balance_lsu02, dec!(3) * reserve_fee);

    let nft_data = lsu_pool::get_nft_data(nft_id.clone(), &mut vars);
    // ASSERT 4
    assert_eq!(nft_data[&lsu01_resource], dec!(0));

    receipt.expect_commit_success();
}

#[test]
fn test_remove_liquidity_credit_receipt_updated() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get liquidity and credit token addresses
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // add liquidity
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);

    // extract the nft id and data from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);
    let nft_data = lsu_pool::get_nft_data(nft_id.clone(), &mut vars);

    // ASSERT 1
    assert_eq!(nft_data[&lsu01_resource], dec!(7));

    // ACT remove liquidity with proof
    lsu_pool::remove_liquidity_with_proof_receipt(
        liquidity_token,
        dec!(3),
        lsu01_resource,
        credit_token,
        nft_id.clone(),
        &mut vars,
    )
    .expect_commit_success();

    let nft_data = lsu_pool::get_nft_data(nft_id.clone(), &mut vars);

    // ASSERT 1
    assert_almost_equal(nft_data[&lsu01_resource], dec!(4), dec!("0.0000000000001"));
}
