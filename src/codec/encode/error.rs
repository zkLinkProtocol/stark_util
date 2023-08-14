use std::fmt::Display;

use starknet::core::types::{FromByteArrayError, FromByteSliceError};

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum EncodeError {
    #[error("squence mast have length")]
    SequenceMustHaveLength,
    #[error("can't collect str")]
    CannotCollectStr,
    #[error("not support")]
    UnSupport,
    #[error("{0}")]
    OtherString(String),
    #[error("only support Hex string")]
    InvalidString,
    #[error("encode U256 error: {0:?}")]
    FromByteArrayError(FromByteArrayError),
    #[error("encode H256 or ZkLinkAddress error: {0:?}")]
    FromByteSliceError(FromByteSliceError),
}

impl serde::ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
        where T: Display
    {
        Self::OtherString(msg.to_string())
    }
}

impl From<FromByteArrayError> for EncodeError {
    fn from(value: FromByteArrayError) -> Self {
        EncodeError::FromByteArrayError(value)
    }
}

impl From<FromByteSliceError> for EncodeError {
    fn from(value: FromByteSliceError) -> Self {
        EncodeError::FromByteSliceError(value)
    }
}
