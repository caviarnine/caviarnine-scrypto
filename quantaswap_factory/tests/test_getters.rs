#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap_factory::*;
pub use crate::common::quantaswap_factory;
pub use crate::common::fee_vaults;
pub use crate::common::fee_controller;
pub use crate::common::token_validator;
pub use crate::common::quantaswap;

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
        AccessRule::AllowAll
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
fn test_get_pool_count_zero() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_pool_count(&mut vars),
        0
    );
}

#[test]
fn test_get_pool_count_few() {
    let mut vars: Vars = setup();

    new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success();
    new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success();
    new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success();

    assert_eq!(
        get_pool_count(&mut vars),
        3
    );
}

#[test]
fn test_get_pools_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_pools(None, None, &mut vars),
        vec![]
    );
}


#[test]
fn test_get_pools_few() {
    let mut vars: Vars = setup();

    let pool_component0 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component1 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component2 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_pools(None, None, &mut vars),
        vec![pool_component0, pool_component1, pool_component2]
    );
}

#[test]
fn test_get_pools_start() {
    let mut vars: Vars = setup();

    let _pool_component0 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component1 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component2 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_pools(Some(1), None, &mut vars),
        vec![pool_component1, pool_component2]
    );
}

#[test]
fn test_get_pools_end() {
    let mut vars: Vars = setup();

    let pool_component0 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component1 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let _pool_component2 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_pools(None, Some(2), &mut vars),
        vec![pool_component0, pool_component1]
    );
}

#[test]
fn test_get_pools_start_end() {
    let mut vars: Vars = setup();

    let _pool_component0 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component1 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let _pool_component2 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_pools(Some(1), Some(2), &mut vars),
        vec![pool_component1]
    );
}

#[test]
fn test_get_pools_end_past() {
    let mut vars: Vars = setup();

    let pool_component0 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component1 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];
    let pool_component2 = new_pool(vars.token_x, vars.token_y, 1, &mut vars).expect_commit_success().new_component_addresses()[0];

    assert_eq!(
        get_pools(None, Some(10), &mut vars),
        vec![pool_component0, pool_component1, pool_component2]
    );
}

#[test]
fn test_get_pool_pair_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_pool_pair(vars.quantaswap_component, &mut vars),
        None
    );
}

#[test]
fn test_get_pool_pair_some() {
    let mut vars: Vars = setup();

    let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
    let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);

    assert_eq!(
        get_pool_pair(quantaswap_component, &mut vars),
        Some((vars.token_x, vars.token_y))
    );
}

#[test]
fn test_get_pools_by_pair_empty() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_get_pools_by_pair_one() {
    let mut vars: Vars = setup();

    let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
    let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![quantaswap_component]
    );
}

#[test]
fn test_get_pools_by_pair_few() {
    let mut vars: Vars = setup();

    let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
    let quantaswap_component_1 = receipt.expect_commit_success().output::<ComponentAddress>(1);

    let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
    let quantaswap_component_2 = receipt.expect_commit_success().output::<ComponentAddress>(1);

    let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
    let quantaswap_component_3 = receipt.expect_commit_success().output::<ComponentAddress>(1);

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        vec![quantaswap_component_1, quantaswap_component_2, quantaswap_component_3]
    );
}

#[test]
fn test_get_pools_by_pair_many() {
    let mut vars: Vars = setup();
    
    let mut pools: Vec<ComponentAddress> = vec![];
    for _ in 0..100 {
        let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
        let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);
        pools.push(quantaswap_component);
    }

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, None, None, &mut vars),
        pools
    );
}

#[test]
fn test_get_pools_by_pair_start() {
    let mut vars: Vars = setup();
    
    let mut pools: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
        let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);
        pools.push(quantaswap_component);
    }

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, Some(3), None, &mut vars),
        pools[3..].to_vec()
    );
}

#[test]
fn test_get_pools_by_pair_end() {
    let mut vars: Vars = setup();
    
    let mut pools: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
        let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);
        pools.push(quantaswap_component);
    }

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, None, Some(7), &mut vars),
        pools[..7].to_vec()
    );
}

#[test]
fn test_get_pools_by_pair_start_end() {
    let mut vars: Vars = setup();
    
    let mut pools: Vec<ComponentAddress> = vec![];
    for _ in 0..10 {
        let receipt = new_pool(vars.token_x, vars.token_y, 1, &mut vars);
        let quantaswap_component = receipt.expect_commit_success().output::<ComponentAddress>(1);
        pools.push(quantaswap_component);
    }

    assert_eq!(
        get_pools_by_pair(vars.token_x, vars.token_y, Some(2), Some(6), &mut vars),
        pools[2..6].to_vec()
    );
}