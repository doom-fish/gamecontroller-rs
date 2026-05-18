//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

/// Mirrors the `GameController` framework counterpart for `DiscoveryCallback`.
pub type DiscoveryCallback = unsafe extern "C" fn(user_info: *mut c_void);
/// Mirrors the `GameController` framework counterpart for `NotificationCallback`.
pub type NotificationCallback = unsafe extern "C" fn(user_info: *mut c_void);

/// Mirrors the `GameController` framework counterpart for `ControllerInfoRaw`.
#[repr(C)]
pub struct ControllerInfoRaw {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: *mut c_char,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: *mut c_char,
/// Mirrors the `GameController` framework property for `player_index`.
    pub player_index: i32,
/// Mirrors the `GameController` framework property for `is_attached_to_device`.
    pub is_attached_to_device: bool,
/// Mirrors the `GameController` framework property for `has_extended_gamepad`.
    pub has_extended_gamepad: bool,

/// Mirrors the `GameController` framework property for `button_a`.
    pub button_a: f32,
/// Mirrors the `GameController` framework property for `button_b`.
    pub button_b: f32,
/// Mirrors the `GameController` framework property for `button_x`.
    pub button_x: f32,
/// Mirrors the `GameController` framework property for `button_y`.
    pub button_y: f32,

/// Mirrors the `GameController` framework property for `left_shoulder`.
    pub left_shoulder: f32,
/// Mirrors the `GameController` framework property for `right_shoulder`.
    pub right_shoulder: f32,
/// Mirrors the `GameController` framework property for `left_trigger`.
    pub left_trigger: f32,
/// Mirrors the `GameController` framework property for `right_trigger`.
    pub right_trigger: f32,

/// Mirrors the `GameController` framework property for `menu_button`.
    pub menu_button: f32,
/// Mirrors the `GameController` framework property for `options_button`.
    pub options_button: f32,
/// Mirrors the `GameController` framework property for `home_button`.
    pub home_button: f32,

/// Mirrors the `GameController` framework property for `left_thumbstick_x`.
    pub left_thumbstick_x: f32,
/// Mirrors the `GameController` framework property for `left_thumbstick_y`.
    pub left_thumbstick_y: f32,
/// Mirrors the `GameController` framework property for `right_thumbstick_x`.
    pub right_thumbstick_x: f32,
/// Mirrors the `GameController` framework property for `right_thumbstick_y`.
    pub right_thumbstick_y: f32,

/// Mirrors the `GameController` framework property for `dpad_up`.
    pub dpad_up: f32,
/// Mirrors the `GameController` framework property for `dpad_down`.
    pub dpad_down: f32,
/// Mirrors the `GameController` framework property for `dpad_left`.
    pub dpad_left: f32,
/// Mirrors the `GameController` framework property for `dpad_right`.
    pub dpad_right: f32,
}

/// Mirrors the `GameController` framework counterpart for `ExtraInfoRaw`.
#[repr(C)]
pub struct ExtraInfoRaw {
/// Mirrors the `GameController` framework property for `has_motion`.
    pub has_motion: bool,
/// Mirrors the `GameController` framework property for `has_haptics`.
    pub has_haptics: bool,
/// Mirrors the `GameController` framework property for `has_light`.
    pub has_light: bool,
/// Mirrors the `GameController` framework property for `has_battery`.
    pub has_battery: bool,
/// Mirrors the `GameController` framework property for `battery_level`.
    pub battery_level: f32,
/// Mirrors the `GameController` framework property for `battery_state`.
    pub battery_state: i32,
/// Mirrors the `GameController` framework property for `gravity_x`.
    pub gravity_x: f64,
/// Mirrors the `GameController` framework property for `gravity_y`.
    pub gravity_y: f64,
/// Mirrors the `GameController` framework property for `gravity_z`.
    pub gravity_z: f64,
/// Mirrors the `GameController` framework property for `user_acceleration_x`.
    pub user_acceleration_x: f64,
/// Mirrors the `GameController` framework property for `user_acceleration_y`.
    pub user_acceleration_y: f64,
/// Mirrors the `GameController` framework property for `user_acceleration_z`.
    pub user_acceleration_z: f64,
}

/// Mirrors the `GameController` framework counterpart for `ConnectionCallback`.
pub type ConnectionCallback = unsafe extern "C" fn(user_info: *mut c_void, connected: bool);

