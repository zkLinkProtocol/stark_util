use starknet::core::types::FieldElement;

use super::{reader::Reader, DecodeError};

pub fn u128_from_field_element(field_element: FieldElement) -> u128 {
    let data = field_element.to_bytes_be();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&data[16..]);
    u128::from_be_bytes(bytes)
}

pub trait Decode: Sized {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        Self::decode_one(decoder)
    }

    fn decode_one<D>(decoder: &mut D) -> Result<Self, DecodeError>
        where D: Decoder
    {
        decoder.claim_field_elements_read(1)?;
        if let Some(buf) = decoder.reader().peek_read(1) {
            let element = buf[0];
            decoder.reader().consume(1);
            Self::decode_element(element)
        } else {
            Err(DecodeError::UnexpectedEnd { additional: 1 })
        }
    }

    fn decode_element(_element: FieldElement) -> Result<Self, DecodeError> {
        unimplemented!("Type unimplemented")
    }
}

pub trait Decoder {
    /// The concrete [Reader] type
    type R: Reader;

    fn reader(&mut self) -> &mut Self::R;
    // /// Claim that `n` field elements are going to be read from the decoder.
    // /// This can be used to validate `Configuration::Limit<N>()`.
    fn claim_field_elements_read(&mut self, n: usize) -> Result<(), DecodeError>;
}

impl<'a, T> Decoder for &'a mut T where T: Decoder
{
    type R = T::R;

    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }

    #[inline]
    fn claim_field_elements_read(&mut self, n: usize) -> Result<(), DecodeError> {
        T::claim_field_elements_read(self, n)
    }
}

#[derive(Debug)]
pub struct DecoderImpl<R> {
    reader: R,
    field_elements_read: usize,
}

impl<R: Reader> DecoderImpl<R> {
    /// Construct a new Decoder
    pub fn new(reader: R) -> DecoderImpl<R> {
        DecoderImpl { reader, field_elements_read: 0 }
    }
}

impl<R> Decoder for DecoderImpl<R> where R: Reader
{
    type R = R;

    fn reader(&mut self) -> &mut Self::R {
        &mut self.reader
    }

    #[inline]
    fn claim_field_elements_read(&mut self, n: usize) -> Result<(), DecodeError> {
        self.field_elements_read += n;
        Ok(())
    }
}

impl Decode for FieldElement {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        Ok(element)
    }
}

impl Decode for bool {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        match u8::decode_element(element)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::OutOfRange),
        }
    }
}

macro_rules! impl_decode_for_unsigned_num {
    ($ty:ty) => {
        impl Decode for $ty {
            #[inline]
            fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
                let num: $ty = element.try_into().map_err(|_e| DecodeError::OutOfRange)?;
                Ok(num)
            }
        }
    };
}

macro_rules! impl_decode_with_error {
    ($ty:ty, $name:expr) => {
        impl Decode for $ty {
            #[inline]
            fn decode<D: Decoder>(_decoder: &mut D) -> Result<Self, DecodeError> {
                Err(DecodeError::NotSupport($name.to_string()))
            }
        }
    };
}

impl_decode_for_unsigned_num!(u8);
impl_decode_for_unsigned_num!(u16);
impl_decode_for_unsigned_num!(u32);
impl_decode_for_unsigned_num!(u64);

impl Decode for u128 {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        let num = u128_from_field_element(element);
        Ok(num)
    }
}

impl Decode for usize {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        let num = u64::try_from(element).unwrap();
        Ok(num as usize)
    }
}

impl_decode_with_error!(isize, "isize");
impl_decode_with_error!(i8, "i8");
impl_decode_with_error!(i16, "i16");
impl_decode_with_error!(i32, "i32");
impl_decode_with_error!(i64, "i64");
impl_decode_with_error!(i128, "i128");
impl_decode_with_error!(f32, "f32");
impl_decode_with_error!(f64, "f64");
impl_decode_with_error!(char, "char");

impl Decode for String {
    fn decode_element(element: FieldElement) -> Result<Self, DecodeError> {
        let s = hex::encode(element.to_bytes_be());
        let s = s.trim_start_matches('0');
        if !s.starts_with("0x") { Ok(String::from("0x") + s) } else { Ok(s.into()) }
    }
}

impl<T, const N: usize> Decode for [T; N] where T: Decode + Sized + 'static
{
    fn decode<D: Decoder>(_decoder: &mut D) -> Result<Self, DecodeError> {
        Err(DecodeError::NotSupport("Slice".into()))
    }
}

impl Decode for () {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(())
    }
}

impl<T> Decode for core::marker::PhantomData<T> {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(core::marker::PhantomData)
    }
}

/// Decodes only the option variant from the decoder. Will not read any more
/// data than that.
#[inline]
pub fn decode_option_variant<D: Decoder>(decoder: &mut D, _type_name: &'static str) -> Result<Option<()>, DecodeError> {
    let is_some = u8::decode(decoder)?;
    match is_some {
        0 => Ok(None),
        1 => Ok(Some(())),
        _x => Err(DecodeError::OutOfRange),
    }
}

impl<T> Decode for Option<T> where T: Decode
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decode_option_variant(decoder, core::any::type_name::<Option<T>>())? {
            Some(_) => {
                let val = T::decode(decoder)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}

impl<T, U> Decode for Result<T, U>
    where T: Decode,
          U: Decode
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let is_ok = u32::decode(decoder)?;
        match is_ok {
            0 => {
                let t = T::decode(decoder)?;
                Ok(Ok(t))
            }
            1 => {
                let u = U::decode(decoder)?;
                Ok(Err(u))
            }
            _ => Err(DecodeError::OutOfRange),
        }
    }
}

impl<T> Decode for Vec<T> where T: Decode
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = usize::decode(decoder)?;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            let e = T::decode(decoder)?;
            result.push(e);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use starknet::core::types::FieldElement;

    use super::*;

    #[test]
    fn test_num_to_field_element() {
        let is_u8 = 100u8;
        let f = FieldElement::from(is_u8);
        let num: u8 = f.try_into().unwrap();
        assert_eq!(is_u8, num);

        let is_u16 = 100u16;
        let f = FieldElement::from(is_u16);
        let num: u16 = f.try_into().unwrap();
        assert_eq!(is_u16, num);

        let is_u32 = 100u32;
        let f = FieldElement::from(is_u32);
        let num: u32 = f.try_into().unwrap();
        assert_eq!(is_u32, num);

        let is_u64 = 100u64;
        let f = FieldElement::from(is_u64);
        let num: u64 = f.try_into().unwrap();
        assert_eq!(is_u64, num);

        let is_u128 = 100u128;
        let data = is_u128.to_be_bytes();
        let f = FieldElement::from_byte_slice_be(&data.to_vec());
        assert!(f.is_ok());
        let num = u128_from_field_element(f.unwrap());
        assert_eq!(is_u128, num);
    }
}
