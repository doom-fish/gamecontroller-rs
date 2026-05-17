//! High-level `Controller` snapshot type.

mod constants;
mod details;

use core::ffi::c_void;
use core::ptr;
use std::{ffi::CString, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::ffi;

pub use constants::*;
pub use details::*;

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

// ---- v0.2: motion / battery / haptics / light snapshots ----

/// Battery charging state mirroring `GCDeviceBattery.batteryState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BatteryState {
    Unknown,
    Discharging,
    Charging,
    Full,
}

/// Optional services available on a controller. Returned by [`first_controller_extras`].
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerExtras {
    pub has_motion: bool,
    pub has_haptics: bool,
    pub has_light: bool,
    pub has_battery: bool,
    /// `0.0..=1.0`, or `None` if no battery is exposed.
    pub battery_level: Option<f32>,
    pub battery_state: BatteryState,
    /// Gravity vector (x, y, z) in g-units, or `None` if no motion.
    pub gravity: Option<(f64, f64, f64)>,
    /// User-acceleration vector (x, y, z) in g-units, or `None`.
    pub user_acceleration: Option<(f64, f64, f64)>,
}

/// Snapshot the first connected controller's optional services
/// (motion / battery / haptics / light), or `None` if no controller is
/// connected.
#[must_use]
pub fn first_controller_extras() -> Option<ControllerExtras> {
    let mut raw = empty_extras_raw();
    let ok = unsafe { ffi::gc_first_controller_extra(&mut raw) };
    if !ok {
        return None;
    }
    Some(extras_from_raw(&raw))
}

/// Snapshot extras for ALL connected controllers (motion / battery /
/// haptics / light per controller).
#[must_use]
pub fn all_controller_extras() -> Vec<ControllerExtras> {
    const MAX: usize = 8;
    let mut buf: Vec<ffi::ExtraInfoRaw> = (0..MAX).map(|_| empty_extras_raw()).collect();
    let n = unsafe { ffi::gc_all_controllers_extras(buf.as_mut_ptr(), MAX) };
    buf.truncate(n);
    buf.iter().map(extras_from_raw).collect()
}

const fn empty_extras_raw() -> ffi::ExtraInfoRaw {
    ffi::ExtraInfoRaw {
        has_motion: false,
        has_haptics: false,
        has_light: false,
        has_battery: false,
        battery_level: -1.0,
        battery_state: 0,
        gravity_x: 0.0,
        gravity_y: 0.0,
        gravity_z: 0.0,
        user_acceleration_x: 0.0,
        user_acceleration_y: 0.0,
        user_acceleration_z: 0.0,
    }
}

const fn extras_from_raw(raw: &ffi::ExtraInfoRaw) -> ControllerExtras {
    let battery_state = match raw.battery_state {
        1 => BatteryState::Discharging,
        2 => BatteryState::Charging,
        3 => BatteryState::Full,
        _ => BatteryState::Unknown,
    };
    ControllerExtras {
        has_motion: raw.has_motion,
        has_haptics: raw.has_haptics,
        has_light: raw.has_light,
        has_battery: raw.has_battery,
        battery_level: if raw.has_battery {
            Some(raw.battery_level)
        } else {
            None
        },
        battery_state,
        gravity: if raw.has_motion {
            Some((raw.gravity_x, raw.gravity_y, raw.gravity_z))
        } else {
            None
        },
        user_acceleration: if raw.has_motion {
            Some((
                raw.user_acceleration_x,
                raw.user_acceleration_y,
                raw.user_acceleration_z,
            ))
        } else {
            None
        },
    }
}

/// True if a mouse is currently connected (macOS 11+).
#[must_use]
pub fn mouse_is_connected() -> bool {
    unsafe { ffi::gc_mouse_is_connected() }
}

/// One-shot snapshot of the current mouse button states. Returns
/// `None` if no mouse is connected.
#[must_use]
pub fn mouse_button_states() -> Option<MouseButtons> {
    let mut l = false;
    let mut r = false;
    let mut m = false;
    let ok = unsafe { ffi::gc_mouse_button_states(&mut l, &mut r, &mut m) };
    if !ok {
        return None;
    }
    Some(MouseButtons {
        left: l,
        right: r,
        middle: m,
    })
}

