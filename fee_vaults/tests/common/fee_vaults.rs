use crate::common::vars::Vars;
use radix_engine::transaction::TransactionReceipt;
use scrypto::{api::ObjectModuleId, prelude::*};
use transaction::builder::ManifestBuilder;
use transaction::prelude::ResolvableGlobalAddress;

pub fn new_fee_vaults_manifest_receipt(
    vars: &mut Vars,
    swap_amount: Decimal,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_function(
            vars.fee_vaults_package_address,
            "FeeVaults",
            "new",
            manifest_args!(
                vars.admin_badge_resource_address,
                vars.token_floop_new_resource_address,
                swap_amount
            ),
        )
        .build();

    

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn new_fee_vaults_manifest(vars: &mut Vars, swap_amount: Decimal) -> ComponentAddress {
    let receipt = new_fee_vaults_manifest_receipt(vars, swap_amount);
    receipt.expect_commit(true).output::<ComponentAddress>(1)
}

// set roles
pub fn set_owner_role_rule_receipt(
    vars: &mut Vars,
    address: impl ResolvableGlobalAddress,
    rule: AccessRule,
    with_proof: bool,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .set_owner_role(address, rule)
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_owner_role(address, rule)
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

pub fn update_role_rule_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    role: String,
    rule: AccessRule,
    with_proof: bool,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .set_role(
                fee_vaults_component,
                ObjectModuleId::Main,
                RoleKey { key: role },
                rule,
            )
            .build()
    } else {
        ManifestBuilder::new()
            // .create_proof_from_account(vars.admin_account_component, vars.admin_badge)
            .set_role(
                fee_vaults_component,
                ObjectModuleId::Main,
                RoleKey { key: role },
                rule,
            )
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

// generic method - resource address
pub fn get_method_with_resource_address_input_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    method_name: &str,
    resource_address: ResourceAddress,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(
            fee_vaults_component,
            method_name,
            manifest_args!(resource_address),
        )
        .build();
    
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

// generic method - no arguments
pub fn get_method_with_no_input_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    method_name: &str,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(fee_vaults_component, method_name, manifest_args!())
        .build();
    
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

// generic setter with decima;
pub fn set_method_with_decimal_input_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    method_name: &str,
    with_proof: bool,
    input: Decimal,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .call_method(fee_controller_component, method_name, manifest_args!(input))
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(fee_controller_component, method_name, manifest_args!(input))
            .build()
    };
    
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn set_max_epochs_with_proof_receipt(
    vars: &mut Vars,
    fee_controller_component: ComponentAddress,
    with_proof: bool,
    input: u64,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .call_method(
                fee_controller_component,
                "set_max_epochs",
                manifest_args!(input),
            )
            .build()
    } else {
        ManifestBuilder::new()
            .call_method(
                fee_controller_component,
                "set_max_epochs",
                manifest_args!(input),
            )
            .build()
    };
    
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

// SET METHODS:

// set_burn_percentage
pub fn set_burn_percentage(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    input: Decimal,
) {
    let receipt = set_method_with_decimal_input_receipt(
        vars,
        fee_vaults_component,
        "set_burn_percentage",
        true,
        input,
    );
    receipt.expect_commit_success();
}

// set_treasury_percentage
pub fn set_treasury_percentage(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    input: Decimal,
) {
    let receipt = set_method_with_decimal_input_receipt(
        vars,
        fee_vaults_component,
        "set_treasury_percentage",
        true,
        input,
    );
    receipt.expect_commit_success();
}

// set_swap_amount
pub fn set_swap_amount(vars: &mut Vars, fee_vaults_component: ComponentAddress, input: Decimal) {
    let receipt = set_method_with_decimal_input_receipt(
        vars,
        fee_vaults_component,
        "set_swap_amount",
        true,
        input,
    );
    receipt.expect_commit_success();
}

// set_max_epochs
pub fn set_max_epochs(vars: &mut Vars, fee_vaults_component: ComponentAddress, input: u64) {
    let receipt = set_max_epochs_with_proof_receipt(vars, fee_vaults_component, true, input);
    receipt.expect_commit_success();
}

// GET METHODS:

// get burn percentage
pub fn get_burn_percentage(vars: &mut Vars, fee_vaults_component: ComponentAddress) -> Decimal {
    let receipt =
        get_method_with_no_input_receipt(vars, fee_vaults_component, "get_burn_percentage");
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get treasury percentage
pub fn get_treasury_percentage(vars: &mut Vars, fee_vaults_component: ComponentAddress) -> Decimal {
    let receipt =
        get_method_with_no_input_receipt(vars, fee_vaults_component, "get_treasury_percentage");
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get swap amount
pub fn get_swap_amount(vars: &mut Vars, fee_vaults_component: ComponentAddress) -> Decimal {
    let receipt = get_method_with_no_input_receipt(vars, fee_vaults_component, "get_swap_amount");
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get max epochs
pub fn get_max_epochs(vars: &mut Vars, fee_vaults_component: ComponentAddress) -> u64 {
    let receipt = get_method_with_no_input_receipt(vars, fee_vaults_component, "get_max_epochs");
    receipt.expect_commit_success().output::<u64>(1)
}

// reserve withdraw
pub fn reserve_withdraw_with_proof_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    with_proof: bool,
    amount: Decimal,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .call_method(
                fee_vaults_component,
                "reserve_withdraw",
                manifest_args!(amount),
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
                fee_vaults_component,
                "reserve_withdraw",
                manifest_args!(amount),
            )
            .call_method(
                vars.admin_account_component_address,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build()
    };
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("RECEIPT: {:?}", receipt);
    receipt
}

// treasury withdraw
pub fn treasury_withdraw_with_proof_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    with_proof: bool,
    resource_address: ResourceAddress,
) -> TransactionReceipt {
    let manifest = if with_proof {
        ManifestBuilder::new()
            .create_proof_from_account_of_amount(vars.admin_account_component_address, vars.admin_badge_resource_address, dec!(1))
            .call_method(
                fee_vaults_component,
                "treasury_withdraw",
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
                fee_vaults_component,
                "treasury_withdraw",
                manifest_args!(resource_address),
            )
            .call_method(
                vars.admin_account_component_address,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build()
    };
    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    println!("RECEIPT: {:?}", receipt);
    receipt
}

// general deposit
pub fn general_deposit_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    method_name: &str,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
) -> TransactionReceipt {
    // create a manifest
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(
            vars.account_component_address,
            bucket_resource_address,
            bucket_amount,
        )
        .take_all_from_worktop(bucket_resource_address, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                fee_vaults_component,
                method_name,
                manifest_args!(lookup.bucket("tokens")),
            )
        })
        .build();

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    );
    println!("RECEIPT: {:?}", receipt);
    receipt
}

