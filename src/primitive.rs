#[rustfmt::skip]
/// A list of primitive types in Cairo (not all), along with a brief description of each type’s purpose and functionality:
/// Primitive Type	         Description
/// u8                       Represents an unsigned 8-bit integer.
/// usize                    Unsigned size integer (typically used for representing indices and lengths)
/// u16                      Represents an unsigned 16-bit integer.
/// u32                      Represents an unsigned 32-bit integer.
/// u64                      Represents an unsigned 64-bit integer.
/// u128                     Represents an unsigned 128-bit integer.
/// u256                     Represents an unsigned 256-bit integer.
/// bool                     Represents a boolean value, which can be either true or false.
/// Felt252                  Represents a field element.
/// ContractAddress          A type representing a Starknet contract address, used for identifying and interacting with smart contracts.
/// T                        Represents a generic type placeholder, which can be replaced with any specific type during compilation.
/// Option<T>                Represents a value that may or may not be present, used for optional values and error handling.
/// Result<T, E>             Represents the outcome of a computation that may result in an error, used for error handling and control flow.
/// Array<T>                 A dynamic array data structure for elements of type T, used for creating and manipulating arrays.

pub use starknet::core::types::FieldElement;
pub use starknet_api::core::ContractAddress;
pub type Felt252 = FieldElement;
