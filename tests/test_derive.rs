#![allow(clippy::derive_partial_eq_without_eq, clippy::ref_option_ref)]

use serde_derive::{Deserialize, Serialize};
use serde_human_bytes::{ByteArray, ByteBuf, Bytes};
use serde_test::{assert_ser_tokens, assert_tokens, Configure, Token};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test<'a> {
    #[serde(with = "serde_human_bytes")]
    slice: &'a [u8],

    #[serde(with = "serde_human_bytes")]
    array: [u8; 314],

    #[serde(with = "serde_human_bytes")]
    borrowed_array: &'a [u8; 314],

    #[serde(with = "serde_human_bytes")]
    vec: Vec<u8>,

    #[serde(with = "serde_human_bytes")]
    bytes: &'a Bytes,

    #[serde(with = "serde_human_bytes")]
    byte_array: ByteArray<314>,

    #[serde(with = "serde_human_bytes")]
    borrowed_byte_array: &'a ByteArray<314>,

    #[serde(with = "serde_human_bytes")]
    byte_buf: ByteBuf,

    #[serde(with = "serde_human_bytes")]
    cow_slice: Cow<'a, [u8]>,

    #[serde(with = "serde_human_bytes")]
    cow_bytes: Cow<'a, Bytes>,

    #[serde(with = "serde_human_bytes")]
    boxed_slice: Box<[u8]>,

    #[serde(with = "serde_human_bytes")]
    boxed_bytes: Box<Bytes>,

    #[serde(with = "serde_human_bytes")]
    opt_slice: Option<&'a [u8]>,

    #[serde(with = "serde_human_bytes")]
    opt_vec: Option<Vec<u8>>,

    #[serde(with = "serde_human_bytes")]
    opt_array: Option<[u8; 314]>,

    #[serde(with = "serde_human_bytes")]
    opt_bytearray: Option<ByteArray<314>>,

    #[serde(with = "serde_human_bytes")]
    opt_cow_slice: Option<Cow<'a, [u8]>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestOwned {
    #[serde(with = "serde_human_bytes")]
    array: [u8; 314],

    #[serde(with = "serde_human_bytes")]
    vec: Vec<u8>,

    #[serde(with = "serde_human_bytes")]
    byte_array: ByteArray<314>,

    #[serde(with = "serde_human_bytes")]
    byte_buf: ByteBuf,

    #[serde(with = "serde_human_bytes")]
    boxed_slice: Box<[u8]>,

    #[serde(with = "serde_human_bytes")]
    boxed_bytes: Box<Bytes>,

    #[serde(with = "serde_human_bytes")]
    opt_vec: Option<Vec<u8>>,

    #[serde(with = "serde_human_bytes")]
    opt_array: Option<[u8; 314]>,

    #[serde(with = "serde_human_bytes")]
    opt_bytearray: Option<ByteArray<314>>,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct Dst {
    #[serde(with = "serde_human_bytes")]
    bytes: [u8],
}

