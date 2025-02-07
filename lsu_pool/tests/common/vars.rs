use radix_engine::vm::NoExtension;
use radix_engine_stores::memory_db::InMemorySubstateDatabase;
use scrypto::prelude::*;
use scrypto_unit::TestRunner;

pub struct Vars {
    pub test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    pub public_key: Secp256k1PublicKey,
    pub account_component_address: ComponentAddress,
    pub admin_public_key: Secp256k1PublicKey,
    pub admin_account_component_address: ComponentAddress,
    pub admin_badge_resource_address: ResourceAddress,
    pub fee_vaults_component_address: ComponentAddress,
    pub token_validator_package_address: PackageAddress,
    pub token_validator_component_address: ComponentAddress,
    pub lsu_pool_package_address: PackageAddress,
    pub lsu_pool_component_address: ComponentAddress,
}
