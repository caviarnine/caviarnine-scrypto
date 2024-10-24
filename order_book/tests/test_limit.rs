#![allow(dead_code)]
use scrypto::prelude::*;
use radix_engine::errors::RuntimeError::ApplicationError;
use radix_engine::errors::ApplicationError::PanicMessage;

use order_book::price::Price;

mod common;
pub use crate::common::vars::*;
pub use crate::common::setup::*;
pub use crate::common::misc::*;
pub use crate::common::order_book::*;
pub use crate::common::fee_controller;

#[test]
fn test_limit_basic_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );
        
    let limits = get_ask_limits(None, None, None, &mut vars);
    
    assert_eq!(
        limits.len(), 
        1
    );

    assert_eq!(
        limits[0], 
        (dec!(1), dec!(1))
    );

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_limit_basic_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );
        
    let limits = get_bid_limits(None, None, None, &mut vars);
    
    assert_eq!(
        limits.len(), 
        1
    );

    assert_eq!(
        limits[0], 
        (dec!(1), dec!(1))
    );

    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_limit_same_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        Some(dec!(1))
    );
        
    let limits = get_ask_limits(None, None, None, &mut vars);
    
    assert_eq!(
        limits.len(), 
        1
    );

    assert_eq!(
        limits[0], 
        (dec!(1), dec!(2))
    );

    assert_balance(vars.token_x, vars.amount_x - dec!(2), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(2)
    );
}

#[test]
fn test_limit_same_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        Some(dec!(1))
    );
        
    let limits = get_bid_limits(None, None, None, &mut vars);
    
    assert_eq!(
        limits.len(), 
        1
    );

    assert_eq!(
        limits[0], 
        (dec!(1), dec!(2))
    );

    assert_balance(vars.token_y, vars.amount_y - dec!(2), &mut vars);

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(2)
    );
}

#[test]
fn test_limit_truncate_price_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!("5.54864"), dec!("213.12333"), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars)[0], 
        (dec!("213.12"), dec!("5.54864"))
    );
}

#[test]
fn test_limit_truncate_price_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!("213.12333"), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars)[0], 
        (dec!("213.12"), dec!("0.004692192192192192"))
    );
}

#[test]
fn test_limit_amount_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars)[0], 
        (dec!(2), dec!(1))
    );
}

#[test]
fn test_limit_cross_same_x_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(1), &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(0)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(1)
    );
}

#[test]
fn test_limit_cross_basic_same_y_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);

    assert_eq!(
        get_amount_x(&mut vars),
        dec!(1)
    );

    assert_eq!(
        get_amount_y(&mut vars),
        dec!(0)
    );
}

#[test]
fn test_limit_cross_larger_size_better_price_x_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(2), dec!("0.5"))]
    );
}

#[test]
fn test_limit_cross_larger_size_better_price_y_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(2), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(2), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_limit_cross_no_collision_x_y() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();
    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_limit_cross_no_collision_y_x() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    limit_order(vars.token_y, dec!(1), dec!(1), &mut vars).expect_commit_success();
    limit_order(vars.token_x, dec!(1), dec!(2), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(dec!(2), dec!(1))]
    );

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(dec!(1), dec!(1))]
    );
}

#[test]
fn test_limit_small_price_x() {
    let mut vars: Vars = setup();

    let amount = dec!(1);
    let price = Price::DECIMAL_MIN;
    limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(price, amount)]
    );
}

#[test]
fn test_limit_small_price_y() {
    let mut vars: Vars = setup();
    
    let amount = dec!(1);
    let price = Price::DECIMAL_MIN;
    limit_order(vars.token_y, amount, price, &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(price, amount / price)]
    );
}

#[test]
fn test_limit_large_amount_x() {
    let mut vars: Vars = setup();

    let amount = dec!("1000000000000000000");
    let price = dec!(1);
    limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(price, amount)]
    );
}

#[test]
fn test_limit_large_amount_y() {
    let mut vars: Vars = setup();

    let amount = dec!("1000000000000000000");
    let price = dec!(1);
    limit_order(vars.token_y, amount, price, &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![(price, amount / price)]
    );
}

#[test]
fn test_limit_large_price_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), Decimal::from(Price::MAX), &mut vars).expect_commit_success();
}

#[test]
fn test_limit_large_price_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), Decimal::from(Price::MAX), &mut vars).expect_commit_success();
}

#[test]
fn test_limit_small_amount_large_price_x() {
    let mut vars: Vars = setup();

    let amount = dec!("0.000000000000000001").checked_round(vars.divisibility_x, RoundingMode::ToPositiveInfinity).unwrap();
    let price = Price::DECIMAL_MAX;
    limit_order(vars.token_x, amount, price, &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![(price, amount)]
    );
}

