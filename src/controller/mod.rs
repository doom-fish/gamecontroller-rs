//! High-level `Controller` snapshot type.

mod constants;
mod details;
mod handle;

use core::ffi::c_void;
use core::ptr;
use std::sync::Mutex;

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic;
use serde::{Deserialize, Serialize};

use crate::ffi;

/// Re-exports the `GameController` framework surface for this item.
pub use constants::*;
/// Re-exports the `GameController` framework surface for this item.
pub use details::*;
pub use handle::{ControllerHandle, DualSenseTriggerEffect};

/// A point-in-time snapshot of one connected `GCController`.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
    /// Player index assigned by the OS (0 for unassigned, then 1..=4).
    pub player_index: i32,
    /// True for built-in controllers like a `MacBook`'s Touch Bar (rare).
    pub is_attached_to_device: bool,
    /// True if this controller exposes the modern `extendedGamepad`
    /// profile (most Xbox / `DualShock` / `DualSense` / `MFi` pads do).
    pub has_extended_gamepad: bool,

/// Mirrors the `GameController` framework property for `buttons`.
    pub buttons: Buttons,
/// Mirrors the `GameController` framework property for `triggers`.
    pub triggers: Triggers,
/// Mirrors the `GameController` framework property for `thumbsticks`.
    pub thumbsticks: Thumbsticks,
/// Mirrors the `GameController` framework property for `dpad`.
    pub dpad: Dpad,
}

/// Mirrors the `GameController` framework counterpart for `Buttons`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Buttons {
    /// `0.0..=1.0` — analog pressure on Xbox-style buttons (`DualShock` /
    /// `DualSense` report 0/1 only).
    pub a: f32,
/// Mirrors the `GameController` framework property for `b`.
    pub b: f32,
/// Mirrors the `GameController` framework property for `x`.
    pub x: f32,
/// Mirrors the `GameController` framework property for `y`.
    pub y: f32,
/// Mirrors the `GameController` framework property for `menu`.
    pub menu: f32,
/// Mirrors the `GameController` framework property for `options`.
    pub options: f32,
/// Mirrors the `GameController` framework property for `home`.
    pub home: f32,
}

/// Mirrors the `GameController` framework counterpart for `Triggers`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triggers {
    /// `0.0..=1.0` analog triggers + shoulder buttons.
    pub left_shoulder: f32,
/// Mirrors the `GameController` framework property for `right_shoulder`.
    pub right_shoulder: f32,
/// Mirrors the `GameController` framework property for `left_trigger`.
    pub left_trigger: f32,
/// Mirrors the `GameController` framework property for `right_trigger`.
    pub right_trigger: f32,
}

/// Mirrors the `GameController` framework counterpart for `Thumbsticks`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Thumbsticks {
    /// `-1.0..=1.0` per axis.
    pub left_x: f32,
/// Mirrors the `GameController` framework property for `left_y`.
    pub left_y: f32,
/// Mirrors the `GameController` framework property for `right_x`.
    pub right_x: f32,
/// Mirrors the `GameController` framework property for `right_y`.
    pub right_y: f32,
}

/// Mirrors the `GameController` framework counterpart for `Dpad`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dpad {
    /// `0.0..=1.0` — pressed amount per direction (most controllers
    /// report 0/1, but some report fractional pressure).
    pub up: f32,
/// Mirrors the `GameController` framework property for `down`.
    pub down: f32,
/// Mirrors the `GameController` framework property for `left`.
    pub left: f32,
/// Mirrors the `GameController` framework property for `right`.
    pub right: f32,
}

