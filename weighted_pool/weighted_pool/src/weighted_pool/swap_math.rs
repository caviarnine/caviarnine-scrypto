use scrypto::prelude::*;
use crate::util_math::*;

pub fn calculate_swap(
    input_reserve: Decimal,
    output_reserve: Decimal,
    input_weight: Decimal,
    output_weight: Decimal,
    input_amount: Decimal,
) -> Decimal {
    assert!(input_reserve > dec!(0));
    assert!(output_reserve > dec!(0));

    let reserve_ratio = input_reserve / (input_reserve + input_amount);
    let weight_ratio = input_weight / output_weight;
    let power_result = pow(reserve_ratio, weight_ratio);
    let output_amount = output_reserve * (Decimal::ONE - power_result);
    
    output_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100); 
        let input_weight = dec!(0.5);
        let output_weight = dec!(0.5);
        let input_amount = dec!(0.000000000000000001);

        let output = calculate_swap(
            input_reserve,
            output_reserve,
            input_weight,
            output_weight,
            input_amount
        );

        println!("output: {}", output);
    }

    #[test]
    fn test_calculate_swap_equal_weights() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100); 
        let input_weight = dec!(0.5);
        let output_weight = dec!(0.5);
        let input_amount = dec!(10);

        let output = calculate_swap(
            input_reserve,
            output_reserve,
            input_weight,
            output_weight,
            input_amount
        );
        println!("output: {}", output);

        assert!((output - dec!(9.090909090909090909)).checked_abs().unwrap() < dec!(0.0000000001));
    }

    #[test]
    fn test_calculate_swap_unequal_weights_01() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100);
        let input_weight = dec!(0.9);
        let output_weight = dec!(0.1);
        let input_amount = dec!(60);

        let output = calculate_swap(
            input_reserve,
            output_reserve,
            input_weight, 
            output_weight,
            input_amount
        );

        println!("output: {}", output);

        assert!((output - dec!(98.544808477163314819)).checked_abs().unwrap() < dec!(0.0000000001));
    }

    #[test]
    fn test_calculate_swap_unequal_weights_02() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100);
        let input_weight = dec!(0.1);
        let output_weight = dec!(0.9);
        let input_amount = dec!(60);

        let output = calculate_swap(
            input_reserve,
            output_reserve,
            input_weight, 
            output_weight,
            input_amount
        );

        assert!((output - dec!(5.088245442031493537)).checked_abs().unwrap() < dec!(0.0000000001));
    }

    #[test]
    fn test_calculate_swap_unequal_reserves() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100000000);
        let input_weight = dec!(0.5);
        let output_weight = dec!(0.5);
        let input_amount = dec!(100);

        let output = calculate_swap(
            input_reserve,
            output_reserve,
            input_weight,
            output_weight,
            input_amount
        );

        assert!((output - dec!(50000000)).checked_abs().unwrap() < dec!(0.000001));
    }

    #[test]
    #[should_panic]
    fn test_calculate_swap_input_amount_greater_than_60_percent() {
        let input_reserve = dec!(100);
        let output_reserve = dec!(100);
        let input_weight = dec!(0.6);
        let output_weight = dec!(0.4);
        let input_amount = dec!(61);

        calculate_swap(
            input_reserve,
            output_reserve,
            input_weight,
            output_weight,
            input_amount
        );
    }

    #[test]
    #[should_panic]
    fn test_calculate_swap_zero_reserve() {
        calculate_swap(
            dec!(0),
            dec!(100),
            dec!(0.5),
            dec!(0.5),
            dec!(10)
        );
    }
}
