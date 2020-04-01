//! # HTTP server capability data structures
//!
//! This module contains data types for the `wascc:http_server` and `wascc:httpclient` capabilities.

use serde::ser::Serialize;
use std::collections::HashMap;

pub const OP_PERFORM_REQUEST: &str = "PerformRequest";
pub const OP_HANDLE_REQUEST: &str = "HandleRequest";

/// Describes an HTTP request received by the HTTP server capability provider
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query_string: String,
    #[serde(default="default_hashmap")]
    pub header: HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

/// Represents an HTTP response that the guest module would like to return in response
/// to a request command
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Response {
    pub status_code: u32,    
    pub status: String,  
    #[serde(default="default_hashmap")]  
    pub header: HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

fn default_hashmap() -> HashMap<String, String> {
    HashMap::new()
}

impl Response {
    /// Creates a response with a given status code and serializes the given payload as JSON
    pub fn json<T>(payload: T, status_code: u32, status: &str) -> Response
    where
        T: Serialize,
    {
        Response {
            body: serde_json::to_string(&payload).unwrap().into_bytes(),
            header: HashMap::new(),
            status: status.to_string(),
            status_code,
        }
    }

    /// Handy shortcut for creating a 404/Not Found response
    pub fn not_found() -> Response {
        Response {
            status: "Not Found".to_string(),
            status_code: 404,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 200/OK response
    pub fn ok() -> Response {
        Response {
            status: "OK".to_string(),
            status_code: 200,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 500/Internal Server Error response
    pub fn internal_server_error(msg: &str) -> Response {
        Response {
            status: "Internal Server Error".to_string(),
            status_code: 500,
            body: msg.to_string().as_bytes().into(),
            ..Default::default()
        }
    }

    /// Shortcut for creating a 400/Bad Request response
    pub fn bad_request() -> Response {
        Response {
            status: "Bad Request".to_string(),
            status_code: 400,
            ..Default::default()
        }
    }
}
