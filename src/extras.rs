//! # Extras
//!
//! Miscellaneous operations that might be common across many different types of actors that
//! shouldn't require a full capability provider plugin, like random numbers, sequence
//! numbers, etc.

use crate::Sample;

pub const OP_REQUEST_GUID: &str = "RequestGuid";
pub const OP_REQUEST_SEQUENCE: &str = "RequestSequence";
pub const OP_REQUEST_RANDOM: &str = "RequestRandom";

#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorResult {
    pub guid: Option<String>,
    pub sequence_number: u64,
    pub random_number: u32,
}

impl Sample for GeneratorResult {
    fn sample() -> Self {
        GeneratorResult {
            guid: Some("insert_generated_guid_here".to_string()),
            sequence_number: 0,
            random_number: 0,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorRequest {
    pub guid: bool,
    pub sequence: bool,
    pub random: bool,
    pub min: u32,
    pub max: u32,
}