/// Snapshot every connected controller right now.
#[must_use]
pub fn connected_controllers() -> Vec<Controller> {
    let mut array: *mut c_void = ptr::null_mut();
    let mut count: usize = 0;
    // SAFETY: `array` and `count` are valid stack locals passed as out-pointers for Swift to fill.
    let status = unsafe { ffi::gc_connected_controllers(&raw mut array, &raw mut count) };
    if status != 0 || array.is_null() || count == 0 {
        return Vec::new();
    }
    let typed = array.cast::<ffi::ControllerInfoRaw>();
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        // SAFETY: `typed` points to `count` contiguous entries returned by Swift, `i < count`, and the array stays live until we free it below.
        let raw = unsafe { &*typed.add(i) };
        out.push(Controller {
            vendor_name: take_string(raw.vendor_name),
            product_category: take_string(raw.product_category),
            player_index: raw.player_index,
            is_attached_to_device: raw.is_attached_to_device != 0,
            has_extended_gamepad: raw.has_extended_gamepad != 0,
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
    // SAFETY: `array`/`count` are the exact allocation returned by `gc_connected_controllers` and are freed exactly once here.
    unsafe { ffi::gc_controller_infos_free(array, count) };
    out
}

fn take_string(p: *mut core::ffi::c_char) -> String {
    unsafe { doom_fish_utils::ffi_string::take_owned_cstring_c(p, |p| ffi::gc_string_free(p)) }
        .unwrap_or_default()
}

// ---- v0.2: motion / battery / haptics / light snapshots ----

/// Battery charging state mirroring `GCDeviceBattery.batteryState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BatteryState {
/// Mirrors the `GameController` framework case `Unknown`.
    Unknown,
/// Mirrors the `GameController` framework case `Discharging`.
    Discharging,
/// Mirrors the `GameController` framework case `Charging`.
    Charging,
/// Mirrors the `GameController` framework case `Full`.
    Full,
}

/// Optional services available on a controller. Returned by [`first_controller_extras`].
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerExtras {
/// Mirrors the `GameController` framework property for `has_motion`.
    pub has_motion: bool,
/// Mirrors the `GameController` framework property for `has_haptics`.
    pub has_haptics: bool,
/// Mirrors the `GameController` framework property for `has_light`.
    pub has_light: bool,
/// Mirrors the `GameController` framework property for `has_battery`.
    pub has_battery: bool,
    /// `0.0..=1.0`, or `None` if no battery is exposed.
    pub battery_level: Option<f32>,
/// Mirrors the `GameController` framework property for `battery_state`.
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
    // SAFETY: `raw` is a valid stack-allocated out-struct for Swift to initialize.
    let ok = unsafe { ffi::gc_first_controller_extra(&raw mut raw) };
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
    // SAFETY: `buf` has capacity for `MAX` elements and its pointer is valid for Swift to write up to that many entries.
    let n = unsafe { ffi::gc_all_controllers_extras(buf.as_mut_ptr(), MAX) };
    buf.truncate(n);
    buf.iter().map(extras_from_raw).collect()
}

const fn empty_extras_raw() -> ffi::ExtraInfoRaw {
    ffi::ExtraInfoRaw {
        has_motion: 0,
        has_haptics: 0,
        has_light: 0,
        has_battery: 0,
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
        has_motion: raw.has_motion != 0,
        has_haptics: raw.has_haptics != 0,
        has_light: raw.has_light != 0,
        has_battery: raw.has_battery != 0,
        battery_level: if raw.has_battery != 0 {
            Some(raw.battery_level)
        } else {
            None
        },
        battery_state,
        gravity: if raw.has_motion != 0 {
            Some((raw.gravity_x, raw.gravity_y, raw.gravity_z))
        } else {
            None
        },
        user_acceleration: if raw.has_motion != 0 {
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
    // SAFETY: this is a read-only FFI query with no preconditions.
    unsafe { ffi::gc_mouse_is_connected() }
}

/// One-shot snapshot of the current mouse button states. Returns
/// `None` if no mouse is connected.
#[must_use]
pub fn mouse_button_states() -> Option<MouseButtons> {
    let mut l = false;
    let mut r = false;
    let mut m = false;
    // SAFETY: `l`, `r`, and `m` are valid stack locals passed as out-pointers for Swift to fill.
    let ok = unsafe { ffi::gc_mouse_button_states(&raw mut l, &raw mut r, &raw mut m) };
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
    // SAFETY: this is a read-only FFI query with no preconditions.
    unsafe { ffi::gc_keyboard_is_connected() }
}

/// True if ANY key is currently pressed on the coalesced keyboard.
#[must_use]
pub fn keyboard_any_key_pressed() -> bool {
    // SAFETY: this is a read-only FFI query with no preconditions.
    unsafe { ffi::gc_keyboard_any_key_pressed() }
}

/// True if the HID-page-7 `keycode` is currently pressed
/// (e.g. `4 = "a"`, `40 = enter`, `44 = space`).
#[must_use]
pub fn keyboard_is_key_pressed(keycode: isize) -> bool {
    // SAFETY: this is a read-only FFI query and any `isize` keycode value is accepted by the bridge.
    unsafe { ffi::gc_keyboard_is_key_pressed(keycode) }
}

/// Snapshot of the three primary mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtons {
/// Mirrors the `GameController` framework property for `left`.
    pub left: bool,
/// Mirrors the `GameController` framework property for `right`.
    pub right: bool,
/// Mirrors the `GameController` framework property for `middle`.
    pub middle: bool,
}

// ---- v0.2: connect/disconnect callbacks ----

type ConnectionCallbackFn = Box<dyn Fn(bool) + Send + Sync + 'static>;
type NotificationCallbackFn = Box<dyn Fn() + Send + Sync + 'static>;

/// RAII guard for a registered bool-state notification callback. Drops the
/// `NSNotificationCenter` observer on scope exit.
pub struct ConnectionWatcher {
    token: *mut core::ffi::c_void,
    context: CallbackContext<ConnectionCallbackFn>,
}

/// RAII guard for a registered notification callback without payload data.
pub struct NotificationWatcher {
    token: *mut core::ffi::c_void,
    context: CallbackContext<NotificationCallbackFn>,
}

// SAFETY: `token` is an opaque retained Swift registration token whose registry is lock-protected, and `context` is `Send + Sync`.
unsafe impl Send for ConnectionWatcher {}
// SAFETY: `token` is an opaque retained Swift registration token whose registry is lock-protected, and `context` is `Send + Sync`.
unsafe impl Sync for ConnectionWatcher {}
// SAFETY: `token` is an opaque retained Swift registration token whose registry is lock-protected, and `context` is `Send + Sync`.
unsafe impl Send for NotificationWatcher {}
// SAFETY: `token` is an opaque retained Swift registration token whose registry is lock-protected, and `context` is `Send + Sync`.
unsafe impl Sync for NotificationWatcher {}

static DISCOVERY_CALLBACK: Mutex<Option<Box<dyn FnOnce() + Send + 'static>>> = Mutex::new(None);

impl Drop for ConnectionWatcher {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.token.is_null() {
            // SAFETY: `self.token` came from the matching Swift registration call, is non-null here, and we unregister it exactly once before nulling it out.
            unsafe { ffi::gc_unregister_connection_callback(self.token) };
            self.token = core::ptr::null_mut();
        }
    }
}

impl Drop for NotificationWatcher {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.token.is_null() {
            // SAFETY: `self.token` came from the matching Swift registration call, is non-null here, and we unregister it exactly once before nulling it out.
            unsafe { ffi::gc_unregister_notification_callback(self.token) };
            self.token = core::ptr::null_mut();
        }
    }
}

