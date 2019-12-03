#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # wascc-codec
//!
//! This library provides the core set of types and associated functions used to facilitate guest module
//! and host runtime communication for wascc.

extern crate serde_derive;

pub mod http {
    //! # HTTP server capability data structures
    //!
    //! This module contains data types for the `wascc:http_server` and `wascc:httpclient` capabilities.
    include!(concat!(env!("OUT_DIR"), "/http.rs"));

    use prost::Message;
    use serde::ser::Serialize;
    use std::collections::HashMap;

    pub const OP_PERFORM_REQUEST: &str = "wascc:http_client!PerformRequest";
    pub const OP_HANDLE_REQUEST: &str = "wascc:http_server!HandleRequest";

    impl Into<Request> for &[u8] {
        fn into(self) -> Request {
            Request::decode(self).unwrap()
        }
    }

    impl Into<Response> for &[u8] {
        fn into(self) -> Response {
            Response::decode(self).unwrap()
        }
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
}

pub mod core {
    //! # Core data types
    //!
    //! This module contains data types used for wascc actor module and host runtime communications
    //! that is not specific to any given capability provider
    include!(concat!(env!("OUT_DIR"), "/core.rs"));

    pub const OP_PERFORM_LIVE_UPDATE: &str = "wascc:core!PerformLiveUpdate";
    pub const OP_IDENTIFY_CAPABILITY: &str = "wascc:core!IdentifyCapability";
    pub const OP_HEALTH_REQUEST: &str = "wascc:core!HealthRequest";
    pub const OP_INITIALIZE: &str = "wascc:core!Initialize";
    pub const OP_CONFIGURE: &str = "wascc:core!Configure";
}

pub mod messaging {
    //! # Message Broker Data Types
    //!
    //! This module contains data types for the `wascc:messaging` capability provider
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/messaging.rs"));

    pub const OP_PUBLISH_MESSAGE: &str = "wascc:messaging!Publish";
    pub const OP_DELIVER_MESSAGE: &str = "wascc:messaging!DeliverMessage";
    pub const OP_PERFORM_REQUEST: &str = "wascc:messaging!Request";

    impl Into<DeliverMessage> for &[u8] {
        fn into(self) -> DeliverMessage {
            DeliverMessage::decode(self).unwrap()
        }
    }

    impl Into<PublishMessage> for &[u8] {
        fn into(self) -> PublishMessage {
            PublishMessage::decode(self).unwrap()
        }
    }

    impl Into<RequestMessage> for &[u8] {
        fn into(self) -> RequestMessage {
            RequestMessage::decode(self).unwrap()
        }
    }
}

pub mod keyvalue {
    //! # Key-Value Store Data Types
    //!
    //! This module contains data types for the `wascc:keyvalue` capability provider
    include!(concat!(env!("OUT_DIR"), "/keyvalue.rs"));

    pub const OP_ADD: &str = "wascc:keyvalue!Add";
    pub const OP_GET: &str = "wascc:keyvalue!Get";
    pub const OP_SET: &str = "wascc:keyvalue!Set";
    pub const OP_DEL: &str = "wascc:keyvalue!Del";
    pub const OP_CLEAR: &str = "wascc:keyvalue!Clear";
    pub const OP_RANGE: &str = "wascc:keyvalue!Range";
    pub const OP_PUSH: &str = "wascc:keyvalue!Push";
    pub const OP_LIST_DEL: &str = "wascc:keyvalue!ListItemDelete";

    pub const OP_SET_ADD: &str = "wascc:keyvalue!SetAdd";
    pub const OP_SET_REMOVE: &str = "wascc:keyvalue!SetRemove";
    pub const OP_SET_UNION: &str = "wascc:keyvalue!SetUnion";
    pub const OP_SET_INTERSECT: &str = "wascc:keyvalue!SetIntersection";
    pub const OP_SET_QUERY: &str = "wascc:keyvalue!SetQuery";
    pub const OP_KEY_EXISTS: &str = "wascc:keyvalue!KeyExists";
}

pub mod capabilities;