#[test]
fn test_compact() {
    let test = Test {
        slice: b"...",
        array: [0; 314],
        borrowed_array: &[1; 314],
        vec: b"...".to_vec(),
        bytes: Bytes::new(b"..."),
        byte_array: ByteArray::new([0; 314]),
        borrowed_byte_array: &ByteArray::new([0; 314]),
        byte_buf: ByteBuf::from(b"...".as_ref()),
        cow_slice: Cow::Borrowed(b"..."),
        cow_bytes: Cow::Borrowed(Bytes::new(b"...")),
        boxed_slice: b"...".to_vec().into_boxed_slice(),
        boxed_bytes: ByteBuf::from(b"...".as_ref()).into_boxed_bytes(),
        opt_slice: Some(b"..."),
        opt_vec: Some(b"...".to_vec()),
        opt_array: Some([0; 314]),
        opt_bytearray: Some(ByteArray::new([0; 314])),
        opt_cow_slice: Some(Cow::Borrowed(b"...")),
    };

    assert_tokens(
        &test.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 17,
            },
            Token::Str("slice"),
            Token::BorrowedBytes(b"..."),
            Token::Str("array"),
            Token::Bytes(&[0; 314]),
            Token::Str("borrowed_array"),
            Token::BorrowedBytes(&[1; 314]),
            Token::Str("vec"),
            Token::Bytes(b"..."),
            Token::Str("bytes"),
            Token::BorrowedBytes(b"..."),
            Token::Str("byte_array"),
            Token::Bytes(&[0; 314]),
            Token::Str("borrowed_byte_array"),
            Token::BorrowedBytes(&[0; 314]),
            Token::Str("byte_buf"),
            Token::Bytes(b"..."),
            Token::Str("cow_slice"),
            Token::BorrowedBytes(b"..."),
            Token::Str("cow_bytes"),
            Token::BorrowedBytes(b"..."),
            Token::Str("boxed_slice"),
            Token::Bytes(b"..."),
            Token::Str("boxed_bytes"),
            Token::Bytes(b"..."),
            Token::Str("opt_slice"),
            Token::Some,
            Token::BorrowedBytes(b"..."),
            Token::Str("opt_vec"),
            Token::Some,
            Token::Bytes(b"..."),
            Token::Str("opt_array"),
            Token::Some,
            Token::Bytes(&[0; 314]),
            Token::Str("opt_bytearray"),
            Token::Some,
            Token::Bytes(&[0; 314]),
            Token::Str("opt_cow_slice"),
            Token::Some,
            Token::BorrowedBytes(b"..."),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_readable() {
    let test = Test {
        slice: b"...",
        array: [0; 314],
        borrowed_array: &[1; 314],
        vec: b"...".to_vec(),
        bytes: Bytes::new(b"..."),
        byte_array: ByteArray::new([0; 314]),
        borrowed_byte_array: &ByteArray::new([0; 314]),
        byte_buf: ByteBuf::from(b"...".as_ref()),
        cow_slice: Cow::Borrowed(b"..."),
        cow_bytes: Cow::Borrowed(Bytes::new(b"...")),
        boxed_slice: b"...".to_vec().into_boxed_slice(),
        boxed_bytes: ByteBuf::from(b"...".as_ref()).into_boxed_bytes(),
        opt_slice: Some(b"..."),
        opt_vec: Some(b"...".to_vec()),
        opt_array: Some([0; 314]),
        opt_bytearray: Some(ByteArray::new([0; 314])),
        opt_cow_slice: Some(Cow::Borrowed(b"...")),
    };

    assert_ser_tokens(
        &test.readable(),
        &[
            Token::Struct {
                name: "Test",
                len: 17,
            },
            Token::Str("slice"),
            Token::Str("2e2e2e"),
            Token::Str("array"),
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("borrowed_array"),
            Token::Str("0101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101"),
            Token::Str("vec"),
            Token::Str("2e2e2e"),
            Token::Str("bytes"),
            Token::Str("2e2e2e"),
            Token::Str("byte_array"),
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("borrowed_byte_array"),
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("byte_buf"),
            Token::Str("2e2e2e"),
            Token::Str("cow_slice"),
            Token::Str("2e2e2e"),
            Token::Str("cow_bytes"),
            Token::Str("2e2e2e"),
            Token::Str("boxed_slice"),
            Token::Str("2e2e2e"),
            Token::Str("boxed_bytes"),
            Token::Str("2e2e2e"),
            Token::Str("opt_slice"),
            Token::Some,
            Token::Str("2e2e2e"),
            Token::Str("opt_vec"),
            Token::Some,
            Token::Str("2e2e2e"),
            Token::Str("opt_array"),
            Token::Some,
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("opt_bytearray"),
            Token::Some,
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("opt_cow_slice"),
            Token::Some,
            Token::Str("2e2e2e"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_readable_owned() {
    let test = TestOwned {
        array: [0; 314],
        vec: b"...".to_vec(),
        byte_array: ByteArray::new([0; 314]),
        byte_buf: ByteBuf::from(b"...".as_ref()),
        boxed_slice: b"...".to_vec().into_boxed_slice(),
        boxed_bytes: ByteBuf::from(b"...".as_ref()).into_boxed_bytes(),
        opt_vec: Some(b"...".to_vec()),
        opt_array: Some([0; 314]),
        opt_bytearray: Some(ByteArray::new([0; 314])),
    };

    assert_ser_tokens(
        &test.readable(),
        &[
            Token::Struct {
                name: "TestOwned",
                len: 9,
            },
            Token::Str("array"),
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("vec"),
            Token::Str("2e2e2e"),
            Token::Str("byte_array"),
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("byte_buf"),
            Token::Str("2e2e2e"),
            Token::Str("boxed_slice"),
            Token::Str("2e2e2e"),
            Token::Str("boxed_bytes"),
            Token::Str("2e2e2e"),
            Token::Str("opt_vec"),
            Token::Some,
            Token::Str("2e2e2e"),
            Token::Str("opt_array"),
            Token::Some,
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::Str("opt_bytearray"),
            Token::Some,
            Token::Str("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            Token::StructEnd
        ],
    );
}
