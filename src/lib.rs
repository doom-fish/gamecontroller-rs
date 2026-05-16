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

pub mod controller;
pub mod error;
pub mod ffi;

pub use controller::{
    all_controller_extras, connected_controller_details, connected_controllers,
    current_controller_details, current_controller_snapshot, dualsense_is_connected,
    dualsense_trigger_feedback, dualsense_trigger_off, dualsense_trigger_slope_feedback,
    dualsense_trigger_vibration, dualsense_trigger_weapon, first_controller_battery_level,
    first_controller_extras, keyboard_any_key_pressed, keyboard_is_connected,
    keyboard_is_key_pressed, mouse_button_states, mouse_is_connected,
    rumble_first_controller, set_first_controller_light,
    set_first_controller_player_index, set_should_monitor_background_events,
    should_monitor_background_events, start_wireless_controller_discovery,
    start_wireless_controller_discovery_with_callback, stop_wireless_controller_discovery,
    watch_connections, BatteryInfo, BatteryState, ButtonInputState, Buttons,
    ConnectionWatcher, Controller, ControllerDetails, ControllerExtras,
    DirectionPadInputState, Dpad, DualSenseAdaptiveTriggerState,
    DualSenseTrigger, DualSenseTriggerMode, DualSenseTriggerStatus, ExtendedGamepadDetails,
    GamepadDetails, MotionDetails, MouseButtons, MicroGamepadDetails,
    NamedAxisInputState, NamedButtonInputState, NamedDirectionPadState,
    NamedTouchpadState, PhysicalInputProfileDetails, Quaternion, Thumbsticks,
    TouchState, TouchpadDetails, Triggers, Vector3,
};
pub use error::GameControllerError;

/// Common imports.
pub mod prelude {
    pub use crate::controller::{
        all_controller_extras, connected_controller_details, connected_controllers,
        current_controller_details, current_controller_snapshot, dualsense_is_connected,
        dualsense_trigger_feedback, dualsense_trigger_off, dualsense_trigger_slope_feedback,
        dualsense_trigger_vibration, dualsense_trigger_weapon, first_controller_battery_level,
        first_controller_extras, keyboard_any_key_pressed, keyboard_is_connected,
        keyboard_is_key_pressed, mouse_button_states, mouse_is_connected,
        rumble_first_controller, set_first_controller_light,
        set_first_controller_player_index, set_should_monitor_background_events,
        should_monitor_background_events, start_wireless_controller_discovery,
        start_wireless_controller_discovery_with_callback, stop_wireless_controller_discovery,
        watch_connections, BatteryInfo, BatteryState, ButtonInputState, Buttons,
        ConnectionWatcher, Controller, ControllerDetails, ControllerExtras,
        DirectionPadInputState, Dpad, DualSenseAdaptiveTriggerState,
        DualSenseTrigger, DualSenseTriggerMode, DualSenseTriggerStatus, ExtendedGamepadDetails,
        GamepadDetails, MotionDetails, MicroGamepadDetails, MouseButtons,
        NamedAxisInputState, NamedButtonInputState, NamedDirectionPadState,
        NamedTouchpadState, PhysicalInputProfileDetails, Quaternion, Thumbsticks,
        TouchState, TouchpadDetails, Triggers, Vector3,
    };
    pub use crate::error::GameControllerError;
}
