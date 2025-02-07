#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;
pub use crate::common::order_book;

#[test]
fn test_claim_cancel_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(0)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_ask_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_claim_cancel_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(0)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_bid_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_claim_cancel_strange_price_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!("9.32579997"), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!("9.32579997"), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(0)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_ask_price(&mut vars), 
        Some(dec!("9.3257"))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_claim_cancel_strange_price_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!("9.32579997"), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!("9.32579997"), &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(0)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_bid_price(&mut vars), 
        Some(dec!("9.3257"))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y - dec!(1), &mut vars);
}

#[test]
fn test_claim_filled_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_ask_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_claim_filled_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_bid_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_claim_filled_strange_price_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!("5.23499348"), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(10), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_claim_filled_strange_price_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!("5.23499348"), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(10), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_claim_active_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(2), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(1)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_ask_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_claim_active_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(2), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(order_data) => assert_eq!(order_data.amount_filled, dec!(1)),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_eq!(
        get_current_bid_price(&mut vars), 
        Some(dec!(1))
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_claim_active_strange_price_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!("30.23"), dec!("5.23499348"), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(30), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_claim_active_strange_price_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!("30.23"), dec!("5.23499348"), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_claim_chain_start_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_chain_start_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_chain_end_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(3)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_chain_end_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(3)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_chain_middle_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    market_order(vars.token_y, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_chain_middle_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([NonFungibleLocalId::integer(2)]);
    claim_orders(ids, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }

    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    match get_order_status(1, &mut vars) {
        OrderStatus::Filled(_) => (),
        _ => panic!(),
    }

    match get_order_status(2, &mut vars) {
        OrderStatus::Claimed => (),
        _ => panic!(),
    }

    match get_order_status(3, &mut vars) {
        OrderStatus::Open(_) => (),
        _ => panic!(),
    }
}

#[test]
fn test_claim_orders_receipt_consumed() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    assert_eq!(
        vars.test_runner.get_component_balance(vars.account_component, vars.order_receipt),
        dec!(1)
    );

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids.clone(), &mut vars).expect_commit_success();
    assert_eq!(
        vars.test_runner.get_component_balance(vars.account_component, vars.order_receipt),
        dec!(0)
    );
}

#[test]
fn test_claim_other_nft_invalid() {
    let mut vars: Vars = setup();

    let nft_address = vars.test_runner.create_non_fungible_resource(vars.account_component);
    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    let manifest = transaction::builder::ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, nft_address, ids.clone())
        .take_non_fungibles_from_worktop(nft_address, ids, "bucket1")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.order_book_component,
                "claim_orders",
                manifest_args!(lookup.bucket("bucket1")))
        })
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    print!("{:?}", receipt);

    receipt.is_commit_failure();
}

#[test]
fn test_claim_other_order_receipt_invalid() {
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

    let order_book_component_1 = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let order_receipt_1 = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    let order_book_component_0 = vars.order_book_component;
    vars.order_book_component = order_book_component_1;
    vars.order_receipt = order_receipt_1;
    limit_order(token_a, dec!(1), dec!(1), &mut vars).expect_commit_success();
    vars.order_book_component = order_book_component_0;

    let ids = BTreeSet::from([NonFungibleLocalId::integer(1)]);
    claim_orders(ids, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid order receipt.")
            }
            _ => false,
        }
    });
}

#[test]
fn test_claim_batch() {
    let mut vars = setup();

    let mut positions: Vec<(ResourceAddress, Decimal, Decimal)> = vec![];
    let mut ids: BTreeSet<NonFungibleLocalId> = BTreeSet::new();
    for i in 0..5 {
        positions.push((vars.token_y, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }
    for i in 5..10 {
        positions.push((vars.token_x, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    claim_orders(ids, &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );
    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance_accept_missing_attos(vars.token_x, vars.amount_x, &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_claim_batch_filled() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let mut positions: Vec<(ResourceAddress, Decimal, Decimal)> = vec![];
    let mut ids: BTreeSet<NonFungibleLocalId> = BTreeSet::new();
    for i in 0..5 {
        positions.push((vars.token_y, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(100), None, &mut vars);

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    claim_orders(ids, &mut vars).expect_commit_success();

    let tolerance_x = (Decimal(I192::ONE)).checked_round(vars.divisibility_x, RoundingMode::ToPositiveInfinity).unwrap() * dec!(5);
    let tolerance_y = (Decimal(I192::ONE)).checked_round(vars.divisibility_y, RoundingMode::ToPositiveInfinity).unwrap() * dec!(5);
    assert_balance_accept_missing_tolerance(vars.token_x, vars.amount_x, tolerance_x, &mut vars);
    assert_balance_accept_missing_tolerance(vars.token_y, vars.amount_y, tolerance_y, &mut vars);
}

#[test]
fn test_claim_batch_receipt_consumed() {
    let mut vars = setup();

    let mut positions: Vec<(ResourceAddress, Decimal, Decimal)> = vec![];
    let mut ids: BTreeSet<NonFungibleLocalId> = BTreeSet::new();
    for i in 0..5 {
        positions.push((vars.token_y, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }
    for i in 5..10 {
        positions.push((vars.token_x, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    claim_orders(ids, &mut vars).expect_commit_success();

    assert_eq!(
        vars.test_runner.get_component_balance(vars.account_component, vars.order_receipt),
        dec!(0)
    );
}
