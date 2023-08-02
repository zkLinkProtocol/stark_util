use crate::decoder::{Decode, Decoder};
use crate::error::DecodeError;

impl<A> Decode for (A,)
where
    A: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((A::decode(decoder)?,))
    }
}

impl<A, B> Decode for (A, B)
where
    A: Decode,
    B: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((A::decode(decoder)?, B::decode(decoder)?))
    }
}

impl<A, B, C> Decode for (A, B, C)
where
    A: Decode,
    B: Decode,
    C: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
        ))
    }
}

impl<A, B, C, D> Decode for (A, B, C, D)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
        ))
    }
}

impl<A, B, C, D, E> Decode for (A, B, C, D, E)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
        ))
    }
}

impl<A, B, C, D, E, F> Decode for (A, B, C, D, E, F)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
        ))
    }
}
