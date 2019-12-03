![crates.io](https://img.shields.io/crates/v/wascc-codec.svg)&nbsp;
![travis](https://travis-ci.org/wascc/wascc-codec.svg?branch=master)&nbsp;
![license](https://img.shields.io/crates/l/wascc-codec.svg)

# waSCC Codec

The _WebAssembly Secure Capabilities Connector_ (waSCC) codec library contains a set of types, protobuf messages, and other primitives that are common to the host runtime, capability providers, and actor modules that are created for use with the [wascc](https://wasc.dev) host runtime, which is in turn built on top of [WebAssembly Procedure Call (waPC)](https://github.com/wapc) primitives.

This crate includes definitions for the well-known messages and operations  that **wascc** supports by default, such as **messaging**, **key-value store**, and **HTTP client/server**.

