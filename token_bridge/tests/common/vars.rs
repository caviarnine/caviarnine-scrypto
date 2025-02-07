use scrypto::prelude::*;
use scrypto_unit::TestRunner;
use radix_engine::vm::NoExtension;
use radix_engine_stores::memory_db::InMemorySubstateDatabase;

pub struct Vars {
    pub test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    pub public_key: Secp256k1PublicKey,
    pub account_component_address: ComponentAddress,
    pub admin_public_key: Secp256k1PublicKey,
    pub admin_account_component_address: ComponentAddress,
    pub admin_badge_resource_address: ResourceAddress,
    pub token_bridge_package_address: PackageAddress,
    pub token_floop_old: ResourceAddress,
    pub token_caviar_old: ResourceAddress,
}
