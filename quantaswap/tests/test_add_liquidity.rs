#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::*;

use ::quantaswap::tick::Tick;
use transaction::builder::ManifestBuilder;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;
pub use crate::common::quantaswap;

#[test]
fn test_add_liquidity() {
    let mut vars: Vars = setup();

    let tick = Tick::ONE.0 + vars.bin_span;
    let lower_sqrt = Decimal::from(Tick(tick));
    let upper_sqrt = Decimal::from(Tick(tick).tick_upper(vars.bin_span));
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity(amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick)
    );
    assert_eq!(
        get_active_bin_price_range(&mut vars), 
        Some((lower_sqrt * lower_sqrt, upper_sqrt * upper_sqrt))
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x, amount_y))
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_initial() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let lower_sqrt = Decimal::from(Tick(tick));
    let upper_sqrt = Decimal::from(Tick(tick).tick_upper(vars.bin_span));
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick)
    );
    assert_eq!(
        get_active_bin_price_range(&mut vars), 
        Some((lower_sqrt * lower_sqrt, upper_sqrt * upper_sqrt))
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x, amount_y))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick, amount_x.max(amount_y))])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(2);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(4);
    let amount_y1 = dec!(3);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0 + amount_x1, amount_y0 + dec!(2)))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0 + amount_x1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - dec!(2), &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active_zero_x() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!(0);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(3);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0 + amount_x1, dec!(0)))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0 + amount_x1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active_zero_y() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(0);
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(3);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((dec!(0), amount_y0 + amount_y1))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_y0 + amount_y1)])
    );
    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y1, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active_large_many_x_to_y() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("10000000000000000");
    let amount_y0 = round_up(dec!("0.000000000000000001"), vars.divisibility_y);
    let ratio = amount_y0 / amount_x0;
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!("10000000000000000");
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0 + amount_x1, amount_y0 + ratio * amount_x1))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0 + amount_x1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - ratio * amount_x1, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active_large_many_y_to_x() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = round_up(dec!("0.000000000000000001"), vars.divisibility_x);
    let amount_y0 = dec!("10000000000000000");
    let ratio = amount_y0 / amount_x0;
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(2);
    let amount_y1 = dec!("10000000000000000");
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0 + amount_y1 / ratio, amount_y0 + amount_y1))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_y0 + amount_y1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_y1 / ratio, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y1, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_above() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0, amount_y0))
    );
    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0), (tick1, amount_x1)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_x1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_below() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0, amount_y0))
    );
    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0), (tick1, amount_y1)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_y1)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y1, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_above_and_below() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let tick2 = tick0 - vars.bin_span;
    let amount_x2 = dec!(1);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick2, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick0)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x0, amount_y0))
    );
    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0), (tick1, amount_x1)]
    );
    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0), (tick2, amount_y2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_x1), (tick2, amount_y2)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y2, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_above_existing() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount_x2 = dec!(3);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick1, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0), (tick1, amount_x1 + amount_x2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_x1 + amount_x2)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1 - amount_x2, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_below_existing() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let amount_x2 = dec!(3);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick1, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0), (tick1, amount_y1 + amount_y2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_y1 + amount_y2)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y1 - amount_y2, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_above_existing_zeroed() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();
    remove_specific_liquidity(id.clone(), vec![(tick1, amount_x1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0)])
    );

    let amount_x2 = dec!(3);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick1, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0), (tick1, amount_x2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_x2)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x2, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_below_existing_zeroed() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();
    remove_specific_liquidity(id.clone(), vec![(tick1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0)])
    );

    let amount_x2 = dec!(3);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick1, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0), (tick1, amount_y2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, amount_y2)])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y2, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_initial_position_zero() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(0);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        None
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([])
    );
    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_active_position_zero() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(1);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id0, amount_x0, amount_y0, vec![(tick, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let active_amounts0 = get_active_amounts(&mut vars);

    let id1 = mint_liquidity_receipt(&mut vars);
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id1.clone(), dec!(1), dec!(1), vec![(tick, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let active_amounts1 = get_active_amounts(&mut vars);

    assert_eq!(
        active_amounts0,
        active_amounts1
    );
    assert_eq!(
        get_liquidity_claims(id1.clone(), &mut vars),
        HashMap::from([])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_above_position_zero() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(1);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id0, amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let id1 = mint_liquidity_receipt(&mut vars);
    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id1.clone(), dec!(1), dec!(1), vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0)]
    );
    assert_eq!(
        get_liquidity_claims(id1.clone(), &mut vars),
        HashMap::from([])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_bin_below_position_zero() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(1);
    let amount_y0 = dec!(1);
    add_liquidity_to_receipt(id0, amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let id1 = mint_liquidity_receipt(&mut vars);
    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(0);
    let amount_y1 = dec!(0);
    add_liquidity_to_receipt(id1.clone(), dec!(1), dec!(1), vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_x0)]
    );
    assert_eq!(
        get_liquidity_claims(id1.clone(), &mut vars),
        HashMap::from([])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_add_liquidity_to_receipt_many_above() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..200 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_x));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_add_liquidity_to_receipt_many_below() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..200 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_add_liquidity_to_receipt_tick_price_x() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt: Decimal = Tick::ONE.into();
    let tick = Tick::from(price_sqrt).0;
    let amount_x = dec!(5);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt * price_sqrt, 
        dec!("0.0000000000001")
    );
}

