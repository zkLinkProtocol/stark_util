use serde::ser;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
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
}

impl ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::OtherString(msg.to_string())
    }
}

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Any not supported")]
    AnyNotSupported,
    /// The reader reached its end but more elements were expected
    #[error("unexpected end, need more {additional} elements")]
    UnexpectedEnd { additional: usize },
    /// The decoder tried to decode an array of length `required`, but the elements data contained an array of length `found`.
    #[error("array length mismatch, required {required}, but get {found}")]
    ArrayLengthMismatch {
        /// The length of the array required by the rust type.
        required: usize,
        /// The length of the array found in the binary format.
        found: usize,
    },
    /// An uncommon error occurred, see the inner text for more information
    #[error("{0}")]
    Other(String),

    #[error("Does not support serde identifiers")]
    IdentifierNotSupported,
    #[error("Does not support serde's `ignored_any`.")]
    IgnoredAnyNotSupported,
    #[error("Serde tried decoding a borrowed value from an owned reader.")]
    CannotBorrowOwnedData,
    #[error("Out of range")]
    OutOfRange,
    #[error("Not support type {0}")]
    NotSupport(String),
    #[error("Invalid string")]
    InvalidString,
}

impl serde::de::Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Other(msg.to_string())
    }
}
