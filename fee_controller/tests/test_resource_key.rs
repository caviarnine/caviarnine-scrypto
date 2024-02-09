#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

use fee_controller::util::*;

#[test]
pub fn test_basic_01() {
    // ARRANGE
    let resources = vec![];

    // ACT
    let key = ResourcesKey::from(resources);

    // // ASSERT
    assert_eq!(key.bytes.len(), 0);
}

#[test]
pub fn test_basic_02() {
    // ARRANGE
    let mut vars = setup();
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_c = vars.test_runner.create_fungible_resource(
        dec!(10000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    let resources = vec![token_a, token_b, token_c];

    // ACT
    let key = ResourcesKey::from(resources);

    // ASSERT
    assert!(key.bytes.len() > 20);
}

#[test]
pub fn test_basic_03() {
    // ARRANGE
    let mut vars = setup();
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let key01 = ResourcesKey::from(vec![token_a]);
    let key02 = ResourcesKey::from(vec![token_a, token_a]);

    // ASSERT
    assert_eq!(key01.bytes, key02.bytes);
}

#[test]
pub fn test_basic_04() {
    // ARRANGE
    let mut vars = setup();
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let key01 = ResourcesKey::from(vec![token_b, token_a]);
    let key02 = ResourcesKey::from(vec![token_a, token_b]);

    // ASSERT
    assert_eq!(key01.bytes, key02.bytes);
}

#[test]
pub fn test_basic_05() {
    // ARRANGE
    let mut vars = setup();
    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_b = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );
    let token_c = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component_address,
    );

    // ACT
    let key01 = ResourcesKey::from(vec![token_b, token_a, token_c]);
    let key02 = ResourcesKey::from(vec![token_a, token_b, token_c]);
    let key03 = ResourcesKey::from(vec![token_c, token_b, token_a]);
    let key04 = ResourcesKey::from(vec![token_a, token_c, token_b]);
    let key05 = ResourcesKey::from(vec![token_b, token_c, token_a]);
    let key06 = ResourcesKey::from(vec![token_c, token_a, token_b]);

    // ASSERT
    assert_eq!(key01.bytes, key02.bytes);
    assert_eq!(key01.bytes, key03.bytes);
    assert_eq!(key01.bytes, key04.bytes);
    assert_eq!(key01.bytes, key05.bytes);
    assert_eq!(key01.bytes, key06.bytes);
}
