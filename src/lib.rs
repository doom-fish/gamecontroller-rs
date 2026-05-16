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
    all_controller_extras, connected_controllers, dualsense_is_connected,
    dualsense_trigger_feedback, dualsense_trigger_off, dualsense_trigger_vibration,
    dualsense_trigger_weapon, first_controller_battery_level, first_controller_extras,
    keyboard_any_key_pressed, keyboard_is_connected, keyboard_is_key_pressed,
    mouse_button_states, mouse_is_connected, rumble_first_controller,
    set_first_controller_light, set_first_controller_player_index, watch_connections,
    BatteryState, Buttons, ConnectionWatcher, Controller, ControllerExtras, Dpad,
    DualSenseTrigger, MouseButtons, Thumbsticks, Triggers,
};

/// Common imports.
pub mod prelude {
    pub use crate::controller::{
        all_controller_extras, connected_controllers, dualsense_is_connected,
        dualsense_trigger_feedback, dualsense_trigger_off, dualsense_trigger_vibration,
        dualsense_trigger_weapon, first_controller_battery_level, first_controller_extras,
        keyboard_any_key_pressed, keyboard_is_connected, keyboard_is_key_pressed,
        mouse_button_states, mouse_is_connected, rumble_first_controller,
        set_first_controller_light, set_first_controller_player_index, watch_connections,
        BatteryState, Buttons, ConnectionWatcher, Controller, ControllerExtras, Dpad,
        DualSenseTrigger, MouseButtons, Thumbsticks, Triggers,
    };
}
