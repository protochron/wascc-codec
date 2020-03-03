//! # Message Broker Data Types
//!
//! This module contains data types for the `wascc:messaging` capability provider

pub const OP_PUBLISH_MESSAGE: &str = "Publish";
pub const OP_DELIVER_MESSAGE: &str = "DeliverMessage";
pub const OP_PERFORM_REQUEST: &str = "Request";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// A request to deliver a message to a guest module
pub struct DeliverMessage {
    pub message: BrokerMessage,
}

/// A request to publish a message from a guest module
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PublishMessage {
    pub message: BrokerMessage,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// A representation of a broker message
pub struct BrokerMessage {
    pub subject: String,
    pub reply_to: String,
    pub body: Vec<u8>,
}

/// A request for the broker to make a request-and-reply publication. Inbox management
/// is handled by the plugin implementation, not by the guest module
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RequestMessage {
    pub subject: String,
    pub body: Vec<u8>,
    pub timeout_ms: i64,
}
