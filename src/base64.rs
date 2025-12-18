//! Base64 encoding support for human-readable serialization.
//!
//! This module provides an alternative to the default hex encoding.
//! Use it with `#[serde(with = "serde_human_bytes::base64")]`.
//!
//! # Example
//!
//! ```
//! use serde_derive::{Deserialize, Serialize};
//!
//! #[derive(Deserialize, Serialize)]
//! struct Example {
//!     #[serde(with = "serde_human_bytes::base64")]
//!     data: Vec<u8>,
//!
//!     #[serde(with = "serde_human_bytes::base64")]
//!     byte_buf: serde_human_bytes::ByteBuf,
//!
//!     #[serde(with = "serde_human_bytes::base64")]
//!     boxed: Box<[u8]>,
//! }
//! ```

mod de;
mod ser;

pub use de::Deserialize;
pub use ser::Serialize;
use serde::{Deserializer, Serializer};

// ============ Public API functions ============

/// Serde `serialize_with` function to serialize bytes as base64.
///
/// Use with `#[serde(serialize_with = "serde_human_bytes::base64::serialize")]`
/// or `#[serde(with = "serde_human_bytes::base64")]`.
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: Serializer,
{
    Serialize::serialize(bytes, serializer)
}

/// Serde `deserialize_with` function to deserialize bytes from base64.
///
/// Use with `#[serde(deserialize_with = "serde_human_bytes::base64::deserialize")]`
/// or `#[serde(with = "serde_human_bytes::base64")]`.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
}
