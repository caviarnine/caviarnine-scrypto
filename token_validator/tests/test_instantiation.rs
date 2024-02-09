#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::token_validator::*;

#[test]
fn test_setup() {
    let mut vars = setup();

    assert!(get_restrict_recallable(&mut vars));
    assert!(get_restrict_freezable(&mut vars));

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);
    assert!(!get_white_listed(token, &mut vars));
    assert!(!get_black_listed(token, &mut vars));
}
