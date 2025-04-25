use scrypto::prelude::*;

#[blueprint]
mod mock_lsu_pool {
    struct LsuPool {
        dex_valuation_xrd: Decimal,
    }

    impl LsuPool {
        pub fn new(dex_valuation_xrd: Decimal) -> Global<LsuPool> {
            Self {
                dex_valuation_xrd,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn set_dex_valuation_xrd(&mut self, dex_valuation_xrd: Decimal) {
            self.dex_valuation_xrd = dex_valuation_xrd;
        }

        pub fn get_dex_valuation_xrd(&self) -> Decimal {
            self.dex_valuation_xrd
        }
    }
}
