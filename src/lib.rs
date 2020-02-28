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

    pub const OP_PERFORM_REQUEST: &str = "PerformRequest";
    pub const OP_HANDLE_REQUEST: &str = "HandleRequest";

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
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/core.rs"));

    pub const OP_PERFORM_LIVE_UPDATE: &str = "PerformLiveUpdate";
    pub const OP_IDENTIFY_CAPABILITY: &str = "IdentifyCapability";
    pub const OP_HEALTH_REQUEST: &str = "HealthRequest";
    pub const OP_INITIALIZE: &str = "Initialize";
    pub const OP_CONFIGURE: &str = "Configure";
    pub const OP_REMOVE_ACTOR: &str = "RemoveActor";

    impl Into<CapabilityConfiguration> for &[u8] {
        fn into(self) -> CapabilityConfiguration {
            CapabilityConfiguration::decode(self).unwrap()
        }
    }
}

pub mod extras {
    //! # Extras
    //!
    //! Miscellaneous operations that might be common across many different types of actors that
    //! shouldn't require a full capability provider plugin, like random numbers, sequence
    //! numbers, etc.

    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/extras.rs"));

    pub const OP_REQUEST_GUID: &str = "RequestGuid";
    pub const OP_REQUEST_SEQUENCE: &str = "RequestSequence";
    pub const OP_REQUEST_RANDOM: &str = "RequestRandom";

    impl Into<GeneratorResult> for &[u8] {
        fn into(self) -> GeneratorResult {
            GeneratorResult::decode(self).unwrap()
        }
    }

    impl Into<GeneratorRequest> for &[u8] {
        fn into(self) -> GeneratorRequest {
            GeneratorRequest::decode(self).unwrap()
        }
    }
}

pub mod eventstreams {
    //! # Event Streaming
    //!
    //! This module contains data types and operation constants for the `wascc:eventstreams` capability
    //! provider. For more information on append-only event streams, event sourcing, and how they apply
    //! to waSCC actor development, check the documentation on [waSCC.dev](https://wascc.dev)

    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/eventstream.rs"));

    /// Capability provider uses this operation to deliver an event to an actor
    pub const OP_DELIVER_EVENT: &str = "DeliverEvent";
    /// Actor invokes this operation on provider to write an event to a given event stream
    pub const OP_WRITE_EVENT: &str = "WriteEvent";
    /// Actor invokes this operation to execute a query against an event stream
    pub const OP_QUERY_STREAM: &str = "QueryStream";

    impl Into<Event> for &[u8] {
        fn into(self) -> Event {
            Event::decode(self).unwrap()
        }
    }

    impl Into<StreamQuery> for &[u8] {
        fn into(self) -> StreamQuery {
            StreamQuery::decode(self).unwrap()
        }
    }

    impl Into<TimeRange> for &[u8] {
        fn into(self) -> TimeRange {
            TimeRange::decode(self).unwrap()
        }
    }

    impl Into<WriteResponse> for &[u8] {
        fn into(self) -> WriteResponse {
            WriteResponse::decode(self).unwrap()
        }
    }

    impl Into<StreamResults> for &[u8] {
        fn into(self) -> StreamResults {
            StreamResults::decode(self).unwrap()
        }
    }
}

pub mod blobstore {
    //! # Binary object storage and streaming
    //!
    //! This module contains data types for the `wascc:blobstore` capability provider. For more information on
    //! how the blob store capability works within the constraints of a WebAssembly host runtime, check out
    //! the documentation on [waSCC.dev](https://wascc.dev)

    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/blobstore.rs"));

    /// Guest sends a Container to the capability provider, receives a Container back
    pub const OP_CREATE_CONTAINER: &str = "CreateContainer";
    /// Guest sends a Container to the capability provider, lack of error indicates success
    pub const OP_REMOVE_CONTAINER: &str = "RemoveContainer";
    /// Guest sends a Blob to the capability provider, lack of error indicates success
    pub const OP_REMOVE_OBJECT: &str = "RemoveObject";
    /// Guest sends a Container to the capability provider, receives a BlobList back
    pub const OP_LIST_OBJECTS: &str = "ListObjects";
    /// Guest sends a FileChunk to capability provider for storing as part of a Blob, lack of error indicates success
    pub const OP_UPLOAD_CHUNK: &str = "UploadChunk";
    /// Guest sends a StreamRequest to the capability provider, immediate termination w/success. Guest will then
    /// start receiving OP_RECEIVE_CHUNK operations from the provider as chunks are streamed to the guest
    pub const OP_START_DOWNLOAD: &str = "StartDownload";
    /// Guest sends a metadata-carrying FileChunk to initiate an upload, lack of error is success
    pub const OP_START_UPLOAD: &str = "StartUpload";
    /// Guest will receive a FileChunk for each piece of a file requested to download
    pub const OP_RECEIVE_CHUNK: &str = "ReceiveChunk";
    /// Query information on a single blob. Guest sends an incomplete blob struct and gets a complete one in return
    pub const OP_GET_OBJECT_INFO: &str = "GetObjectInfo";

    impl Into<FileChunk> for &[u8] {
        fn into(self) -> FileChunk {
            FileChunk::decode(self).unwrap()
        }
    }

    impl Into<Container> for &[u8] {
        fn into(self) -> Container {
            Container::decode(self).unwrap()
        }
    }

    impl Into<Blob> for &[u8] {
        fn into(self) -> Blob {
            Blob::decode(self).unwrap()
        }
    }

    impl Into<ContainerList> for &[u8] {
        fn into(self) -> ContainerList {
            ContainerList::decode(self).unwrap()
        }
    }

    impl Into<BlobList> for &[u8] {
        fn into(self) -> BlobList {
            BlobList::decode(self).unwrap()
        }
    }

    impl Into<StreamRequest> for &[u8] {
        fn into(self) -> StreamRequest {
            StreamRequest::decode(self).unwrap()
        }
    }
}

pub mod messaging {
    //! # Message Broker Data Types
    //!
    //! This module contains data types for the `wascc:messaging` capability provider
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/messaging.rs"));

    pub const OP_PUBLISH_MESSAGE: &str = "Publish";
    pub const OP_DELIVER_MESSAGE: &str = "DeliverMessage";
    pub const OP_PERFORM_REQUEST: &str = "Request";

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

    pub const OP_ADD: &str = "Add";
    pub const OP_GET: &str = "Get";
    pub const OP_SET: &str = "Set";
    pub const OP_DEL: &str = "Del";
    pub const OP_CLEAR: &str = "Clear";
    pub const OP_RANGE: &str = "Range";
    pub const OP_PUSH: &str = "Push";
    pub const OP_LIST_DEL: &str = "ListItemDelete";

    pub const OP_SET_ADD: &str = "SetAdd";
    pub const OP_SET_REMOVE: &str = "SetRemove";
    pub const OP_SET_UNION: &str = "SetUnion";
    pub const OP_SET_INTERSECT: &str = "SetIntersection";
    pub const OP_SET_QUERY: &str = "SetQuery";
    pub const OP_KEY_EXISTS: &str = "KeyExists";
}

pub mod capabilities;
