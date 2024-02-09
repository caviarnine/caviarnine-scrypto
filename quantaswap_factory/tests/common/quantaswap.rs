use scrypto::{prelude::*, api::ObjectModuleId};
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::*;

pub fn build_manifest(
    quantaswap_package: PackageAddress,
    owner_rule: AccessRule,
    user_rule: AccessRule,
    token_x_address: ResourceAddress,
    token_y_address: ResourceAddress,
    bin_span: u32,
    ) -> TransactionManifestV1 {
        ManifestBuilder::new()
        .call_function(
            quantaswap_package,
            "QuantaSwap",
            "new",
            manifest_args!(owner_rule, user_rule,token_x_address, token_y_address, bin_span, None::<ManifestAddressReservation>))
        .build()
}

pub fn set_owner_rule(rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest: TransactionManifestV1 = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_owner_role(vars.quantaswap_component, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_owner_role(vars.quantaswap_component, rule)
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET OWNER RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn set_role_rule(role: String, rule: AccessRule, with_proof: bool, vars: &mut Vars) -> TransactionReceipt {
    let manifest: TransactionManifestV1 = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component, vars.admin_badge, dec!(1))
            .set_role(vars.quantaswap_component, ObjectModuleId::Main, RoleKey { key: role }, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_role(vars.quantaswap_component, ObjectModuleId::Main, RoleKey { key: role }, rule)
            .build()
    };

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("\nSET ROLE RULE\n");
    println!("{:?}", receipt);
    receipt
}

pub fn get_fee_controller_address(vars: &mut Vars) -> ComponentAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_fee_controller_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET FEE CONTROLLER ADDRESS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ComponentAddress>(1)
}

pub fn get_fee_vaults_address(vars: &mut Vars) -> ComponentAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_fee_vaults_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET FEE VAULTS ADDRESS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ComponentAddress>(1)
}

pub fn get_token_x_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_token_x_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET TOKEN X ADDRESS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_token_y_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_token_y_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET TOKEN Y ADDRESS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_liquidity_receipt_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_liquidity_receipt_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET LIQUIDITY RECEIPT ADDRESS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_bin_span(vars: &mut Vars) -> u32 {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_bin_span",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET BIN SPAN\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<u32>(1)
}

pub fn get_liquidity_claims(liquidity_receipt_id: NonFungibleLocalId, vars: &mut Vars) -> HashMap<u32, Decimal> {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.quantaswap_component,
        "get_liquidity_claims",
        manifest_args!(liquidity_receipt_id),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET LIQUIDITY CLAIMS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<HashMap<u32, Decimal>>(1)
}

pub fn get_active_tick(vars: &mut Vars) -> Option<u32> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_active_tick",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET ACTIVE TICK\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<u32>>(1)
}

pub fn get_price(vars: &mut Vars) -> Option<Decimal> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_price",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    println!("\nGET PRICE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<Decimal>>(1)
}

pub fn get_amount_x(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_amount_x",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET AMOUNT X\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_amount_y(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_amount_y",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],

    );
    println!("\nGET AMOUNT Y\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_active_bin_price_range(vars: &mut Vars) -> Option<(Decimal, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_active_bin_price_range",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET ACTIVE BIN PRICE RANGE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<(Decimal, Decimal)>>(1)
}

pub fn get_active_amounts(vars: &mut Vars) -> Option<(Decimal, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_active_amounts",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nGET ACTIVE AMOUNTS\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<(Decimal, Decimal)>>(1)
}

