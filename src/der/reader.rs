use crate::error::DecodeError;
use starknet::core::types::FieldElement;

/// A reader for owned data. See the module documentation for more information.
pub trait Reader {
    /// Fill the given `bytes` argument with values. Exactly the length of the given slice must be filled, or else an error must be returned.
    fn read(&mut self, bytes: &mut [FieldElement]) -> Result<(), DecodeError>;

    /// If this reader wraps a buffer of any kind, this function lets callers access contents of
    /// the buffer without passing data through a buffer first.
    fn peek_read(&mut self, _: usize) -> Option<&[FieldElement]>;

    /// If an implementation of `peek_read` is provided, an implementation of this function
    /// must be provided so that subsequent reads or peek-reads do not return the same bytes
    fn consume(&mut self, _: usize);
}

impl<T> Reader for &mut T
where
    T: Reader,
{
    #[inline]
    fn read(&mut self, bytes: &mut [FieldElement]) -> Result<(), DecodeError> {
        (**self).read(bytes)
    }

    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&[FieldElement]> {
        (**self).peek_read(n)
    }

    #[inline]
    fn consume(&mut self, n: usize) {
        (*self).consume(n)
    }
}

/// A reader type for `&[FieldElement]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data.
pub struct SliceReader<'storage> {
    pub(crate) slice: &'storage [FieldElement],
}

impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub fn new(field_elements: &'storage [FieldElement]) -> SliceReader<'storage> {
        SliceReader {
            slice: field_elements,
        }
    }
}

impl<'storage> Reader for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, field_elements: &mut [FieldElement]) -> Result<(), DecodeError> {
        if field_elements.len() > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd {
                additional: field_elements.len() - self.slice.len(),
            });
        }
        let (read_slice, remaining) = self.slice.split_at(field_elements.len());
        field_elements.copy_from_slice(read_slice);
        self.slice = remaining;

        Ok(())
    }

    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&'storage [FieldElement]> {
        self.slice.get(..n)
    }

    #[inline]
    fn consume(&mut self, n: usize) {
        self.slice = self.slice.get(n..).unwrap_or_default();
    }
}
