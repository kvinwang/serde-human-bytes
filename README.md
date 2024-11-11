# serde-human-bytes

Serialize and deserialize bytes to hex strings for human-readable formats, and keep raw bytes for binary formats.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-human-bytes = "0.1.0"
```

### Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(with = "serde_human_bytes")]
    bytes: Vec<u8>,
}

fn main() {
    // Create test data
    let data = Data {
        bytes: vec![0x01, 0x02, 0x03],
    };

    // Serialize to JSON (human-readable format)
    let json = serde_json::to_string(&data).unwrap();
    println!("JSON: {}", json);  // Output: {"bytes":"010203"}

    // Deserialize from JSON
    let decoded: Data = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.bytes, vec![0x01, 0x02, 0x03]);

    // Works with binary formats too (like bincode)
    let binary = bincode::serialize(&data).unwrap();
    let decoded: Data = bincode::deserialize(&binary).unwrap();
    assert_eq!(decoded.bytes, vec![0x01, 0x02, 0x03]);
}
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.