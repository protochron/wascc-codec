//! # Extras
//!
//! Miscellaneous operations that might be common across many different types of actors that
//! shouldn't require a full capability provider plugin, like random numbers, sequence
//! numbers, etc.

pub const OP_REQUEST_GUID: &str = "RequestGuid";
pub const OP_REQUEST_SEQUENCE: &str = "RequestSequence";
pub const OP_REQUEST_RANDOM: &str = "RequestRandom";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratorResult {
    pub value: GeneratorResultType,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratorRequest {
    pub typ: GeneratorRequestType,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum GeneratorResultType {
    Guid(String),
    SequenceNumber(u64),
    RandomNumber(u32),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum GeneratorRequestType {
    Guid,
    SequenceNumber,
    RandomNumber(u32, u32),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{deserialize, serialize};

    // A quick test to certify that the enum round trip
    // works fine in message pack
    #[test]
    fn round_trip() {
        let req1 = GeneratorRequest {
            typ: GeneratorRequestType::RandomNumber(5, 10),
        };
        let buf = serialize(&req1).unwrap();
        let req2: GeneratorRequest = deserialize(&buf).unwrap();
        assert_eq!(req1, req2);
    }
}
