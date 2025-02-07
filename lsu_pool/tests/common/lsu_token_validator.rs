use scrypto::prelude::*;
use radix_engine::transaction::TransactionReceipt;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

use super::vars::*;

pub fn build_manifest(
    lsu_token_validator_package: PackageAddress, 
    admin_badge: ResourceAddress, 
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            lsu_token_validator_package,
            "LsuTokenValidator",
            "new",
            manifest_args!(admin_badge))
        .build()
}

pub fn update_active_set(resource_address: ResourceAddress, contains: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
        .call_method(
            vars.token_validator_component_address,
            "update_active_set",
            manifest_args!(resource_address, contains),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );

    receipt
}
