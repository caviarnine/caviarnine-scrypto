use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

pub fn build_manifest(
    token_validator_package: PackageAddress,
    admin_badge: ResourceAddress,
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            token_validator_package,
            "TokenValidator",
            "new",
            manifest_args!(admin_badge))
        .build()
}
