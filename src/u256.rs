pub use primitive_types::U256 as PrimitiveU256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct U256 {
    low: u128,
    high: u128,
}

impl From<PrimitiveU256> for U256 {
    fn from(value: PrimitiveU256) -> Self {
        let low = value.low_u128();
        let high = (value >> 128).low_u128();
        Self { low, high }
    }
}

impl From<U256> for PrimitiveU256 {
    fn from(value: U256) -> Self {
        let mut num = PrimitiveU256::from(value.high) << 128;
        let low = PrimitiveU256::from(value.low);
        num += low;
        num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u256() {
        let num = PrimitiveU256::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                                       30, 31, 32]);
        let u: U256 = num.into();
        println!("{:?}", u);
        let num2: PrimitiveU256 = u.into();
        assert_eq!(num, num2);
    }
}
