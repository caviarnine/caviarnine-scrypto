use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

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
