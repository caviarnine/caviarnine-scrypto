#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

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
fn test_remove_liquidity_empty() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    remove_liquidity(id.clone(), &mut vars).expect_commit_success();

    assert_balance(vars.liquidity_receipt, dec!(0), &mut vars);
}

#[test]
fn test_remove_liquidity() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    remove_liquidity(id.clone(), &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        None
    );
    assert_balance(vars.liquidity_receipt, dec!(0), &mut vars);
    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_final() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick, amount_x)], &mut vars).expect_commit_success();

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
fn test_remove_specific_liquidity_many_final_ascending() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0 + vars.bin_span;
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    let mut claims = vec![];
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = dec!(2);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
        claims.push((tick, amount_x))
    }

    remove_specific_liquidity(id.clone(), claims, &mut vars).expect_commit_success();

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
fn test_remove_specific_liquidity_many_final_decending() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0 + vars.bin_span;
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    let mut claims = vec![];
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = dec!(2);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
        claims.push((tick, amount_y))
    }

    remove_specific_liquidity(id.clone(), claims, &mut vars).expect_commit_success();

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
fn test_remove_specific_liquidity_active_part() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(4);
    let amount_y = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick, amount_x / 2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x / 2, amount_y / 2))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick, (amount_x / 2))])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x / 2, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y / 2, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_active_ascending_tick_movement() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0 + vars.bin_span;
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    let mut claims: Vec<(u32, Decimal)> = vec![];
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = dec!(2);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
        claims.push((tick, amount_x))
    }

    let mut expected_claims: HashMap<u32, Decimal> = claims.clone().into_iter().collect();
    for (tick, amount) in claims {
        assert_eq!(
            get_active_tick(&mut vars),
            Some(tick)
        );
        assert_eq!(
            get_liquidity_claims(id.clone(), &mut vars),
            expected_claims
        );

        remove_specific_liquidity(id.clone(), vec![(tick, amount)], &mut vars).expect_commit_success();
        expected_claims.remove(&tick);
    }

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
fn test_remove_specific_liquidity_active_decending_tick_movement() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0 + vars.bin_span;
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    let mut claims = vec![];
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = dec!(2);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
        claims.push((tick, amount_y))
    }

    let mut expected_claims: HashMap<u32, Decimal> = claims.clone().into_iter().collect();
    for (tick, amount) in claims {
        assert_eq!(
            get_active_tick(&mut vars),
            Some(tick)
        );
        assert_eq!(
            get_liquidity_claims(id.clone(), &mut vars),
            expected_claims
        );

        remove_specific_liquidity(id.clone(), vec![(tick, amount)], &mut vars).expect_commit_success();
        expected_claims.remove(&tick);
    }

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
fn test_remove_specific_liquidity_active_move_up_only() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick0, amount_x0)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x1, dec!(0)))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick1, amount_x1)])
    );
}

#[test]
fn test_remove_specific_liquidity_active_move_down_only() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(1);
    let amount_y1 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick0, amount_x0)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((dec!(0), amount_y1))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick1, amount_y1)])
    );
}

#[test]
fn test_remove_specific_liquidity_active_move_up() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let tick2 = tick0 - vars.bin_span * 100;
    let amount_x2 = dec!(1);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick2, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick0, amount_x0)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick1)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x1, dec!(0)))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick1, amount_x1), (tick2, amount_y2)])
    );
}

#[test]
fn test_remove_specific_liquidity_active_move_down() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(3);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span * 100;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    let tick2 = tick0 - vars.bin_span;
    let amount_x2 = dec!(1);
    let amount_y2 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x2, amount_y2, vec![(tick2, amount_x2, amount_y2)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick0, amount_x0)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars),
        Some(tick2)
    );
    assert_eq!(
        get_active_amounts(&mut vars),
        Some((dec!(0), amount_y2))
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick1, amount_x1), (tick2, amount_y2)])
    );
}

