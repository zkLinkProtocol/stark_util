mod de_owned;
mod reader;
mod impl_tuples;
mod decoder;
mod error;

pub use reader::SliceReader;
pub use de_owned::SerdeDecoder;
pub use error::DecodeError;
pub use decoder::{Decode, Decoder, DecoderImpl};

use super::compat::Compat;

impl<T> Decode for Compat<T> where T: serde::de::DeserializeOwned
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let serde_decoder = SerdeDecoder { de: decoder };
        T::deserialize(serde_decoder).map(Compat)
    }
}
