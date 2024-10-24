use radix_engine::transaction::TransactionReceipt;
use scrypto::{api::ObjectModuleId, prelude::*};
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};

use crate::common::vars::Vars;

pub fn build_manifest(
    lsu_pool_package_address: PackageAddress,
    admin_badge_resource_address: ResourceAddress,
    token_validator_component_address: ComponentAddress,
) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            lsu_pool_package_address,
            "LsuPool",
            "new",
            manifest_args!(admin_badge_resource_address, token_validator_component_address),
        )
        .build()
}

// add liquidity
pub fn add_liquidity_no_proof_receipt(
    resource_address: ResourceAddress,
    amount: Decimal,
    vars: &mut Vars,
) -> TransactionReceipt {
    let my_proof: Option<ManifestProof> = None;
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component_address, resource_address, amount)
        .take_from_worktop(resource_address, amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "add_liquidity",
                manifest_args!(lookup.bucket("tokens"), my_proof),
            )
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

// add liquidity
pub fn add_liquidity_with_proof_receipt(
    resource_address: ResourceAddress,
    amount: Decimal,
    nft_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    // let my_proof: Option<ManifestProof> = None;
    let id_vec: Vec<NonFungibleLocalId> = vec![id].into_iter().collect();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            vars.account_component_address,
            nft_resource_address,
            id_vec.iter().cloned(),
        )
        .create_proof_from_auth_zone_of_non_fungibles(
            nft_resource_address,
            id_vec.iter().cloned(),
            "new_proof",
        )
        .withdraw_from_account(vars.account_component_address, resource_address, amount)
        .take_from_worktop(resource_address, amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "add_liquidity",
                manifest_args!(lookup.bucket("tokens"), Some(lookup.proof("new_proof"))),
            )
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

// remove liquidity - no proof
pub fn remove_liquidity_no_proof_receipt(
    liquidity_token_resource: ResourceAddress,
    liquidity_amount: Decimal,
    lsu_resource_address: ResourceAddress,
    vars: &mut Vars,
) -> TransactionReceipt {
    let my_proof: Option<ManifestProof> = None;
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(
            vars.account_component_address,
            liquidity_token_resource,
            liquidity_amount,
        )
        .take_from_worktop(liquidity_token_resource, liquidity_amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "remove_liquidity",
                manifest_args!(lookup.bucket("tokens"), lsu_resource_address, my_proof),
            )
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

// remove liquidity - with proof
pub fn remove_liquidity_with_proof_receipt(
    liquidity_token_resource: ResourceAddress,
    liquidity_amount: Decimal,
    lsu_resource_address: ResourceAddress,
    nft_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    // let my_proof: Option<ManifestProof> = None;
    let id_vec: Vec<NonFungibleLocalId> = vec![id].into_iter().collect();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            vars.account_component_address,
            nft_resource_address,
            id_vec.iter().cloned(),
        )
        .create_proof_from_auth_zone_of_non_fungibles(
            nft_resource_address,
            id_vec.iter().cloned(),
            "new_proof",
        )
        .withdraw_from_account(
            vars.account_component_address,
            liquidity_token_resource,
            liquidity_amount,
        )
        .take_from_worktop(liquidity_token_resource, liquidity_amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "remove_liquidity",
                manifest_args!(
                    lookup.bucket("tokens"),
                    lsu_resource_address,
                    Some(lookup.proof("new_proof"))
                ),
            )
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

// swap
pub fn swap_receipt(
    resource_address: ResourceAddress,
    resource_amount: Decimal,
    lsu_resource_address: ResourceAddress,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(
            vars.account_component_address,
            resource_address,
            resource_amount,
        )
        .take_from_worktop(resource_address, resource_amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "swap",
                manifest_args!(lookup.bucket("tokens"), lsu_resource_address),
            )
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

// CREDIT TOOLS

pub fn check_withdrawable_receipt(
    credit_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    let id_vec: Vec<NonFungibleLocalId> = vec![id].into_iter().collect();
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(
            vars.account_component_address,
            credit_resource_address,
            id_vec.iter().cloned(),
        )
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

pub fn burn_nft_receipt(
    credit_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    let ids = BTreeSet::from_iter(vec![id]);
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.account_component_address,
            "burn_non_fungibles",
            (credit_resource_address, ids),
        )
        .build();

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn get_id_resources_from_credit_proof_receipt(
    credit_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    // let my_proof: Option<ManifestProof> = None;
    let id_vec: Vec<NonFungibleLocalId> = vec![id].into_iter().collect();
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            vars.account_component_address,
            credit_resource_address,
            id_vec.iter().cloned(),
        )
        .create_proof_from_auth_zone_of_non_fungibles(
            credit_resource_address,
            id_vec.iter().cloned(),
            "new_proof",
        )
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "get_id_resources_from_credit_proof",
                manifest_args!(lookup.proof("new_proof")),
            )
        })
        .build();

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn merge_credit_receipt(
    credit_resource_address: ResourceAddress,
    id1: NonFungibleLocalId,
    id2: NonFungibleLocalId,
    vars: &mut Vars,
) -> TransactionReceipt {
    let id_vec12: Vec<NonFungibleLocalId> = vec![id1.clone(), id2.clone()].into_iter().collect();
    let id_vec1: Vec<NonFungibleLocalId> = vec![id1.clone()].into_iter().collect();
    let id_vec2: Vec<NonFungibleLocalId> = vec![id2.clone()].into_iter().collect();

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_non_fungibles(
            vars.account_component_address,
            credit_resource_address,
            id_vec12.iter().cloned(),
        )
        .create_proof_from_auth_zone_of_non_fungibles(
            credit_resource_address,
            id_vec1.iter().cloned(),
            "proof_1",
        )
        .create_proof_from_auth_zone_of_non_fungibles(
            credit_resource_address,
            id_vec2.iter().cloned(),
            "proof_2",
        )
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.lsu_pool_component_address,
                "merge_credit",
                manifest_args!(lookup.proof("proof_1"), lookup.proof("proof_2")),
            )
        })
        .build();

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

