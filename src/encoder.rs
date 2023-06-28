use crate::error::EncodeError;
use starknet::core::types::FieldElement;

pub struct EncoderImpl {
    pub filed_elements: Vec<FieldElement>,
}

pub trait Encode {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), EncodeError> {
        Err(EncodeError::UnSupport)
    }
}

pub trait Encoder {
    fn push_field_element(&mut self, element: FieldElement);
}

impl Encoder for EncoderImpl {
    #[inline]
    fn push_field_element(&mut self, element: FieldElement) {
        self.filed_elements.push(element);
    }
}

#[inline]
pub(crate) fn encode_option_variant<E: Encoder, T>(
    encoder: &mut E,
    value: &Option<T>,
) -> Result<(), EncodeError> {
    match value {
        None => 0u8.encode(encoder),
        Some(_) => 1u8.encode(encoder),
    }
}

#[inline]
pub(crate) fn encode_slice_len<E: Encoder>(encoder: &mut E, len: usize) -> Result<(), EncodeError> {
    len.encode(encoder)
}

impl Encode for () {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u8::from(*self).encode(encoder)
    }
}

macro_rules! impl_encode_for_num {
    ($ty: ty) => {
        impl Encode for $ty {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
                let f = FieldElement::from(*self);
                encoder.push_field_element(f);
                Ok(())
            }
        }
    };
}

impl_encode_for_num!(u8);
impl_encode_for_num!(u16);
impl_encode_for_num!(u32);
impl_encode_for_num!(u64);
impl_encode_for_num!(usize);

impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let data = self.to_be_bytes();
        let element = FieldElement::from_byte_slice_be(data.as_ref()).unwrap();
        encoder.push_field_element(element);
        Ok(())
    }
}

impl Encode for i8 {}
impl Encode for i16 {}
impl Encode for i32 {}
impl Encode for i64 {}
impl Encode for i128 {}
impl Encode for f32 {}
impl Encode for f64 {}
impl Encode for char {}

impl<T> Encode for [T]
where
    T: Encode + 'static,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_slice_len(encoder, self.len())?;
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

/// only encode hash, which starts with "0x" or not
impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let s = self.strip_prefix("0x").unwrap_or(self);
        let field = FieldElement::from_hex_be(s).map_err(|_| EncodeError::InvalidString)?;
        field.encode(encoder)
    }
}

impl Encode for FieldElement {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.push_field_element(*self);
        Ok(())
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_option_variant(encoder, self)?;
        if let Some(val) = self {
            val.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T, U> Encode for Result<T, U>
where
    T: Encode,
    U: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Ok(val) => {
                0u32.encode(encoder)?;
                val.encode(encoder)
            }
            Err(err) => {
                1u32.encode(encoder)?;
                err.encode(encoder)
            }
        }
    }
}

impl<'a, T> Encode for &'a T
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
