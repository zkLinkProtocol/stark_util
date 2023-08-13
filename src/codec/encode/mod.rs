mod encoder;
mod error;
mod impl_tuples;

pub use encoder::{Encode, Encoder, EncoderImpl};
pub use error::EncodeError;
use serde::{
    ser,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize,
};

pub struct SerdeEncoder<'a, ENC: Encoder> {
    pub enc: &'a mut ENC,
}

impl<'a, ENC> ser::Serializer for SerdeEncoder<'a, ENC> where ENC: Encoder
{
    type Ok = ();
    // The error type when some error occurs during serialization.
    type Error = EncodeError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    serde::serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            v.encode(self.enc)
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        0u8.encode(self.enc)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        1u8.encode(self.enc)?;
        value.serialize(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(self,
                              _name: &'static str,
                              variant_index: u32,
                              _variant: &'static str)
                              -> Result<Self::Ok, Self::Error> {
        variant_index.encode(self.enc)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self,
                                            _name: &'static str,
                                            variant_index: u32,
                                            _variant: &'static str,
                                            value: &T)
                                            -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        variant_index.encode(self.enc)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or(EncodeError::SequenceMustHaveLength)?;
        len.encode(self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound { enc: self.enc })
    }

    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<Self::SerializeTupleVariant, Self::Error> {
        variant_index.encode(self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or(EncodeError::SequenceMustHaveLength)?;
        len.encode(self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { enc: self.enc })
    }

    fn serialize_struct_variant(self,
                                _name: &'static str,
                                variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant, Self::Error> {
        variant_index.encode(self.enc)?;
        Ok(Compound { enc: self.enc })
    }

    fn collect_str<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
        where T: core::fmt::Display
    {
        let s = v.to_string();
        s.encode(self.enc)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        v.encode(self.enc)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

type Compound<'a, ENC> = SerdeEncoder<'a, ENC>;

impl<'a, ENC: Encoder> SerializeSeq for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTuple for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTupleStruct for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeTupleVariant for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeMap for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        key.serialize(SerdeEncoder { enc: self.enc })
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeStruct for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, ENC: Encoder> SerializeStructVariant for Compound<'a, ENC> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: Serialize
    {
        value.serialize(SerdeEncoder { enc: self.enc })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
