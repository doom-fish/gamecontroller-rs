//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

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

extern "C" {
    pub fn gc_string_free(s: *mut c_char);
    pub fn gc_connected_controllers(
        out_array: *mut *mut c_void,
        out_count: *mut usize,
    ) -> i32;
    pub fn gc_controller_infos_free(array: *mut c_void, count: usize);
}