unsafe extern "C" fn connection_trampoline(user_info: *mut core::ffi::c_void, value: bool) {
    // SAFETY: `user_info` is the watcher's `CallbackContext`; the Swift registration holds a reference to it until the registration is deallocated.
    let _ = unsafe {
        CallbackContext::<ConnectionCallbackFn>::with(user_info, "connection_trampoline", |callback| {
            callback(value);
        })
    };
}

unsafe extern "C" fn notification_trampoline(user_info: *mut core::ffi::c_void) {
    // SAFETY: `user_info` is the watcher's `CallbackContext`; the Swift registration holds a reference to it until the registration is deallocated.
    let _ = unsafe {
        CallbackContext::<NotificationCallbackFn>::with(user_info, "notification_trampoline", |callback| {
            callback();
        })
    };
}

fn watch_connection_with<F>(
    register: unsafe extern "C" fn(
        ffi::ConnectionCallback,
        *mut c_void,
        ffi::ContextCallback,
        ffi::ContextCallback,
    ) -> *mut c_void,
    callback: F,
) -> ConnectionWatcher
where
    F: Fn(bool) + Send + Sync + 'static,
{
    let context = CallbackContext::new(Box::new(callback) as ConnectionCallbackFn);
    // SAFETY: `connection_trampoline` has the expected C ABI, and the Swift registration retains `context` through the RETAIN/RELEASE pair for as long as it can call back.
    let token = unsafe {
        register(
            connection_trampoline,
            context.as_ptr(),
            CallbackContext::<ConnectionCallbackFn>::RETAIN,
            CallbackContext::<ConnectionCallbackFn>::RELEASE,
        )
    };
    ConnectionWatcher { token, context }
}

