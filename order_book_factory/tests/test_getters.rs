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
fn test_get_owner_rule_default() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_owner_rule_default(&mut vars),
        rule!(require(vars.admin_badge))
    );
}

#[test]
fn test_get_user_rule_default() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_user_rule_default(&mut vars),
        rule!(allow_all)
    );
}

#[test]
fn test_get_token_validator_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_token_validator_address(&mut vars),
        vars.token_validator_component
    );
}

#[test]
fn test_get_order_book_count_zero() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_book_count(&mut vars),
        0
    );
}

#[test]
fn test_get_order_book_count_few() {
    let mut vars: Vars = setup();

    new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success();
    new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success();
    new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success();

    assert_eq!(
        get_order_book_count(&mut vars),
        3
    );
}

#[test]
fn test_get_order_books_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_books(None, None, &mut vars),
        vec![]
    );
}


#[test]
fn test_get_order_books_few() {
    let mut vars: Vars = setup();

    let order_book_component0 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component1 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component2 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books(None, None, &mut vars),
        vec![order_book_component0, order_book_component1, order_book_component2]
    );
}

#[test]
fn test_get_order_books_start() {
    let mut vars: Vars = setup();

    let _order_book_component0 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component1 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component2 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books(Some(1), None, &mut vars),
        vec![order_book_component1, order_book_component2]
    );
}

#[test]
fn test_get_order_books_end() {
    let mut vars: Vars = setup();

    let order_book_component0 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component1 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let _order_book_component2 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books(None, Some(2), &mut vars),
        vec![order_book_component0, order_book_component1]
    );
}

#[test]
fn test_get_order_books_start_end() {
    let mut vars: Vars = setup();

    let _order_book_component0 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component1 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let _order_book_component2 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books(Some(1), Some(2), &mut vars),
        vec![order_book_component1]
    );
}

#[test]
fn test_get_order_books_end_past() {
    let mut vars: Vars = setup();

    let order_book_component0 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component1 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];
    let order_book_component2 = new_order_book(vars.token_x, vars.token_y, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books(None, Some(10), &mut vars),
        vec![order_book_component0, order_book_component1, order_book_component2]
    );
}

#[test]
fn test_get_order_book_pair_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_book_pair(vars.order_book_component, &mut vars),
        None
    );
}

#[test]
fn test_get_order_book_pair_some() {
    let mut vars: Vars = setup();

    let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
    let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_book_pair(order_book_component, &mut vars),
        Some((vars.token_x, vars.token_y))
    );
}

#[test]
fn test_get_order_books_by_pair_empty() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_order_books_by_pair_one() {
    let mut vars: Vars = setup();

    let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
    let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![order_book_component]
    );
}

#[test]
fn test_get_order_books_by_pair_few() {
    let mut vars: Vars = setup();

    let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
    let order_book_component_1 = receipt.expect_commit_success().new_component_addresses()[0];

    let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
    let order_book_component_2 = receipt.expect_commit_success().new_component_addresses()[0];

    let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
    let order_book_component_3 = receipt.expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![order_book_component_1, order_book_component_2, order_book_component_3]
    );
}

#[test]
fn test_get_order_books_by_pair_many() {
    let mut vars: Vars = setup();
    
    let mut order_books: Vec<ComponentAddress> = vec![];
    for _ in 0..100 {
        let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
        let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];
        order_books.push(order_book_component);
    }

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        order_books
    );
}

#[test]
fn test_get_order_books_by_pair_start() {
    let mut vars: Vars = setup();
    
    let mut order_books: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
        let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];
        order_books.push(order_book_component);
    }

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, Some(3), None, &mut vars),
        order_books[3..].to_vec()
    );
}

#[test]
fn test_get_order_books_by_pair_end() {
    let mut vars: Vars = setup();
    
    let mut order_books: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
        let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];
        order_books.push(order_book_component);
    }

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, None, Some(7), &mut vars),
        order_books[..7].to_vec()
    );
}

#[test]
fn test_get_order_books_by_pair_start_end() {
    let mut vars: Vars = setup();
    
    let mut order_books: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_order_book(vars.token_x, vars.token_y, &mut vars);
        let order_book_component = receipt.expect_commit_success().new_component_addresses()[0];
        order_books.push(order_book_component);
    }

    assert_eq!(
        get_order_books_by_pair(vars.token_x, vars.token_y, Some(2), Some(6), &mut vars),
        order_books[2..6].to_vec()
    );
}