#[test]
fn test_add_liquidity_to_receipt_tick_price_y() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt_lower: Decimal = Tick::ONE.into();
    let price_sqrt_upper: Decimal = Tick::ONE.tick_upper(vars.bin_span).into();
    let tick = Tick::from(price_sqrt_lower).0;
    let amount_x = dec!(0);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt_upper * price_sqrt_upper, 
        dec!("0.0000000000001")
    );
}

#[test]
fn test_add_liquidity_to_receipt_low_tick_price_x() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt: Decimal = Tick::MIN.into();
    let tick = Tick::from(price_sqrt).0;
    let amount_x = dec!(5);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt * price_sqrt, 
        dec!("0.0000000000001")
    );
}

#[test]
fn test_add_liquidity_to_receipt_low_tick_price_y() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt_lower: Decimal = Tick::MIN.into();
    let price_sqrt_upper: Decimal = Tick::MIN.tick_upper(vars.bin_span).into();
    let tick = Tick::from(price_sqrt_lower).0;
    let amount_x = dec!(0);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt_upper * price_sqrt_upper, 
        dec!("0.0000000000001")
    );
}

#[test]
fn test_add_liquidity_to_receipt_high_tick_price_x() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt: Decimal = Tick::MAX.into();
    let tick = Tick::from(price_sqrt).0;
    let amount_x = dec!(5);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt * price_sqrt, 
        dec!("0.0000000000001")
    );
}

#[test]
fn test_add_liquidity_to_receipt_high_tick_price_y() {
    let mut vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let price_sqrt_lower: Decimal = Tick::MAX.tick_lower(vars.bin_span).into();
    let price_sqrt_upper: Decimal = Tick::MAX.into();
    let tick = Tick::from(price_sqrt_lower).0;
    let amount_x = dec!(0);
    let amount_y = dec!(5);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_within_error_margin(
        get_price(&mut vars).unwrap(), 
        price_sqrt_upper * price_sqrt_upper, 
        dec!("0.01") // allow 1 bps error for this extreme case
    );
}

#[test]
fn test_add_liquidity_to_receipt_too_many_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..201 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Too many liquidity claims for this liquidity receipt.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_tick_high_valid() {
    let mut vars: Vars = setup();

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        1,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    vars.quantaswap_component = quantaswap_component;
    vars.liquidity_receipt = liquidity_receipt;
    vars.bin_span = 1;

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
}

#[test]
fn test_add_liquidity_to_receipt_tick_low_valid() {
    let mut vars: Vars = setup();

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        1,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    vars.quantaswap_component = quantaswap_component;
    vars.liquidity_receipt = liquidity_receipt;
    vars.bin_span = 1;

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MIN.0;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
}

#[test]
fn test_add_liquidity_to_receipt_tick_high_invalid() {
    let mut vars: Vars = setup();

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        1,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let quantaswap_component = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    vars.quantaswap_component = quantaswap_component;
    vars.liquidity_receipt = liquidity_receipt;
    vars.bin_span = 1;

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::MAX.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid tick")
            },
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_tick_misaligned_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span / 2;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid tick")
            },
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_initial_too_few_tokens_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(2);
    let amount_y = dec!(2);
    add_liquidity_to_receipt(id, dec!(1), dec!(1), vec![(tick, amount_x, amount_y)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(BucketError(_)) => true,
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_active_too_few_tokens_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let amount_x1 = dec!(3);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick0, amount_x1, amount_y1)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(BucketError(_)) => true,
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_bin_above_and_below_too_few_tokens_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(BucketError(_)) => true,
            _ => false,
        }
    });

    let tick2 = tick0 - vars.bin_span;
    let amount_x2 = dec!(2);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick2, amount_x2, amount_y2)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(BucketError(_)) => true,
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_initial_negative_position_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(1);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, -amount_x, amount_y)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Amounts must be greater than or equal to zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_other_liquidity_receipt_invalid() {
    let mut vars: Vars = setup();

    let manifest = quantaswap::build_manifest(
        vars.quantaswap_package,
        rule!(require(vars.admin_badge)),
        AccessRule::AllowAll,
        vars.token_x, 
        vars.token_y, 
        10,
    );

    let receipt = vars.test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&vars.admin_public_key)],
    );
    receipt.expect_commit_success();

    let quantaswap_component_1 = receipt
        .expect_commit(true)
        .new_component_addresses()[0];
    let liquidity_receipt_1 = receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    let quantaswap_component_0 = vars.quantaswap_component;
    vars.quantaswap_component = quantaswap_component_1;
    vars.liquidity_receipt = liquidity_receipt_1;
    let id = mint_liquidity_receipt(&mut vars);
    vars.quantaswap_component = quantaswap_component_0;

    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!("1.5");
    let amount_y0 = dec!("0.5");
    add_liquidity_to_receipt(id, amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid liquidity receipt.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_add_liquidity_to_receipt_multiple_liquidity_receipts_invalid() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let id1 = mint_liquidity_receipt(&mut vars);

    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(2);
    let amount_y = dec!(2);
    let positions = vec![(tick, amount_x, amount_y)];

    let ids = BTreeSet::from([id0, id1]);

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
                "add_liquidity_to_receipt",
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
    print!("{:?}", receipt);
    receipt.expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Expecting singleton NFT bucket")
            },
            _ => false,
        }
    });
}