// GENERIC METHODS
// generic method - NO INPUT
pub fn get_method_with_no_input_receipt(method_name: &str, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_pool_component_address,
            method_name,
            manifest_args!(),
        )
        .build();

    // println!("receipt: {:?}", receipt);
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn get_method_with_global_address_receipt(
    global_address: GlobalAddress,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_pool_component_address,
            method_name,
            manifest_args!(global_address),
        )
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn get_method_with_u32_receipt(
    value: u32,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_pool_component_address,
            method_name,
            manifest_args!(value),
        )
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

// generic set method with proof - Component address input
pub fn set_method_with_component_address(
    value: ComponentAddress,
    with_proof: bool,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1),
            )
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    };

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

// generic set method with proof - Decimal Input
pub fn set_method_with_decimal(
    value: Decimal,
    with_proof: bool,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1),
            )
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    };

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

// generic set method with proof - Decimal Input
pub fn set_method_with_u32(
    value: u32,
    with_proof: bool,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1),
            )
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                vars.lsu_pool_component_address,
                method_name,
                manifest_args!(value),
            )
            .build()
    };

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn take_from_reserve_vaults_receipt(
    resource_address: ResourceAddress,
    with_proof: bool,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1),
            )
            .call_method(
                vars.lsu_pool_component_address,
                "take_from_reserve_vaults",
                manifest_args!(resource_address),
            )
            .call_method(
                vars.admin_account_component_address,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                vars.lsu_pool_component_address,
                "take_from_reserve_vaults",
                manifest_args!(resource_address),
            )
            .call_method(
                vars.admin_account_component_address,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build()
    };

    // println!("receipt: {:?}", receipt);

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn update_role_rule_receipt(
    role: String,
    rule: AccessRule,
    with_proof: bool,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(
                vars.admin_account_component_address,
                vars.admin_badge_resource_address,
                dec!(1),
            )
            .set_role(
                vars.lsu_pool_component_address,
                ObjectModuleId::Main,
                RoleKey { key: role },
                rule,
            )
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_role(
                vars.lsu_pool_component_address,
                ObjectModuleId::Main,
                RoleKey { key: role },
                rule,
            )
            .build()
    };

    // println!("{:?}", receipt);
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

