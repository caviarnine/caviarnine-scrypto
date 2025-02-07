// use radix_engine::errors::{ApplicationError, PanicMessage, RuntimeError};
use crate::common::vars::Vars;
use radix_engine::errors::ApplicationError::PanicMessage;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::transaction::TransactionReceipt;
use scrypto::prelude::*;

pub fn assert_almost_equal(a: Decimal, b: Decimal, d: Decimal) {
    if a.is_zero() {
        assert!(
            b.is_zero(),
            "a is zero, but b is not zero. a: {:?}, b: {:?}",
            a,
            b
        );
    } else {
        assert!(
            Decimal::checked_abs(&(a - b)).unwrap() / a < d,
            "The difference is not less than d. a: {:?}, b: {:?}",
            a,
            b
        );
    }
}

pub fn assert_balance(resource: ResourceAddress, amount: Decimal, vars: &mut Vars) {
    assert_eq!(
        vars.test_runner
            .get_component_balance(vars.account_component_address, resource),
        amount
    );
}

pub fn assert_contains_message(receipt: TransactionReceipt, message: &str) {
    receipt.expect_specific_failure(|err| match err {
        ApplicationError(PanicMessage(msg)) => msg.contains(message),
        _ => false,
    });
}

pub fn extract_nftlocalid(
    receipt: TransactionReceipt,
    nft_resource_address: ResourceAddress,
) -> NonFungibleLocalId {
    receipt
        .expect_commit_success()
        .vault_balance_changes()
        .clone()
        .into_iter()
        .find(|(_, (address, _))| address == &nft_resource_address)
        .unwrap()
        .1
         .1
        .added_non_fungibles()
        .pop_first()
        .unwrap()
}
