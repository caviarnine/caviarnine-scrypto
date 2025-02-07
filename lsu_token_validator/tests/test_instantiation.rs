#![allow(dead_code)]
mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::lsu_token_validator::*;

#[test]
fn test_setup() {
    let mut vars = setup();

    assert!(get_require_active(&mut vars));
}