#[test]
fn test_limit_small_amount_large_price_y() {
    let mut vars: Vars = setup();

    let amount = dec!("0.000000000000000001").checked_round(vars.divisibility_y, RoundingMode::ToPositiveInfinity).unwrap();
    let price = Price::DECIMAL_MAX;
    limit_order(vars.token_y, amount, price, &mut vars).expect_commit_success();

    if amount / price > dec!(0) {
        assert_eq!(
            get_bid_limits(None, None, None, &mut vars),
            vec![(price, amount / price)]
        );
    } else {
        assert_eq!(
            get_bid_limits(None, None, None, &mut vars),
            vec![]
        );
    }
}

#[test]
fn test_limit_amount_zero_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(0), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_ask_limits(None, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_limit_amount_zero_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(0), dec!(1), &mut vars).expect_commit_success();

    assert_eq!(
        get_bid_limits(None, None, None, &mut vars),
        vec![]
    );
}

#[test]
fn test_limit_price_zero_invalid_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!(0), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("0 is not in valid price range")
            },
            _ => false,
        }
    });
}

#[test]
fn test_limit_price_negative_invalid_x() {
    let mut vars: Vars = setup();

    limit_order(vars.token_x, dec!(1), dec!("-0.1"), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("-0.1 is not in valid price range")
            },
            _ => false,
        }
    });
}

#[test]
fn test_limit_price_zero_invalid_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!(0), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("0 is not in valid price range")
            },
            _ => false,
        }
    });
}

#[test]
fn test_limit_price_negative_invalid_y() {
    let mut vars: Vars = setup();

    limit_order(vars.token_y, dec!(1), dec!("-0.1"), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("-0.1 is not in valid price range")
            },
            _ => false,
        }
    });
}

#[test]
fn test_limit_other_token_invalid() {
    let mut vars: Vars = setup();

    let token_a = vars.test_runner.create_fungible_resource(
        dec!(1000),
        DIVISIBILITY_MAXIMUM,
        vars.account_component,
    );

    limit_order(token_a, dec!(1), dec!(1), &mut vars).expect_specific_failure(|err| {
        match err {
            ApplicationError(PanicMessage(msg)) => {
                msg.contains("Invalid token address.")
            },
            _ => false,
        }
    });
}

#[test]
fn test_limit_batch_none_x() {
    let mut vars: Vars = setup();

    let positions = vec![];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let ask_limits = get_ask_limits(None, None, None, &mut vars);
    
    assert_eq!(
        ask_limits.len(), 
        0
    );
}

#[test]
fn test_limit_batch_none_y() {
    let mut vars: Vars = setup();

    let positions = vec![];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let bid_limits = get_bid_limits(None, None, None, &mut vars);
    
    assert_eq!(
        bid_limits.len(), 
        0
    );
}

#[test]
fn test_limit_batch_ascending_x() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_x, dec!(1), dec!(1)), 
            (vars.token_x, dec!(1), dec!(2)), 
            (vars.token_x, dec!(1), dec!(3)), 
            (vars.token_x, dec!(1), dec!(4)), 
            (vars.token_x, dec!(1), dec!(5)), 
            (vars.token_x, dec!(1), dec!(6)), 
            (vars.token_x, dec!(1), dec!(7)), 
            (vars.token_x, dec!(1), dec!(8)), 
            (vars.token_x, dec!(1), dec!(9)), 
            (vars.token_x, dec!(1), dec!(10)),
            (vars.token_x, dec!(1), dec!(11)),
            (vars.token_x, dec!(1), dec!(12)),
            (vars.token_x, dec!(1), dec!(13)),
            (vars.token_x, dec!(1), dec!(14)),
            (vars.token_x, dec!(1), dec!(15)),
            (vars.token_x, dec!(1), dec!(16)),
            (vars.token_x, dec!(1), dec!(17)),
            (vars.token_x, dec!(1), dec!(18)),
            (vars.token_x, dec!(1), dec!(19)),
            (vars.token_x, dec!(1), dec!(20)),
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let ask_limits = get_ask_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| a.2.cmp(&b.2));
    
    for i in 0..ask_limits.len() {
        assert_eq!(
            ask_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            ask_limits[i].1, 
            positions[i].1,
        );
    }
}