/// True if a keyboard is currently connected (macOS 11+).
#[must_use]
pub fn keyboard_is_connected() -> bool {
    unsafe { ffi::gc_keyboard_is_connected() }
}

/// True if ANY key is currently pressed on the coalesced keyboard.
#[must_use]
pub fn keyboard_any_key_pressed() -> bool {
    unsafe { ffi::gc_keyboard_any_key_pressed() }
}

/// True if the HID-page-7 `keycode` is currently pressed
/// (e.g. `4 = "a"`, `40 = enter`, `44 = space`).
#[must_use]
pub fn keyboard_is_key_pressed(keycode: isize) -> bool {
    unsafe { ffi::gc_keyboard_is_key_pressed(keycode) }
}

/// Snapshot of the three primary mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

// ---- v0.2: connect/disconnect callbacks ----

/// RAII guard for a registered bool-state notification callback. Drops the
/// `NSNotificationCenter` observer on scope exit.
pub struct ConnectionWatcher {
    token: *mut core::ffi::c_void,
    _callback: Box<Box<dyn Fn(bool) + Send + Sync + 'static>>,
}

/// RAII guard for a registered notification callback without payload data.
pub struct NotificationWatcher {
    token: *mut core::ffi::c_void,
    _callback: Box<Box<dyn Fn() + Send + Sync + 'static>>,
}

unsafe impl Send for ConnectionWatcher {}
unsafe impl Sync for ConnectionWatcher {}
unsafe impl Send for NotificationWatcher {}
unsafe impl Sync for NotificationWatcher {}

static DISCOVERY_CALLBACK: Mutex<Option<Box<dyn FnOnce() + Send + 'static>>> = Mutex::new(None);

impl Drop for ConnectionWatcher {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::gc_unregister_connection_callback(self.token) };
            self.token = core::ptr::null_mut();
        }
    }
}

impl Drop for NotificationWatcher {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::gc_unregister_notification_callback(self.token) };
            self.token = core::ptr::null_mut();
        }
    }
}

unsafe extern "C" fn connection_trampoline(user_info: *mut core::ffi::c_void, value: bool) {
    let cb_ptr = user_info.cast::<Box<dyn Fn(bool) + Send + Sync + 'static>>();
    if cb_ptr.is_null() {
        return;
    }
    let cb = unsafe { &*cb_ptr };
    cb(value);
}

unsafe extern "C" fn notification_trampoline(user_info: *mut core::ffi::c_void) {
    let cb_ptr = user_info.cast::<Box<dyn Fn() + Send + Sync + 'static>>();
    if cb_ptr.is_null() {
        return;
    }
    let cb = unsafe { &*cb_ptr };
    cb();
}

fn watch_connection_with<F>(
    register: unsafe extern "C" fn(ffi::ConnectionCallback, *mut c_void) -> *mut c_void,
    callback: F,
) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    let boxed: Box<dyn Fn(bool) + Send + Sync + 'static> = Box::new(callback);
    let raw_box = Box::into_raw(Box::new(boxed));
    let token = unsafe { register(connection_trampoline, raw_box.cast::<c_void>()) };
    ConnectionWatcher {
        token,
        _callback: unsafe { Box::from_raw(raw_box) },
    }
}

fn watch_notification_with<F>(
    register: unsafe extern "C" fn(ffi::NotificationCallback, *mut c_void) -> *mut c_void,
    callback: F,
) -> NotificationWatcher
where
    F: Fn() + Send + Sync + 'static,
{
    let boxed: Box<dyn Fn() + Send + Sync + 'static> = Box::new(callback);
    let raw_box = Box::into_raw(Box::new(boxed));
    let token = unsafe { register(notification_trampoline, raw_box.cast::<c_void>()) };
    NotificationWatcher {
        token,
        _callback: unsafe { Box::from_raw(raw_box) },
    }
}

/// Register a closure that fires when any controller connects (`true`)
/// or disconnects (`false`). The returned [`ConnectionWatcher`] guards
/// the registration — drop it to stop receiving notifications.
///
/// Callbacks fire on the main run loop's queue, so make sure your app
/// has an active run loop (`CFRunLoopRun`, `NSApplication.run`, or
/// Carbon `RunApplicationEventLoop`).
#[must_use]
pub fn watch_connections<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_connection_callback, callback)
}

