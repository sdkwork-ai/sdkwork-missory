//! Wire-level serde helpers shared by Missory DTOs.
//!
//! `sdkwork_utils_rust::serde_uint64` covers scalar and `Option` ids; this module
//! adds the id-vector form required by story participants and memory links
//! (`API_SPEC.md` section 13.6: int64 wire fields are strings).

use serde::{Deserialize, Deserializer, Serializer};

/// Serializes a `Vec<u64>` as an array of decimal strings.
pub fn serialize_u64_vec<S>(values: &[u64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(values.iter().map(ToString::to_string))
}

/// Deserializes an array of decimal id strings into a `Vec<u64>`.
pub fn deserialize_u64_vec<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Vec::<String>::deserialize(deserializer)?;
    raw.iter()
        .map(|item| {
            sdkwork_utils_rust::serde_uint64::deserialize(serde::de::value::StrDeserializer::<
                D::Error,
            >::new(item.as_str()))
        })
        .collect()
}

/// Serializes an `Option<Vec<u64>>` as an array of decimal strings (or null).
pub fn serialize_u64_vec_option<S>(
    values: &Option<Vec<u64>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match values {
        Some(values) => serialize_u64_vec(values, serializer),
        None => serializer.serialize_none(),
    }
}

/// Deserializes an optional array of decimal id strings into `Option<Vec<u64>>`.
pub fn deserialize_u64_vec_option<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<Vec<String>>::deserialize(deserializer)?;
    match raw {
        None => Ok(None),
        Some(items) => items
            .iter()
            .map(|item| {
                sdkwork_utils_rust::serde_uint64::deserialize(serde::de::value::StrDeserializer::<
                    D::Error,
                >::new(item.as_str()))
            })
            .collect::<Result<Vec<u64>, D::Error>>()
            .map(Some),
    }
}
