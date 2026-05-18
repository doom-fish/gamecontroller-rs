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

/// Groups `GameController` framework constants for `async_api`.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
/// Groups `GameController` framework constants for `controller`.
pub mod controller;
/// Groups `GameController` framework constants for `error`.
pub mod error;
/// Groups `GameController` framework constants for `ffi`.
pub mod ffi;

/// Re-exports the `GameController` framework surface for this item.
pub use controller::*;
/// Re-exports the `GameController` framework surface for this item.
pub use error::GameControllerError;

/// Common imports.
pub mod prelude {
/// Re-exports the `GameController` framework surface for this item.
    pub use crate::controller::*;
/// Re-exports the `GameController` framework surface for this item.
    pub use crate::error::GameControllerError;
}
