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

impl serde::ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
        where T: Display {
        Self::OtherString(msg.to_string())
    }
}