#[test]
fn test_limit_batch_ascending_y() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_y, dec!(1), dec!(1)), 
            (vars.token_y, dec!(1), dec!(2)), 
            (vars.token_y, dec!(1), dec!(3)), 
            (vars.token_y, dec!(1), dec!(4)), 
            (vars.token_y, dec!(1), dec!(5)), 
            (vars.token_y, dec!(1), dec!(6)), 
            (vars.token_y, dec!(1), dec!(7)), 
            (vars.token_y, dec!(1), dec!(8)), 
            (vars.token_y, dec!(1), dec!(9)), 
            (vars.token_y, dec!(1), dec!(10)),
            (vars.token_y, dec!(1), dec!(11)),
            (vars.token_y, dec!(1), dec!(12)),
            (vars.token_y, dec!(1), dec!(13)),
            (vars.token_y, dec!(1), dec!(14)),
            (vars.token_y, dec!(1), dec!(15)),
            (vars.token_y, dec!(1), dec!(16)),
            (vars.token_y, dec!(1), dec!(17)),
            (vars.token_y, dec!(1), dec!(18)),
            (vars.token_y, dec!(1), dec!(19)),
            (vars.token_y, dec!(1), dec!(20)),
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let bid_limits = get_bid_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| b.2.cmp(&a.2));
    
    for i in 0..bid_limits.len() {
        assert_eq!(
            bid_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            bid_limits[i].1, 
            positions[i].1 / positions[i].2,
        );
    }
}

#[test]
fn test_limit_batch_descending_x() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_x, dec!(1), dec!(20)), 
            (vars.token_x, dec!(1), dec!(19)), 
            (vars.token_x, dec!(1), dec!(18)), 
            (vars.token_x, dec!(1), dec!(17)), 
            (vars.token_x, dec!(1), dec!(16)), 
            (vars.token_x, dec!(1), dec!(15)), 
            (vars.token_x, dec!(1), dec!(14)), 
            (vars.token_x, dec!(1), dec!(13)), 
            (vars.token_x, dec!(1), dec!(12)), 
            (vars.token_x, dec!(1), dec!(11)),
            (vars.token_x, dec!(1), dec!(10)),
            (vars.token_x, dec!(1), dec!(9)),
            (vars.token_x, dec!(1), dec!(8)),
            (vars.token_x, dec!(1), dec!(7)),
            (vars.token_x, dec!(1), dec!(6)),
            (vars.token_x, dec!(1), dec!(5)),
            (vars.token_x, dec!(1), dec!(4)),
            (vars.token_x, dec!(1), dec!(3)),
            (vars.token_x, dec!(1), dec!(2)),
            (vars.token_x, dec!(1), dec!(1)),
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let ask_limits = get_ask_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| a.2.cmp(&b.2));
    
    for i in 0..ask_limits.len() {
        assert_eq!(
            ask_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            ask_limits[i].1, 
            positions[i].1,
        );
    }
}

#[test]
fn test_limit_batch_descending_y() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_x, dec!(1), dec!(20)), 
            (vars.token_x, dec!(1), dec!(19)), 
            (vars.token_x, dec!(1), dec!(18)), 
            (vars.token_x, dec!(1), dec!(17)), 
            (vars.token_x, dec!(1), dec!(16)), 
            (vars.token_x, dec!(1), dec!(15)), 
            (vars.token_x, dec!(1), dec!(14)), 
            (vars.token_x, dec!(1), dec!(13)), 
            (vars.token_x, dec!(1), dec!(12)), 
            (vars.token_x, dec!(1), dec!(11)),
            (vars.token_x, dec!(1), dec!(10)),
            (vars.token_x, dec!(1), dec!(9)),
            (vars.token_x, dec!(1), dec!(8)),
            (vars.token_x, dec!(1), dec!(7)),
            (vars.token_x, dec!(1), dec!(6)),
            (vars.token_x, dec!(1), dec!(5)),
            (vars.token_x, dec!(1), dec!(4)),
            (vars.token_x, dec!(1), dec!(3)),
            (vars.token_x, dec!(1), dec!(2)),
            (vars.token_x, dec!(1), dec!(1)),
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let bid_limits = get_bid_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| b.2.cmp(&a.2));
    
    for i in 0..bid_limits.len() {
        assert_eq!(
            bid_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            bid_limits[i].1, 
            positions[i].1 / positions[i].2,
        );
    }
}

#[test]
fn test_limit_batch_random_x() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_x, dec!(1), dec!(10)),
            (vars.token_x, dec!(1), dec!(6)),
            (vars.token_x, dec!(1), dec!(19)), 
            (vars.token_x, dec!(1), dec!(13)), 
            (vars.token_x, dec!(1), dec!(18)), 
            (vars.token_x, dec!(1), dec!(1)),
            (vars.token_x, dec!(1), dec!(4)),
            (vars.token_x, dec!(1), dec!(14)), 
            (vars.token_x, dec!(1), dec!(17)), 
            (vars.token_x, dec!(1), dec!(8)),
            (vars.token_x, dec!(1), dec!(7)),
            (vars.token_x, dec!(1), dec!(20)), 
            (vars.token_x, dec!(1), dec!(9)),
            (vars.token_x, dec!(1), dec!(5)),
            (vars.token_x, dec!(1), dec!(15)), 
            (vars.token_x, dec!(1), dec!(12)), 
            (vars.token_x, dec!(1), dec!(3)),
            (vars.token_x, dec!(1), dec!(11)),
            (vars.token_x, dec!(1), dec!(2)),
            (vars.token_x, dec!(1), dec!(16)), 
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let ask_limits = get_ask_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| a.2.cmp(&b.2));
    
    for i in 0..ask_limits.len() {
        assert_eq!(
            ask_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            ask_limits[i].1, 
            positions[i].1,
        );
    }

    assert_balance(vars.token_x, vars.amount_x - dec!(20), &mut vars);
    assert_balance(vars.token_y, vars.amount_y, &mut vars);
}

