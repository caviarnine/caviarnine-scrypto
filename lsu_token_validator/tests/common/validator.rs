use scrypto::prelude::*;

use super::vars::*;

pub fn create_validator(vars: &mut Vars) -> (ComponentAddress, ResourceAddress) {
    let validator_component_address = vars.test_runner.new_validator_with_pub_key(vars.admin_public_key, vars.admin_account_component);
    let pool_unit_metadata: MetadataValue = vars.test_runner.get_metadata(validator_component_address.into(), "pool_unit").unwrap();
    let lsu: ResourceAddress = match pool_unit_metadata {
        MetadataValue::GlobalAddress(address) => address.try_into().unwrap(),
        _ => panic!(),
    };

    (validator_component_address, lsu)
}