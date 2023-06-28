mod der;
mod num;
mod ser;
mod serde;

pub mod client;
pub(crate) mod decoder;
pub(crate) mod encoder;
pub mod error;
pub mod proto;

use crate::decoder::DecoderImpl;
use crate::der::de_owned::SerdeDecoder;
use crate::der::reader::SliceReader;
use crate::encoder::EncoderImpl;
use crate::error::{DecodeError, EncodeError};
use crate::ser::SerdeEncoder;
use ::serde::de::DeserializeOwned;
use ::serde::Serialize;
pub use num::U256;
use starknet::core::types::FieldElement;

pub fn to_field_elements<T>(t: T) -> Result<Vec<FieldElement>, EncodeError>
where
    T: Serialize,
{
    let mut encoder = EncoderImpl {
        filed_elements: vec![],
    };
    let serializer = SerdeEncoder { enc: &mut encoder };
    t.serialize(serializer)?;
    Ok(encoder.filed_elements)
}

/// Attempt to decode a given type `D` from the given slice. Returns the decoded output.
pub fn from_slice<T>(slice: &[FieldElement]) -> Result<T, DecodeError>
where
    T: DeserializeOwned,
{
    let reader = SliceReader::new(slice);
    let mut decoder = DecoderImpl::new(reader);
    let serde_decoder = SerdeDecoder { de: &mut decoder };
    let result = T::deserialize(serde_decoder)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::num::U256;
    use crate::{from_slice, to_field_elements};
    use primitive_types::U256 as PrimitiveU256;
    use serde::{Deserialize, Serialize};
    use starknet::core::types::FieldElement;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    struct TestStruct {
        is_u8: u8,
        is_u16: u16,
        is_u32: u32,
        is_u64: u64,
        is_u128: u128,
        is_u256: PrimitiveU256,
        is_usize: usize,
        element: FieldElement,
        addr: String,
        is_raw_data: Vec<u8>,
        is_vec_u256: Vec<U256>,
    }

    #[test]
    fn test_to_filed_elements() {
        // test u8
        let is_u8 = vec![18, 2];
        let v = to_field_elements(is_u8.clone()).unwrap();
        let v2: Vec<u8> = from_slice(&v).unwrap();
        assert_eq!(is_u8, v2);

        // test [u8; 32]
        let u8_slice = [0; 32];
        let v = to_field_elements(u8_slice).unwrap();
        assert_eq!(v.len(), 32);
        let v2: [u8; 32] = from_slice(&v).unwrap();
        assert_eq!(u8_slice, v2);

        // test U256
        let is_u256: Vec<U256> = vec![PrimitiveU256::from(1).into(), PrimitiveU256::from(2).into()];
        let v = to_field_elements(is_u256.clone()).unwrap();
        assert_eq!(v.len(), 5);
        println!("{v:?}");
        let v2: Vec<U256> = from_slice(&v).unwrap();
        assert_eq!(is_u256, v2);

        // test struct
        let s = TestStruct {
            is_u8: 8,
            is_u16: 16,
            is_u32: 32,
            is_u64: 64,
            is_usize: 1,
            is_u128: u128::from(128u32),
            is_u256: PrimitiveU256::one().into(),
            element: FieldElement::from(10u8),
            addr: "9e290521bb937cebdbd1b5636037f089f7bf34de51f9fc019b07cdb8ed98a1".to_string(),
            is_raw_data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            is_vec_u256: vec![
                PrimitiveU256::from(1u8).into(),
                PrimitiveU256::from(2).into(),
            ],
        };
        let v = to_field_elements(s.clone()).unwrap();
        println!("encode struct: {v:?}");
        let s2: TestStruct = from_slice(&v).unwrap();
        println!("{s2:?}");
        assert_eq!(s, s2);
    }

    #[test]
    fn test_from_field_elements() {
        let is_u8_vec = vec![1u8; 10];
        let v = to_field_elements(is_u8_vec.clone()).unwrap();
        let is_u8_vec_expect: Result<Vec<u8>, _> = from_slice(&v);
        println!("{is_u8_vec_expect:?}");
        assert_eq!(is_u8_vec, is_u8_vec_expect.unwrap());
    }
}
