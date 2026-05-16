//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type DiscoveryCallback = unsafe extern "C" fn(user_info: *mut c_void);

#[repr(C)]
pub struct ControllerInfoRaw {
    pub vendor_name: *mut c_char,
    pub product_category: *mut c_char,
    pub player_index: i32,
    pub is_attached_to_device: bool,
    pub has_extended_gamepad: bool,

    pub button_a: f32,
    pub button_b: f32,
    pub button_x: f32,
    pub button_y: f32,

    pub left_shoulder: f32,
    pub right_shoulder: f32,
    pub left_trigger: f32,
    pub right_trigger: f32,

    pub menu_button: f32,
    pub options_button: f32,
    pub home_button: f32,

    pub left_thumbstick_x: f32,
    pub left_thumbstick_y: f32,
    pub right_thumbstick_x: f32,
    pub right_thumbstick_y: f32,

    pub dpad_up: f32,
    pub dpad_down: f32,
    pub dpad_left: f32,
    pub dpad_right: f32,
}

#[repr(C)]
pub struct ExtraInfoRaw {
    pub has_motion: bool,
    pub has_haptics: bool,
    pub has_light: bool,
    pub has_battery: bool,
    pub battery_level: f32,
    pub battery_state: i32,
    pub gravity_x: f64,
    pub gravity_y: f64,
    pub gravity_z: f64,
    pub user_acceleration_x: f64,
    pub user_acceleration_y: f64,
    pub user_acceleration_z: f64,
}

pub type ConnectionCallback = unsafe extern "C" fn(user_info: *mut c_void, connected: bool);

extern "C" {
    pub fn gc_string_free(s: *mut c_char);
    pub fn gc_connected_controllers(out_array: *mut *mut c_void, out_count: *mut usize) -> i32;
    pub fn gc_controller_infos_free(array: *mut c_void, count: usize);
    pub fn gc_controller_details_json(current_only: bool) -> *mut c_char;

    pub fn gc_first_controller_extra(out_info: *mut ExtraInfoRaw) -> bool;
    pub fn gc_register_connection_callback(
        callback: ConnectionCallback,
        user_info: *mut c_void,
    ) -> *mut c_void;
    pub fn gc_unregister_connection_callback(token: *mut c_void);
    pub fn gc_start_wireless_controller_discovery(
        callback: Option<DiscoveryCallback>,
        user_info: *mut c_void,
    );
    pub fn gc_stop_wireless_controller_discovery();
    pub fn gc_should_monitor_background_events() -> bool;
    pub fn gc_set_should_monitor_background_events(enabled: bool);

    pub fn gc_first_controller_set_light(red: f32, green: f32, blue: f32) -> bool;
    pub fn gc_first_controller_set_player_index(index: i32) -> bool;
    pub fn gc_first_controller_battery_level() -> f32;
    pub fn gc_first_controller_rumble(intensity: f32, sharpness: f32, duration: f64) -> bool;

    pub fn gc_mouse_is_connected() -> bool;
    pub fn gc_mouse_button_states(
        out_left: *mut bool,
        out_right: *mut bool,
        out_middle: *mut bool,
    ) -> bool;
    pub fn gc_keyboard_is_connected() -> bool;
    pub fn gc_keyboard_any_key_pressed() -> bool;
    pub fn gc_keyboard_is_key_pressed(keycode: isize) -> bool;
    pub fn gc_keyboard_snapshot_json() -> *mut c_char;
    pub fn gc_mouse_snapshot_json() -> *mut c_char;
    pub fn gc_event_view_controller_snapshot_json() -> *mut c_char;
    pub fn gc_racing_wheels_json() -> *mut c_char;

    pub fn gc_all_controllers_extras(out_buf: *mut ExtraInfoRaw, max: usize) -> usize;

    pub fn gc_dualsense_is_connected() -> bool;
    pub fn gc_dualsense_set_trigger_mode(
        which: i32,
        mode: i32,
        start_position: f32,
        end_position: f32,
        strength: f32,
        frequency: f32,
    ) -> bool;
}
