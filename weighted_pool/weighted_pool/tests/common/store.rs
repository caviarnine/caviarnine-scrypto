pub use scrypto_test::prelude::*;
use std::collections::HashMap;
use std::any::Any;

pub struct Store {
    pub vars: HashMap<String, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn store<T: 'static>(&mut self, key: &str, value: T) {
        self.vars.insert(key.to_string(), Box::new(value));
    }

    pub fn fetch<T: 'static>(&mut self, key: &str) -> T {
        *self.vars.remove(key)
            .and_then(|any| any.downcast::<T>().ok())
            .expect(&format!("Key not found: {}", key))
    }
}