// METHODS
pub fn add_liquidity_no_proof(resource_address: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    add_liquidity_no_proof_receipt(resource_address, amount, vars).expect_commit_success();
}

pub fn swap(
    resource_address: ResourceAddress,
    resource_amount: Decimal,
    lsu_resource_address: ResourceAddress,
    vars: &mut Vars,
) {
    swap_receipt(
        resource_address,
        resource_amount,
        lsu_resource_address,
        vars,
    )
    .expect_commit_success();
}

// SETTERS
// set_protocol_fee
pub fn set_protocol_fee(value: Decimal, vars: &mut Vars) {
    set_method_with_decimal(value, true, "set_protocol_fee", vars).expect_commit_success();
}

// set_liquidity_fee
pub fn set_liquidity_fee(value: Decimal, vars: &mut Vars) {
    set_method_with_decimal(value, true, "set_liquidity_fee", vars).expect_commit_success();
}

// set_reserve_fee
pub fn set_reserve_fee(value: Decimal, vars: &mut Vars) {
    set_method_with_decimal(value, true, "set_reserve_fee", vars).expect_commit_success();
}

// take_from_reserve_vaults
// TODO: take_from_reserve_vaults

// set_validator_max_before_fee
pub fn set_validator_max_before_fee(validator_max_before_fee: u32, vars: &mut Vars) {
    set_method_with_u32(
        validator_max_before_fee,
        true,
        "set_validator_max_before_fee",
        vars,
    )
    .expect_commit_success();
}

// GETTERS

pub fn get_token_validator_address(vars: &mut Vars) -> ComponentAddress {
    let receipt = get_method_with_no_input_receipt("get_token_validator_address", vars);
    receipt.expect_commit(true).output::<ComponentAddress>(1)
}

pub fn get_fee_vaults_address(vars: &mut Vars) -> ComponentAddress {
    let receipt = get_method_with_no_input_receipt("get_fee_vaults_address", vars);
    receipt.expect_commit(true).output::<ComponentAddress>(1)
}

// get_vault_balance
pub fn get_vault_balance(resource_address: ResourceAddress, vars: &mut Vars) -> Option<Decimal> {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_vault_balance",
        vars,
    );
    receipt.expect_commit(true).output::<Option<Decimal>>(1)
}

// get_reserve_vault_balance
pub fn get_reserve_vault_balance(
    resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Option<Decimal> {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_reserve_vault_balance",
        vars,
    );
    receipt.expect_commit(true).output::<Option<Decimal>>(1)
}

// get_price_lsu_xrd_cached
pub fn get_price_lsu_xrd_cached(
    resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Option<Decimal> {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_price_lsu_xrd_cached",
        vars,
    );
    receipt.expect_commit(true).output::<Option<Decimal>>(1)
}