fn watch_notification_with<F>(
    register: unsafe extern "C" fn(
        ffi::NotificationCallback,
        *mut c_void,
        ffi::ContextCallback,
        ffi::ContextCallback,
    ) -> *mut c_void,
    callback: F,
) -> NotificationWatcher
where
    F: Fn() + Send + Sync + 'static,
{
    let context = CallbackContext::new(Box::new(callback) as NotificationCallbackFn);
    // SAFETY: `notification_trampoline` has the expected C ABI, and the Swift registration retains `context` through the RETAIN/RELEASE pair for as long as it can call back.
    let token = unsafe {
        register(
            notification_trampoline,
            context.as_ptr(),
            CallbackContext::<NotificationCallbackFn>::RETAIN,
            CallbackContext::<NotificationCallbackFn>::RELEASE,
        )
    };
    NotificationWatcher { token, context }
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
            catch_user_panic("discovery_trampoline", callback);
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
    // SAFETY: the no-callback discovery variant accepts a null context pointer and has no additional preconditions.
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
    // SAFETY: `discovery_trampoline` has the expected C ABI and a null context is valid because the callback reads the global slot.
    unsafe {
        ffi::gc_start_wireless_controller_discovery(Some(discovery_trampoline), ptr::null_mut());
    }
}

/// Stop the current wireless controller discovery session, if any.
pub fn stop_wireless_controller_discovery() {
    if let Ok(mut slot) = DISCOVERY_CALLBACK.lock() {
        *slot = None;
    }
    // SAFETY: stopping discovery is a plain FFI command with no preconditions.
    unsafe { ffi::gc_stop_wireless_controller_discovery() }
}

/// Whether `GameController` should keep routing controller events while your app
/// is in the background.
#[must_use]
pub fn should_monitor_background_events() -> bool {
    // SAFETY: this is a read-only FFI query with no preconditions.
    unsafe { ffi::gc_should_monitor_background_events() }
}

/// Control whether `GameController` keeps routing controller events while your
/// app is in the background.
pub fn set_should_monitor_background_events(enabled: bool) {
    // SAFETY: `enabled` is passed by value and the Swift setter has no extra preconditions.
    unsafe { ffi::gc_set_should_monitor_background_events(enabled) }
}

/// Set the light bar / status LED color on the first connected
/// controller (`DualSense`, `DualShock`). Each channel is 0.0..=1.0.
/// Returns `false` if no controller is connected or it has no light.
#[must_use]
pub fn set_first_controller_light(red: f32, green: f32, blue: f32) -> bool {
    ControllerHandle::first()
        .is_some_and(|controller| controller.set_light(Color { red, green, blue }))
}

/// Set the light bar / status LED color on the first connected controller using
/// a typed [`Color`] value.
#[must_use]
pub fn set_first_controller_light_color(color: Color) -> bool {
    set_first_controller_light(color.red, color.green, color.blue)
}

/// Assign the player index on the first connected controller: the raw
/// `GCControllerPlayerIndex` value, `-1` to unset or `0..=3` for players 1–4.
///
/// This typically lights up the corresponding LED on consoles or sets
/// the player slot on Xbox / `DualSense`. Returns `false` if no
/// controller or the index is out of range.
#[must_use]
pub fn set_first_controller_player_index(index: i32) -> bool {
    ControllerHandle::first().is_some_and(|controller| controller.set_player_index(index))
}

/// Read the battery level (0.0..=1.0) of the first connected
/// controller. Returns `-1.0` if the controller has no battery
/// (wired) or none is connected.
#[must_use]
pub fn first_controller_battery_level() -> f32 {
    ControllerHandle::first()
        .and_then(|controller| controller.battery_level())
        .unwrap_or(-1.0)
}

/// Play a simple continuous haptic on the first controller for
/// `duration` seconds. `intensity` and `sharpness` are 0.0..=1.0.
///
/// Returns `false` if no controller, no haptic support, or Core
/// Haptics failed to start.
#[must_use]
pub fn rumble_first_controller(intensity: f32, sharpness: f32, duration: f64) -> bool {
    ControllerHandle::first().is_some_and(|controller| {
        controller.rumble(haptics_localities::DEFAULT, intensity, sharpness, duration)
    })
}

