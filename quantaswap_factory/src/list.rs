use scrypto::prelude::*;

use crate::quantaswap_factory::quantaswap_factory::*;

#[derive(ScryptoSbor)]
pub struct List<T: ScryptoSbor + Clone + QuantaSwapFactoryRegisteredType> {
    pointer: u64,
    kvs: KeyValueStore<u64, T>,
}

impl<T: ScryptoSbor + Clone + QuantaSwapFactoryRegisteredType> List<T> {
    pub fn new() -> Self {
        Self { 
            pointer: 0,
            kvs: KeyValueStore::new_with_registered_type(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.kvs.insert(self.pointer, item);
        self.pointer += 1;
    }

    pub fn get(&self, index: u64) -> Option<T> where T: Clone {
        self.kvs.get(&index).map(|item| item.clone())
    }

    pub fn range(&self, start: u64, end: u64) -> Vec<T> {
        let mut result = Vec::new();
        for i in start..end {
            if let Some(item) = self.get(i) {
                result.push(item);
            } else {
                break;
            }
        }
        result
    }

    pub fn len(&self) -> u64 {
        self.pointer
    }
}