// get_dex_valuation_xrd
pub fn get_dex_valuation_xrd(vars: &mut Vars) -> Decimal {
    let receipt = get_method_with_no_input_receipt("get_dex_valuation_xrd", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get_liquidity_token_resource_address
pub fn get_liquidity_token_resource_address(vars: &mut Vars) -> ResourceAddress {
    let receipt = get_method_with_no_input_receipt("get_liquidity_token_resource_address", vars);
    receipt.expect_commit(true).output::<ResourceAddress>(1)
}

// get_liquidity_token_total_supply
pub fn get_liquidity_token_total_supply(vars: &mut Vars) -> Decimal {
    let receipt = get_method_with_no_input_receipt("get_liquidity_token_total_supply", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get_credit_receipt_resource_address
pub fn get_credit_receipt_resource_address(vars: &mut Vars) -> ResourceAddress {
    let receipt = get_method_with_no_input_receipt("get_credit_receipt_resource_address", vars);
    receipt.expect_commit(true).output::<ResourceAddress>(1)
}

// get_protocol_fee
pub fn get_protocol_fee(vars: &mut Vars) -> Decimal {
    let receipt = get_method_with_no_input_receipt("get_protocol_fee", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get_liquidity_fee
pub fn get_liquidity_fee(vars: &mut Vars) -> Decimal {
    let receipt = get_method_with_no_input_receipt("get_liquidity_fee", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get_reserve_fee
pub fn get_reserve_fee(vars: &mut Vars) -> Decimal {
    let receipt = get_method_with_no_input_receipt("get_reserve_fee", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get price
pub fn get_price(
    lhs_resource_address: ResourceAddress,
    rhs_resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Option<Decimal> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_pool_component_address,
            "get_price",
            manifest_args!(lhs_resource_address, rhs_resource_address),
        )
        .build();
    vars.test_runner
        .execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
        )
        .expect_commit(true)
        .output::<Option<Decimal>>(1)
}

// get_nft_data
pub fn get_nft_data(id: NonFungibleLocalId, vars: &mut Vars) -> HashMap<ResourceAddress, Decimal> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.lsu_pool_component_address,
            "get_nft_data",
            manifest_args!(id),
        )
        .build();
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );

    receipt
        .expect_commit(true)
        .output::<HashMap<ResourceAddress, Decimal>>(1)
}

// is_lsu_token
pub fn is_lsu_token(resource_address: ResourceAddress, vars: &mut Vars) -> bool {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "is_lsu_token",
        vars,
    );
    receipt.expect_commit(true).output::<bool>(1)
}

// is_validator
pub fn is_validator(component_address: ComponentAddress, vars: &mut Vars) -> bool {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(component_address),
        "is_validator",
        vars,
    );
    receipt.expect_commit(true).output::<bool>(1)
}

// get_validator_address
pub fn get_validator_address(
    resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Option<ComponentAddress> {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_validator_address",
        vars,
    );
    receipt
        .expect_commit(true)
        .output::<Option<ComponentAddress>>(1)
}

// validator_price_lsu_xrd
pub fn get_validator_price_lsu_xrd(
    resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Option<Decimal> {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_validator_price_lsu_xrd",
        vars,
    );
    receipt.expect_commit(true).output::<Option<Decimal>>(1)
}

// get_validator_price_lsu_xrd_and_update_valuation
pub fn get_validator_price_lsu_xrd_and_update_valuation(
    resource_address: ResourceAddress,
    vars: &mut Vars,
) -> Decimal {
    let receipt = get_method_with_global_address_receipt(
        GlobalAddress::from(resource_address),
        "get_validator_price_lsu_xrd_and_update_valuation",
        vars,
    );
    receipt.expect_commit(true).output::<Decimal>(1)
}

// get_validator_max_before_fee
pub fn get_validator_max_before_fee(vars: &mut Vars) -> u32 {
    let receipt = get_method_with_no_input_receipt("get_validator_max_before_fee", vars);
    receipt.expect_commit(true).output::<u32>(1)
}

// get_validator_counter
pub fn get_validator_counter(vars: &mut Vars) -> u32 {
    let receipt = get_method_with_no_input_receipt("get_validator_counter", vars);
    receipt.expect_commit(true).output::<u32>(1)
}
// get_validator_pointer
pub fn get_validator_pointer(vars: &mut Vars) -> u32 {
    let receipt = get_method_with_no_input_receipt("get_validator_pointer", vars);
    receipt.expect_commit(true).output::<u32>(1)
}
// get_validator_address_map
pub fn get_validator_address_map(index: u32, vars: &mut Vars) -> ResourceAddress {
    let receipt = get_method_with_u32_receipt(index, "get_validator_address_map", vars);
    receipt.expect_commit(true).output::<ResourceAddress>(1)
}

// update_multiple_validator_prices
pub fn update_multiple_validator_prices(index: u32, vars: &mut Vars) {
    get_method_with_u32_receipt(index, "update_multiple_validator_prices", vars);
}

// get_id_resources_from_credit_proof
pub fn get_id_resources_from_credit_proof(
    credit_resource_address: ResourceAddress,
    id: NonFungibleLocalId,
    vars: &mut Vars,
) -> (NonFungibleLocalId, HashMap<ResourceAddress, Decimal>) {
    let receipt = get_id_resources_from_credit_proof_receipt(credit_resource_address, id, vars);
    receipt
        .expect_commit(true)
        .output::<(NonFungibleLocalId, HashMap<ResourceAddress, Decimal>)>(3)
}
