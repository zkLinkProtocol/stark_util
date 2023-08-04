use serde::{Deserialize, Serialize};

use super::FieldElement;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TxHash(FieldElement);

impl From<TxHash> for String {
    fn from(value: TxHash) -> Self {
        let raw = value.0.to_bytes_be();
        let s = hex::encode(raw);
        let s = s.trim_start_matches('0');
        s.into()
    }
}

impl From<FieldElement> for TxHash {
    fn from(value: FieldElement) -> Self {
        Self(value)
    }
}

impl AsRef<FieldElement> for TxHash {
    fn as_ref(&self) -> &FieldElement {
        &self.0
    }
}
