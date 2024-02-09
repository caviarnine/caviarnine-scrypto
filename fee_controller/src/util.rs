use scrypto::prelude::*;

/// A key for the liquidity fee map
/// The key is the XOR of all unique resource addresses
#[derive(ScryptoSbor)]
pub struct ResourcesKey {
    pub bytes: Vec<u8>,
}

/// Implement from for ResourcesKey
impl From<Vec<ResourceAddress>> for ResourcesKey {
    fn from(resources: Vec<ResourceAddress>) -> Self {
        if resources.is_empty() {
            return Self { bytes: vec![] };
        }

        let mut bytes = resources[0].to_vec();
        let mut used_resources = vec![resources[0]];
        for &resource in resources[1..].iter() {
            if used_resources.contains(&resource) {
                continue;
            }

            bytes = bytes
                .iter()
                .zip(resource.to_vec())
                .map(|(a, b)| a ^ b)
                .collect();
            used_resources.push(resource);
        }

        Self { bytes }
    }
}

/// Basis points math
pub trait BasisPoints {
    fn from_basis_point_hundredths(value: u16) -> Self;
}

/// Implement BasisPoints for Decimal
impl BasisPoints for Decimal {
    fn from_basis_point_hundredths(value: u16) -> Self {
        let multiplier: I192 = I192::from(10u8).pow(12);
        Decimal(I192::from(value) * multiplier)
    }
}