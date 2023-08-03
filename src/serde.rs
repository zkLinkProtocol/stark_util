use crate::{
    decoder::{Decode, Decoder},
    der::de_owned::SerdeDecoder,
    encoder::{Encode, Encoder},
    error::DecodeError,
    ser::SerdeEncoder,
};

/// Wrapper struct that implements [Decode] and [Encode] on any type that implements serde's [DeserializeOwned] and [Serialize] respectively.
///
/// This works for most types, but if you're dealing with borrowed data consider using [BorrowCompat] instead.
///
pub struct Compat<T>(pub T);

impl<T> Decode for Compat<T> where T: serde::de::DeserializeOwned
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let serde_decoder = SerdeDecoder { de: decoder };
        T::deserialize(serde_decoder).map(Compat)
    }
}

impl<T> Encode for Compat<T> where T: serde::Serialize
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), crate::error::EncodeError> {
        let serializer = SerdeEncoder { enc: encoder };
        self.0.serialize(serializer)?;
        Ok(())
    }
}
