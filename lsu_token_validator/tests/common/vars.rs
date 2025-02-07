use scrypto::prelude::*;
use scrypto_unit::TestRunner;
use scrypto_test::prelude::*;

pub struct Vars {
    pub test_runner: TestRunner<NoExtension, InMemorySubstateDatabase>,
    pub public_key: Secp256k1PublicKey,
    pub account_component: ComponentAddress,
    pub admin_public_key: Secp256k1PublicKey,
    pub admin_account_component: ComponentAddress,
    pub admin_badge: ResourceAddress,
    pub lsu_token_validator_package: PackageAddress,
    pub lsu_token_validator_component: ComponentAddress,
}