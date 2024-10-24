use scrypto::prelude::*;

mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_credit_receipt_is_not_withdrawable() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get credit receipt
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT
    let receipt = lsu_pool::check_withdrawable_receipt(credit_token, nft_id, &mut vars);

    // ASSERT
    receipt.expect_commit_failure();
}

#[test]
fn test_credit_receipt_is_burnable() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get credit receipt
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT
    let receipt = lsu_pool::burn_nft_receipt(credit_token, nft_id, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
// TODO: This is a bit of a hack, but it works for now
fn test_credit_receipt_is_burnable_with_two_receipts_which_one() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // ACT
    // get two credit receipts
    let receipt_01 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    let _receipt_02 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt_01, credit_token);

    // ASSERT
    // wallet contains two credit receipts
    assert_balance(credit_token, dec!(2), &mut vars);

    // ACT
    let receipt = lsu_pool::burn_nft_receipt(credit_token, nft_id, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_get_id_resources_from_credit_proof_receipt() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get credit receipt
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT
    let receipt =
        lsu_pool::get_id_resources_from_credit_proof_receipt(credit_token, nft_id, &mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_get_id_resources_from_credit_proof() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get credit receipt
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT - ASSERT
    lsu_pool::get_id_resources_from_credit_proof(credit_token, nft_id, &mut vars);
}

#[test]
fn test_get_id_resources_from_credit_proof_check_values() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get credit receipt
    let receipt = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get the id from the receipt
    let nft_id = extract_nftlocalid(receipt, credit_token);

    // ACT
    let (id, credits) =
        lsu_pool::get_id_resources_from_credit_proof(credit_token, nft_id.clone(), &mut vars);

    // ASSERT
    assert_eq!(id, nft_id);
    assert_eq!(credits[&lsu01_resource], dec!(10));
}

#[test]
fn test_assert_two_credit_receipts() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // ACT
    // get two credit receipts
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ASSERT
    // wallet contains two credit receipts
    assert_balance(credit_token, dec!(2), &mut vars);
}

#[test]
fn test_merge_two_credit_receipts() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get two credit receipts
    let receipt_01 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);
    let receipt_02 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // extract the nft id from both of the receipts
    let nft_id_01 = extract_nftlocalid(receipt_01, credit_token);
    let nft_id_02 = extract_nftlocalid(receipt_02, credit_token);

    // ACT
    let receipt = lsu_pool::merge_credit_receipt(
        credit_token,
        nft_id_01.clone(),
        nft_id_02.clone(),
        &mut vars,
    );

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_merge_two_credit_receipts_same_resource() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // get the credit token address
    let credit_token = lsu_pool::get_credit_receipt_resource_address(&mut vars);

    // get two credit receipts
    let receipt_01 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(7), &mut vars);
    let receipt_02 = lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(11), &mut vars);

    // extract the nft id from both of the receipts
    let nft_id_01 = extract_nftlocalid(receipt_01, credit_token);
    let nft_id_02 = extract_nftlocalid(receipt_02, credit_token);

    // ACT
    lsu_pool::merge_credit_receipt(
        credit_token,
        nft_id_01.clone(),
        nft_id_02.clone(),
        &mut vars,
    )
    .expect_commit_success();

    // get data out of them:
    let (_, credits_01) =
        lsu_pool::get_id_resources_from_credit_proof(credit_token, nft_id_01.clone(), &mut vars);
    let (_, credits_02) =
        lsu_pool::get_id_resources_from_credit_proof(credit_token, nft_id_02.clone(), &mut vars);

    // ASSERT
    assert_eq!(credits_01.len(), 1);
    assert_eq!(credits_02.len(), 0);
    assert_eq!(credits_01[&lsu01_resource], dec!(18));
}
