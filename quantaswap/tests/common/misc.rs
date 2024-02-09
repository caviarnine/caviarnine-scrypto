use scrypto::{prelude::*, api::ObjectModuleId};
use transaction::{builder::ManifestBuilder, prelude::ResolvableGlobalAddress};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::*;

pub fn round_down(amount: Decimal, divisibility: u8) -> Decimal {
    amount.checked_round(divisibility, RoundingMode::ToZero).unwrap()
}

pub fn round_up(amount: Decimal, divisibility: u8) -> Decimal {
    amount.checked_round(divisibility, RoundingMode::ToPositiveInfinity).unwrap()
}

pub fn assert_within_error_margin(actual: Decimal, expected: Decimal, margin_percent: Decimal) {
    let margin = expected * margin_percent / dec!(100);
    assert!(
        actual <= expected + margin && actual >= expected - margin,
        "actual: {}, expected: {}, margin: {} diff: {}",
        actual,
        expected,
        margin,
        actual - expected
    );
}

pub fn get_balance(resource: ResourceAddress, vars: &mut Vars) -> Decimal {
    vars.test_runner.get_component_balance(vars.account_component, resource)
}

pub fn assert_balance(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    assert_eq!(
        vars.test_runner.get_component_balance(vars.account_component, resource),
        amount
    );
}

pub fn assert_balance_accept_missing_attos(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    let account_amount = vars.test_runner.get_component_balance(vars.account_component, resource);

    assert!(
        account_amount <= amount &&
        account_amount >= amount - dec!("0.00000000000000001")
    );
}

pub fn set_owner_rule(
    address: impl ResolvableGlobalAddress,
    rule: AccessRule, 
    proof_resource: ResourceAddress, 
    account: ComponentAddress, 
    public_key: Secp256k1PublicKey, 
    vars: &mut Vars,
    ) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, proof_resource, dec!(1))
        .set_owner_role(address, rule)
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("\nSET OWNER RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_role_rule(
    component_address: ComponentAddress, 
    role: impl Into<String>,
    rule: AccessRule, 
    proof_resource: ResourceAddress, 
    account: ComponentAddress, 
    public_key: Secp256k1PublicKey, 
    vars: &mut Vars,
    ) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, proof_resource, dec!(1))
        .set_role(component_address, ObjectModuleId::Main, RoleKey { key: role.into() }, rule)
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("\nSET ROLE RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_metadata(
    address: impl ResolvableGlobalAddress,
    key: impl Into<String>,
    value: impl ToMetadataEntry,
    proof_resource: ResourceAddress,
    account: ComponentAddress,
    public_key: Secp256k1PublicKey,
    vars: &mut Vars,
    ) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, proof_resource, dec!(1))
        .set_metadata(address, key, value)
        .build();

        let receipt = vars.test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        );
        println!("\nSET METADATA\n");
        println!("{:?}", receipt);
        receipt
}
