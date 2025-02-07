#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::lsu_token_validator::*;
pub use crate::common::validator::*;

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

    set_require_active(false, true, &mut vars).expect_auth_failure();

    let (_, lsu) = create_validator(&mut vars);

    update_active_set(lsu, true, true, &mut vars).expect_auth_failure();
}
