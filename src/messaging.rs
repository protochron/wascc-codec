//! # Message Broker Data Types
//!
//! This module contains data types for the `wascc:messaging` capability provider

use crate::Sample;

pub const OP_PUBLISH_MESSAGE: &str = "Publish";
pub const OP_DELIVER_MESSAGE: &str = "DeliverMessage";
pub const OP_PERFORM_REQUEST: &str = "Request";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// A representation of a broker message
#[serde(rename_all = "camelCase")]
pub struct BrokerMessage {
    pub subject: String,
    pub reply_to: String,
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

/// A request for the broker to make a request-and-reply publication. Inbox management
/// is handled by the plugin implementation, not by the guest module
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMessage {
    pub subject: String,
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
    #[serde(rename = "timeout")]
    pub timeout_ms: i64,
}

impl Sample for RequestMessage {
    fn sample() -> Self {
        RequestMessage {
            subject: "user.profile.175".to_string(),
            body: b"raw query bytes".to_vec(),
            timeout_ms: 100,
        }
    }
}
