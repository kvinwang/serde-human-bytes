use serde_derive::{Deserialize, Serialize};
use serde_human_bytes::{ByteArray, ByteBuf, Bytes};
use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Configure, Token};

#[test]
fn test_bytes_readable() {
    let empty = Bytes::new(&[]).readable();
    assert_ser_tokens(&empty, &[Token::Str("")]);
    assert_ser_tokens(&empty, &[Token::Str("")]);

    let buf = vec![65, 66, 67];
    let bytes = Bytes::new(&buf).readable();
    assert_ser_tokens(&bytes, &[Token::Str("414243")]);
    assert_ser_tokens(&bytes, &[Token::Str("414243")]);
}

#[test]
fn test_bytes() {
    let empty = Bytes::new(&[]).compact();
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = vec![65, 66, 67];
    let bytes = Bytes::new(&buf).compact();
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}

#[test]
fn test_byte_buf_readable() {
    let empty = ByteBuf::new().readable();
    assert_tokens(&empty, &[Token::Str("")]);
    assert_ser_tokens(&empty, &[Token::Str("")]);
    assert_de_tokens(&empty, &[Token::Str("")]);

    let buf = ByteBuf::from(vec![65, 66, 67]).readable();
    assert_tokens(&buf, &[Token::Str("414243")]);
    assert_de_tokens(&buf, &[Token::Str("414243")]);
}

#[test]
fn test_byte_buf() {
    let empty = ByteBuf::new().compact();
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_tokens(&empty, &[Token::Bytes(b"")]);
    assert_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);
    assert_de_tokens(&empty, &[Token::Str("")]);
    assert_de_tokens(&empty, &[Token::String("")]);
    assert_de_tokens(&empty, &[Token::Seq { len: None }, Token::SeqEnd]);
    assert_de_tokens(&empty, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);

    let buf = ByteBuf::from(vec![65, 66, 67]).compact();
    assert_tokens(&buf, &[Token::BorrowedBytes(b"ABC")]);
    assert_tokens(&buf, &[Token::Bytes(b"ABC")]);
    assert_tokens(&buf, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&buf, &[Token::BorrowedStr("ABC")]);
    assert_de_tokens(&buf, &[Token::Str("ABC")]);
    assert_de_tokens(&buf, &[Token::String("ABC")]);
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: None },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
    assert_de_tokens(
        &buf,
        &[
            Token::Seq { len: Some(3) },
            Token::U8(65),
            Token::U8(66),
            Token::U8(67),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_bytearray_readable() {
    let buf = [65, 66, 67];
    let empty_readable = ByteArray::new([]).readable();
    assert_tokens(&empty_readable, &[Token::Str("")]);
    assert_ser_tokens(&empty_readable, &[Token::Str("")]);
    assert_ser_tokens(&empty_readable, &[Token::Str("")]);
    assert_de_tokens(&empty_readable, &[Token::Str("")]);

    let bytes_readable = ByteArray::new(buf).readable();
    assert_tokens(&bytes_readable, &[Token::Str("414243")]);
    assert_ser_tokens(&bytes_readable, &[Token::Str("414243")]);
    assert_ser_tokens(&bytes_readable, &[Token::Str("414243")]);
    assert_de_tokens(&bytes_readable, &[Token::Str("414243")]);
}

#[test]
fn test_bytearray() {
    let empty_compact = ByteArray::new([]).compact();
    assert_tokens(&empty_compact, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty_compact, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty_compact, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty_compact, &[Token::BorrowedBytes(b"")]);

    let buf = [65, 66, 67];
    let bytes_compact = ByteArray::new(buf).compact();
    assert_tokens(&bytes_compact, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes_compact, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes_compact, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes_compact, &[Token::BorrowedStr("ABC")]);
}

// ============ Base64 module tests ============

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Base64Example {
    #[serde(with = "serde_human_bytes::base64")]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Base64ArrayExample {
    #[serde(with = "serde_human_bytes::base64")]
    data: [u8; 3],
}

#[test]
fn test_base64_vec_readable() {
    // "ABC" (bytes 65, 66, 67) -> base64 "QUJD"
    let example = Base64Example {
        data: vec![65, 66, 67],
    }
    .readable();
    assert_tokens(
        &example,
        &[
            Token::Struct {
                name: "Base64Example",
                len: 1,
            },
            Token::Str("data"),
            Token::Str("QUJD"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_base64_vec_compact() {
    let example = Base64Example {
        data: vec![65, 66, 67],
    }
    .compact();
    assert_tokens(
        &example,
        &[
            Token::Struct {
                name: "Base64Example",
                len: 1,
            },
            Token::Str("data"),
            Token::Bytes(b"ABC"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_base64_array_readable() {
    // "ABC" (bytes 65, 66, 67) -> base64 "QUJD"
    let example = Base64ArrayExample { data: [65, 66, 67] }.readable();
    assert_tokens(
        &example,
        &[
            Token::Struct {
                name: "Base64ArrayExample",
                len: 1,
            },
            Token::Str("data"),
            Token::Str("QUJD"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_base64_array_compact() {
    let example = Base64ArrayExample { data: [65, 66, 67] }.compact();
    assert_tokens(
        &example,
        &[
            Token::Struct {
                name: "Base64ArrayExample",
                len: 1,
            },
            Token::Str("data"),
            Token::Bytes(b"ABC"),
            Token::StructEnd,
        ],
    );
}
