use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

use crate::Vars;

pub fn build_manifest(
    fee_controller_package: PackageAddress, 
    admin_badge: ResourceAddress, 
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            fee_controller_package,
            "FeeController",
            "new",
            manifest_args!(admin_badge))
        .build()
}

pub fn get_protocol_fee_default(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.fee_controller_component,
            "get_protocol_fee_default",
            manifest_args!())
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn set_protocol_fee_default_zero(vars: &mut Vars) {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
        .call_method(
            vars.fee_controller_component,
            "set_protocol_fee_default",
            manifest_args!(0u16))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();
}