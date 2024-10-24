#![allow(dead_code)]
use scrypto::{api::ObjectModuleId, prelude::*};

use ::order_book::events::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::order_book;
pub use crate::common::fee_controller;
use crate::fee_controller::get_protocol_fee_default;

#[test]
fn test_new_order_book_event() {
    let mut vars: Vars = setup();

    let manifest = order_book::build_manifest(
        vars.order_book_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );

    let order_book_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let order_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];
    let events = receipt.expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<NewOrderBookEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<NewOrderBookEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Function(BlueprintId::new(&vars.order_book_package, "OrderBook")),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.component_address, order_book_component);
    assert_eq!(event.order_receipt_address, order_receipt);
    assert_eq!(event.token_x_address, vars.token_x);
    assert_eq!(event.token_y_address, vars.token_y);
}

#[test]
fn test_limit_order_event_x() {
    let mut vars: Vars = setup();

    let events = limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<LimitOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<LimitOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(event.is_ask);
    assert_eq!(event.amount, dec!(1));
    assert_eq!(event.price, dec!(1));
}

#[test]
fn test_limit_order_event_y() {
    let mut vars: Vars = setup();

    let events = limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<LimitOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<LimitOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(!event.is_ask);
    assert_eq!(event.amount, dec!(1));
    assert_eq!(event.price, dec!(1));
}

#[test]
fn test_market_order_event_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(3), &mut vars).expect_commit_success();

    let events = market_order(vars.token_x, dec!(10), None, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<MarketOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<MarketOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert!(!event.is_buy);
    assert_eq!(
        event.fills, 
        vec![(dec!(3), dec!(1) / dec!(3)), (dec!(2), dec!(1) / dec!(2)), (dec!(1), dec!(1))]
    );
}

#[test]
fn test_market_order_event_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(3), &mut vars).expect_commit_success();

    let events = market_order(vars.token_y, dec!(10), None, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<MarketOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<MarketOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert!(event.is_buy);
    assert_eq!(
        event.fills, 
        vec![(dec!(1), dec!(1)), (dec!(2), dec!(1)), (dec!(3), dec!(1))]
    );
}

#[test]
fn test_claim_order_event_unfilled_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    let ids = BTreeSet::from_iter(vec![NonFungibleLocalId::integer(1)]);
    let events = claim_orders(ids, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ClaimOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ClaimOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(event.is_ask);
    assert_eq!(event.amount_canceled, dec!(1));
    assert_eq!(event.amount_filled, dec!(0));
}

#[test]
fn test_claim_order_event_canceled_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    let ids = BTreeSet::from_iter(vec![NonFungibleLocalId::integer(1)]);
    let events = claim_orders(ids, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ClaimOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ClaimOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(event.is_ask);
    assert_eq!(event.price, dec!(2));
    assert_eq!(event.amount_canceled, dec!(1));
    assert_eq!(event.amount_filled, dec!(0));
    assert_eq!(event.amount_x, dec!(1));
    assert_eq!(event.amount_y, dec!(0));
}

#[test]
fn test_claim_order_event_canceled_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();

    let ids = BTreeSet::from_iter(vec![NonFungibleLocalId::integer(1)]);
    let events = claim_orders(ids, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ClaimOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ClaimOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(!event.is_ask);
    assert_eq!(event.price, dec!(2));
    assert_eq!(event.amount_canceled, dec!(1));
    assert_eq!(event.amount_filled, dec!(0));
    assert_eq!(event.amount_x, dec!(0));
    assert_eq!(event.amount_y, dec!(2));
}

#[test]
fn test_claim_order_event_filled_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    market_order(vars.token_y, dec!(2), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from_iter(vec![NonFungibleLocalId::integer(1)]);
    let events = claim_orders(ids, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ClaimOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ClaimOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(event.is_ask);
    assert_eq!(event.price, dec!(2));
    assert_eq!(event.amount_canceled, dec!(0));
    assert_eq!(event.amount_filled, dec!(1));
    assert_eq!(event.amount_x, dec!(0));
    assert_eq!(event.amount_y, dec!(2));
}

#[test]
fn test_claim_order_event_filled_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();
    market_order(vars.token_x, dec!(1), None, &mut vars).expect_commit_success();

    let ids = BTreeSet::from_iter(vec![NonFungibleLocalId::integer(1)]);
    let events = claim_orders(ids, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ClaimOrderEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ClaimOrderEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.order_id, NonFungibleLocalId::integer(1));
    assert!(!event.is_ask);
    assert_eq!(event.price, dec!(2));
    assert_eq!(event.amount_canceled, dec!(0));
    assert_eq!(event.amount_filled, dec!(1));
    assert_eq!(event.amount_x, dec!(1));
    assert_eq!(event.amount_y, dec!(0));
}

#[test]
fn test_protocol_fee_event_x() {
    let mut vars: Vars = setup();
    let fee = get_protocol_fee_default(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(0.5), &mut vars).expect_commit_success();

    let amount = dec!(2);
    let amount_fee = round_up(amount * fee, vars.divisibility_x);
    let events = market_order(vars.token_x, dec!(2), None, &mut vars)
        .expect_commit_success()
        .application_events
        .clone();        

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ProtocolFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ProtocolFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_x);
    assert_eq!(event.amount, amount_fee);
}

#[test]
fn test_protocol_fee_event_y() {
    let mut vars: Vars = setup();
    let fee = get_protocol_fee_default(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    let amount = dec!(2);
    let amount_fee = round_up(amount * fee, vars.divisibility_y);
    let events = market_order(vars.token_y, dec!(2), None, &mut vars)
        .expect_commit_success()
        .application_events
        .clone();        

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ProtocolFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ProtocolFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.order_book_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_y);
    assert_eq!(event.amount, amount_fee);
}
