mod de_owned;
mod decoder;
mod error;
mod impl_tuples;
mod reader;

pub use de_owned::SerdeDecoder;
pub use decoder::{Decode, Decoder, DecoderImpl};
pub use error::DecodeError;
pub use reader::SliceReader;
