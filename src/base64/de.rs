use crate::{ByteArray, Bytes};
use core::convert::TryInto;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{Error, Visitor};
use serde::Deserializer;

use crate::ByteBuf;

use core::cmp;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use serde::de::SeqAccess;

pub(crate) fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    ::base64::decode(s).map_err(D::Error::custom)
}

/// Types that can be deserialized via `#[serde(with = "serde_human_bytes")]`.
pub trait Deserialize<'de>: Sized {
    #[allow(missing_docs)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a [u8] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            // Not supported
            Err(D::Error::custom(
                "human readable mode is not supported for &[u8]",
            ))
        } else {
            // serde::Deserialize for &[u8] is already optimized, so simply forward to that.
            serde::Deserialize::deserialize(deserializer)
        }
    }
}

impl<'de> Deserialize<'de> for Vec<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer)
        } else {
            Deserialize::deserialize(deserializer).map(ByteBuf::into_vec)
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Err(D::Error::custom(
                "human readable mode is not supported for &Bytes",
            ))
        } else {
            // serde::Deserialize for &[u8] is already optimized, so simply forward to that.
            serde::Deserialize::deserialize(deserializer).map(Bytes::new)
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer)?
                .try_into()
                .map_err(|_| D::Error::custom("invalid array length"))
        } else {
            let arr: ByteArray<N> = serde::Deserialize::deserialize(deserializer)?;
            Ok(*arr)
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for &'de [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Err(D::Error::custom(
                "human readable mode is not supported for &[u8; N]",
            ))
        } else {
            let arr: &ByteArray<N> = serde::Deserialize::deserialize(deserializer)?;
            Ok(arr)
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer)?
                .try_into()
                .map(ByteArray::new)
                .map_err(|_| D::Error::custom("invalid array length"))
        } else {
            // Via the serde::Deserialize impl for ByteArray.
            serde::Deserialize::deserialize(deserializer)
        }
    }
}

impl<'de: 'a, 'a, const N: usize> Deserialize<'de> for &'a ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Err(D::Error::custom(
                "human readable mode is not supported for &ByteArray<N>",
            ))
        } else {
            // Via the serde::Deserialize impl for &ByteArray.
            serde::Deserialize::deserialize(deserializer)
        }
    }
}

impl<'de> Deserialize<'de> for ByteBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer).map(ByteBuf::from)
        } else {
            // Via the serde::Deserialize impl for ByteBuf.
            serde::Deserialize::deserialize(deserializer)
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, [u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CowVisitor;

        impl<'de> Visitor<'de> for CowVisitor {
            type Value = Cow<'de, [u8]>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array")
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Borrowed(v))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Borrowed(v.as_bytes()))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.to_vec()))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.as_bytes().to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.into_bytes()))
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let len = cmp::min(visitor.size_hint().unwrap_or(0), 4096);
                let mut bytes = Vec::with_capacity(len);

                while let Some(b) = visitor.next_element()? {
                    bytes.push(b);
                }

                Ok(Cow::Owned(bytes))
            }
        }

        if deserializer.is_human_readable() {
            deserialize_base64(deserializer).map(Cow::Owned)
        } else {
            deserializer.deserialize_bytes(CowVisitor)
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer)
                .map(ByteBuf::from)
                .map(Cow::Owned)
        } else {
            let cow: Cow<[u8]> = Deserialize::deserialize(deserializer)?;
            match cow {
                Cow::Borrowed(bytes) => Ok(Cow::Borrowed(Bytes::new(bytes))),
                Cow::Owned(bytes) => Ok(Cow::Owned(ByteBuf::from(bytes))),
            }
        }
    }
}

impl<'de> Deserialize<'de> for Box<[u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer).map(Vec::into_boxed_slice)
        } else {
            Deserialize::deserialize(deserializer).map(Vec::into_boxed_slice)
        }
    }
}

impl<'de> Deserialize<'de> for Box<Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserialize_base64(deserializer)
                .map(Vec::into_boxed_slice)
                .map(Into::into)
        } else {
            let bytes: Box<[u8]> = Deserialize::deserialize(deserializer)?;
            Ok(bytes.into())
        }
    }
}

impl<'de, T> Deserialize<'de> for Option<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor<T> {
            out: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BytesVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("optional byte array")
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Some)
            }
        }

        let visitor = BytesVisitor { out: PhantomData };
        deserializer.deserialize_option(visitor)
    }
}
