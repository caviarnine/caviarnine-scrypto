#![allow(dead_code)]
use scrypto::{api::ObjectModuleId, prelude::*};
use transaction::builder::ManifestBuilder;

use ::quantaswap::events::*;
use ::quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;
pub use crate::common::quantaswap;

#[test]
fn test_new_pool_event() {
    let mut vars: Vars = setup();

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        5
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );

    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];
    let events = receipt.expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<NewPoolEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<NewPoolEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Function(BlueprintId::new(&vars.quantaswap_package, "QuantaSwap")),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.component_address, quantaswap_component);
    assert_eq!(event.liquidity_receipt_address, liquidity_receipt);
    assert_eq!(event.token_x_address, vars.token_x);
    assert_eq!(event.token_y_address, vars.token_y);
    assert_eq!(event.bin_span, 5);
}

#[test]
fn test_mint_liquidity_receipt_event() {
    let mut vars: Vars = setup();

    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "mint_liquidity_receipt",
            manifest_args!(),
        )
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    let liquidity_receipt_id = receipt.expect_commit_success()
        .vault_balance_changes().clone()
        .into_iter()
        .find(|(_, (address, _))| address == &vars.liquidity_receipt).unwrap().1.1
        .added_non_fungibles().pop_first().unwrap();

    let events = receipt.expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<MintLiquidityReceiptEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<MintLiquidityReceiptEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, liquidity_receipt_id);
}

#[test]
fn test_burn_liquidity_receipt_event() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let receipt = burn_liquidity_receipt(id.clone(), &mut vars);

    let events = receipt.expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<BurnLiquidityReceiptEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<BurnLiquidityReceiptEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
}

#[test]
fn test_add_liquidity_event_initial() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");

    let events = add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.added_x, vec![(tick, amount_x)]);
    assert_eq!(event.added_y, vec![(tick, amount_y)]);
}

#[test]
fn test_add_liquidity_event_active() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(2);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(4);
    let amount_y1 = dec!(3);
    
    let events = add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, dec!(4));
    assert_eq!(event.amount_change_y, dec!(2));
    assert_eq!(event.added_x, vec![(tick0, dec!(4))]);
    assert_eq!(event.added_y, vec![(tick0, dec!(2))]);
}

#[test]
fn test_add_liquidity_event_above() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
   
    let events = add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, amount_x1);
    assert_eq!(event.amount_change_y, dec!(0));
    assert_eq!(event.added_x, vec![(tick1, amount_x1)]);
    assert_eq!(event.added_y, vec![]);
}

#[test]
fn test_add_liquidity_event_below() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(2);
    
    let events = add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
    .into_iter()
    .find(|(event_type_identifier, _)| {
        vars.test_runner.is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
    }).expect("Event not found");

    let event = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, dec!(0));
    assert_eq!(event.amount_change_y, amount_y1);
    assert_eq!(event.added_x, vec![]);
    assert_eq!(event.added_y, vec![(tick1, amount_y1)]);
}

#[test]
fn test_add_liquidity_event_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);

    let tick2 = tick0 - vars.bin_span;
    let amount_y2 = dec!(1);
    
    let positions = vec![(tick0, amount_x0, amount_y0), (tick1, amount_x1, dec!(0)), (tick2, dec!(0), amount_y2)];
    let events = add_liquidity_to_receipt(id.clone(), dec!(10), dec!(10), positions.clone(), &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<AddLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<AddLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, amount_x0 + amount_x1);
    assert_eq!(event.amount_change_y, amount_y0 + amount_y2);
    assert_eq!(event.added_x, vec![(tick0, amount_x0), (tick1, amount_x1)]);
    assert_eq!(event.added_y, vec![(tick0, amount_y0), (tick2, amount_y2)]);
}

#[test]
fn test_remove_liquidity_event_active() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(2);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let events = remove_specific_liquidity(id.clone(), vec![(tick0, amount_x0)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<RemoveLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<RemoveLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, -amount_x0);
    assert_eq!(event.amount_change_y, -amount_y0);
    assert_eq!(event.removed_x, vec![(tick0, -amount_x0)]);
    assert_eq!(event.removed_y, vec![(tick0, -amount_y0)]);
}

#[test]
fn test_remove_liquidity_event_above() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(2);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), dec!(10), dec!(10), vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();
    
    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(4);
    add_liquidity_to_receipt(id.clone(), amount_x1, dec!(0), vec![(tick1, amount_x1, dec!(0))], &mut vars).expect_commit_success();

    let events = remove_specific_liquidity(id.clone(), vec![(tick1, amount_x1)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<RemoveLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<RemoveLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, -amount_x1);
    assert_eq!(event.amount_change_y, dec!(0));
    assert_eq!(event.removed_x, vec![(tick1, -amount_x1)]);
    assert_eq!(event.removed_y, vec![]);
}

#[test]
fn test_remove_liquidity_event_below() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(2);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), dec!(10), dec!(10), vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();
    
    let tick1 = tick0 - vars.bin_span;
    let amount_y1 = dec!(4);
    add_liquidity_to_receipt(id.clone(), dec!(0), amount_y1, vec![(tick1, dec!(0), amount_y1)], &mut vars).expect_commit_success();

    let events = remove_specific_liquidity(id.clone(), vec![(tick1, amount_y1)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<RemoveLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<RemoveLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, dec!(0));
    assert_eq!(event.amount_change_y, -amount_y1);
    assert_eq!(event.removed_x, vec![]);
    assert_eq!(event.removed_y, vec![(tick1, -amount_y1)]);
}

