//! # Core data types
//!
//! This module contains data types used for wascc actor module and host runtime communications
//! that is not specific to any given capability provider

use std::collections::HashMap;

pub const OP_PERFORM_LIVE_UPDATE: &str = "PerformLiveUpdate";
pub const OP_IDENTIFY_CAPABILITY: &str = "IdentifyCapability";
pub const OP_HEALTH_REQUEST: &str = "HealthRequest";
pub const OP_INITIALIZE: &str = "Initialize";
pub const OP_CONFIGURE: &str = "Configure";
pub const OP_REMOVE_ACTOR: &str = "RemoveActor";

/// LiveUpdate is used when a new module has been uploaded through the HTTP server capability provider. The bytes
/// contained in this message will, if valid, replace the existing guest module
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LiveUpdate {
    pub new_module: Vec<u8>,
}

/// A health request is passed to a guest module to allow it to return an empty result. If the guest module
/// returns the empty result, it is considered healthy
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct HealthRequest {
    pub placeholder: bool,
}

/// Capability providers must be able to accept configuration values on a per-module basis. The module
/// field will be the public key of the module (the subject field of its embedded JWT), though providers
/// should make no assumptions about the contents of that field.
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CapabilityConfiguration {
    pub module: String,
    pub values: HashMap<String, String>,
}
