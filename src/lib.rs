#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use serde::{Deserialize, Deserializer, Serializer};

/// Serialize bytes to a human-readable hex string or raw bytes.
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    if serializer.is_human_readable() {
        // Convert bytes to hex string for human-readable formats
        serializer.serialize_str(&hex::encode(bytes.as_ref()))
    } else {
        // Use raw bytes for non-human-readable formats
        serializer.serialize_bytes(bytes.as_ref())
    }
}

/// Deserialize bytes from a human-readable hex string or raw bytes.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<Vec<u8>>,
{
    use serde::de::Error;

    if deserializer.is_human_readable() {
        // Parse hex string for human-readable formats
        let hex_str: String = Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(hex_str).map_err(D::Error::custom)?;
        Ok(T::try_from(bytes).or_else(|_| Err(D::Error::custom("invalid bytes")))?)
    } else {
        // Parse raw bytes for non-human-readable formats
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        Ok(T::try_from(bytes).or_else(|_| Err(D::Error::custom("invalid bytes")))?)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Data {
        #[serde(with = "super")]
        bytes: Vec<u8>,
    }

    #[test]
    fn it_works_with_serde_json() {
        let data = Data {
            bytes: vec![0x01, 0x02, 0x03],
        };
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"{"bytes":"010203"}"#);

        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.bytes, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn it_works_with_binary_format() {
        let data = Data {
            bytes: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        };

        let serialized = bincode::serialize(&data).unwrap();
        assert_eq!(serialized, vec![5u8, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5]);
        let deserialized: Data = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.bytes, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    }

    #[test]
    fn it_works_with_array() {
        #[derive(Serialize, Deserialize)]
        struct Data {
            #[serde(with = "super")]
            bytes: [u8; 3],
        }

        let data = Data { bytes: [0x01, 0x02, 0x03] };
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"{"bytes":"010203"}"#);

        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.bytes, [0x01, 0x02, 0x03]);
    }
}