/// Register a closure that fires when `GCController.current` becomes available
/// (`true`) or stops being current (`false`).
#[must_use]
pub fn watch_current_controller<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_controller_current_callback, callback)
}

/// Register a closure that fires when keyboards connect (`true`) or disconnect (`false`).
#[must_use]
pub fn watch_keyboard_connections<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_keyboard_connection_callback, callback)
}

/// Register a closure that fires when mice connect (`true`) or disconnect (`false`).
#[must_use]
pub fn watch_mouse_connections<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_mouse_connection_callback, callback)
}

/// Register a closure that fires when the current mouse changes (`true` = became current).
#[must_use]
pub fn watch_mouse_current<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_mouse_current_callback, callback)
}

/// Register a closure that fires when racing wheels connect (`true`) or disconnect (`false`).
#[must_use]
pub fn watch_racing_wheel_connections<F>(callback: F) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    watch_connection_with(ffi::gc_register_racing_wheel_connection_callback, callback)
}

/// Register a closure that fires when controller user customizations change.
#[must_use]
pub fn watch_controller_customizations<F>(callback: F) -> NotificationWatcher
where
    F: Fn() + Send + Sync + 'static,
{
    watch_notification_with(
        ffi::gc_register_controller_customizations_callback,
        callback,
    )
}

unsafe extern "C" fn discovery_trampoline(_user_info: *mut c_void) {
    if let Ok(mut slot) = DISCOVERY_CALLBACK.lock() {
        if let Some(callback) = slot.take() {
            callback();
        }
    }
}

/// Start `GameController`'s wireless discovery flow.
///
/// Apple delivers the completion handler on the main run loop once the current
/// discovery session finishes. Use [`watch_connections`] if you also want live
/// connect and disconnect notifications during discovery.
pub fn start_wireless_controller_discovery() {
    if let Ok(mut slot) = DISCOVERY_CALLBACK.lock() {
        *slot = None;
    }
    unsafe { ffi::gc_start_wireless_controller_discovery(None, ptr::null_mut()) }
}

/// Start wireless controller discovery and invoke `callback` once the discovery
/// session's completion handler fires.
pub fn start_wireless_controller_discovery_with_callback<F>(callback: F)
where
    F: FnOnce() + Send + 'static,
{
    if let Ok(mut slot) = DISCOVERY_CALLBACK.lock() {
        *slot = Some(Box::new(callback));
    }
    unsafe {
        ffi::gc_start_wireless_controller_discovery(Some(discovery_trampoline), ptr::null_mut());
    }
}

/// Stop the current wireless controller discovery session, if any.
pub fn stop_wireless_controller_discovery() {
    if let Ok(mut slot) = DISCOVERY_CALLBACK.lock() {
        *slot = None;
    }
    unsafe { ffi::gc_stop_wireless_controller_discovery() }
}

/// Whether `GameController` should keep routing controller events while your app
/// is in the background.
#[must_use]
pub fn should_monitor_background_events() -> bool {
    unsafe { ffi::gc_should_monitor_background_events() }
}

/// Control whether `GameController` keeps routing controller events while your
/// app is in the background.
pub fn set_should_monitor_background_events(enabled: bool) {
    unsafe { ffi::gc_set_should_monitor_background_events(enabled) }
}

/// Set the light bar / status LED color on the first connected
/// controller (`DualSense`, `DualShock`). Each channel is 0.0..=1.0.
/// Returns `false` if no controller is connected or it has no light.
#[must_use]
pub fn set_first_controller_light(red: f32, green: f32, blue: f32) -> bool {
    unsafe { ffi::gc_first_controller_set_light(red, green, blue) }
}

/// Set the light bar / status LED color on the first connected controller using
/// a typed [`Color`] value.
#[must_use]
pub fn set_first_controller_light_color(color: Color) -> bool {
    set_first_controller_light(color.red, color.green, color.blue)
}

/// Assign the player index (1..=4) on the first connected controller.
///
/// This typically lights up the corresponding LED on consoles or sets
/// the player slot on Xbox / `DualSense`. Returns `false` if no
/// controller or the index is out of range.
#[must_use]
pub fn set_first_controller_player_index(index: i32) -> bool {
    unsafe { ffi::gc_first_controller_set_player_index(index) }
}

