use crate::common::vars::Vars;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

pub fn setup() -> Vars {
    // Setup the environment
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();

    // Create accounts
    let (public_key, _private_key, account_component_address) = test_runner.new_allocated_account();
    let (admin_public_key, _private_key, admin_account_component_address) =
        test_runner.new_allocated_account();

    // FLOOP token
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: None,
                burn_roles: burn_roles!(
                    burner => AccessRule::AllowAll;
                    burner_updater => AccessRule::DenyAll;
                ),
                freeze_roles: None,
                recall_roles: None,
                withdraw_roles: None,
                deposit_roles: None,
            },
            metadata!(
                init {
                    "name" => "FLOOP", locked;
                    "symbol" => "FLOOP", locked;
                    "description" => "FLOOP description", locked;
                }
            ),
            Some(dec!(1000)),
        )
        .call_method(
            account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    let token_floop_new_resource_address =
        receipt.expect_commit_success().new_resource_addresses()[0];

    // create Admin Badge NFT
    let admin_badge_resource_address = test_runner.create_fungible_resource(dec!(1), DIVISIBILITY_NONE, admin_account_component_address);

    // Setup the fee_vaults package
    let fee_vaults_package_address = test_runner.compile_and_publish("../fee_vaults");

    Vars {
        test_runner,
        public_key,
        account_component_address,
        admin_public_key,
        admin_account_component_address,
        admin_badge_resource_address,
        fee_vaults_package_address,
        token_floop_new_resource_address,
    }
}
