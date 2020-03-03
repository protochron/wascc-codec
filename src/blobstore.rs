//! # Binary object storage and streaming
//!
//! This module contains data types for the `wascc:blobstore` capability provider. For more information on
//! how the blob store capability works within the constraints of a WebAssembly host runtime, check out
//! the documentation on [waSCC.dev](https://wascc.dev)

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileChunk {
    pub sequence_no: u64,
    pub container: String,
    pub id: String,
    pub total_bytes: u64,
    pub chunk_size: u64,
    pub chunk_bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Container {
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ContainerList {
    pub containers: Vec<Container>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Blob {
    pub id: String,
    pub container: String,
    pub byte_size: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct BlobList {
    pub blobs: Vec<Blob>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamRequest {
    pub id: String,
    pub container: String,
    pub chunk_size: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Transfer {
    pub blob_id: String,
    pub container: String,
    pub chunk_size: u64,
    pub total_size: u64,
    pub total_chunks: u64,
}
