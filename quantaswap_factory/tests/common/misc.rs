use scrypto::prelude::*;

use crate::common::vars::*;

pub fn assert_within_error_margin(actual: Decimal, expected: Decimal, margin_percent: Decimal) {
    let margin = expected * margin_percent / dec!(100);
    assert!(
        actual <= expected + margin && actual >= expected - margin,
        "actual: {}, expected: {}, margin: {} diff: {}",
        actual,
        expected,
        margin,
        actual - expected
    );
}

pub fn get_balance(resource: ResourceAddress, vars: &mut Vars) -> Decimal {
    vars.test_runner.get_component_balance(vars.account_component, resource)
}

pub fn assert_balance(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    assert_eq!(
        vars.test_runner.get_component_balance(vars.account_component, resource),
        amount
    );
}

pub fn assert_balance_accept_missing_attos(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    let account_amount = vars.test_runner
        .get_component_balance(vars.account_component, resource);


    assert!(
        account_amount <= amount &&
        account_amount >= amount - dec!("0.00000000000000001"),
    );
}
