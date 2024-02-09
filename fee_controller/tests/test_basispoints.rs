#![allow(dead_code)]
use scrypto::prelude::*;

mod common;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

use fee_controller::util::*;

#[test]
pub fn test_basic_01() {
    // ARRANGE
    // ACT
    // ASSERT
    assert_eq!(
        Decimal::from_basis_point_hundredths(1),
        Decimal::from_str("0.000001").unwrap()
    );
}

#[test]
pub fn test_basic_02() {
    // ARRANGE
    // ACT
    // ASSERT
    assert_eq!(
        Decimal::from_basis_point_hundredths(200),
        Decimal::from_str("0.0002").unwrap()
    );
}

#[test]
pub fn test_basic_03() {
    // ARRANGE
    // ACT
    // ASSERT
    assert_eq!(Decimal::from_basis_point_hundredths(0), Decimal::ZERO);
}

#[test]
pub fn test_basic_04() {
    // ARRANGE
    // ACT
    // ASSERT
    assert_eq!(
        Decimal::from_basis_point_hundredths(65000),
        Decimal::from_str("0.065").unwrap()
    );
}
