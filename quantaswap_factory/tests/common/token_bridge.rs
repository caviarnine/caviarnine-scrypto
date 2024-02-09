use radix_engine::transaction::TransactionReceipt;
use scrypto::prelude::*;
use scrypto_unit::TestRunner;
use transaction::{builder::ManifestBuilder, model::TransactionManifest};

pub fn new_manifest(
    token_bridge_package: PackageAddress,
    old_resource_address: ResourceAddress,
    bridge_token_name: String,
    bridge_token_symbol: String,
    bridge_token_description: String,
) -> TransactionManifest {
    ManifestBuilder::new()
        .call_function(
            token_bridge_package,
            "TokenBridge",
            "instantiate_bridge",
            manifest_args!(
                old_resource_address,
                bridge_token_name,
                bridge_token_symbol,
                bridge_token_description
            ),
        )
        .build()
}

pub fn get_bridge_token_resource_address(
    test_runner: &mut TestRunner,
    public_key: EcdsaSecp256k1PublicKey,
    token_bridge_component: ComponentAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            token_bridge_component,
            "bridge_token_resource_address",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    receipt
}
