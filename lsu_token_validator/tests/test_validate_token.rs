#![allow(dead_code)]
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::lsu_token_validator::*;
pub use crate::common::validator::*;

#[test]
fn test_validate_token_active_not_required_valid() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);
    set_require_active(false, true, &mut vars).expect_commit_success();

    validate_token(lsu, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_active_required_valid() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);
    update_active_set(lsu, true, true, &mut vars).expect_commit_success();
    set_require_active(true, true, &mut vars).expect_commit_success();

    validate_token(lsu, &mut vars).expect_commit_success();
}

#[test]
fn test_validate_token_active_required_invalid() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);
    set_require_active(true, true, &mut vars);

    validate_token(lsu, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("LSU must be for an active validator.")
            },
            _ => false,
        }
    });
}