pub fn get_bins_above(start_tick: Option<u32>, stop_tick: Option<u32>, number: Option<u32>, vars: &mut Vars) -> Vec<(u32, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_bins_above",
            manifest_args!(start_tick, stop_tick, number),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    println!("\nGET BINS ABOVE\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Vec<(u32, Decimal)>>(1)
}

pub fn get_bins_below(start_tick: Option<u32>, stop_tick: Option<u32>, number: Option<u32>, vars: &mut Vars) -> Vec<(u32, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "get_bins_below",
            manifest_args!(start_tick, stop_tick, number),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    println!("\nGET BINS BELOW\n");
    println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Vec<(u32, Decimal)>>(1)
}

pub fn mint_liquidity_receipt(vars: &mut Vars) -> NonFungibleLocalId {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.quantaswap_component,
            "mint_liquidity_receipt",
            manifest_args!(),
        )
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![
            NonFungibleGlobalId::from_public_key(&vars.public_key),
        ],
    );
    println!("\nMINT LIQUIDITY RECEIPT\n");
    println!("{:?}", receipt);

    receipt.expect_commit_success()
        .vault_balance_changes().clone()
        .into_iter()
        .find(|(_, (address, _))| address == &vars.liquidity_receipt).unwrap().1.1
        .added_non_fungibles().pop_first().unwrap()
}

pub fn add_liquidity(
    liquidity_receipt_id: NonFungibleLocalId, 
    amount_x: Decimal, 
    amount_y: Decimal, 
    positions: Vec<(u32, Decimal, Decimal)>, 
    vars: &mut Vars) -> TransactionReceipt {
    let ids = BTreeSet::from([liquidity_receipt_id]);

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.liquidity_receipt, ids.clone())
        .withdraw_from_account(vars.account_component, vars.token_x, amount_x)
        .withdraw_from_account(vars.account_component, vars.token_y, amount_y)
        .take_non_fungibles_from_worktop(vars.liquidity_receipt, ids, "liquidity_receipt")
        .take_from_worktop(vars.token_x, amount_x, "tokens_x")
        .take_from_worktop(vars.token_y, amount_y, "tokens_y")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.quantaswap_component,
                "add_liquidity",
                manifest_args!(lookup.bucket("liquidity_receipt"), lookup.bucket("tokens_x"), lookup.bucket("tokens_y"), positions),
            )
        })
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nADD LIQUIDITY\n");
    println!("{:?}", receipt);
    receipt
}

pub fn remove_liquidity(
    liquidity_receipt_id: NonFungibleLocalId, 
    claims: Vec<(u32, Decimal)>, 
    vars: &mut Vars,
    ) -> TransactionReceipt {
    let ids = BTreeSet::from([liquidity_receipt_id]);

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.liquidity_receipt, ids.clone())
        .take_non_fungibles_from_worktop(vars.liquidity_receipt, ids, "liquidity_receipt")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.quantaswap_component,
                "remove_liquidity",
                manifest_args!(lookup.bucket("liquidity_receipt"), claims),
            )
        })
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nREMOVE LIQUIDITY\n");
    println!("{:?}", receipt);
    receipt
}

pub fn swap(token: ResourceAddress, amount: Decimal, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component, token, amount)
        .take_from_worktop(token, amount, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.quantaswap_component,
                "swap",
                manifest_args!(lookup.bucket("tokens"))
            )
        })
        .call_method(
            vars.account_component,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("\nSWAP\n");
    println!("{:?}", receipt);
    receipt
}

pub fn assert_amount_sums(vars: &mut Vars) {
    let bins_above = get_bins_above(None, None, None, vars);
    let bins_below = get_bins_below(None, None, None, vars);

    let bins_above_sum = bins_above.iter().fold(Decimal::ZERO, |acc, (_, amount)| acc + *amount);
    let bins_below_sum = bins_below.iter().fold(Decimal::ZERO, |acc, (_, amount)| acc + *amount);
    let amount_x = get_amount_x(vars);
    let amount_y = get_amount_y(vars);

    println!("bin_above_sum: {}, amount_x: {}, ", bins_above_sum, amount_x);
    println!("bin_below_sum: {}, amount_y: {}, ", bins_below_sum, amount_y);
    
    assert!(bins_above_sum <= amount_x);
    assert!(bins_below_sum <= amount_y);
}
