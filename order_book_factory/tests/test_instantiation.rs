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
fn test_setup() {
    let _vars = setup();
}

#[test]
fn test_instantiation() {
    let mut vars = setup();

    let manifest = order_book_factory::build_manifest(
        vars.order_book_factory_package,
        vars.admin_badge,
        vars.token_validator_component,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    receipt.expect_commit_success();

    vars.order_book_factory_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];

    assert_eq!(
        get_owner_rule_default(&mut vars),
        rule!(require(vars.admin_badge))
    );

    assert_eq!(
        get_user_rule_default(&mut vars),
        rule!(allow_all)
    );

    assert_eq!(
        get_token_validator_address(&mut vars),
        vars.token_validator_component
    );
}
