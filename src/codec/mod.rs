pub mod decode;
pub mod encode;

use decode::{DecodeError, DecoderImpl, SerdeDecoder, SliceReader};
use encode::{EncodeError, EncoderImpl, SerdeEncoder};
use serde::Serialize;

use crate::primitive::FieldElement;

pub fn to_field_elements<T>(t: T) -> Result<Vec<FieldElement>, EncodeError>
    where T: Serialize
{
    let mut encoder = EncoderImpl { field_elements: vec![] };
    let serializer = SerdeEncoder { enc: &mut encoder };
    t.serialize(serializer)?;
    Ok(encoder.field_elements)
}

/// Attempt to decode a given type `D` from the given slice. Returns the decoded
/// output.
pub fn from_slice<T>(slice: &[FieldElement]) -> Result<T, DecodeError>
    where T: serde::de::DeserializeOwned
{
    let reader = SliceReader::new(slice);
    let mut decoder = DecoderImpl::new(reader);
    let serde_decoder = SerdeDecoder { de: &mut decoder };
    let result = T::deserialize(serde_decoder)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use primitive_types::U256 as PrimitiveU256;
    use serde::{Deserialize, Serialize};

    use crate::{
        from_slice,
        primitive::{FieldElement, U256},
        to_field_elements,
    };

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    struct TestStruct {
        is_u8: u8,
        is_u16: u16,
        is_u32: u32,
        is_u64: u64,
        is_usize: usize,
        is_u128: u128,
        is_u256: PrimitiveU256,
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
        println!("is_u256: {is_u256:?}");
        let v = to_field_elements(is_u256.clone()).unwrap();
        assert_eq!(v.len(), 5);
        println!("encode v: {v:?}");
        let v2: Vec<U256> = from_slice(&v).unwrap();
        assert_eq!(is_u256, v2);

        // test struct
        let s = TestStruct { is_u8: 8,
                             is_u16: 16,
                             is_u32: 32,
                             is_u64: 64,
                             is_usize: 1,
                             is_u128: u128::from(128u32),
                             is_u256: PrimitiveU256::one().into(),
                             element: FieldElement::from(10u8),
                             addr: "0x5686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad".to_string(),
                             is_raw_data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                             is_vec_u256: vec![PrimitiveU256::from(1u8).into(), PrimitiveU256::from(2).into()] };
        println!("encode struct s: {s:?}");

        let v = to_field_elements(s.clone()).unwrap();
        println!("encode struct v: {v:?}");
        let s2: TestStruct = from_slice(&v).unwrap();
        println!("{s2:?}");
        assert_eq!(s, s2);
    }

    #[test]
    fn test_from_field_elements() {
        let is_u8_vec = vec![1u8; 10];
        let v = to_field_elements(is_u8_vec.clone()).unwrap();
        let is_u8_vec_expect: Vec<u8> = from_slice(v.as_slice()).unwrap();
        println!("{:?}", is_u8_vec_expect);
        assert_eq!(is_u8_vec, is_u8_vec_expect);
    }

    #[test]
    fn test_field_element() {
        let address = "0x05686c52b6f38639eb9cfb3dfff1b3260315099aa045fcc0b4a865068ba36aad";
        let address = FieldElement::from_str(address).unwrap();
        let v = to_field_elements(address).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], address);
    }
}
