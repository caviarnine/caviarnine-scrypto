use scrypto::prelude::*;
mod common;

pub use crate::common::fee_vaults;
pub use crate::common::setup::*;
pub use crate::common::vars::Vars;

#[test]
fn test_fees_01() {
    // ARRANGE
    let mut vars = setup();
    let new_fee_vaults_component = fee_vaults::new_fee_vaults_manifest(&mut vars, dec!("0.1"));

    // set treasury percentage = 0
    fee_vaults::set_treasury_percentage(&mut vars, new_fee_vaults_component, dec!("0.5"));

    // get treasury percentage
    let treasury_percentage =
        fee_vaults::get_treasury_percentage(&mut vars, new_fee_vaults_component);

    // create 10 resources
    let n = 10u64;

    // loop through and create resources
    let resources: Vec<ResourceAddress> = (0..n)
        .map(|_| {
            vars.test_runner.create_fungible_resource(
                dec!(1000),
                DIVISIBILITY_MAXIMUM,
                vars.account_component_address,
            )
        })
        .collect();

    // ACT 1 - Setup all the tokens
    for resource_address in resources.iter() {
        fee_vaults::deposit(
            &mut vars,
            new_fee_vaults_component,
            *resource_address,
            dec!(10),
        );
    }

    println!("treasury_percentage: {}", treasury_percentage);
    // ACT 2 - Deposit and print out fees
    for m in 0..n {
        let mut my_buckets: Vec<(ResourceAddress, Decimal)> = vec![];

        for i in 0..m {
            my_buckets.push((resources[i as usize], dec!(1)));
        }

        let receipt =
            fee_vaults::deposit_batch_reciept(&mut vars, new_fee_vaults_component, my_buckets);
        // ASSERT
        let fee_summary = receipt.fee_summary;
        println!(
            "fee_summary depositing: {} Tokens, costs: {} XRD",
            m, fee_summary.total_cost()
        );
    }
}
