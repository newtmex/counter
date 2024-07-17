use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[cfg_attr(all(test, feature = "std"), derive(motsu::DefaultStorageLayout))]
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

#[external]
impl Counter {
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    pub fn get_number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use stylus_sdk::alloy_primitives::U256;

    use super::Counter;

    #[motsu::test]
    fn test_set_number(contract: Counter) {
        let expected = U256::from(4);
        let _ = contract.set_number(expected);
        let new_value = contract.get_number().unwrap();

        assert_eq!(expected, new_value);
    }
}
