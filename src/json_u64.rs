// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Serialization and deserialization for a U64 in JSON

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents u64 using string, when serializing to Json
/// Javascript integers are not 64 bit, and so it is not really proper json.
/// Using string avoids issues with some json parsers not handling large
/// numbers well.
///
/// This does not rely on the serde-json arbitrary precision feature, which
/// (we fear) might break other things (e.g. https://github.com/serde-rs/json/issues/505)
#[serde_as]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct JsonU64(#[serde_as(as = "DisplayFromStr")] pub u64);

impl From<&u64> for JsonU64 {
    fn from(src: &u64) -> Self {
        Self(*src)
    }
}

impl From<&JsonU64> for u64 {
    fn from(src: &JsonU64) -> u64 {
        src.0
    }
}

impl From<JsonU64> for u64 {
    fn from(src: JsonU64) -> u64 {
        src.0
    }
}

impl AsRef<u64> for JsonU64 {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    struct TestStruct {
        nums: Vec<JsonU64>,
        block: JsonU64,
    }

    #[test]
    fn serialize_jsonu64_struct() {
        let the_struct = TestStruct {
            nums: [0, 1, 2, u64::MAX].iter().map(Into::into).collect(),
            block: JsonU64(u64::MAX - 1),
        };
        let serialized = crate::serialize(&the_struct).unwrap();
        let deserialized =
            crate::deserialize::<TestStruct>(&serialized).expect("Could not deserialize struct");
        assert_eq!(deserialized, the_struct);

        // Sanity that serde_as works as expected: it should accept and hand us back
        // strings.
        let expected_json =
            r#"{"nums":["0","1","2","18446744073709551615"],"block":"18446744073709551614"}"#;
        assert_eq!(
            expected_json,
            serde_json::to_string(&the_struct).expect("Could not convert struct to json string")
        );
        assert_eq!(
            the_struct,
            serde_json::from_str(expected_json).expect("Could not convert json string to struct")
        );
    }
}
