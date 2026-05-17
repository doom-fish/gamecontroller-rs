#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [GameController](https://developer.apple.com/documentation/gamecontroller)
//! framework on macOS — enumerate controllers, inspect current/profile state,
//! and drive richer controller-specific helpers such as `DualSense` triggers.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
pub mod controller;
pub mod error;
pub mod ffi;

pub use controller::*;
pub use error::GameControllerError;

/// Common imports.
pub mod prelude {
    pub use crate::controller::*;
    pub use crate::error::GameControllerError;
}