#[test]
fn test_limit_batch_random_y() {
    let mut vars: Vars = setup();

    let mut positions = 
        vec![
            (vars.token_y, dec!(1), dec!(10)),
            (vars.token_y, dec!(1), dec!(6)),
            (vars.token_y, dec!(1), dec!(19)), 
            (vars.token_y, dec!(1), dec!(13)), 
            (vars.token_y, dec!(1), dec!(18)), 
            (vars.token_y, dec!(1), dec!(1)),
            (vars.token_y, dec!(1), dec!(4)),
            (vars.token_y, dec!(1), dec!(14)), 
            (vars.token_y, dec!(1), dec!(17)), 
            (vars.token_y, dec!(1), dec!(8)),
            (vars.token_y, dec!(1), dec!(7)),
            (vars.token_y, dec!(1), dec!(20)), 
            (vars.token_y, dec!(1), dec!(9)),
            (vars.token_y, dec!(1), dec!(5)),
            (vars.token_y, dec!(1), dec!(15)), 
            (vars.token_y, dec!(1), dec!(12)), 
            (vars.token_y, dec!(1), dec!(3)),
            (vars.token_y, dec!(1), dec!(11)),
            (vars.token_y, dec!(1), dec!(2)),
            (vars.token_y, dec!(1), dec!(16)),
        ];

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let bid_limits = get_bid_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| b.2.cmp(&a.2));
    
    for i in 0..bid_limits.len() {
        assert_eq!(
            bid_limits[i].0, 
            positions[i].2,
        );
        assert_eq!(
            bid_limits[i].1, 
            positions[i].1 / positions[i].2,
        );
    }

    assert_balance(vars.token_x, vars.amount_x, &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(20), &mut vars);
}

#[test]
fn test_limit_batch_mix() {
    let mut vars: Vars = setup();

    let mut positions: Vec<(ResourceAddress, Decimal, Decimal)> = vec![];
    let mut ids: BTreeSet<NonFungibleLocalId> = BTreeSet::new();
    for i in 0..5 {
        positions.push((vars.token_y, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }
    for i in 5..10 {
        positions.push((vars.token_x, dec!(1), Decimal::from(i + 1)));
        ids.insert(NonFungibleLocalId::integer(i + 1));
    }

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    let ask_limits = get_ask_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| a.2.cmp(&b.2));
    let ask_positions = &positions[5..];

    for i in 0..ask_limits.len() {
        assert_eq!(
            ask_limits[i].0, 
            ask_positions[i].2,
        );
        assert_eq!(
            ask_limits[i].1, 
            ask_positions[i].1,
        );
    }

    let bid_limits = get_bid_limits(None, None, None, &mut vars);
    positions.sort_by(|a, b| b.2.cmp(&a.2));
    let bid_positions = &positions[5..];

    for i in 0..bid_limits.len() {
        assert_eq!(
            bid_limits[i].0, 
            bid_positions[i].2,
        );
        assert_eq!(
            bid_limits[i].1, 
            bid_positions[i].1 / bid_positions[i].2,
        );
    }

    assert_balance(vars.token_x, vars.amount_x - dec!(5), &mut vars);
    assert_balance(vars.token_y, vars.amount_y - dec!(5), &mut vars);
}

#[test]
fn test_limit_batch_mix_overlap() {
    let mut vars: Vars = setup();
    fee_controller::set_protocol_fee_default_zero(&mut vars);

    let mut positions: Vec<(ResourceAddress, Decimal, Decimal)> = vec![];
    let mut ids: BTreeSet<NonFungibleLocalId> = BTreeSet::new();

    positions.push((vars.token_y, dec!(1), Decimal::from(1)));
    ids.insert(NonFungibleLocalId::integer(1));

    positions.push((vars.token_x, dec!(1), Decimal::from(1)));
    ids.insert(NonFungibleLocalId::integer(2));

    limit_order_batch(positions.clone(), &mut vars).expect_commit_success();

    assert_eq!(
        get_current_ask_price(&mut vars),
        None
    );
    assert_eq!(
        get_current_bid_price(&mut vars),
        None
    );

    assert_balance(vars.token_x, vars.amount_x - dec!(1), &mut vars);
    assert_balance_accept_missing_attos(vars.token_y, vars.amount_y, &mut vars);
}