#[test]
fn test_remove_liquidity_event_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);

    let tick2 = tick0 - vars.bin_span;
    let amount_y2 = dec!(1);
    
    let positions = vec![(tick0, amount_x0, amount_y0), (tick1, amount_x1, dec!(0)), (tick2, dec!(0), amount_y2)];
    add_liquidity_to_receipt(id.clone(), dec!(10), dec!(10), positions.clone(), &mut vars).expect_commit_success();

    let events = remove_specific_liquidity(id.clone(), vec![(tick1, amount_x1), (tick2, amount_y2), (tick0, amount_x0)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<RemoveLiquidityEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<RemoveLiquidityEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.liquidity_receipt_id, id);
    assert_eq!(event.amount_change_x, -amount_x1 - amount_x0);
    assert_eq!(event.amount_change_y, -amount_y2 - amount_y0);
    assert_eq!(event.removed_x, vec![(tick1, -amount_x1), (tick0, -amount_x0)]);
    assert_eq!(event.removed_y, vec![(tick2, -amount_y2), (tick0, -amount_y0)]);
}

#[test]
fn test_swap_event_x() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);

    let events = swap(vars.token_x, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SwapEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SwapEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(event.amount_change_x, -change_balance_x);
    assert_eq!(event.amount_change_y, -change_balance_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_swap_event_x_with_remaining() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    fee_controller::set_liquidity_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);
    
    let events = swap(vars.token_x, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SwapEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SwapEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(event.amount_change_x, -change_balance_x);
    assert_eq!(event.amount_change_y, -change_balance_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_swap_event_y() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(1);

    let events = swap(vars.token_y, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SwapEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SwapEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(event.amount_change_x, -change_balance_x);
    assert_eq!(event.amount_change_y, -change_balance_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_swap_event_y_with_remaining() {
    let mut vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let balance_x = get_balance(vars.token_x, &mut vars);
    let balance_y = get_balance(vars.token_y, &mut vars);

    let amount = dec!(10);

    let events = swap(vars.token_y, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SwapEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SwapEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_price = get_price(&mut vars).unwrap();
    let new_balance_x = get_balance(vars.token_x, &mut vars);
    let new_balance_y = get_balance(vars.token_y, &mut vars);
    
    let change_balance_x = new_balance_x - balance_x;
    let change_balance_y = new_balance_y - balance_y;

    assert_eq!(event.amount_change_x, -change_balance_x);
    assert_eq!(event.amount_change_y, -change_balance_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_valuation_event_add_liquidity() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(1);

    let events = add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ValuationEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ValuationEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_price = get_price(&mut vars).unwrap();

    assert_eq!(event.amount_after_x, amount_x);
    assert_eq!(event.amount_after_y, amount_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_valuation_event_remove_liquidity() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let events = remove_specific_liquidity(id.clone(), vec![(tick, amount_x)], &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ValuationEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ValuationEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.amount_after_x, dec!(0));
    assert_eq!(event.amount_after_y, dec!(0));
    assert_eq!(event.price_after, dec!(0));
}

#[test]
fn test_valuation_event_swap_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let events = swap(vars.token_x, dec!(1), &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ValuationEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ValuationEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_amount_x = get_amount_x(&mut vars);
    let new_amount_y = get_amount_y(&mut vars);
    let new_price = get_price(&mut vars).unwrap();

    assert_eq!(event.amount_after_x, new_amount_x);
    assert_eq!(event.amount_after_y, new_amount_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_valuation_event_swap_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let events = swap(vars.token_y, dec!(1), &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ValuationEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ValuationEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    let new_amount_x = get_amount_x(&mut vars);
    let new_amount_y = get_amount_y(&mut vars);
    let new_price = get_price(&mut vars).unwrap();

    assert_eq!(event.amount_after_x, new_amount_x);
    assert_eq!(event.amount_after_y, new_amount_y);
    assert_eq!(event.price_after, new_price);
}

#[test]
fn test_protocol_fee_event_swap_x() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    let amount_fee = round_up(amount * fee, vars.divisibility_x);
    let events = swap(vars.token_x, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ProtocolFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ProtocolFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_x);
    assert_eq!(event.amount, amount_fee);
}

#[test]
fn test_protocol_fee_event_swap_y() {
    let mut vars: Vars = setup();
    let fee = fee_controller::get_protocol_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    let amount_fee = round_up(amount * fee, vars.divisibility_y);
    let events = swap(vars.token_y, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<ProtocolFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<ProtocolFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_y);
    assert_eq!(event.amount, amount_fee);
}

#[test]
fn test_liquidity_fee_event_swap_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    let amount_fee = round_up(amount * fee, vars.divisibility_x);
    let events = swap(vars.token_x, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<LiquidityFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<LiquidityFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_x);
    assert_eq!(event.amount, amount_fee);
}

#[test]
fn test_liquidity_fee_event_swap_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);
    let fee = fee_controller::get_liquidity_fee_default(&mut vars);

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(5);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let amount = dec!(1);
    let amount_fee = round_up(amount * fee, vars.divisibility_y);
    let events = swap(vars.token_y, amount, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<LiquidityFeeEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<LiquidityFeeEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.token_address, vars.token_y);
    assert_eq!(event.amount, amount_fee);
}
