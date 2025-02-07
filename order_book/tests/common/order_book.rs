use scrypto::prelude::*;
use transaction::{builder::ManifestBuilder, model::TransactionManifestV1};
use radix_engine::transaction::TransactionReceipt;

use crate::common::vars::*;

/// Order data.
#[derive(ScryptoSbor, Clone, Copy)]
pub struct OrderData {
    /// Is ask or bid order.
    pub is_ask: bool,
    /// Price of order.
    pub price: Decimal,
    /// Filled amount of tokens for order calculated in tokens x.
    pub amount_filled: Decimal,
    /// Total amount of tokens for order calculated in tokens x.
    pub amount_total: Decimal,
}

/// Order status.
///
/// * `Open(OrderData)` - Order is open and has the contained order data.
/// * `Filled(OrderData)` - Order has been filled and has the contained order data.
/// * `Claimed` - Order has been claimed and no longer exists.
/// * `Invalid` - Order id is invalid.
#[derive(ScryptoSbor)]
pub enum OrderStatus {
    Open(OrderData),
    Filled(OrderData),
    Claimed,
    Invalid,
}

pub fn build_manifest(
    order_book_package: PackageAddress,
    owner_rule: AccessRule,
    user_rule: AccessRule,
    token_x_address: ResourceAddress, 
    token_y_address: ResourceAddress, 
    ) -> TransactionManifestV1 {
    ManifestBuilder::new()
        .call_function(
            order_book_package,
            "OrderBook",
            "new",
            manifest_args!(owner_rule, user_rule, token_x_address, token_y_address, None::<ManifestAddressReservation>))
        .build()
}

pub fn get_fee_controller_address(vars: &mut Vars) -> ComponentAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_fee_controller_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET FEE CONTROLLER ADDRESS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ComponentAddress>(1)
}

pub fn get_fee_vaults_address(vars: &mut Vars) -> ComponentAddress {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_fee_vaults_address",
            manifest_args!(),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET FEE VAULTS ADDRESS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ComponentAddress>(1)
}

pub fn get_token_x_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_token_x_address",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET TOKEN X ADDRESS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_token_y_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_token_y_address",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET TOKEN Y ADDRESS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_order_receipt_address(vars: &mut Vars) -> ResourceAddress {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_order_receipt_address",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET ORDER RECEIPT ADDRESS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<ResourceAddress>(1)
}

pub fn get_amount_x(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_amount_x",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET AMOUNT X\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_amount_y(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_amount_y",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET AMOUNT Y\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_last_price(vars: &mut Vars) -> Decimal {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_last_price",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET LAST PRICE\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Decimal>(1)
}

pub fn get_current_ask_price(vars: &mut Vars) -> Option<Decimal> {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_current_ask_price",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET CURRENT ASK PRICE\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<Decimal>>(1)
}

pub fn get_current_bid_price(vars: &mut Vars) -> Option<Decimal> {
    let manifest = ManifestBuilder::new()
    .call_method(
        vars.order_book_component,
        "get_current_bid_price",
        manifest_args!(),
    )
    .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET CURRENT BID PRICE\n");
    // print!("{:?}", receipt);
    receipt.expect_commit_success().output::<Option<Decimal>>(1)
}

pub fn get_ask_limits(start_price: Option<Decimal>, stop_price: Option<Decimal>, number: Option<u32>, vars: &mut Vars) -> Vec<(Decimal, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_ask_limits",
            manifest_args!(start_price, stop_price, number),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET ASK LIMITS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Vec<(Decimal, Decimal)>>(1)
}

pub fn get_bid_limits(start_price: Option<Decimal>, stop_price: Option<Decimal>, number: Option<u32>, vars: &mut Vars) -> Vec<(Decimal, Decimal)> {
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_bid_limits",
            manifest_args!(start_price, stop_price, number),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET BID LIMITS\n");
    // print!("{:?}", receipt);
    receipt.expect_commit_success().output::<Vec<(Decimal, Decimal)>>(1)
}

pub fn get_order_status(id: u64, vars: &mut Vars) -> OrderStatus {
    let id = NonFungibleLocalId::integer(id);
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_order_status",
            manifest_args!(id),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    // println!("\nGET ORDER STATUS\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<OrderStatus>(1)
}

pub fn get_order_statuses(ids: Vec<u64>, vars: &mut Vars) -> Vec<OrderStatus> {
    let ids: Vec<NonFungibleLocalId> = ids.into_iter().map(|id| NonFungibleLocalId::integer(id)).collect();
    let manifest = ManifestBuilder::new()
        .call_method(
            vars.order_book_component,
            "get_order_statuses",
            manifest_args!(ids),
        )
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)]
    );
    // println!("\nGET ORDER STATUSES\n");
    // println!("{:?}", receipt);
    receipt.expect_commit_success().output::<Vec<OrderStatus>>(1)
}

pub fn limit_order(token: ResourceAddress, amount: Decimal, price: Decimal, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component, token, amount)
        .take_all_from_worktop(token, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.order_book_component,
                "limit_order",
                manifest_args!(lookup.bucket("tokens") , price))
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
    // println!("\nLIMIT ORDER\n");
    // println!("{:?}", receipt);
    receipt
}

pub fn limit_order_batch(positions: Vec<(ResourceAddress, Decimal, Decimal)>, vars: &mut Vars) -> TransactionReceipt {
    let mut counter = 0;
    let mut args0: Vec<(String, Decimal)> = vec![];

    let mut builder = ManifestBuilder::new();
    for (token, amount, price) in &positions {
        let bucket_id = format!("bucket_{}", counter);
        args0.push((bucket_id.clone(), *price));

        builder = builder
            .withdraw_from_account(
                vars.account_component,
                *token,
                *amount,
            )
            .take_all_from_worktop(*token, bucket_id);

        counter += 1;
    }

    let manifest = builder.with_name_lookup(|builder, lookup| {
            let args1: Vec<_> = args0.iter().map(|arg| (lookup.bucket(arg.0.clone()), arg.1)).collect();
            builder.call_method(
                vars.order_book_component,
                "limit_order_batch",
                manifest_args!(args1))
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
    // println!("\nLIMIT ORDER BATCH\n");
    // println!("{:?}", receipt);
    receipt
}

pub fn market_order(token: ResourceAddress, amount: Decimal, stop_price: Option<Decimal>, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component, token, amount)
        .take_all_from_worktop(token, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.order_book_component,
                "market_order",
                manifest_args!(lookup.bucket("tokens") , stop_price))
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
    // println!("\nMARKET ORDER\n");
    // println!("{:?}", receipt);
    receipt
}

pub fn claim_orders(ids: BTreeSet<NonFungibleLocalId>, vars: &mut Vars) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.order_receipt, ids.clone())
        .take_non_fungibles_from_worktop(vars.order_receipt, ids, "order_receipts")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.order_book_component,
                "claim_orders",
                manifest_args!(lookup.bucket("order_receipts")))
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
    // println!("\nCLAIM ORDER\n");
    // println!("{:?}", receipt);
    receipt
}
