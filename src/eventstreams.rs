//! # Event Streaming
//!
//! This module contains data types and operation constants for the `wascc:eventstreams` capability
//! provider. For more information on append-only event streams, event sourcing, and how they apply
//! to waSCC actor development, check the documentation on [waSCC.dev](https://wascc.dev)

use crate::Sample;
use std::collections::HashMap;

/// Capability provider uses this operation to deliver an event to an actor
pub const OP_DELIVER_EVENT: &str = "DeliverEvent";
/// Actor invokes this operation on provider to write an event to a given event stream
pub const OP_WRITE_EVENT: &str = "WriteEvent";
/// Actor invokes this operation to execute a query against an event stream
pub const OP_QUERY_STREAM: &str = "QueryStream";

/// Represents an immutable event within a stream
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: String,
    pub stream: String,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

/// The response from the provider after writing an event to a stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteResponse {
    pub event_id: String,
}

/// A query against a given stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamQuery {
    pub stream_id: String,
    #[serde(default)]
    pub range: Option<TimeRange>,
    pub count: u64,
}

impl Sample for StreamQuery {
    fn sample() -> Self {
        StreamQuery {
            stream_id: "stream1".to_string(),
            range: Some(TimeRange {
                min_time: 0,
                max_time: 1000,
            }),
            count: 42,
        }
    }
}

/// Results of a stream query
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamResults {
    #[serde(default)]
    pub events: Vec<Event>,
}

/// Represents a timeslice range for a stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub min_time: u64,
    pub max_time: u64,
}
