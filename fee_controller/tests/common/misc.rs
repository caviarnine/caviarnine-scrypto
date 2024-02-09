use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, prelude::ResolvableGlobalAddress};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::Vars;

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
        .set_main_role(component_address, RoleKey { key: role.into() }, rule)
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
