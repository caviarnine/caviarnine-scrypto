use scrypto::prelude::*;
use scrypto_unit::*;

use super::token_validator;
use super::vars::Vars;

pub fn setup() -> Vars {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create accounts
    let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
    let (admin_public_key, _private_key, admin_account_component) =
        test_runner.new_allocated_account();

    // Create admin Badge
    let admin_badge = test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, admin_account_component);

    // Publish token_validator
    let token_validator_package = test_runner.compile_and_publish("../token_validator");

    // Create token_validator
    let manifest = token_validator::build_manifest(
        token_validator_package,
        admin_badge,
    );
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&admin_public_key)],
    );
    receipt.expect_commit_success();

    let token_validator_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];


    Vars {
        test_runner,
        public_key,
        account_component,
        admin_public_key,
        admin_account_component,
        admin_badge,
        token_validator_package,
        token_validator_component,
    }
}
