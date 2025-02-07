#![allow(dead_code)]
use ::order_book::price::Price;
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;
pub use crate::common::order_book;

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

    let manifest = order_book::build_manifest(
        vars.order_book_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        token_a, 
        token_b, 
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();
    println!("{:?}", receipt);

    let order_book_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let order_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    vars.order_book_component = order_book_component;
    vars.order_receipt = order_receipt;

    assert_eq!(
        get_token_x_address(&mut vars), 
        token_a
    );

    assert_eq!(
        get_token_y_address(&mut vars), 
        token_b
    );

    assert_eq!(
        get_order_receipt_address(&mut vars),
        vars.order_receipt
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
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_last_price(&mut vars),
        Decimal::from(Price::MIN)
    );

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );
}

#[test]
fn test_instantiation_with_limit_after() {
    let mut vars: Vars = setup();

    let owner_rule = rule!(require(vars.admin_badge));
    let user_rule = rule!(allow_all);
    let manifest = transaction::prelude::ManifestBuilder::new()
        .allocate_global_address(
            vars.order_book_package, 
            "OrderBook", 
            "order_book_reservation", 
            "order_book_address"
        )
        .withdraw_from_account(vars.account_component, vars.token_x, dec!(1))
        .take_all_from_worktop(vars.token_x, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder
                .call_function(
                    vars.order_book_package,
                    "OrderBook",
                    "new",
                manifest_args!(owner_rule, user_rule, vars.token_x, vars.token_y, Some(lookup.address_reservation("order_book_reservation")))
                )
                .call_method(
                    lookup.named_address("order_book_address"),
                    "limit_order",
                    manifest_args!(lookup.bucket("tokens") , dec!(1))
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
