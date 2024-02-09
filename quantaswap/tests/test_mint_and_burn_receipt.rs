#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::*;

use ::quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;
pub use crate::common::quantaswap;

#[test]
fn test_mint_liquidity_receipt_one() {
    let mut vars: Vars = setup();

    mint_liquidity_receipt(&mut vars);

    assert_balance(vars.liquidity_receipt, dec!(1), &mut vars);
}

#[test]
fn test_mint_liquidity_receipt_many() {
    let mut vars: Vars = setup();

    for _ in 0..10 {
        mint_liquidity_receipt(&mut vars);
    }

    assert_balance(vars.liquidity_receipt, dec!(10), &mut vars);
}

#[test]
fn test_burn_liquidity_receipt() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    burn_liquidity_receipt(id, &mut vars).expect_commit_success();

    assert_balance(vars.liquidity_receipt, dec!(0), &mut vars);
}

#[test]
fn test_burn_liquidity_receipt_random_nft_invalid() {
    let mut vars: Vars = setup();

    let nft = vars.test_runner.create_freely_mintable_and_burnable_non_fungible_resource(
        OwnerRole::None, 
        NonFungibleIdType::Integer, 
        Some(vec![(NonFungibleLocalId::integer(1), {})]), 
        vars.account_component
    );

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    let manifest = transaction::prelude::ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, nft, ids.clone())
        .take_non_fungibles_from_worktop(nft, ids, "liquidity_receipt")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.quantaswap_component,
                "burn_liquidity_receipt",
                manifest_args!(lookup.bucket("liquidity_receipt")),
            )
        })
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid liquidity receipt.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_burn_liquidity_receipt_other_pool_invalid() {
    let mut vars: Vars = setup();

    // Mint liquidity receipt for pool 1
    let id = mint_liquidity_receipt(&mut vars);

    // Create pool 2
    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        3,
    );
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();
    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

    // Burn liquidity receipt from pool 1 with pool 2
    let ids = BTreeSet::from([id]);
    let manifest = transaction::prelude::ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.liquidity_receipt, ids.clone())
        .take_non_fungibles_from_worktop(vars.liquidity_receipt, ids, "liquidity_receipt")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                quantaswap_component,
                "burn_liquidity_receipt",
                manifest_args!(lookup.bucket("liquidity_receipt")),
            )
        })
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid liquidity receipt.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_burn_liquidity_receipt_with_claims_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(Tick::ONE.0, dec!(1), dec!(1))], &mut vars).expect_commit_success();

    burn_liquidity_receipt(id, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Cannot burn liquidity receipt with liquidity claims.")
            },
            _ => false,
        }
    });
}