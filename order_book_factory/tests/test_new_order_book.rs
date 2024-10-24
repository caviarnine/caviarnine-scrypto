#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::order_book_factory::*;
pub use crate::common::order_book_factory;
pub use crate::common::fee_vaults;
pub use crate::common::fee_controller;
pub use crate::common::token_validator;
pub use crate::common::order_book;

#[test]
fn test_new_order_book() {
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

    let receipt = new_order_book(token_a, token_b, &mut vars);
    let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];

    vars.order_book_component = order_book_component;

    assert_eq!(
        order_book::get_token_x_address(&mut vars), 
        token_a
    );

    assert_eq!(
        order_book::get_token_y_address(&mut vars), 
        token_b
    );

    assert_eq!(
        order_book::get_fee_controller_address(&mut vars), 
        vars.fee_controller_component
    );

    assert_eq!(
        order_book::get_fee_vaults_address(&mut vars), 
        vars.fee_vaults_component
    );
}

#[test]
fn test_new_order_book_with_limit() {
    let mut vars: Vars = setup();

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
                .call_method(
                vars.order_book_factory_component,
                "new_order_book",
                manifest_args!(vars.token_x, vars.token_y, Some(lookup.address_reservation("order_book_reservation")))
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
