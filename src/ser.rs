use crate::{ByteArray, Bytes};
use serde::Serializer;

use crate::ByteBuf;

use alloc::borrow::Cow;

use alloc::boxed::Box;

use alloc::vec::Vec;

pub(crate) fn serialize_bytes<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if serializer.is_human_readable() {
        serializer.serialize_str(&hex::encode(bytes))
    } else {
        serializer.serialize_bytes(bytes)
    }
}

/// Types that can be serialized via `#[serde(with = "serde_human_bytes")]`.
pub trait Serialize {
    #[allow(missing_docs)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl Serialize for [u8] {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl Serialize for Vec<u8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl<const N: usize> Serialize for [u8; N] {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl<const N: usize> Serialize for ByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(&**self, serializer)
    }
}

impl Serialize for ByteBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl<'a> Serialize for Cow<'a, [u8]> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl<'a> Serialize for Cow<'a, Bytes> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bytes(self, serializer)
    }
}

impl<T> Serialize for &T
where
    T: ?Sized + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<T> Serialize for Box<T>
where
    T: ?Sized + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct AsBytes<T>(T);

        impl<T> serde::Serialize for AsBytes<T>
        where
            T: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        match self {
            Some(b) => serializer.serialize_some(&AsBytes(b)),
            None => serializer.serialize_none(),
        }
    }
}
