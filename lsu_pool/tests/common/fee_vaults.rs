use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

pub fn build_manifest(
    fee_vaults_package_address: PackageAddress,
    admin_badge_resource_address: ResourceAddress,
    floop_token: ResourceAddress,
    floop_amount: Decimal,
) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            fee_vaults_package_address,
            "FeeVaults",
            "new",
            manifest_args!(admin_badge_resource_address, floop_token, floop_amount),
        )
        .build()
}
