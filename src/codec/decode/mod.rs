pub mod de_owned;
pub mod decoder;
pub mod error;
mod impl_tuples;
pub mod reader;

pub use de_owned::SerdeDecoder;
pub use decoder::{Decode, Decoder, DecoderImpl};
pub use error::DecodeError;
pub use reader::SliceReader;
