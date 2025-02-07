use crate::common::vars::Vars;
use radix_engine::transaction::TransactionReceipt;
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;

pub fn new_token_bridge_manifest_receipt(
    admin_badge_address: ResourceAddress,
    resource_address: ResourceAddress,
    bridge_token_name: String,
    bridge_token_symbol: String,
    bridge_token_description: String,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_function(
            vars.token_bridge_package_address,
            "TokenBridge",
            "new",
            manifest_args!(
                admin_badge_address,
                resource_address,
                bridge_token_name,
                bridge_token_symbol,
                bridge_token_description
            ),
        )
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    )
}

pub fn new_token_bridge_manifest(
    admin_badge_address: ResourceAddress,
    resource_address: ResourceAddress,
    bridge_token_name: String,
    bridge_token_symbol: String,
    bridge_token_description: String,
    vars: &mut Vars,
) -> ComponentAddress {
    let receipt = new_token_bridge_manifest_receipt(
        admin_badge_address,
        resource_address,
        bridge_token_name,
        bridge_token_symbol,
        bridge_token_description,
        vars,
    );
    receipt.expect_commit(true).output::<ComponentAddress>(1)
}

// generic method
pub fn get_method_with_no_input_receipt(
    token_bridge_component: ComponentAddress,
    method_name: &str,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .call_method(token_bridge_component, method_name, manifest_args!())
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn get_new_token_address(
    token_bridge_component: ComponentAddress,
    vars: &mut Vars,
) -> ResourceAddress {
    let receipt =
        get_method_with_no_input_receipt(token_bridge_component, "get_new_token_address", vars);
    receipt.expect_commit(true).output::<ResourceAddress>(1)
}

pub fn get_old_token_address(
    token_bridge_component: ComponentAddress,
    vars: &mut Vars,
) -> ResourceAddress {
    let receipt =
        get_method_with_no_input_receipt(token_bridge_component, "get_old_token_address", vars);
    receipt.expect_commit(true).output::<ResourceAddress>(1)
}

pub fn get_old_tokens_amount(token_bridge_component: ComponentAddress, vars: &mut Vars) -> Decimal {
    let receipt =
        get_method_with_no_input_receipt(token_bridge_component, "get_old_tokens_amount", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

pub fn get_new_tokens_amount(token_bridge_component: ComponentAddress, vars: &mut Vars) -> Decimal {
    let receipt =
        get_method_with_no_input_receipt(token_bridge_component, "get_new_tokens_amount", vars);
    receipt.expect_commit(true).output::<Decimal>(1)
}

pub fn bridge_receipt(
    token_bridge_component: ComponentAddress,
    token_resource: ResourceAddress,
    token_amount: Decimal,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component_address, token_resource, token_amount)
        .take_all_from_worktop(token_resource, "tokens")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                token_bridge_component,
                "bridge",
                manifest_args!(lookup.bucket("tokens")),
            )
        })
        .call_method(
            vars.account_component_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}

pub fn bridge(
    token_bridge_component: ComponentAddress,
    token_resource: ResourceAddress,
    token_amount: Decimal,
    vars: &mut Vars,
) {
    let receipt = bridge_receipt(token_bridge_component, token_resource, token_amount, vars);
    receipt.expect_commit(true);
}

pub fn burn_token(
    token_resource: ResourceAddress,
    token_amount: Decimal,
    vars: &mut Vars,
) -> TransactionReceipt {
    let manifest = ManifestBuilder::new()
        .withdraw_from_account(vars.account_component_address, token_resource, token_amount)
        .burn_all_from_worktop(token_resource)
        .build();

    vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.public_key)],
    )
}