pub fn deposit_batch_reciept(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    buckets: Vec<(ResourceAddress, Decimal)>,
) -> TransactionReceipt {
    // counter
    let mut counter = 0;

    // args
    let mut args: Vec<String> = vec![];

    // start new manifest
    let mut manifest = ManifestBuilder::new();
    for (resource_address, amount) in &buckets {
        let arg = format!("bucket_{}", counter);
        // add to args
        args.push(arg.clone());

        // withdraw from the account
        manifest = manifest
            .withdraw_from_account(
                vars.account_component_address,
                *resource_address,
                *amount,
            )
            .take_all_from_worktop(*resource_address, arg);

        // increment counter
        counter += 1;
    }

    let manifest = manifest
        .with_name_lookup(|builder, lookup| {
            let args_vec: Vec<_> = args.iter().map(|arg| lookup.bucket(arg)).collect();
            builder.call_method(
                fee_vaults_component,
                "deposit_batch",
                manifest_args!(args_vec),
            )
        })
        .build();

    
    // println!("RECEIPT: {:?}", receipt);
    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

// get last swapped epoch
pub fn get_last_swapped_epoch(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    resource_address: ResourceAddress,
) -> Epoch {
    let receipt = get_method_with_resource_address_input_receipt(
        vars,
        fee_vaults_component,
        "get_last_swapped_epoch",
        resource_address,
    );
    receipt.expect_commit_success().output::<Epoch>(1)
}

// get swap vault amount
pub fn get_swap_vault_amount(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    resource_address: ResourceAddress,
) -> Decimal {
    let receipt = get_method_with_resource_address_input_receipt(
        vars,
        fee_vaults_component,
        "get_swap_vault_amount",
        resource_address,
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get reserve amount
pub fn get_reserve_amount(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
) -> Decimal {
    let receipt = get_method_with_no_input_receipt(
        vars,
        fee_vaults_component,
        "get_reserve_amount",
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get treasury vault amount
pub fn get_treasury_vault_amount(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    resource_address: ResourceAddress,
) -> Decimal {
    let receipt = get_method_with_resource_address_input_receipt(
        vars,
        fee_vaults_component,
        "get_treasury_vault_amount",
        resource_address,
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

// get swap price
pub fn get_swap_price(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    resource_address: ResourceAddress,
) -> Decimal {
    let receipt = get_method_with_resource_address_input_receipt(
        vars,
        fee_vaults_component,
        "get_swap_price",
        resource_address,
    );
    receipt.expect_commit_success().output::<Decimal>(1)
}

// treasury deposit
pub fn treasury_deposit(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
) {
    let receipt = general_deposit_receipt(
        vars,
        fee_vaults_component,
        "treasury_deposit",
        bucket_resource_address,
        bucket_amount,
    );
    receipt.expect_commit_success();
}

// swap vault deposit
pub fn swap_vault_deposit(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
) {
    let receipt = general_deposit_receipt(
        vars,
        fee_vaults_component,
        "swap_vault_deposit",
        bucket_resource_address,
        bucket_amount,
    );
    receipt.expect_commit_success();
}

// reserve deposit
pub fn reserve_deposit(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_amount: Decimal,
) {
    let receipt = general_deposit_receipt(
        vars,
        fee_vaults_component,
        "reserve_deposit",
        vars.token_floop_new_resource_address,
        bucket_amount,
    );
    receipt.expect_commit_success();
}

// deposit
pub fn deposit(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
) {
    let receipt = general_deposit_receipt(
        vars,
        fee_vaults_component,
        "deposit",
        bucket_resource_address,
        bucket_amount,
    );
    receipt.expect_commit_success();
}

pub fn deposit_batch(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    buckets: Vec<(ResourceAddress, Decimal)>,
) {
    let receipt = deposit_batch_reciept(vars, fee_vaults_component, buckets);
    receipt.expect_commit_success();
}

pub fn swap_receipt(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
    return_resource_address: ResourceAddress,
) -> TransactionReceipt {
    // create a manifest
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(
            vars.account_component_address,
            bucket_resource_address,
            bucket_amount,
        )
        .take_all_from_worktop(bucket_resource_address, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                fee_vaults_component,
                "swap",
                manifest_args!(lookup.bucket("tokens"), return_resource_address),
            )
        })
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
    println!("RECEIPT: {:?}", receipt);
    receipt
}

pub fn swap(
    vars: &mut Vars,
    fee_vaults_component: ComponentAddress,
    bucket_resource_address: ResourceAddress,
    bucket_amount: Decimal,
    return_resource_address: ResourceAddress,
) {
    let receipt = swap_receipt(
        vars,
        fee_vaults_component,
        bucket_resource_address,
        bucket_amount,
        return_resource_address,
    );
    receipt.expect_commit_success();
}
