use scrypto::prelude::*;
use scrypto_unit::*;

use crate::common::vars::Vars;

pub fn setup() -> Vars {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create accounts
    let (public_key, _private_key, account_component_address) = test_runner.new_allocated_account();
    let (admin_public_key, _private_key, admin_account_component_address) =
        test_runner.new_allocated_account();

    // Create admin badge
    let admin_badge_resource_address = test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, admin_account_component_address);

    // Setup the fee_controller
    let fee_controller_package_address = test_runner.compile_and_publish("../fee_controller");

    Vars {
        test_runner,
        public_key,
        account_component_address,
        admin_public_key,
        admin_account_component_address,
        admin_badge_resource_address,
        fee_controller_package_address,
    }
}
