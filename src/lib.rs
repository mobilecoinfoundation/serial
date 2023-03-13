// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Serialization and deserialization utilities for MobileCoin.

#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

extern crate alloc;

use alloc::vec::Vec;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    mem,
};
use serde::{Deserialize, Serialize};

pub use prost::{self, DecodeError, EncodeError, Message};

/// Decoding-specific types, here for backwards compatibility.
pub mod decode {
    use super::*;
    use alloc::{
        format,
        string::{String, ToString},
    };
    use ciborium::de::Error as CiboriumError;
    use core::fmt::Debug;

    /// An error structure for CBOR decoding
    #[derive(Debug)]
    pub struct Error(String);

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "CBOR Decode Error: {:#?}", self.0)
        }
    }

    impl<T: Debug> From<CiboriumError<T>> for Error {
        fn from(src: CiboriumError<T>) -> Self {
            let err_msg = match src {
                CiboriumError::Io(underlying) => format!("IO Error: {underlying:#?}"),
                CiboriumError::Syntax(offset) => format!("Syntax Error at byte {offset:#?}"),
                CiboriumError::Semantic(offset, inner_msg) => {
                    format!("Semantic Error at byte {offset:#?}: {inner_msg:#?}")
                }
                CiboriumError::RecursionLimitExceeded => "Recursion limit exceeded".to_string(),
            };

            Self(err_msg)
        }
    }
}

/// Encoding-specific types, here for backwards compatibility.
pub mod encode {
    use super::*;

    /// CBOR encoding errors (this shouldn't actually be seen in practice)
    #[derive(Debug)]
    pub struct Error;

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "CBOR Encode Error")
        }
    }
}

/// Serialize the given data structure.
///
/// This method produces CBOR-encoded bytes.
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, encode::Error>
where
    T: Serialize + Sized,
{
    let mut writer = Vec::with_capacity(2 * mem::size_of::<T>());
    // NOTE: this depends on the precise behavior of ciborium-io 0.2:
    //       https://docs.rs/ciborium-io/0.2.0/src/ciborium_io/lib.rs.html#151
    ciborium::ser::into_writer(value, &mut writer)
        .expect("into_writer with a Vec should be infallible");

    Ok(writer)
}

/// Deserialize the given bytes to a data structure.
///
/// This method expects CBOR-encoded bytes.
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, decode::Error>
where
    T: Deserialize<'a>,
{
    Ok(ciborium::de::from_reader(bytes)?)
}

/// Encode the give data structure to protobuf using prost::Message.
pub fn encode<T: Message>(value: &T) -> Vec<u8> {
    value.encode_to_vec()
}

/// Decode the protobuf-encoded bytes into a data structure using prost::Message.
pub fn decode<T: Message + Default>(buf: &[u8]) -> Result<T, DecodeError> {
    T::decode(buf)
}

#[cfg(feature = "jsonu64")]
/// This module contains a JsonU64 type which is used to represent u64 values safely in JSON.
mod json_u64;

/// JsonU64 is exported if it is available -- the serde_with crate which it
/// depends on relies on std, so it must be optional.
#[cfg(feature = "jsonu64")]
pub use json_u64::JsonU64;

/// Take a prost type and try to roundtrip it through a protobuf type
#[cfg(feature = "test_utils")]
pub fn round_trip_message<SRC: Message + Eq + Default, DEST: protobuf::Message>(prost_val: &SRC) {
    let prost_bytes = encode(prost_val);

    let dest_val =
        DEST::parse_from_bytes(&prost_bytes).expect("Parsing protobuf from prost bytes failed");

    let protobuf_bytes = dest_val
        .write_to_bytes()
        .expect("Writing protobuf to bytes failed");

    let final_val: SRC = decode(&protobuf_bytes).expect("Parsing prost from protobuf bytes failed");

    assert_eq!(
        *prost_val, final_val,
        "Round-trip check failed!\nprost: {prost_val:?}\nprotobuf: {final_val:?}"
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{string::String, vec};
    use serde::{Deserialize, Serialize};

    #[test]
    fn serialize_string() {
        const THE_STRING: &str = "There goes the baker with his tray, like always";

        let serialized = serialize(&THE_STRING).unwrap();
        let deserialized = deserialize::<String>(&serialized).unwrap();
        assert_eq!(&deserialized, THE_STRING);
    }

    #[test]
    fn serialize_struct() {
        #[derive(PartialEq, Serialize, Deserialize, Debug)]
        struct TestStruct {
            vec: Vec<u8>,
            integer: u64,
            float: f64,
        }

        let the_struct = TestStruct {
            vec: vec![233, 123, 0, 12],
            integer: 4_242_424_242,
            float: 1.2345,
        };
        let serialized = serialize(&the_struct).unwrap();
        let deserialized = deserialize::<TestStruct>(&serialized).unwrap();
        assert_eq!(deserialized, the_struct);
    }

    #[test]
    fn serialize_array() {
        let bytes = [0x55u8; 32];

        let serialized = serialize(&bytes).expect("Could not serialize byte array.");
        let deserialized =
            deserialize::<[u8; 32]>(&serialized).expect("Could not deserialize byte array.");

        assert_eq!(deserialized, bytes);
    }

    #[test]
    fn serialize_array_in_struct() {
        #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
        struct ByteStruct {
            bytes: [u8; 32],
        }

        let value = ByteStruct::default();
        let serialized = serialize(&value).expect("Could not serialize byte struct.");
        let deserialized =
            deserialize::<ByteStruct>(&serialized).expect("Could not deserialize byte struct.");

        assert_eq!(deserialized, value);

        let value = ByteStruct {
            bytes: [0x55u8; 32],
        };
        let serialized = serialize(&value).expect("Could not serialize byte struct.");
        let deserialized =
            deserialize::<ByteStruct>(&serialized).expect("Could not deserialize byte struct.");

        assert_eq!(deserialized, value);
    }
}
