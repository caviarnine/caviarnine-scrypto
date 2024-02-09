#![allow(dead_code)]
use scrypto::prelude::*;
use quantaswap::tick::Tick;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::quantaswap::*;
pub use crate::common::fee_controller;

#[test]
fn test_get_fee_controller_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_fee_controller_address(&mut vars), 
        vars.fee_controller_component
    );
}

#[test]
fn test_get_fee_vaults_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_fee_vaults_address(&mut vars), 
        vars.fee_vaults_component
    );
}

#[test]
fn test_get_token_x_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_token_x_address(&mut vars), 
        vars.token_x
    );
}

#[test]
fn test_get_token_y_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_token_y_address(&mut vars), 
        vars.token_y
    );
}

#[test]
fn test_get_liquidity_receipt_address() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_liquidity_receipt_address(&mut vars), 
        vars.liquidity_receipt
    );
}

#[test]
fn test_get_bin_span() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_bin_span(&mut vars), 
        vars.bin_span
    );
}

#[test]
fn test_get_active_tick_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_active_tick(&mut vars), 
        None
    );
}

#[test]
fn test_get_active_tick_some() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id, dec!(1), dec!(0), vec![(tick, dec!(1), dec!(0))], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_tick(&mut vars), 
        Some(tick)
    );
}

#[test]
fn test_get_price_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_price(&mut vars), 
        None
    );
}

#[test]
fn test_get_price_some() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id, dec!(1), dec!(0), vec![(tick, dec!(1), dec!(0))], &mut vars).expect_commit_success();
    
    let price_sqrt = Decimal::from(Tick(tick));
    let price_expected = price_sqrt * price_sqrt;
    let range = get_active_bin_price_range(&mut vars).unwrap();
    let price = get_price(&mut vars).unwrap();

    assert_within_error_margin(price, price_expected, dec!("0.0000000000001"));
    assert_within_error_margin(price, range.0, dec!("0.0000000000001"));
}

#[test]
fn test_get_active_bin_price_range_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_active_bin_price_range(&mut vars), 
        None
    );
}

#[test]
fn test_get_active_bin_price_range_some() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let lower_sqrt = Decimal::from(Tick(tick));
    let upper_sqrt = Decimal::from(Tick(tick).tick_upper(vars.bin_span));
    add_liquidity_to_receipt(id, dec!(1), dec!(0), vec![(tick, dec!(1), dec!(0))], &mut vars).expect_commit_success();

    let (lower, upper) = get_active_bin_price_range(&mut vars).unwrap();
    assert_eq!(
        lower, 
        lower_sqrt * lower_sqrt
    );
    assert_eq!(
        upper, 
        upper_sqrt * upper_sqrt
    );
}


#[test]
fn test_get_active_amounts_none() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_active_amounts(&mut vars),
        None
    )
}

#[test]
fn test_get_active_amounts_some() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(2);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_active_amounts(&mut vars),
        Some((amount_x, amount_y))
    )
}

#[test]
fn test_get_amount_x_initial() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_x(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_amount_y_initial() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_amount_y(&mut vars), 
        dec!(0)
    );
}

#[test]
fn test_get_amount_x_after_add_liquidity_to_receipt() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(1);
    let amount_y = dec!(0);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_x(&mut vars), 
        amount_x
    );
}

#[test]
fn test_get_amount_y_after_add_liquidity_to_receipt() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(0);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_amount_y(&mut vars), 
        amount_y
    );
}

#[test]
fn test_get_bins_above_initial() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_below_initial() {
    let mut vars: Vars = setup();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_above_after_add_liquidity_to_receipt() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(2);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        vec![(tick, amount_x)]
    );
}

#[test]
fn test_get_bins_below_after_add_liquidity_to_receipt() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    let amount_x = dec!(2);
    let amount_y = dec!(1);
    add_liquidity_to_receipt(id, amount_x, amount_y, vec![(tick, amount_x, amount_y)], &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        vec![(tick, amount_y)]
    );
}

#[test]
fn test_get_bins_above_after_add_liquidity_to_receipt_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
fn test_get_bins_below_after_add_liquidity_to_receipt_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
fn test_get_bins_above_after_add_liquidity_to_receipt_with_gap() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let tick1 = tick0 + vars.bin_span * 50;
    let amount_x = dec!(1);
    let amount_y = dec!(0);
    let total_amount_x = amount_x * dec!(2);
    let total_amount_y = amount_y * dec!(2);
    
    let positions = vec![
        (tick0, amount_x, amount_y), 
        (tick1, amount_x, amount_y)
    ];
    let expected = vec![
        (tick0, amount_x),
        (tick1, amount_x),
    ];

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_after_add_liquidity_to_receipt_with_gap() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick0 = Tick::ONE.0;
    let tick1 = tick0 - vars.bin_span * 50;
    let amount_x = dec!(0);
    let amount_y = dec!(1);
    let total_amount_x = amount_x * dec!(2);
    let total_amount_y = amount_y * dec!(2);
    
    let positions = vec![
        (tick0, amount_x, amount_y), 
        (tick1, amount_x, amount_y)
    ];
    let expected = vec![
        (tick0, amount_y),
        (tick1, amount_y),
    ];

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_after_add_liquidity_to_receipt_both_sides() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_x));
    }
    for i in 1..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_after_add_liquidity_to_receipt_both_sides() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_y));
    }
    for i in 1..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_start_tick_before() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(Some(start_tick -1), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_start_tick_before() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(Some(start_tick +1), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_start_tick_at() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_x));
    }
    expected.remove(0);

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(Some(start_tick), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_start_tick_at() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_y));
    }
    expected.remove(0);

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(Some(start_tick), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_start_tick_after() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_x));
    }
    expected.remove(0);

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(Some(start_tick +1), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_start_tick_after() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
        expected.push((tick, amount_y));
    }
    expected.remove(0);

    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(Some(start_tick -1), None, None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_stop_tick_before() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_above(None, Some(start_tick -1), None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_below_stop_tick_before() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    add_liquidity_to_receipt(id, total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();

    assert_eq!(
        get_bins_below(None, Some(start_tick +1), None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_above_stop_tick_at() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(None, Some(start_tick), None, &mut vars), 
        vec![expected[0]]
    );
}

#[test]
fn test_get_bins_below_stop_tick_at() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(None, Some(start_tick), None, &mut vars), 
        vec![expected[0]]
    );
}

#[test]
fn test_get_bins_above_stop_tick_after() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(None, Some(start_tick + 10 * vars.bin_span), None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_stop_tick_after() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(None, Some(start_tick - 10 * vars.bin_span), None, &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_number_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(None, None, Some(5), &mut vars), 
        expected[..5].to_vec()
    );
}

#[test]
fn test_get_bins_below_number_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(None, None, Some(5), &mut vars), 
        expected[..5].to_vec()
    );
}

