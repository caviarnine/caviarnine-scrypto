#![allow(dead_code)]
mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::lsu_token_validator::*;
pub use crate::common::validator::*;

#[test]
fn test_get_in_active_set() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);

    assert!(!get_in_active_set(lsu, &mut vars));
}

#[test]
fn test_get_require_active() {
    let mut vars = setup();

    assert!(get_require_active(&mut vars));
}

#[test]
fn test_get_is_lsu_token() {
    let mut vars = setup();

    let (_, lsu) = create_validator(&mut vars);

    assert!(get_is_lsu_token(lsu, &mut vars));
}