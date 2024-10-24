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

    // Create tokens
    let token_floop_old = test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        account_component_address,
    );
    let token_caviar_old = test_runner.create_fungible_resource(
        dec!(1000000000),
        DIVISIBILITY_MAXIMUM,
        account_component_address,
    );

    // create Admin Badge
    let admin_badge_resource_address =
        test_runner.create_non_fungible_resource(admin_account_component_address);

    // Setup the TOKEN BRIDGE
    let token_bridge_package_address = test_runner.compile_and_publish("../token_bridge");

    Vars {
        test_runner,
        public_key,
        account_component_address,
        admin_public_key,
        admin_account_component_address,
        admin_badge_resource_address,
        token_bridge_package_address,
        token_floop_old,
        token_caviar_old,
    }
}
