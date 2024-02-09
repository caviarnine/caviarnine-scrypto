#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::token_validator::*;

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

    set_restrict_recallable(false, true, &mut vars).expect_auth_failure();
    set_restrict_freezable(false, true, &mut vars).expect_auth_failure();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);
    update_white_list(token, true, true, &mut vars).expect_auth_failure();
    update_black_list(token, true, true, &mut vars).expect_auth_failure();
}
