//! Application use cases and interface-neutral ports.
//!
//! Interfaces call this layer. Infrastructure adapters implement the ports
//! defined here. Transport and presentation concerns do not belong here.

#![forbid(unsafe_code)]

pub use zashikura_core as core;
