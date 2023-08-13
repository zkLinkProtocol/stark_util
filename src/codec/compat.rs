/// Wrapper struct that implements [Decode] and [Encode] on any type that
/// implements serde's [DeserializeOwned] and [Serialize] respectively.
///
/// This works for most types, but if you're dealing with borrowed data consider
/// using [BorrowCompat] instead.
pub struct Compat<T>(pub T);
