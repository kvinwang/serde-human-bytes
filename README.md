# serde-human-bytes

A fork of [serde_bytes](https://github.com/serde-rs/bytes) that serialize bytes to hex string when the format is human readable.

```toml
[dependencies]
serde-human-bytes = "0.1"
```

## Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_human_bytes")]
    bytes: &'a [u8],

    #[serde(with = "serde_human_bytes")]
    byte_buf: Vec<u8>,
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
