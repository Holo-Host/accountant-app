use hdk::{self, holochain_core_types::hash::HashString};
use multihash::Hash as Multihash;
use serde::{Deserialize, Serialize};

pub fn make_hash<S: Serialize>(value: S) -> HashString {
    HashString::encode_from_serializable(&value, Multihash::SHA2256)
}