#[test]
fn test_remove_specific_liquidity_bin_above_part() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 + vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick1, amount_x1 / 2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars),
        vec![(tick0, amount_x0), (tick1, amount_x1 / 2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, (amount_x1 / 2))])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0 - amount_x1 / 2, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_bin_below_part() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
    add_liquidity_to_receipt(id.clone(), amount_x1, amount_y1, vec![(tick1, amount_x1, amount_y1)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick1, amount_y1 / 2)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars),
        vec![(tick0, amount_y0), (tick1, amount_y1 / 2)]
    );
    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars),
        HashMap::from([(tick0, amount_x0), (tick1, (amount_y1 / 2))])
    );
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0 - amount_y1 / 2, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_bin_above_all() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
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
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_bin_below_all() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0 + vars.bin_span;
    let amount_x0 = dec!(4);
    let amount_y0 = dec!(2);
    add_liquidity_to_receipt(id.clone(), amount_x0, amount_y0, vec![(tick0, amount_x0, amount_y0)], &mut vars).expect_commit_success();

    let tick1 = tick0 - vars.bin_span;
    let amount_x1 = dec!(2);
    let amount_y1 = dec!(1);
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
    assert_balance(vars.token_x, vars.amount_x - amount_x0, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - amount_y0, &mut vars);
}

#[test]
fn test_remove_specific_liquidity_more_than_claim() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick, amount_x + dec!(1))], &mut vars).expect_commit_success();

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
fn test_remove_liquidity_from_receipt_many_above() {
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
    add_liquidity_to_receipt(id.clone(), total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        expected
    );

    remove_liquidity(id.clone(), &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        vec![]
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
    add_liquidity_to_receipt(id.clone(), total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        expected
    );

    remove_liquidity(id.clone(), &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_remove_specific_liquidity_negative_claim_invalid() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    remove_specific_liquidity(id.clone(), vec![(tick, dec!(-1))], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Claim must be greater than zero.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_remove_specific_liquidity_without_claim_invalid() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id0, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    let id1 = mint_liquidity_receipt(&mut vars);
    remove_specific_liquidity(id1, vec![(tick, amount_x)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Claim does not exist.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_remove_specific_liquidity_other_liquidity_receipt_invalid() {
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
    let liquidity_receipt_0 = vars.liquidity_receipt;

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!("1.5");
    let amount_y = dec!("0.5");
    add_liquidity_to_receipt(id0.clone(), amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();
    
    vars.quantaswap_component = quantaswap_component_1;
    vars.liquidity_receipt = liquidity_receipt_1;

    let id1 = mint_liquidity_receipt(&mut vars);
    add_liquidity_to_receipt(id1, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    vars.liquidity_receipt = liquidity_receipt_0;
    remove_specific_liquidity(id0.clone(), vec![(tick, amount_x)], &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid liquidity receipt.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_remove_specific_liquidity_multiple_liquidity_receipts_invalid() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let id1 = mint_liquidity_receipt(&mut vars);

    let tick = Tick::ONE.0 + vars.bin_span;
    let amount_x = dec!(2);
    let amount_y = dec!(1);
    let positions = vec![(tick, amount_x, amount_y)];
    let claims = vec![(tick, amount_x)];

    add_liquidity_to_receipt(id0.clone(), amount_x, amount_y, positions.clone(), &mut vars).expect_commit_success();
    add_liquidity_to_receipt(id1.clone(), amount_x, amount_y, positions.clone(), &mut vars).expect_commit_success();

    let ids = BTreeSet::from([id0, id1]);

    let manifest = ManifestBuilder::new()
        .withdraw_non_fungibles_from_account(vars.account_component, vars.liquidity_receipt, ids.clone())
        .take_non_fungibles_from_worktop(vars.liquidity_receipt, ids.clone(), "liquidity_receipt")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                vars.quantaswap_component,
                "remove_specific_liquidity",
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
