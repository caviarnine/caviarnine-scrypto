#![allow(dead_code)]

mod common;
pub use crate::common::fee_controller;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_initalisation_success_01() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    let receipt = fee_controller::new_fee_controller_manifest_receipt(&mut vars);

    // ASSERT
    receipt.expect_commit_success();
}

#[test]
fn test_initalisation_success_02() {
    // ARRANGE
    let mut vars = setup();

    // ACT
    // ASSERT
    fee_controller::new_fee_controller_manifest(&mut vars);
}
