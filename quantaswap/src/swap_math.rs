use scrypto::prelude::*;

use crate::consts::*;

/// Calculate the amount of tokens to swap given the input amount, reserve of the input token, and reserve of the output token.
/// 
/// # Arguments
/// 
/// * `input_a` - The amount of tokens to swap in `Decimal`.
/// * `reserve_a` - The reserve of the input token in `I512` with base `10^36`.
/// * `reserve_b` - The reserve of the output token in `I512` with base `10^36`.
/// 
/// # Returns
/// 
/// * `Decimal` - The amount of tokens to swap.
/// 
pub fn calculate_swap(input_a: &Decimal, reserve_a: &I512, reserve_b: &I512) -> Decimal {
    let input_a = I512::from(input_a.0) * _10_E18;
    let output_b = input_a * reserve_b / (reserve_a + input_a) / _10_E18;

    Decimal(I192::try_from(output_b).unwrap())
}

/// Calculate the amount of tokens to swap given the output amount, reserve of the input token, and reserve of the output token.
/// 
/// # Arguments
/// 
/// * `output_b` - The amount of tokens to swap in `Decimal`.
/// * `reserve_a` - The reserve of the input token in `I512` with base `10^36`.
/// * `reserve_b` - The reserve of the output token in `I512` with base `10^36`
/// 
/// # Returns
/// 
/// * `Decimal` - The amount of tokens to swap.
/// 
pub fn calculate_swap_inverse(output_b: &Decimal, reserve_a: &I512, reserve_b: &I512) -> Decimal {
    let output_b = I512::from(output_b.0) * _10_E18;
    let input_a = reserve_a * output_b / (reserve_b - output_b) / _10_E18;

    Decimal(I192::try_from(input_a).unwrap())
}

/// Calculate the amount of virtual amounts of tokens x and y given the real amounts of tokens x and y, 
/// such that real tokens x and y are exhausted at the upper and lower limits respectively.
/// 
/// # Arguments
/// 
/// * `real_x` - Real amount of tokens x.
/// * `real_y` - Real amount of tokens y.
/// * `upper_limit` - Upper limit of price sqrt at which real tokens x are exhausted.
/// * `lower_limit` - Lower limit of price sqrt at which real tokens y are exhausted.
/// 
/// # Returns
/// 
/// * `Decimal` - Virtual amount of tokens x in `I512` with base `10^36`.
/// * `Decimal` - Virtual amount of tokens y in `I512` with base `10^36`.
/// 
/// # Require
/// 
/// * Either `real_x` or `real_y` must be greater than 0.
/// * Both `upper_limit` and `lower_limit` must be in range [0.00000137559234317, 726959.55670651710488994].
/// * `upper_limit` must be greater than `lower_limit` by at least one tick.
/// 
pub fn calculate_virtual_amounts(real_x: &Decimal, real_y: &Decimal, upper_limit: &Decimal, lower_limit: &Decimal) -> (I512, I512) {
    // Convert to raw form
    let x = I512::from(real_x.0);
    let y = I512::from(real_y.0);
    let ll = I512::from(lower_limit.0);
    let ul = I512::from(upper_limit.0);

    // Solve quadratic for liquidity
    let a = ll * _10_E36 / ul - _10_E36; // base e36
    let b = x * ll + y * _10_E36 / ul; // base e36
    let c = x * y; // base e36

    let d = b * b - _4 * a * c; // base e72
    let d_sqrt = d.sqrt(); // base e36
    let liq = (-b - d_sqrt) * _10_E36 / (_2 * a); // base e36

    // Solve for virtual amounts
    let virtual_x = x * _10_E18 + liq * _10_E18 / ul; // base e36
    let virtual_y = y * _10_E18 + liq * ll / _10_E18; // base e36

    // Return virtual amounts
    (virtual_x, virtual_y)
}

/// Calculate the price sqrt given the virtual amounts of tokens x and y.
/// 
/// # Arguments
/// 
/// * `virtual_x` - Virtual amount of tokens x in `I512` with base `10^36`.
/// * `virtual_y` - Virtual amount of tokens y in `I512` with base `10^36`.
/// 
/// # Returns
/// 
/// * `Decimal` - Price sqrt in `I192` with base `10^18`.
/// 
pub fn calculate_price(virtual_x: &I512, virtual_y: &I512) -> Decimal {
    let price = virtual_y * _10_E18 / virtual_x;
    Decimal(I192::try_from(price).unwrap())
}