extern "C" {
/// Calls the `GameController` framework counterpart for `gc_string_free`.
    pub fn gc_string_free(s: *mut c_char);
/// Calls the `GameController` framework counterpart for `gc_connected_controllers`.
    pub fn gc_connected_controllers(out_array: *mut *mut c_void, out_count: *mut usize) -> i32;
/// Calls the `GameController` framework counterpart for `gc_controller_infos_free`.
    pub fn gc_controller_infos_free(array: *mut c_void, count: usize);
/// Calls the `GameController` framework counterpart for `gc_controller_details_json`.
    pub fn gc_controller_details_json(current_only: bool) -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_current_controller_input_source_json`.
    pub fn gc_current_controller_input_source_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_connected_devices_json`.
    pub fn gc_connected_devices_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_current_controller_elements_json`.
    pub fn gc_current_controller_elements_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_current_controller_physical_input_elements_json`.
    pub fn gc_current_controller_physical_input_elements_json() -> *mut c_char;

/// Calls the `GameController` framework counterpart for `gc_first_controller_extra`.
    pub fn gc_first_controller_extra(out_info: *mut ExtraInfoRaw) -> bool;
/// Calls the `GameController` framework counterpart for `gc_register_connection_callback`.
    pub fn gc_register_connection_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_register_controller_current_callback`.
    pub fn gc_register_controller_current_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_register_keyboard_connection_callback`.
    pub fn gc_register_keyboard_connection_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_register_mouse_connection_callback`.
    pub fn gc_register_mouse_connection_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_register_mouse_current_callback`.
    pub fn gc_register_mouse_current_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_register_racing_wheel_connection_callback`.
    pub fn gc_register_racing_wheel_connection_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_unregister_connection_callback`.
    pub fn gc_unregister_connection_callback(token: *mut c_void);
/// Calls the `GameController` framework counterpart for `gc_register_controller_customizations_callback`.
    pub fn gc_register_controller_customizations_callback(
        callback: NotificationCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
/// Calls the `GameController` framework counterpart for `gc_unregister_notification_callback`.
    pub fn gc_unregister_notification_callback(token: *mut c_void);
/// Calls the `GameController` framework counterpart for `gc_start_wireless_controller_discovery`.
    pub fn gc_start_wireless_controller_discovery(
        callback: Option<DiscoveryCallback>,
        user_info: *mut c_void,
    );
/// Calls the `GameController` framework counterpart for `gc_stop_wireless_controller_discovery`.
    pub fn gc_stop_wireless_controller_discovery();
/// Calls the `GameController` framework counterpart for `gc_should_monitor_background_events`.
    pub fn gc_should_monitor_background_events() -> bool;
/// Calls the `GameController` framework counterpart for `gc_set_should_monitor_background_events`.
    pub fn gc_set_should_monitor_background_events(enabled: bool);

/// Calls the `GameController` framework counterpart for `gc_first_controller_set_light`.
    pub fn gc_first_controller_set_light(red: f32, green: f32, blue: f32) -> bool;
/// Calls the `GameController` framework counterpart for `gc_first_controller_set_player_index`.
    pub fn gc_first_controller_set_player_index(index: i32) -> bool;
/// Calls the `GameController` framework counterpart for `gc_first_controller_battery_level`.
    pub fn gc_first_controller_battery_level() -> f32;
/// Calls the `GameController` framework counterpart for `gc_first_controller_rumble`.
    pub fn gc_first_controller_rumble(intensity: f32, sharpness: f32, duration: f64) -> bool;
/// Calls the `GameController` framework counterpart for `gc_first_controller_rumble_with_locality`.
    pub fn gc_first_controller_rumble_with_locality(
        locality: *const c_char,
        intensity: f32,
        sharpness: f32,
        duration: f64,
    ) -> bool;

/// Calls the `GameController` framework counterpart for `gc_mouse_is_connected`.
    pub fn gc_mouse_is_connected() -> bool;
/// Calls the `GameController` framework counterpart for `gc_mouse_button_states`.
    pub fn gc_mouse_button_states(
        out_left: *mut bool,
        out_right: *mut bool,
        out_middle: *mut bool,
    ) -> bool;
/// Calls the `GameController` framework counterpart for `gc_keyboard_is_connected`.
    pub fn gc_keyboard_is_connected() -> bool;
/// Calls the `GameController` framework counterpart for `gc_keyboard_any_key_pressed`.
    pub fn gc_keyboard_any_key_pressed() -> bool;
/// Calls the `GameController` framework counterpart for `gc_keyboard_is_key_pressed`.
    pub fn gc_keyboard_is_key_pressed(keycode: isize) -> bool;
/// Calls the `GameController` framework counterpart for `gc_keyboard_snapshot_json`.
    pub fn gc_keyboard_snapshot_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_mouse_snapshot_json`.
    pub fn gc_mouse_snapshot_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_event_view_controller_snapshot_json`.
    pub fn gc_event_view_controller_snapshot_json() -> *mut c_char;
/// Calls the `GameController` framework counterpart for `gc_racing_wheels_json`.
    pub fn gc_racing_wheels_json() -> *mut c_char;

/// Calls the `GameController` framework counterpart for `gc_all_controllers_extras`.
    pub fn gc_all_controllers_extras(out_buf: *mut ExtraInfoRaw, max: usize) -> usize;

/// Calls the `GameController` framework counterpart for `gc_dualsense_is_connected`.
    pub fn gc_dualsense_is_connected() -> bool;
/// Calls the `GameController` framework counterpart for `gc_dualsense_set_trigger_mode`.
    pub fn gc_dualsense_set_trigger_mode(
        which: i32,
        mode: i32,
        start_position: f32,
        end_position: f32,
        strength: f32,
        frequency: f32,
    ) -> bool;
/// Calls the `GameController` framework counterpart for `gc_dualsense_set_trigger_feedback_resistive_strengths`.
    pub fn gc_dualsense_set_trigger_feedback_resistive_strengths(
        which: i32,
        values: *const f32,
        len: usize,
    ) -> bool;
/// Calls the `GameController` framework counterpart for `gc_dualsense_set_trigger_vibration_amplitudes`.
    pub fn gc_dualsense_set_trigger_vibration_amplitudes(
        which: i32,
        values: *const f32,
        len: usize,
        frequency: f32,
    ) -> bool;
}
