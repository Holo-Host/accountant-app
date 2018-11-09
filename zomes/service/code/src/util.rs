use hdk::{
    self,
    holochain_core_types::{hash::HashString, json::JsonString},
};
use multihash::Hash as Multihash;
use serde::{Deserialize, Serialize};

pub fn make_hash<S: Into<JsonString>>(value: S) -> HashString {
    HashString::encode_from_json_string(value.into(), Multihash::SHA2256)
}