#[test]
fn test_get_bins_above_number_stopped_zero() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(None, None, Some(0), &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_below_number_stopped_zero() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(None, None, Some(0), &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_above_number_not_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(None, None, Some(10), &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_below_number_not_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(None, None, Some(10), &mut vars), 
        expected
    );
}

#[test]
fn test_get_bins_above_start_tick_number_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(Some(start_tick -1), None, Some(5), &mut vars), 
        expected[..5].to_vec()
    );
}

#[test]
fn test_get_bins_below_start_tick_number_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(Some(start_tick +1), None, Some(5), &mut vars), 
        expected[..5].to_vec()
    );
}

#[test]
fn test_get_bins_above_start_tick_number_stopped_zero() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(Some(start_tick -1), None, Some(0), &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_below_start_tick_number_stopped_zero() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(Some(start_tick +1), None, Some(0), &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_above_start_tick_stop_tick_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_above(Some(start_tick -1), Some(start_tick -1), None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_bins_below_start_tick_stop_tick_stopped() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut expected: Vec<(u32, Decimal)> = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
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
        get_bins_below(Some(start_tick +1), Some(start_tick +1), None, &mut vars), 
        vec![]
    );
}

#[test]
fn test_get_liquidity_claims_empty() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    assert_eq!(
        get_liquidity_claims(id, &mut vars), 
        HashMap::new()
    );
}

#[test]
fn test_get_liquidity_claims_one() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(0), vec![(tick, dec!(1), dec!(0))], &mut vars).expect_commit_success();

    assert_eq!(
        get_liquidity_claims(id.clone(), &mut vars), 
        HashMap::from([(tick, dec!(1))])
    );
}

#[test]
fn test_get_redemption_value_empty_no_current() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);

    assert_eq!(
        get_redemption_value(id, &mut vars), 
        (dec!(0), dec!(0))
    );
}

#[test]
fn test_get_redemption_value_empty_some_current() {
    let mut vars: Vars = setup();

    let id0 = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id0.clone(), dec!(1), dec!(1), vec![(tick, dec!(1), dec!(1))], &mut vars).expect_commit_success();


    let id1 = mint_liquidity_receipt(&mut vars);

    assert_eq!(
        get_redemption_value(id1, &mut vars), 
        (dec!(0), dec!(0))
    );
}

#[test]
fn test_get_redemption_value_one() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick, dec!(1), dec!(1))], &mut vars).expect_commit_success();

    assert_eq!(
        get_redemption_value(id, &mut vars), 
        (dec!(1), dec!(1))
    );
}

#[test]
fn test_get_redemption_value_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    for i in 1..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }

    add_liquidity_to_receipt(id.clone(), total_amount_x, total_amount_y, positions, &mut vars).expect_commit_success();
    
    assert_eq!(
        get_redemption_value(id, &mut vars), 
        (total_amount_x, total_amount_y)
    );
}

#[test]
fn test_get_redemption_bin_values_one() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let tick = Tick::ONE.0;
    add_liquidity_to_receipt(id.clone(), dec!(1), dec!(1), vec![(tick, dec!(1), dec!(1))], &mut vars).expect_commit_success();

    assert_eq!(
        get_redemption_bin_values(id, &mut vars), 
        vec![(tick, dec!(1), dec!(1))]
    );
}

#[test]
fn test_get_redemption_bin_values_many() {
    let mut vars: Vars = setup();

    let id = mint_liquidity_receipt(&mut vars);
    let start_tick = Tick::ONE.0;
    
    let mut positions = vec![];
    let mut total_amount_x = dec!(0);
    let mut total_amount_y = dec!(0);
    for i in 0..10 {
        let tick = start_tick + i * vars.bin_span;
        let amount_x = Decimal::from(i + 1);
        let amount_y = dec!(0);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }
    for i in 1..10 {
        let tick = start_tick - i * vars.bin_span;
        let amount_x = dec!(0);
        let amount_y = Decimal::from(i + 1);
        total_amount_x += amount_x;
        total_amount_y += amount_y;
        positions.push((tick, amount_x, amount_y));
    }

    add_liquidity_to_receipt(id.clone(), total_amount_x, total_amount_y, positions.clone(), &mut vars).expect_commit_success();
    
    positions.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", positions);
    assert_eq!(
        get_redemption_bin_values(id, &mut vars), 
        positions
    );
}
