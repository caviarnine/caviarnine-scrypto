#![allow(dead_code)]

use radix_engine::transaction::TransactionReceipt;
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;

use crate::common::vars::Vars;
use crate::common::lsu_token_validator::*;

/// This module is for general validator related functions
/// creating validators, registering and staking to them

pub fn fake_lsu_with_meta_data(
    vars: &mut Vars,
    amount: Decimal,
    validator_address: ComponentAddress,
) -> ResourceAddress {
    // set up Babylon FLOOP token
    let manifest = ManifestBuilder::new()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            DIVISIBILITY_MAXIMUM,
            FungibleResourceRoles {
                mint_roles: mint_roles! {
                    minter => rule!(require(global_caller(validator_address)));
                    minter_updater => rule!(deny_all);
                },
                burn_roles: burn_roles! {
                    burner => rule!(require(global_caller(validator_address)));
                    burner_updater => rule!(deny_all);
                },
                ..Default::default()
            },
            metadata!(
                init {
                    "name" => "Liquid Stake Units".to_owned(), locked;
                    "description" => "Liquid Stake Unit tokens that represent a proportion of XRD stake delegated to a Radix Network validator.".to_owned(), locked;
                    "icon_url" => UncheckedUrl("https://assets.radixdlt.com/icons/icon-liquid_stake_units.png".to_owned()), locked;
                    "validator" => GlobalAddress::from(validator_address), locked;
                    "tags" => Vec::<String>::new(), locked;
                }
            ),
            Some(amount),
        )
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    receipt.expect_commit_success().new_resource_addresses()[0]
}

pub fn stake_validator_receipt(
    vars: &mut Vars,
    validator_component_address: ComponentAddress,
    xrd_amount: Decimal,
) -> TransactionReceipt {
    // create a manifest
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component_address, XRD, xrd_amount)
        .take_all_from_worktop(XRD, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.stake_validator(validator_component_address, lookup.bucket("tokens"))
        })
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();

    // println!("receipt: {:?}", receipt);
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn stake_validator(
    vars: &mut Vars,
    validator_component_address: ComponentAddress,
    xrd_amount: Decimal,
) -> ResourceAddress {
    // stake
    let _receipt = stake_validator_receipt(vars, validator_component_address, xrd_amount);

    // get the LSU resource address
    let pool_unit_metadata: MetadataValue = vars
        .test_runner
        .get_metadata(validator_component_address.into(), "pool_unit")
        .unwrap();

    let lsu: ResourceAddress = match pool_unit_metadata {
        MetadataValue::GlobalAddress(address) => address.try_into().unwrap(),
        _ => panic!(),
    };

    // return
    lsu
}

pub fn register_validator_receipt(
    vars: &mut Vars,
    validator_component_address: ComponentAddress,
    update_accept_delegated_stake: bool,
) -> TransactionReceipt {
    let ids = vec![NonFungibleLocalId::bytes(validator_component_address.as_node_id().0).unwrap()];
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            vars.admin_account_component_address,
            VALIDATOR_OWNER_BADGE,
            ids.clone(),
        )
        .register_validator(validator_component_address)
        .call_method(
            validator_component_address,
            "update_accept_delegated_stake",
            manifest_args!(update_accept_delegated_stake),
        )
        .build();

    // println!("receipt: {:?}", receipt);
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn validator_update_accept_delegated_stake(
    vars: &mut Vars,
    validator_component_address: ComponentAddress,
    update_accept_delegated_stake: bool,
) {
    let receipt = register_validator_receipt(
        vars,
        validator_component_address,
        update_accept_delegated_stake,
    );
    // println!("receipt: {:?}", receipt);
    receipt.expect_commit_success();
}

pub fn create_lsu_resource(vars: &mut Vars, amount: Decimal) -> ResourceAddress {
    // create validator component
    let validator_component_address = vars
        .test_runner
        .new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component_address);

    // register validator
    validator_update_accept_delegated_stake(vars, validator_component_address, true);

    // stake validator
    let lsu = stake_validator(vars, validator_component_address, amount);

    // add to active set for token validator
    update_active_set(lsu, true, vars).expect_commit_success();

    lsu
}
