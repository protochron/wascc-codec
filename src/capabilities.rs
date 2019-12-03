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

//! # Common types used for managing native capability providers

use std::error::Error;

use std::any::Any;

/// The dispatcher is used by a native capability provider to send commands to an actor module, expecting
/// a result containing a binary blob in return
pub trait Dispatcher: Any + Send + Sync {
    fn dispatch(&self, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// The NullDispatcher is as its name implies--a dispatcher that does nothing. This is convenient for
/// initializing a capability provider with a null dispatcher, and then swapping it for a real dispatcher
/// when the host runtime provides one tethered with the appropriate channels
#[derive(Default)]
pub struct NullDispatcher {}

impl NullDispatcher {
    pub fn new() -> NullDispatcher {
        NullDispatcher {}
    }
}

impl Dispatcher for NullDispatcher {
    fn dispatch(&self, _op: &str, _msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        unimplemented!()
    }
}

/// Every capability provider must implement this trait
pub trait CapabilityProvider: Any + Send + Sync {
    /// This function will be called on the provider when the host runtime is ready and has configured a dispatcher
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>>;
    /// The capability provider will return either one of the well-known capability IDs or a custom capability ID using `namespace:id` notation
    fn capability_id(&self) -> &'static str;
    /// The human-readable, friendly name of this capability provider. By convention, the provider should include its specific implementation,
    /// .e.g. include "Redis" in the name for a Redis-based capability provider.
    fn name(&self) -> &'static str;
    /// This function is called by the host runtime when an actor module is requesting a command be executed by the capability provider
    fn handle_call(&self, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Wraps a constructor inside an FFI function to allow the `CapabilityProvider` trait implementation
/// to be instantiated and used by the host runtime
#[macro_export]
macro_rules! capability_provider {
    ($provider_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn __capability_provider_create(
        ) -> *mut $crate::capabilities::CapabilityProvider {
            let constructor: fn() -> $provider_type = $constructor;
            let object = constructor();
            let boxed: Box<$crate::capabilities::CapabilityProvider> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
