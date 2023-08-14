use zklink_basic_types::{ZkLinkAddress, H256, U256};

use crate::{
    codec::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError, Encoder},
    },
    primitive::FieldElement,
};

impl Encode for U256 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let mut inner = [0; 32];
        self.to_big_endian(&mut inner);
        let val = FieldElement::from_bytes_be(&inner)?;
        encoder.push_field_element(val);
        Ok(())
    }
}

impl Encode for H256 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = FieldElement::from_byte_slice_be(self.as_bytes())?;
        encoder.push_field_element(val);
        Ok(())
    }
}

impl Encode for ZkLinkAddress {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let val = FieldElement::from_byte_slice_be(self.as_bytes())?;
        encoder.push_field_element(val);
        Ok(())
    }
}

impl Decode for U256 {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        Ok(U256::from(element.to_bytes_be()))
    }
}

impl Decode for H256 {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        Ok(H256::from(element.to_bytes_be()))
    }
}

impl Decode for ZkLinkAddress {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        Ok(ZkLinkAddress::from_slice(element.to_bytes_be().as_slice()).unwrap())
    }
}
