#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::lsu_token_validator::*;
pub use crate::common::validator::*;

#[test]
fn test_update_active_set_add() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);
    update_active_set(lsu, true, true, &mut vars).expect_commit_success();
}

#[test]
fn test_update_active_set_add_not_lsu_invalid() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);
    update_active_set(token, true, true, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Can only add LSUs to the active set.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_update_active_set_remove() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);

    assert!(!get_in_active_set(lsu, &mut vars));

    update_active_set(lsu, false, true, &mut vars).expect_commit_success();

    assert!(!get_in_active_set(lsu, &mut vars));

    update_active_set(lsu, true, true, &mut vars).expect_commit_success();

    assert!(get_in_active_set(lsu, &mut vars));

    update_active_set(lsu, false, true, &mut vars).expect_commit_success();

    assert!(!get_in_active_set(lsu ,&mut vars));
}

#[test]
fn test_set_require_active() {
    let mut vars = setup();

    assert!(get_require_active(&mut vars));

    set_require_active(false, true, &mut vars).expect_commit_success();

    assert!(!get_require_active(&mut vars));

    set_require_active(true, true, &mut vars).expect_commit_success();

    assert!(get_require_active(&mut vars));
}
