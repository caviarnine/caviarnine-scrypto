use scrypto::prelude::*;
use scrypto_unit::TestRunner;
use radix_engine::vm::NoExtension;
use radix_engine_stores::memory_db::InMemorySubstateDatabase;

pub struct Vars {
    pub test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    pub public_key: Secp256k1PublicKey,
    pub admin_public_key: Secp256k1PublicKey,
    pub account_component: ComponentAddress,
    pub admin_account_component: ComponentAddress,
    pub fee_controller_package: PackageAddress,
    pub fee_controller_component: ComponentAddress,
    pub fee_vaults_package: PackageAddress,
    pub fee_vaults_component: ComponentAddress,
    pub quantaswap_package: PackageAddress,
    pub quantaswap_component: ComponentAddress,
    pub admin_badge: ResourceAddress,
    pub floop_token: ResourceAddress,
    pub token_x: ResourceAddress,
    pub amount_x: Decimal,
    pub divisibility_x: u8,
    pub token_y: ResourceAddress,
    pub amount_y: Decimal,
    pub divisibility_y: u8,
    pub liquidity_receipt: ResourceAddress,
    pub bin_span: u32,
}
