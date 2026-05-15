//! High-level `Controller` snapshot type.

use core::ffi::c_void;
use core::ptr;

use crate::ffi;

/// A point-in-time snapshot of one connected `GCController`.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub vendor_name: String,
    pub product_category: String,
    /// Player index assigned by the OS (0 for unassigned, then 1..=4).
    pub player_index: i32,
    /// True for built-in controllers like a `MacBook`'s Touch Bar (rare).
    pub is_attached_to_device: bool,
    /// True if this controller exposes the modern `extendedGamepad`
    /// profile (most Xbox / `DualShock` / `DualSense` / `MFi` pads do).
    pub has_extended_gamepad: bool,

    pub buttons: Buttons,
    pub triggers: Triggers,
    pub thumbsticks: Thumbsticks,
    pub dpad: Dpad,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Buttons {
    /// `0.0..=1.0` — analog pressure on Xbox-style buttons (`DualShock` /
    /// `DualSense` report 0/1 only).
    pub a: f32,
    pub b: f32,
    pub x: f32,
    pub y: f32,
    pub menu: f32,
    pub options: f32,
    pub home: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triggers {
    /// `0.0..=1.0` analog triggers + shoulder buttons.
    pub left_shoulder: f32,
    pub right_shoulder: f32,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Thumbsticks {
    /// `-1.0..=1.0` per axis.
    pub left_x: f32,
    pub left_y: f32,
    pub right_x: f32,
    pub right_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dpad {
    /// `0.0..=1.0` — pressed amount per direction (most controllers
    /// report 0/1, but some report fractional pressure).
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32,
}

/// Snapshot every connected controller right now.
#[must_use]
pub fn connected_controllers() -> Vec<Controller> {
    let mut array: *mut c_void = ptr::null_mut();
    let mut count: usize = 0;
    let status = unsafe { ffi::gc_connected_controllers(&mut array, &mut count) };
    if status != 0 || array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::ControllerInfoRaw>();
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let raw = unsafe { &*typed.add(i) };
        out.push(Controller {
            vendor_name: take_string(raw.vendor_name),
            product_category: take_string(raw.product_category),
            player_index: raw.player_index,
            is_attached_to_device: raw.is_attached_to_device,
            has_extended_gamepad: raw.has_extended_gamepad,
            buttons: Buttons {
                a: raw.button_a,
                b: raw.button_b,
                x: raw.button_x,
                y: raw.button_y,
                menu: raw.menu_button,
                options: raw.options_button,
                home: raw.home_button,
            },
            triggers: Triggers {
                left_shoulder: raw.left_shoulder,
                right_shoulder: raw.right_shoulder,
                left_trigger: raw.left_trigger,
                right_trigger: raw.right_trigger,
            },
            thumbsticks: Thumbsticks {
                left_x: raw.left_thumbstick_x,
                left_y: raw.left_thumbstick_y,
                right_x: raw.right_thumbstick_x,
                right_y: raw.right_thumbstick_y,
            },
            dpad: Dpad {
                up: raw.dpad_up,
                down: raw.dpad_down,
                left: raw.dpad_left,
                right: raw.dpad_right,
            },
        });
    }
    unsafe { ffi::gc_controller_infos_free(array, count) };
    out
}

fn take_string(p: *mut core::ffi::c_char) -> String {
    if p.is_null() {
        return String::new();
    }
    let s = unsafe { core::ffi::CStr::from_ptr(p) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::gc_string_free(p) };
    s
}
