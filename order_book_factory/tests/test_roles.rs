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
fn test_set_owner_rule() {
    let mut vars: Vars = setup();

    set_owner_rule(AccessRule::DenyAll, false, &mut vars).expect_auth_failure();

    set_owner_rule(AccessRule::DenyAll, true, &mut vars).expect_commit_success();
    set_owner_rule(AccessRule::DenyAll, true, &mut vars).expect_auth_failure();
}

#[test]
fn test_set_owner_rule_setters() {
    let mut vars: Vars = setup();

    set_owner_rule(AccessRule::DenyAll, true, &mut vars).expect_commit_success();

    set_owner_rule_default(AccessRule::DenyAll, true, &mut vars).expect_auth_failure();
    set_user_rule_default(AccessRule::DenyAll, true, &mut vars).expect_auth_failure();
    set_token_validator(vars.token_validator_component, true, &mut vars).expect_auth_failure();
}

#[test]
fn test_set_role_rule_valid() {
    let mut vars: Vars = setup();

    set_role_rule("user".to_string(), AccessRule::DenyAll, true, &mut vars).expect_commit_success();

    new_order_book(vars.token_x, vars.token_y, &mut vars).expect_auth_failure();
}

#[test]
fn test_set_role_rule_invalid() {
    let mut vars: Vars = setup();

    set_role_rule("user".to_string(), AccessRule::DenyAll, false, &mut vars).expect_auth_failure();
}