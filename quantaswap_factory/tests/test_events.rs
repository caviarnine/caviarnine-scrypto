#![allow(dead_code)]
use scrypto::{api::ObjectModuleId, prelude::*};

use ::quantaswap_factory::events::*;

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
fn test_set_owner_rule_default_event() {
    let mut vars: Vars = setup();

    let events = set_owner_rule_default(AccessRule::DenyAll, true, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SetOwnerRuleDefaultEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SetOwnerRuleDefaultEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_factory_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(
        event.owner_rule_default,
        AccessRule::DenyAll
    );
}

#[test]
fn test_set_user_rule_default_event() {
    let mut vars: Vars = setup();

    let events = set_user_rule_default(AccessRule::DenyAll, true, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SetUserRuleDefaultEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SetUserRuleDefaultEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_factory_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(
        event.user_rule_default,
        AccessRule::DenyAll
    );
}

#[test]
fn test_set_token_validator_event() {
    let mut vars: Vars = setup();

    let events = set_token_validator(vars.token_validator_component, true, &mut vars).expect_commit_success().application_events.clone();

    let (event_type_identifier, event_data) = events
        .into_iter()
        .find(|(event_type_identifier, _)| {
            vars.test_runner.is_event_name_equal::<SetTokenValidatorEvent>(event_type_identifier)
        }).expect("Event not found");

    let event = scrypto_decode::<SetTokenValidatorEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_factory_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(
        event.token_validator_address,
        vars.token_validator_component
    );
}

#[test]
fn test_new_pool_event() {
    let mut vars: Vars = setup();

    let receipt = new_pool(
        vars.token_x,
        vars.token_y,
        10,
        &mut vars,
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
            vars.test_runner.is_event_name_equal::<NewPoolEvent>(event_type_identifier) &&
            event_type_identifier.0 == Emitter::Method(*vars.quantaswap_factory_component.as_node_id(), ObjectModuleId::Main)
        }).expect("Event not found");

    let event = scrypto_decode::<NewPoolEvent>(&event_data).unwrap();

    assert_eq!(
        event_type_identifier,
        EventTypeIdentifier(
            Emitter::Method(*vars.quantaswap_factory_component.as_node_id(), ObjectModuleId::Main),
            event_type_identifier.1.clone(),
        )
    );

    assert_eq!(event.component_address, quantaswap_component);
    assert_eq!(event.liquidity_receipt_address, liquidity_receipt);
    assert_eq!(event.token_x_address, vars.token_x);
    assert_eq!(event.token_y_address, vars.token_y);
    assert_eq!(event.bin_span, 10);
}
