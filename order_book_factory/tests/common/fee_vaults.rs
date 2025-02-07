use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

pub fn build_manifest(
    fee_vaults_package: PackageAddress, 
    admin_badge: ResourceAddress,
    floop_token: ResourceAddress, 
    floop_amount: Decimal, 
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            fee_vaults_package,
            "FeeVaults",
            "new",
            manifest_args!(admin_badge, floop_token, floop_amount))
        .build()
}