#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [GameController](https://developer.apple.com/documentation/gamecontroller)
//! framework on macOS — enumerate connected gamepads and snapshot their
//! button / stick / trigger / dpad state.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod controller;
pub mod ffi;

pub use controller::{
    connected_controllers, first_controller_battery_level, first_controller_extras,
    rumble_first_controller, set_first_controller_light, set_first_controller_player_index,
    watch_connections, BatteryState, Buttons, ConnectionWatcher, Controller, ControllerExtras,
    Dpad, Thumbsticks, Triggers,
};

/// Common imports.
pub mod prelude {
    pub use crate::controller::{
        connected_controllers, first_controller_battery_level, first_controller_extras,
        rumble_first_controller, set_first_controller_light, set_first_controller_player_index,
        watch_connections, BatteryState, Buttons, ConnectionWatcher, Controller,
        ControllerExtras, Dpad, Thumbsticks, Triggers,
    };
}
