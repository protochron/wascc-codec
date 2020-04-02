//! # Logging Data Types
//!
//! This module contains data types for the `wascc:logging` capability provider

use crate::Sample;

pub const OP_LOG: &str = "WriteLog";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A representation of a request to write a log entry
pub struct WriteLogRequest {
    /// level corresponds to the log level
    ///
    /// "OFF"=0 , "ERROR"=1, "WARN"=2, "INFO"=3, "DEBUG"=4, "TRACE"=5
    pub level: u32,
    /// A string that represents the body of the log message
    pub body: String,
}

impl Sample for WriteLogRequest {
    fn sample() -> Self {
        WriteLogRequest {
            level: 4,
            body: "This is a debug message".to_string(),
        }
    }
}
