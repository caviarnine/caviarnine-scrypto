use scrypto::prelude::*;

use quantaswap::swap_math::*;
use quantaswap::tick::*;
use quantaswap::consts::*;

#[test]
fn test_swap_01() {
    let input_a = dec!(1);
    let reserve_a = dec!(1);
    let reserve_b = dec!(1);
    let k0 = reserve_a * reserve_b;

    let output_b = calculate_swap(&input_a, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("output_b: {}", output_b);

    assert_eq!(output_b, dec!("0.5"));
    assert_eq!(k0, k1);
}

#[test]
fn test_swap_02() {
    let input_a = dec!(1);
    let reserve_a = dec!(1);
    let reserve_b = dec!(2);
    let k0 = reserve_a * reserve_b;

    let output_b = calculate_swap(&input_a, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("output_b: {}", output_b);

    assert_eq!(output_b, dec!(1));
    assert_eq!(k0, k1);
}

#[test]
fn test_swap_03() {
    let input_a = dec!("0.0000000042");
    let reserve_a = dec!("519.423445");
    let reserve_b = dec!("0.2333333");
    let k0 = reserve_a * reserve_b;

    let output_b = calculate_swap(&input_a, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("output_b: {}", output_b);

    assert!(k0 < k1 && k1 - k0 <= dec!("0.000000000000000001") * (reserve_a + input_a));
}

#[test]
fn test_inverse_swap_01() {
    let output_b = dec!("0.5");
    let reserve_a = dec!(1);
    let reserve_b = dec!(1);
    let k0 = reserve_a * reserve_b;

    let input_a = calculate_swap_inverse(&output_b, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("input_a: {}", input_a);

    assert_eq!(input_a, dec!(1));
    assert_eq!(k0, k1);
}

#[test]
fn test_inverse_swap_02() {
    let output_b = dec!(1);
    let reserve_a = dec!(1);
    let reserve_b = dec!(2);
    let k0 = reserve_a * reserve_b;

    let input_a = calculate_swap_inverse(&output_b, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("input_a: {}", input_a);

    assert_eq!(input_a, dec!(1));
    assert_eq!(k0, k1);
}

#[test]
fn test_inverse_swap_03() {
    let output_b = dec!("7.0000000042");
    let reserve_a = dec!("519.423445");
    let reserve_b = dec!("888.2333333");
    let k0 = reserve_a * reserve_b;

    let input_a = calculate_swap_inverse(&output_b, &(I512::from(reserve_a.0) * _10_E18), &(I512::from(reserve_b.0) * _10_E18));
    let k1 = (reserve_a + input_a) * (reserve_b - output_b);

    println!("input_a: {}", input_a);
    println!("k0: {}", k0);
    println!("k1: {}", k1);
    assert!((k1 - k0).checked_abs().unwrap() <= dec!("0.000000000000000001") * (reserve_b - output_b));
}

#[test]
fn test_virtual_amounts_no_x() {
    let real_x = dec!(0);
    let real_y = dec!(1);
    let lower_limit = dec!(1).checked_sqrt().unwrap();
    let upper_limit = dec!(2).checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    assert!(
        (price - price_upper_limit).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_no_y() {
    let real_x = dec!(1);
    let real_y = dec!(0);
    let lower_limit = dec!(1).checked_sqrt().unwrap();
    let upper_limit = dec!(2).checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    assert!(
        (price - price_lower_limit).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_max_range() {
    let real_x = Decimal(I192::from(2).pow(152));
    let real_y = Decimal(I192::from(2).pow(152));
    let lower_limit = dec!("0.000000102236575544");
    let upper_limit = dec!("9781235.283719825766962222");

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    assert!(
        (price - dec!(1)).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_max_range_no_x() {
    let real_x = dec!(0);
    let real_y = Decimal(I192::from(2).pow(152));
    let lower_limit = dec!("0.000000102236575544");
    let upper_limit = dec!("9781235.283719825766962222");

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    assert!(
        (price - price_upper_limit).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_max_range_no_y() {
    let real_x = Decimal(I192::from(2).pow(152));
    let real_y = dec!(0);
    let lower_limit = dec!("0.000000102236575544");
    let upper_limit = dec!("9781235.283719825766962222");

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    assert!(
        (price - price_lower_limit).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_swap_x_01() {
    let real_x = dec!(1);
    let real_y = dec!(1);
    let lower_limit = dec!(1).checked_sqrt().unwrap();
    let upper_limit = dec!(2).checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_x = calculate_swap_inverse(&real_y, &virtual_x_512, &virtual_y_512);
    virtual_x += input_x;
    virtual_y -= real_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_lower_limit).checked_abs().unwrap() <= dec!("0.000000000000000001"),
    );
}

#[test]
fn test_virtual_amounts_swap_x_02() {
    let real_x = dec!("0.2133");
    let real_y = dec!("9.231");
    let lower_limit = dec!("32.9482").checked_sqrt().unwrap();
    let upper_limit = dec!("52.3441").checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_x = calculate_swap_inverse(&real_y, &virtual_x_512, &virtual_y_512);
    virtual_x += input_x;
    virtual_y -= real_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_lower_limit).checked_abs().unwrap() <= dec!("0.000000000000000001") * (price_lower_limit + price_upper_limit) / 2,
    );
}

#[test]
fn test_virtual_amounts_swap_x_03() {
    let real_x = dec!("0.325");
    let real_y = dec!("0.267");
    let lower_limit = dec!("5554.062334").checked_sqrt().unwrap();
    let upper_limit = dec!("6566.556").checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_x = calculate_swap_inverse(&real_y, &virtual_x_512, &virtual_y_512);
    virtual_x += input_x;
    virtual_y -= real_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_lower_limit).checked_abs().unwrap() <= dec!("0.000000000000000001") * (price_lower_limit + price_upper_limit) / 2,
    );
}


#[test]
fn test_virtual_amounts_swap_y_01() {
    let real_x = dec!(1);
    let real_y = dec!(1);
    let lower_limit = dec!(1).checked_sqrt().unwrap();
    let upper_limit = dec!(2).checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_y = calculate_swap_inverse(&real_x, &virtual_y_512, &virtual_x_512);
    virtual_x -= real_x;
    virtual_y += input_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_upper_limit).checked_abs().unwrap() <= dec!("0.000000000000000001") * (price_lower_limit + price_upper_limit) / 2,
    );
}

#[test]
fn test_virtual_amounts_swap_y_02() {
    let real_x = dec!("213.43");
    let real_y = dec!("2.231");
    let lower_limit = dec!("512.355").checked_sqrt().unwrap();
    let upper_limit = dec!("542.3441").checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_y = calculate_swap_inverse(&real_x, &virtual_y_512, &virtual_x_512);
    virtual_x -= real_x;
    virtual_y += input_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_upper_limit).checked_abs().unwrap() <= dec!("0.000000000000000001") * (price_lower_limit + price_upper_limit) / 2,
    );
}

#[test]
fn test_virtual_amounts_swap_y_03() {
    let real_x = dec!("0.8325");
    let real_y = dec!("0.2867");
    let lower_limit = dec!("6754.062334").checked_sqrt().unwrap();
    let upper_limit = dec!("7866.556").checked_sqrt().unwrap();

    let (virtual_x_512, virtual_y_512) = calculate_virtual_amounts(&real_x, &real_y, &upper_limit, &lower_limit);
    let mut virtual_x = Decimal(I192::try_from(virtual_x_512 / _10_E18).unwrap());
    let mut virtual_y = Decimal(I192::try_from(virtual_y_512 / _10_E18).unwrap());
    let price = virtual_y / virtual_x;
    let price_lower_limit = lower_limit * lower_limit;
    let price_upper_limit = upper_limit * upper_limit;

    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);
    println!("price: {}", price);
    println!("price_lower_limit: {}", price_lower_limit);
    println!("price_upper_limit: {}", price_upper_limit);

    let input_y = calculate_swap_inverse(&real_x, &virtual_y_512, &virtual_x_512);
    virtual_x -= real_x;
    virtual_y += input_y;
    let price = virtual_y / virtual_x;

    println!("price: {}", price);
    println!("virtual_x: {}", virtual_x);
    println!("virtual_y: {}", virtual_y);

    assert!(
        (price - price_upper_limit).checked_abs().unwrap() <= dec!("0.000000000000000001") * (price_lower_limit + price_upper_limit) / 2,
    );
}

#[test]
fn test_extremes() {
    let tick_min = Tick::MIN.0;
    let limit_min = Tick(tick_min).into();
    let limit_min_up = Tick(tick_min + 1).into();    

    let tick_max = Tick::MAX.0;
    let limit_max = Tick(tick_max).into();
    let limit_max_down = Tick(tick_max - 1).into();

    let price_limits = vec![limit_min, limit_max, limit_min_up, limit_max_down];

    println!("limit_min: {:?}", limit_min);
    println!("limit_max: {:?}", limit_max);

    let amount_zero = dec!(0);
    let amount_attos = Decimal(I192::from(1));
    let amount_max = Decimal(I192::from(2).pow(152));

    let amounts = vec![amount_zero, amount_attos, amount_max];

    for real_x in &amounts {
        for real_y in &amounts {
            for upper_limit in &price_limits {
                for lower_limit in &price_limits {
                    if upper_limit <= lower_limit {
                        continue;
                    }
                    if real_x.is_zero() && real_y.is_zero() {
                        continue;
                    }

                    println!("real_x: {:?}, real_y: {:?}, upper_limit: {:?}, lower_limit: {:?}", real_x, real_y, upper_limit, lower_limit);

                    let (virtual_x, virtual_y) = calculate_virtual_amounts(real_x, real_y, upper_limit, lower_limit);                    
                    assert!(!virtual_x.is_zero());
                    assert!(!virtual_y.is_zero());

                    for input_a in &amounts {
                        calculate_swap(input_a, &virtual_x, &virtual_y);
                        calculate_swap(input_a, &virtual_y, &virtual_x);
                    }

                    calculate_swap_inverse(real_y, &virtual_x, &virtual_y);
                    calculate_swap_inverse(real_x, &virtual_y, &virtual_x);
                    calculate_swap_inverse(&amount_zero, &virtual_x, &virtual_y);
                    calculate_swap_inverse(&amount_zero, &virtual_y, &virtual_x);
                    calculate_swap_inverse(&amount_attos, &virtual_x, &virtual_y);
                    calculate_swap_inverse(&amount_attos, &virtual_y, &virtual_x);

                    let price_sqrt = calculate_price(&virtual_x, &virtual_y).checked_sqrt().unwrap();
                    println!("price_sqrt: {:?}", price_sqrt);
                    assert!(
                        price_sqrt >= *lower_limit || (price_sqrt - *lower_limit) / *lower_limit <= dec!("0.0000001"),
                    );
                    assert!(
                        price_sqrt <= *upper_limit || (price_sqrt - *upper_limit) / *upper_limit <= dec!("0.0000001"),
                    );
                }
            }
        }
    }
}
