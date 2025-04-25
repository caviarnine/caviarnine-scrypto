use scrypto::prelude::*;

#[blueprint]
mod mock_fee_vaults {
    struct FeeVaults {
        vaults: KeyValueStore<ResourceAddress, Vault>,
    }

    impl FeeVaults {
        pub fn new() -> Global<FeeVaults> {
            Self {
                vaults: KeyValueStore::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn deposit(&mut self, token: Bucket) {
            let resource_address = token.resource_address();
            if self.vaults.get(&resource_address).is_none() {
                self.vaults.insert(resource_address, Vault::with_bucket(token));
            } else {
                let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                vault.put(token);
            };
        }

        pub fn treasury_deposit(&mut self, token: Bucket) {
            let resource_address = token.resource_address();
            if self.vaults.get(&resource_address).is_none() {
                self.vaults.insert(resource_address, Vault::with_bucket(token));
            } else {
                let mut vault = self.vaults.get_mut(&resource_address).unwrap();
                vault.put(token);
            };
        }
    }
}
