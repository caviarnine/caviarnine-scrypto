mod common;
pub use crate::common::lsu_pool;
pub use crate::common::misc::*;
pub use crate::common::setup::*;
pub use crate::common::validator;
pub use crate::common::vars::Vars;
// use scrypto::prelude::*;

#[test]
fn test_setup() {
    // ARRANGE
    let _vars = setup();
}
