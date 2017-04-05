use serde::de::{Deserializer, Visitor, Error};
use serde::de::value::ValueDeserializer;

#[cfg(feature = "std")]
use std::marker::PhantomData;

#[cfg(not(feature = "std"))]
use core::marker::PhantomData;

#[cfg(feature = "collections")]
use collections::Vec;

//////////////////////////////////////////////////////////////////////////////

impl<'a, E> ValueDeserializer<E> for super::Bytes<'a>
    where E: Error
{
    type Deserializer = BytesDeserializer<'a, E>;

    fn into_deserializer(self) -> Self::Deserializer {
        BytesDeserializer {
            value: self.into(),
            error: PhantomData,
        }
    }
}

/// A deserializer that deserializes a `&[u8]`.
pub struct BytesDeserializer<'a, E> {
    value: &'a [u8],
    error: PhantomData<E>,
}

impl<'a, E> Deserializer for BytesDeserializer<'a, E>
    where E: Error
{
    type Error = E;

    fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor
    {
        visitor.visit_bytes(self.value)
    }

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq seq_fixed_size bytes map unit_struct newtype_struct tuple_struct
        struct struct_field tuple enum ignored_any byte_buf
    }
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(any(feature = "std", feature = "collections"))]
impl<E> ValueDeserializer<E> for super::ByteBuf
    where E: Error
{
    type Deserializer = ByteBufDeserializer<E>;

    fn into_deserializer(self) -> Self::Deserializer {
        ByteBufDeserializer {
            value: self.into(),
            error: PhantomData,
        }
    }
}

/// A deserializer that deserializes a `Vec<u8>`.
#[cfg(any(feature = "std", feature = "collections"))]
pub struct ByteBufDeserializer<E> {
    value: Vec<u8>,
    error: PhantomData<E>,
}

#[cfg(any(feature = "std", feature = "collections"))]
impl<E> Deserializer for ByteBufDeserializer<E>
    where E: Error
{
    type Error = E;

    fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor
    {
        visitor.visit_byte_buf(self.value)
    }

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq seq_fixed_size bytes map unit_struct newtype_struct tuple_struct
        struct struct_field tuple enum ignored_any byte_buf
    }
}