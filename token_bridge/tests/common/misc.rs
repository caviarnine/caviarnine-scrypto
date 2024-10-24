use crate::common::vars::*;
use scrypto::prelude::*;

pub fn assert_almost_equal(a: Decimal, b: Decimal, d: Decimal) {
    if a.is_zero() {
        assert!(b.is_zero());
    } else {
        assert!((a - b).checked_abs().unwrap() / a < d);
    }
}

pub fn assert_balance(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    assert_eq!(
        vars.test_runner
            .get_component_resources(vars.account_component_address)
            .get(&resource)
            .cloned()
            .unwrap(),
        amount
    );
}
