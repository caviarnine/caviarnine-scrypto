#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

use ::quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;
pub use crate::common::quantaswap;

#[test]
fn test_setup() {
    setup();
}

#[test]
fn test_instantiation() {
    let mut vars: Vars = setup();

    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        token_a, 
        token_b, 
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
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    vars.quantaswap_component = quantaswap_component;
    vars.liquidity_receipt = liquidity_receipt;

    assert_eq!(
        get_token_x_address(&mut vars), 
        token_a
    );

    assert_eq!(
        get_token_y_address(&mut vars), 
        token_b
    );

    assert_eq!(
        get_liquidity_receipt_address(&mut vars),
        vars.liquidity_receipt
    );

    assert_eq!(
        get_fee_controller_address(&mut vars), 
        vars.fee_controller_component
    );

    assert_eq!(
        get_fee_vaults_address(&mut vars), 
        vars.fee_vaults_component
    );

    assert_eq!(
        get_bin_span(&mut vars),
        3
    );

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_active_tick(&mut vars),
        None
    );
}

#[test]
fn test_instantiation_bin_span_zero_invalid() {
    let mut vars: Vars = setup();

    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        token_a, 
        token_b, 
        0,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Bin span must be greater than zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_instantiation_with_add_liquidity_after() {
    let mut vars: Vars = setup();

    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!("3");
    let amount_y1 = dec!(0);

    let tick2 = tick0 - vars.bin_span;
    let amount_x2 = dec!(0);
    let amount_y2 = dec!("2");

    let total_x = amount_x0 + amount_x1 + amount_x2;
    let total_y = amount_y0 + amount_y1 + amount_y2;
    let positions = vec![
        (tick0, amount_x0, amount_y0),
        (tick1, amount_x1, amount_y1),
        (tick2, amount_x2, amount_y2),
    ];

    let owner_rule = rule!(require(vars.admin_badge));
    let user_rule = rule!(allow_all);
    let manifest = transaction::prelude::ManifestBuilder::new()
        .allocate_global_address(
            vars.quantaswap_package, 
            "QuantaSwap", 
            "pool_reservation", 
            "pool_address"
        )
        .withdraw_from_account(vars.account_component, vars.token_x, total_x)
        .withdraw_from_account(vars.account_component, vars.token_y, total_y)
        .take_all_from_worktop(vars.token_x, "tokens_x")
        .take_all_from_worktop(vars.token_y, "tokens_y")
        .with_name_lookup(|builder, lookup| {
            builder
                .call_function(
                    vars.quantaswap_package,
                    "QuantaSwap",
                    "new",
                manifest_args!(owner_rule, user_rule, vars.token_x, vars.token_y, vars.bin_span, Some(lookup.address_reservation("pool_reservation")))
                )
                .call_method(
                    lookup.named_address("pool_address"),
                    "add_liquidity",
                    manifest_args!(lookup.bucket("tokens_x"), lookup.bucket("tokens_y"), positions),
                )
        })
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop)
        )
        .build();
    
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("{:?}", receipt);
    receipt.expect_commit_success();
}
