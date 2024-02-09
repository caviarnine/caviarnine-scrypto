#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::token_validator::*;

#[test]
fn test_get_restrict_recallable() {
    let mut vars = setup();

    set_restrict_recallable(true, true, &mut vars).expect_commit_success();

    assert!(get_restrict_recallable(&mut vars));

    set_restrict_recallable(false, true, &mut vars).expect_commit_success();

    assert!(!get_restrict_recallable(&mut vars));
}

#[test]
fn test_get_restrict_freezable() {
    let mut vars = setup();

    set_restrict_freezable(true, true, &mut vars).expect_commit_success();

    assert!(get_restrict_freezable(&mut vars));

    set_restrict_freezable(false, true, &mut vars).expect_commit_success();

    assert!(!get_restrict_freezable(&mut vars));
}

#[test]
fn test_get_white_listed() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);

    assert!(!get_white_listed(token, &mut vars));

    update_white_list(token, true, true, &mut vars).expect_commit_success();

    assert!(get_white_listed(token, &mut vars));
}

#[test]
fn test_get_black_listed() {
    let mut vars = setup();

    let token = vars.test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_MAXIMUM, vars.account_component);

    assert!(!get_black_listed(token, &mut vars));

    update_black_list(token, true, true, &mut vars).expect_commit_success();

    assert!(get_black_listed(token, &mut vars));
}

#[test]
fn test_get_minimum_divisibility() {
    let mut vars = setup();

    set_minimum_divisibility(1, true, &mut vars).expect_commit_success();

    assert_eq!(get_minimum_divisibility(&mut vars), 1);
}