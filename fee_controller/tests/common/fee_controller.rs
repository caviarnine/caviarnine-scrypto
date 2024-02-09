use crate::common::vars::Vars;
use radix_engine::transaction::TransactionReceipt;
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;

pub fn new_fee_controller_manifest_receipt(vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_function(
            vars.fee_controller_package_address,
            "FeeController",
            "new",
            manifest_args!(vars.admin_badge_resource_address),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

pub fn new_fee_controller_manifest(vars: &mut Vars) -> ComponentAddress {
    let receipt = new_fee_controller_manifest_receipt(vars);
    receipt.expect_commit(true).output::<ComponentAddress>(1)
}

// generic method - no arguments
pub fn get_method_with_no_input_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(fee_controller_component, method_name, manifest_args!())
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// generic method - package address
pub fn get_method_with_package_address_input_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
    package_address: PackageAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            fee_controller_component,
            method_name,
            manifest_args!(package_address),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// generic method - resource address
pub fn get_method_with_vec_resource_address_input_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
    resource_addresses: Vec<ResourceAddress>,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            fee_controller_component,
            method_name,
            manifest_args!(resource_addresses),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// get fees receipt
pub fn get_fees_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
    package_address: PackageAddress,
    resource_addresses: Vec<ResourceAddress>,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            fee_controller_component,
            method_name,
            manifest_args!(package_address, resource_addresses),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// generic setter
pub fn set_method_with_u16_input_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
    with_proof: bool,
    input: u16,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1))
            .call_method(fee_controller_component, method_name, manifest_args!(input))
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(fee_controller_component, method_name, manifest_args!(input))
            .build()
    };
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// set protocol fee receipt
pub fn set_protocol_fee_receipt_with_proof(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    with_proof: bool,
    package_address: PackageAddress,
    fee: u16,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1))
            .call_method(
                fee_controller_component,
                "set_protocol_fee",
                manifest_args!(package_address, fee),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                fee_controller_component,
                "set_protocol_fee",
                manifest_args!(package_address, fee),
            )
            .build()
    };
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// set liquidity fee receipt
pub fn set_liquidity_fee_receipt_with_proof(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    with_proof: bool,
    resource_addresses: Vec<ResourceAddress>,
    fee: u16,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1))
            .call_method(
                fee_controller_component,
                "set_liquidity_fee",
                manifest_args!(resource_addresses, fee),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                fee_controller_component,
                "set_liquidity_fee",
                manifest_args!(resource_addresses, fee),
            )
            .build()
    };
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("{:?}", receipt);
    receipt
}

// SET methods
pub fn set_protocol_fee_default(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    input: u16,
) {
    let receipt = set_method_with_u16_input_receipt(
        vars,
        fee_controller_component,
        "set_protocol_fee_default",
        true,
        input,
    );
    receipt.expect_commit_success();
}

pub fn set_liquidity_fee_default(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    input: u16,
) {
    let receipt = set_method_with_u16_input_receipt(
        vars,
        fee_controller_component,
        "set_liquidity_fee_default",
        true,
        input,
    );
    receipt.expect_commit_success();
}

pub fn set_protocol_fee(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    package_address: PackageAddress,
    fee: u16,
) {
    let receipt = set_protocol_fee_receipt_with_proof(
        vars,
        fee_controller_component,
        true,
        package_address,
        fee,
    );
    receipt.expect_commit_success();
}

pub fn set_liquidity_fee(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    resource_addresses: Vec<ResourceAddress>,
    fee: u16,
) {
    let receipt = set_liquidity_fee_receipt_with_proof(
        vars,
        fee_controller_component,
        true,
        resource_addresses,
        fee,
    );
    receipt.expect_commit_success();
}

// GET METHODS:
pub fn get_protocol_fee_default(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
) -> Decimal {
    let receipt = get_method_with_no_input_receipt(
        vars,
        fee_controller_component,
        "get_protocol_fee_default",
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_liquidity_fee_default(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
) -> Decimal {
    let receipt = get_method_with_no_input_receipt(
        vars,
        fee_controller_component,
        "get_liquidity_fee_default",
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_protocol_fee(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    package_address: PackageAddress,
) -> Decimal {
    let receipt = get_method_with_package_address_input_receipt(
        vars,
        fee_controller_component,
        "get_protocol_fee",
        package_address,
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_liquidity_fee(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    resource_addresses: Vec<ResourceAddress>,
) -> Decimal {
    let receipt = get_method_with_vec_resource_address_input_receipt(
        vars,
        fee_controller_component,
        "get_liquidity_fee",
        resource_addresses,
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_fees(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    package_address: PackageAddress,
    resource_addresses: Vec<ResourceAddress>,
) -> (Decimal, Decimal) {
    let receipt = get_fees_receipt(
        vars,
        fee_controller_component,
        "get_fees",
        package_address,
        resource_addresses,
    );
    receipt
        .expect_commit_success()
        .output::<(Decimal, Decimal)>(1)
}