/// Read the battery level (0.0..=1.0) of the first connected
/// controller. Returns `-1.0` if the controller has no battery
/// (wired) or none is connected.
#[must_use]
pub fn first_controller_battery_level() -> f32 {
    unsafe { ffi::gc_first_controller_battery_level() }
}

/// Play a simple continuous haptic on the first controller for
/// `duration` seconds. `intensity` and `sharpness` are 0.0..=1.0.
///
/// Returns `false` if no controller, no haptic support, or Core
/// Haptics failed to start.
#[must_use]
pub fn rumble_first_controller(intensity: f32, sharpness: f32, duration: f64) -> bool {
    unsafe { ffi::gc_first_controller_rumble(intensity, sharpness, duration) }
}

/// Play a continuous haptic on the first controller using a specific haptics locality.
#[must_use]
pub fn rumble_first_controller_with_locality(
    locality: GCHapticsLocality,
    intensity: f32,
    sharpness: f32,
    duration: f64,
) -> bool {
    let Ok(locality) = CString::new(locality.as_str()) else {
        return false;
    };
    unsafe {
        ffi::gc_first_controller_rumble_with_locality(
            locality.as_ptr(),
            intensity,
            sharpness,
            duration,
        )
    }
}

/// Which `DualSense` trigger to address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualSenseTrigger {
    Left = 0,
    Right = 1,
}

/// True if a `DualSense` controller is currently connected.
#[must_use]
pub fn dualsense_is_connected() -> bool {
    unsafe { ffi::gc_dualsense_is_connected() }
}

/// Disable adaptive resistance/vibration on the requested `DualSense`
/// trigger. Wraps `setModeOff`.
#[must_use]
pub fn dualsense_trigger_off(which: DualSenseTrigger) -> bool {
    unsafe { ffi::gc_dualsense_set_trigger_mode(which as i32, 0, 0.0, 0.0, 0.0, 0.0) }
}

/// Apply a resistive "feedback" mode on a `DualSense` trigger.
/// `start_position` is `0.0..=1.0` (when resistance kicks in),
/// `strength` is `0.0..=1.0`.
#[must_use]
pub fn dualsense_trigger_feedback(
    which: DualSenseTrigger,
    start_position: f32,
    strength: f32,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_mode(which as i32, 1, start_position, 0.0, strength, 0.0)
    }
}

/// Apply a "weapon" mode (resist, then snap) on a `DualSense` trigger.
/// All inputs `0.0..=1.0`.
#[must_use]
pub fn dualsense_trigger_weapon(
    which: DualSenseTrigger,
    start_position: f32,
    end_position: f32,
    strength: f32,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_mode(
            which as i32,
            2,
            start_position,
            end_position,
            strength,
            0.0,
        )
    }
}

/// Apply a vibration mode on a `DualSense` trigger. `frequency` is in
/// Hz (typically `1.0..=100.0`); `amplitude` is `0.0..=1.0`.
#[must_use]
pub fn dualsense_trigger_vibration(
    which: DualSenseTrigger,
    start_position: f32,
    amplitude: f32,
    frequency: f32,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_mode(
            which as i32,
            3,
            start_position,
            0.0,
            amplitude,
            frequency,
        )
    }
}

/// Apply slope feedback on a `DualSense` trigger, varying the force between two
/// strength values across a trigger range.
#[must_use]
pub fn dualsense_trigger_slope_feedback(
    which: DualSenseTrigger,
    start_position: f32,
    end_position: f32,
    start_strength: f32,
    end_strength: f32,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_mode(
            which as i32,
            4,
            start_position,
            end_position,
            start_strength,
            end_strength,
        )
    }
}

/// Apply per-position resistive strengths to a `DualSense` trigger.
#[must_use]
pub fn dualsense_trigger_feedback_resistive_strengths(
    which: DualSenseTrigger,
    strengths: GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_feedback_resistive_strengths(
            which as i32,
            strengths.values.as_ptr(),
            strengths.values.len(),
        )
    }
}

/// Apply per-position vibration amplitudes to a `DualSense` trigger.
#[must_use]
pub fn dualsense_trigger_vibration_amplitudes(
    which: DualSenseTrigger,
    amplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes,
    frequency: f32,
) -> bool {
    unsafe {
        ffi::gc_dualsense_set_trigger_vibration_amplitudes(
            which as i32,
            amplitudes.values.as_ptr(),
            amplitudes.values.len(),
            frequency,
        )
    }
}
