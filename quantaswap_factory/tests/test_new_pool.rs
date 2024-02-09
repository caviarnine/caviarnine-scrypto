#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap_factory::*;
pub use crate::common::quantaswap_factory;
pub use crate::common::fee_vaults;
pub use crate::common::fee_controller;
pub use crate::common::token_validator;
pub use crate::common::quantaswap;

#[test]
fn test_new_pool() {
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

    let receipt = new_pool(token_a, token_b, 5, &mut vars);
    let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);

    vars.quantaswap_component = quantaswap_component;

    assert_eq!(
        quantaswap::get_token_x_address(&mut vars), 
        token_a
    );

    assert_eq!(
        quantaswap::get_token_y_address(&mut vars), 
        token_b
    );

    assert_eq!(
        quantaswap::get_fee_controller_address(&mut vars), 
        vars.fee_controller_component
    );

    assert_eq!(
        quantaswap::get_fee_vaults_address(&mut vars), 
        vars.fee_vaults_component
    );

    assert_eq!(
        quantaswap::get_bin_span(&mut vars),
        5
    );
}

#[test]
fn test_new_pool_with_add_liquidity() {
    let mut vars: Vars = setup();

    let tick0 = 25000;
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
                .call_method(
                    vars.quantaswap_factory_component,
                    "new_pool",
                    manifest_args!(vars.token_x, vars.token_y, vars.bin_span, Some(lookup.address_reservation("pool_reservation")))
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