use scrypto::prelude::*;
mod common;

pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;

#[test]
fn test_get_token_validator_address() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let token_validator_address = lsu_pool::get_token_validator_address(&mut vars);

    // ASSERT
    assert_eq!(token_validator_address, vars.token_validator_component_address);
}

#[test]
fn test_get_fee_vaults_address() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let fee_vaults_address = lsu_pool::get_fee_vaults_address(&mut vars);

    // ASSERT
    assert_eq!(fee_vaults_address, vars.fee_vaults_component_address);
}

#[test]
fn test_get_vault_balance_none() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars);

    // ASSERT
    assert!(vault_balance.is_none());
}

#[test]
fn test_get_vault_balance_some() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT
    let vault_balance = lsu_pool::get_vault_balance(lsu01_resource, &mut vars);

    // ASSERT
    assert_eq!(vault_balance, Some(dec!(10)));
}

#[test]
fn test_get_reserve_vault_balance_none() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let reserve_vault_balance = lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars);

    // ASSERT
    assert!(reserve_vault_balance.is_none());
}

#[test]
fn test_get_reserve_vault_balance_some() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // set fees to zero except reserve fee
    lsu_pool::set_reserve_fee(dec!("0.01"), &mut vars);

    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars),
        None
    );

    // add liquidity
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(100), &mut vars);

    // get liquidity token resource address
    let liquidity_token = lsu_pool::get_liquidity_token_resource_address(&mut vars);

    // remove all liquidity
    lsu_pool::remove_liquidity_no_proof_receipt(
        liquidity_token,
        dec!(100),
        lsu01_resource,
        &mut vars,
    )
    .expect_commit_success();

    // ASSERT
    assert_eq!(
        lsu_pool::get_reserve_vault_balance(lsu01_resource, &mut vars),
        Some(dec!(1))
    );
}

#[test]
fn get_price_lsu_xrd_cached_none() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let price = lsu_pool::get_price_lsu_xrd_cached(lsu01_resource, &mut vars);

    // ASSERT
    assert_eq!(price, None);
}

#[test]
fn get_price_lsu_xrd_cached_some() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT
    let price = lsu_pool::get_price_lsu_xrd_cached(lsu01_resource, &mut vars);

    // ASSERT
    assert!(price.is_some());
}

#[test]
fn get_dex_valuation_xrd_zero() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let dex_valuation_xrd = lsu_pool::get_dex_valuation_xrd(&mut vars);

    // ASSERT
    assert_eq!(dex_valuation_xrd, dec!(0));
}

#[test]
fn get_dex_valuation_xrd_value_01() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // get price
    let price_lsu_xrd = lsu_pool::get_price_lsu_xrd_cached(lsu01_resource, &mut vars).unwrap();

    // ACT
    let dex_valuation_xrd = lsu_pool::get_dex_valuation_xrd(&mut vars);

    // ASSERT
    assert_eq!(dex_valuation_xrd, dec!(10) * price_lsu_xrd);
}

#[test]
fn get_liquidity_token_resource_address() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::get_liquidity_token_resource_address(&mut vars);
}

#[test]
fn get_liquidity_token_total_supply_zero() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let total_supply = lsu_pool::get_liquidity_token_total_supply(&mut vars);

    // ASSERT
    assert_eq!(total_supply, dec!(0));
}

#[test]
fn get_liquidity_token_total_supply_value_01() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    lsu_pool::add_liquidity_no_proof_receipt(lsu01_resource, dec!(10), &mut vars);

    // ACT
    let total_supply = lsu_pool::get_liquidity_token_total_supply(&mut vars);

    // ASSERT
    assert_eq!(total_supply, dec!(10));
}

#[test]
fn get_credit_receipt_resource_address() {
    // ARRANGE
    let mut vars = setup();

    // ACT - ASSERT
    lsu_pool::get_credit_receipt_resource_address(&mut vars);
}

#[test]
fn get_protocol_fee() {
    // ARRANGE
    let mut vars = setup();

    lsu_pool::set_protocol_fee(dec!("0.01"), &mut vars);

    // ACT
    let protocol_fee = lsu_pool::get_protocol_fee(&mut vars);

    // ASSERT
    assert_eq!(protocol_fee, dec!("0.01"));
}

#[test]
fn get_liquidity_fee() {
    // ARRANGE
    let mut vars = setup();

    lsu_pool::set_liquidity_fee(dec!("0.01"), &mut vars);

    // ACT
    let liquidity_fee = lsu_pool::get_liquidity_fee(&mut vars);

    // ASSERT
    assert_eq!(liquidity_fee, dec!("0.01"));
}

#[test]
fn get_reserve_fee() {
    // ARRANGE
    let mut vars = setup();

    lsu_pool::set_reserve_fee(dec!("0.01"), &mut vars);

    // ACT
    let reserve_fee = lsu_pool::get_reserve_fee(&mut vars);

    // ASSERT
    assert_eq!(reserve_fee, dec!("0.01"));
}

#[test]
fn get_price_pass_trash_xrd() {
    // ARRANGE
    let mut vars = setup();
    let token_a = token(&mut vars, dec!(10));

    // ACT
    let price = lsu_pool::get_price(token_a, XRD, &mut vars);

    // ASSERT
    assert_eq!(price, None);
}

#[test]
fn get_price_pass_xrd_xrd() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let price = lsu_pool::get_price(XRD, XRD, &mut vars);

    // ASSERT
    assert_eq!(price, Some(dec!(1)));
}

#[test]
// TODO: update this VALIDATOR_PRICE_LSU_XRD
fn get_price_pass_lsu01_xrd() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let price = lsu_pool::get_price(lsu01_resource, XRD, &mut vars);

    // ASSERT
    assert_eq!(price, Some(dec!(1)));
}

#[test]
// TODO: update this VALIDATOR_PRICE_LSU_XRD
fn get_price_pass_xrd_lsu01() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let price = lsu_pool::get_price(XRD, lsu01_resource, &mut vars);

    // ASSERT
    assert_eq!(price, Some(dec!(1) / dec!(1)));
}

#[test]
// TODO: update this VALIDATOR_PRICE_LSU_XRD
fn get_price_pass_lus01_lsu01() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let price = lsu_pool::get_price(lsu01_resource, lsu01_resource, &mut vars);

    // ASSERT
    assert_eq!(price, Some(dec!(1)));
}

#[test]
// TODO: update this VALIDATOR_PRICE_LSU_XRD
fn get_price_pass_lus01_lsu02() {
    // ARRANGE
    let mut vars = setup();
    let lsu01_resource = validator::create_lsu_resource(&mut vars, dec!(1000));
    let lsu02_resource = validator::create_lsu_resource(&mut vars, dec!(1000));

    // ACT
    let price_lsu01_xrd = lsu_pool::get_price(lsu01_resource, XRD, &mut vars);
    let price_lsu02_xrd = lsu_pool::get_price(lsu02_resource, XRD, &mut vars);
    let price_lsu01_lsu02 = lsu_pool::get_price(lsu01_resource, lsu02_resource, &mut vars);

    // ASSERT
    assert_eq!(price_lsu01_lsu02, Some(dec!(1)));
    assert_eq!(price_lsu01_xrd, Some(dec!(1)));
    assert_eq!(price_lsu02_xrd, Some(dec!(1)));
}

#[test]
fn get_nft_data() {
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