/// Play a continuous haptic on the first controller using a specific haptics locality.
#[must_use]
pub fn rumble_first_controller_with_locality(
    locality: GCHapticsLocality,
    intensity: f32,
    sharpness: f32,
    duration: f64,
) -> bool {
    ControllerHandle::first()
        .is_some_and(|controller| controller.rumble(locality, intensity, sharpness, duration))
}

/// Which `DualSense` trigger to address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualSenseTrigger {
/// Mirrors the `GameController` framework case `Left`.
    Left = 0,
/// Mirrors the `GameController` framework case `Right`.
    Right = 1,
}

/// True if a `DualSense` controller is currently connected.
#[must_use]
pub fn dualsense_is_connected() -> bool {
    ControllerHandle::all()
        .iter()
        .any(ControllerHandle::is_dualsense)
}

fn set_first_dualsense_trigger(which: DualSenseTrigger, effect: DualSenseTriggerEffect) -> bool {
    ControllerHandle::first_dualsense()
        .is_some_and(|controller| controller.set_dualsense_trigger(which, effect))
}

/// Disable adaptive resistance/vibration on the requested `DualSense`
/// trigger. Wraps `setModeOff`.
#[must_use]
pub fn dualsense_trigger_off(which: DualSenseTrigger) -> bool {
    set_first_dualsense_trigger(which, DualSenseTriggerEffect::Off)
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
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::Feedback {
            start_position,
            strength,
        },
    )
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
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::Weapon {
            start_position,
            end_position,
            strength,
        },
    )
}

/// Apply a vibration mode on a `DualSense` trigger. `amplitude` and
/// `frequency` are normalized `0.0..=1.0`.
#[must_use]
pub fn dualsense_trigger_vibration(
    which: DualSenseTrigger,
    start_position: f32,
    amplitude: f32,
    frequency: f32,
) -> bool {
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::Vibration {
            start_position,
            amplitude,
            frequency,
        },
    )
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
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::SlopeFeedback {
            start_position,
            end_position,
            start_strength,
            end_strength,
        },
    )
}

/// Apply per-position resistive strengths to a `DualSense` trigger.
#[must_use]
pub fn dualsense_trigger_feedback_resistive_strengths(
    which: DualSenseTrigger,
    strengths: GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
) -> bool {
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::FeedbackResistiveStrengths(strengths),
    )
}

/// Apply per-position vibration amplitudes to a `DualSense` trigger.
#[must_use]
pub fn dualsense_trigger_vibration_amplitudes(
    which: DualSenseTrigger,
    amplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes,
    frequency: f32,
) -> bool {
    set_first_dualsense_trigger(
        which,
        DualSenseTriggerEffect::VibrationAmplitudes {
            amplitudes,
            frequency,
        },
    )
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use doom_fish_utils::callback_context::CallbackContext;

    use super::{
        connection_trampoline, notification_trampoline, ConnectionCallbackFn,
        NotificationCallbackFn,
    };

    #[test]
    fn connection_callback_outlives_the_watcher_but_stops_firing() {
        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);
        let context = CallbackContext::new(Box::new(move |connected: bool| {
            if connected {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }) as ConnectionCallbackFn);
        let swift_reference = context.retained_ptr();

        unsafe { connection_trampoline(swift_reference, true) };
        assert_eq!(hits.load(Ordering::SeqCst), 1);

        drop(context);
        unsafe { connection_trampoline(swift_reference, true) };
        assert_eq!(hits.load(Ordering::SeqCst), 1);
        assert_eq!(Arc::strong_count(&hits), 2);

        unsafe { (CallbackContext::<ConnectionCallbackFn>::RELEASE)(swift_reference) };
        assert_eq!(Arc::strong_count(&hits), 1);
    }

    #[test]
    fn notification_callback_panics_are_contained() {
        let context = CallbackContext::new(Box::new(|| panic!("watcher panic")) as NotificationCallbackFn);
        unsafe { notification_trampoline(context.as_ptr()) };
        unsafe { notification_trampoline(core::ptr::null_mut()) };
        assert!(context.is_active());
    }
}
