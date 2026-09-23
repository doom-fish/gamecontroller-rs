//! Async event-stream wrappers for `GameController` framework callbacks.
//!
//! Each type wraps a `GameController` handler/notification as a
//! [`doom_fish_utils::stream::BoundedAsyncStream`] event stream.
//! The stream is bounded and lossy-by-default (drops oldest on overflow).
//!
//! # Feature gate
//!
//! This module is only compiled when the `async` cargo feature is enabled:
//! ```toml
//! gamecontroller = { version = "0.9", features = ["async"] }
//! ```
//!
//! # Async surfaces
//!
//! | Type | Source |
//! |---|---|
//! | [`WirelessDiscoveryFuture`] | `GCController.startWirelessControllerDiscovery(completionHandler:)` |
//! | [`ControllerConnectionStream`] | `GCControllerDidConnect` / `GCControllerDidDisconnect` |
//! | [`GamepadValueStream`] | `GCExtendedGamepad.valueChangedHandler` |
//! | [`KeyboardKeyStream`] | `GCKeyboard.keyboardInput.keyChangedHandler` |
//! | [`MouseInputStream`] | `GCMouse.mouseInput` moved / button handlers |
//! | [`MotionStream`] | `GCMotion.valueChangedHandler` |
//! | [`MicroGamepadValueStream`] | `GCMicroGamepad.valueChangedHandler` |

#![cfg(feature = "async")]
#![allow(clippy::module_name_repetitions, clippy::struct_field_names)]

use crate::controller::{self, Buttons, Dpad, Thumbsticks, Triggers};
use crate::error::GameControllerError;
use crate::ffi::{ContextCallback, ControllerInfoRaw};
use core::ffi::{c_char, c_void};
use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::completion::{AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};
use std::ffi::CStr;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ── FFI declarations ──────────────────────────────────────────────────────────

type StreamCallback = unsafe extern "C" fn(i32, *const c_void, *mut c_void);
type SubscribeFn = unsafe extern "C" fn(
    StreamCallback,
    *mut c_void,
    ContextCallback,
    ContextCallback,
) -> *mut c_void;

unsafe extern "C" {
    fn gc_stream_controller_connection_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_controller_connection_unsubscribe(handle: *mut c_void);
    fn gc_stream_gamepad_value_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_gamepad_value_unsubscribe(handle: *mut c_void);
    fn gc_stream_keyboard_key_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_keyboard_key_unsubscribe(handle: *mut c_void);
    fn gc_stream_mouse_input_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_mouse_input_unsubscribe(handle: *mut c_void);
    fn gc_stream_motion_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_motion_unsubscribe(handle: *mut c_void);
    fn gc_stream_micro_gamepad_subscribe(
        cb: StreamCallback,
        ctx: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void;
    fn gc_stream_micro_gamepad_unsubscribe(handle: *mut c_void);
}

// ── Raw payload types (must match @frozen Swift structs byte-for-byte) ────────

/// Matches Swift `GCKeyEventRaw`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct RawKeyEvent {
    keycode: i64,
    pressed: bool,
    value: f32,
}

/// Matches Swift `GCMouseEventRaw`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct RawMouseEvent {
    delta_x: f32,
    delta_y: f32,
    pressed: bool,
    value: f32,
}

/// Matches Swift `GCMotionEventRaw`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct RawMotionEvent {
    gravity_x: f64,
    gravity_y: f64,
    gravity_z: f64,
    user_acceleration_x: f64,
    user_acceleration_y: f64,
    user_acceleration_z: f64,
    attitude_x: f64,
    attitude_y: f64,
    attitude_z: f64,
    attitude_w: f64,
    rotation_rate_x: f64,
    rotation_rate_y: f64,
    rotation_rate_z: f64,
}

/// Matches Swift `GCMicroGamepadEventRaw`.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct RawMicroGamepadEvent {
    button_a: f32,
    button_x: f32,
    dpad_x: f32,
    dpad_y: f32,
}

// ── RAII subscription handle ──────────────────────────────────────────────────

/// RAII guard: cancels the Swift subscription and releases the Rust sender on drop.
struct SubscriptionHandle<T: Send + 'static> {
    swift_handle: *mut c_void,
    context: CallbackContext<AsyncStreamSender<T>>,
    unsubscribe: unsafe extern "C" fn(*mut c_void),
}

impl<T: Send + 'static> Drop for SubscriptionHandle<T> {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.swift_handle.is_null() {
            // SAFETY: `swift_handle` came from the matching subscribe call, is non-null here, and drop runs this unsubscribe exactly once.
            unsafe { (self.unsubscribe)(self.swift_handle) };
        }
    }
}

// SAFETY: the Swift handle is a retained object that is only passed to its thread-safe unsubscribe function in drop, and `context` is `Send + Sync`.
unsafe impl<T: Send + 'static> Send for SubscriptionHandle<T> {}
// SAFETY: the Swift handle is only touched through drop, and `context` is `Send + Sync`.
unsafe impl<T: Send + 'static> Sync for SubscriptionHandle<T> {}

fn subscribe_stream<T: Send + 'static>(
    capacity: usize,
    subscribe: SubscribeFn,
    callback: StreamCallback,
    unsubscribe: unsafe extern "C" fn(*mut c_void),
) -> (BoundedAsyncStream<T>, SubscriptionHandle<T>) {
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    let context = CallbackContext::new(sender);
    // SAFETY: `callback` has the expected C ABI, and the Swift subscriber retains `context` through the RETAIN/RELEASE pair for as long as it can call back.
    let swift_handle = unsafe {
        subscribe(
            callback,
            context.as_ptr(),
            CallbackContext::<AsyncStreamSender<T>>::RETAIN,
            CallbackContext::<AsyncStreamSender<T>>::RELEASE,
        )
    };
    (
        stream,
        SubscriptionHandle {
            swift_handle,
            context,
            unsubscribe,
        },
    )
}

/// Future returned by [`start_wireless_controller_discovery`].
#[must_use = "futures do nothing unless awaited"]
pub struct WirelessDiscoveryFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for WirelessDiscoveryFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WirelessDiscoveryFuture")
            .finish_non_exhaustive()
    }
}

impl Future for WirelessDiscoveryFuture {
    type Output = Result<(), GameControllerError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|result| result.map_err(|_| GameControllerError::NullBridgeResponse))
    }
}

/// Start `GameController`'s wireless discovery flow and await its completion.
pub fn start_wireless_controller_discovery() -> WirelessDiscoveryFuture {
    let (future, ctx) = AsyncCompletion::create();
    let ctx = ctx as usize;
    controller::start_wireless_controller_discovery_with_callback(move || unsafe {
        AsyncCompletion::complete_ok(ctx as *mut c_void, ());
    });
    WirelessDiscoveryFuture { inner: future }
}

// ── 1. ControllerConnectionStream ─────────────────────────────────────────────

/// A controller connect / disconnect event.
#[derive(Debug, Clone)]
pub struct ControllerConnectionEvent {
    /// Vendor name reported by the controller, if available.
    pub vendor_name: Option<String>,
    /// `true` = connected, `false` = disconnected.
    pub connected: bool,
}

/// Async stream of `GCController` connect / disconnect events.
///
/// Fires when any controller connects or disconnects via
/// `NSNotificationCenter` (`GCControllerDidConnect` /
/// `GCControllerDidDisconnect`).
///
/// # Example
/// ```no_run
/// # async fn run() {
/// use gamecontroller::async_api::ControllerConnectionStream;
/// let stream = ControllerConnectionStream::subscribe(16);
/// while let Some(ev) = stream.next().await {
///     println!(
///         "{} {}",
///         if ev.connected { "connected" } else { "disconnected" },
///         ev.vendor_name.as_deref().unwrap_or("unknown")
///     );
/// }
/// # }
/// ```
pub struct ControllerConnectionStream {
    inner: BoundedAsyncStream<ControllerConnectionEvent>,
    _handle: SubscriptionHandle<ControllerConnectionEvent>,
}

unsafe extern "C" fn controller_connection_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let push = |sender: &AsyncStreamSender<ControllerConnectionEvent>| {
        let vendor_name = if payload.is_null() {
            None
        } else {
            // SAFETY: `payload` is a non-null NUL-terminated string owned by the Swift bridge for the duration of this callback.
            unsafe { CStr::from_ptr(payload.cast::<c_char>()) }
                .to_str()
                .ok()
                .map(str::to_owned)
        };
        sender.push(ControllerConnectionEvent {
            vendor_name,
            connected: kind == 0,
        });
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ = unsafe {
        CallbackContext::<AsyncStreamSender<ControllerConnectionEvent>>::with(
            ctx,
            "controller_connection_cb",
            push,
        )
    };
}

impl ControllerConnectionStream {
    /// Subscribe to controller connection events.
    ///
    /// `capacity` is the ring-buffer size. The oldest event is dropped on
    /// overflow. A value of 16 is appropriate for most use cases.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_controller_connection_subscribe,
            controller_connection_cb,
            gc_stream_controller_connection_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next connection event. Returns `None` when the stream closes.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, ControllerConnectionEvent> {
        self.inner.next()
    }

    /// Non-blocking pop. Returns `None` if no event is buffered.
    #[must_use]
    pub fn try_next(&self) -> Option<ControllerConnectionEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ── 2. GamepadValueStream ─────────────────────────────────────────────────────

/// A snapshot of all `GCExtendedGamepad` input values at one instant.
#[derive(Debug, Clone, PartialEq)]
pub struct GamepadValueEvent {
    /// Face / shoulder buttons and menu buttons.
    pub buttons: Buttons,
    /// Analog triggers and shoulder buttons.
    pub triggers: Triggers,
    /// Thumbstick axes.
    pub thumbsticks: Thumbsticks,
    /// D-pad axes.
    pub dpad: Dpad,
}

/// Async stream of `GCExtendedGamepad` value changes.
///
/// Fires on every input change via `valueChangedHandler`. Attaches to all
/// currently connected controllers and any that connect later.
pub struct GamepadValueStream {
    inner: BoundedAsyncStream<GamepadValueEvent>,
    _handle: SubscriptionHandle<GamepadValueEvent>,
}

unsafe extern "C" fn gamepad_value_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let _ = kind;
    if payload.is_null() {
        return;
    }
    let push = |sender: &AsyncStreamSender<GamepadValueEvent>| {
        // SAFETY: `payload` points to a stack-allocated `ControllerInfoRaw` whose lifetime spans this callback.
        let raw = unsafe { &*payload.cast::<ControllerInfoRaw>() };
        sender.push(GamepadValueEvent {
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
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ = unsafe {
        CallbackContext::<AsyncStreamSender<GamepadValueEvent>>::with(ctx, "gamepad_value_cb", push)
    };
}

impl GamepadValueStream {
    /// Subscribe to extended gamepad value changes.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_gamepad_value_subscribe,
            gamepad_value_cb,
            gc_stream_gamepad_value_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next gamepad value event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, GamepadValueEvent> {
        self.inner.next()
    }

    /// Non-blocking pop.
    #[must_use]
    pub fn try_next(&self) -> Option<GamepadValueEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ── 3. KeyboardKeyStream ──────────────────────────────────────────────────────

/// A single key press or release event from `GCKeyboard`.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardKeyEvent {
    /// Raw `GCKeyCode` value.
    pub keycode: i64,
    /// `true` = key down, `false` = key up.
    pub pressed: bool,
    /// Analog value (`0.0` or `1.0` for most keys).
    pub value: f32,
}

/// Async stream of `GCKeyboard` key events (macOS 11.0+).
///
/// Fires on every key state change via `keyChangedHandler`.
pub struct KeyboardKeyStream {
    inner: BoundedAsyncStream<KeyboardKeyEvent>,
    _handle: SubscriptionHandle<KeyboardKeyEvent>,
}

unsafe extern "C" fn keyboard_key_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let _ = kind;
    if payload.is_null() {
        return;
    }
    let push = |sender: &AsyncStreamSender<KeyboardKeyEvent>| {
        // SAFETY: `payload` points to a stack-allocated `RawKeyEvent` whose lifetime spans this callback.
        let raw = unsafe { &*payload.cast::<RawKeyEvent>() };
        sender.push(KeyboardKeyEvent {
            keycode: raw.keycode,
            pressed: raw.pressed,
            value: raw.value,
        });
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ = unsafe {
        CallbackContext::<AsyncStreamSender<KeyboardKeyEvent>>::with(ctx, "keyboard_key_cb", push)
    };
}

impl KeyboardKeyStream {
    /// Subscribe to keyboard key events.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_keyboard_key_subscribe,
            keyboard_key_cb,
            gc_stream_keyboard_key_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next key event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, KeyboardKeyEvent> {
        self.inner.next()
    }

    /// Non-blocking pop.
    #[must_use]
    pub fn try_next(&self) -> Option<KeyboardKeyEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ── 4. MouseInputStream ───────────────────────────────────────────────────────

/// An input event from `GCMouse` (macOS 11.0+).
#[derive(Debug, Clone, PartialEq)]
pub enum MouseInputEvent {
    /// Mouse movement — delta from last position.
    Moved { delta_x: f32, delta_y: f32 },
    /// Left button pressed or released.
    LeftButton { pressed: bool, value: f32 },
    /// Right button pressed or released.
    RightButton { pressed: bool, value: f32 },
    /// Middle button pressed or released.
    MiddleButton { pressed: bool, value: f32 },
}

/// Async stream of `GCMouse` input events (macOS 11.0+).
///
/// Fires on mouse movement and button state changes via the corresponding
/// `GCMouseInput` handlers.
pub struct MouseInputStream {
    inner: BoundedAsyncStream<MouseInputEvent>,
    _handle: SubscriptionHandle<MouseInputEvent>,
}

unsafe extern "C" fn mouse_input_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    if payload.is_null() {
        return;
    }
    let push = |sender: &AsyncStreamSender<MouseInputEvent>| {
        // SAFETY: `payload` points to a stack-allocated `RawMouseEvent` whose lifetime spans this callback.
        let raw = unsafe { &*payload.cast::<RawMouseEvent>() };
        let event = match kind {
            0 => MouseInputEvent::Moved {
                delta_x: raw.delta_x,
                delta_y: raw.delta_y,
            },
            1 => MouseInputEvent::LeftButton {
                pressed: raw.pressed,
                value: raw.value,
            },
            2 => MouseInputEvent::RightButton {
                pressed: raw.pressed,
                value: raw.value,
            },
            3 => MouseInputEvent::MiddleButton {
                pressed: raw.pressed,
                value: raw.value,
            },
            _ => return,
        };
        sender.push(event);
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ = unsafe {
        CallbackContext::<AsyncStreamSender<MouseInputEvent>>::with(ctx, "mouse_input_cb", push)
    };
}

impl MouseInputStream {
    /// Subscribe to mouse input events.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_mouse_input_subscribe,
            mouse_input_cb,
            gc_stream_mouse_input_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next mouse event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, MouseInputEvent> {
        self.inner.next()
    }

    /// Non-blocking pop.
    #[must_use]
    pub fn try_next(&self) -> Option<MouseInputEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ── 5. MotionStream ───────────────────────────────────────────────────────────

/// A motion update from `GCMotion.valueChangedHandler`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionEvent {
    /// Gravity vector (units of g).
    pub gravity: (f64, f64, f64),
    /// User acceleration (units of g, gravity subtracted).
    pub user_acceleration: (f64, f64, f64),
    /// Device attitude as a quaternion `(x, y, z, w)`.
    pub attitude: (f64, f64, f64, f64),
    /// Rotation rate in radians/second `(x, y, z)`.
    pub rotation_rate: (f64, f64, f64),
}

/// Async stream of `GCMotion` updates.
///
/// Fires on every motion update via `valueChangedHandler`. Attaches to all
/// currently connected controllers that expose motion and any that connect
/// later.
pub struct MotionStream {
    inner: BoundedAsyncStream<MotionEvent>,
    _handle: SubscriptionHandle<MotionEvent>,
}

unsafe extern "C" fn motion_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let _ = kind;
    if payload.is_null() {
        return;
    }
    let push = |sender: &AsyncStreamSender<MotionEvent>| {
        // SAFETY: `payload` points to a stack-allocated `RawMotionEvent` whose lifetime spans this callback.
        let raw = unsafe { &*payload.cast::<RawMotionEvent>() };
        sender.push(MotionEvent {
            gravity: (raw.gravity_x, raw.gravity_y, raw.gravity_z),
            user_acceleration: (
                raw.user_acceleration_x,
                raw.user_acceleration_y,
                raw.user_acceleration_z,
            ),
            attitude: (
                raw.attitude_x,
                raw.attitude_y,
                raw.attitude_z,
                raw.attitude_w,
            ),
            rotation_rate: (
                raw.rotation_rate_x,
                raw.rotation_rate_y,
                raw.rotation_rate_z,
            ),
        });
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ =
        unsafe { CallbackContext::<AsyncStreamSender<MotionEvent>>::with(ctx, "motion_cb", push) };
}

impl MotionStream {
    /// Subscribe to motion updates.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_motion_subscribe,
            motion_cb,
            gc_stream_motion_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next motion event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, MotionEvent> {
        self.inner.next()
    }

    /// Non-blocking pop.
    #[must_use]
    pub fn try_next(&self) -> Option<MotionEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ── 6. MicroGamepadValueStream ────────────────────────────────────────────────

/// A snapshot of `GCMicroGamepad` input values (Siri Remote / Apple TV remote).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MicroGamepadValueEvent {
    /// Button A value (`0.0` or `1.0`).
    pub button_a: f32,
    /// Button X value (`0.0` or `1.0`).
    pub button_x: f32,
    /// D-pad X axis (`-1.0..=1.0`).
    pub dpad_x: f32,
    /// D-pad Y axis (`-1.0..=1.0`).
    pub dpad_y: f32,
}

/// Async stream of `GCMicroGamepad` value changes.
///
/// Fires on every input change via `valueChangedHandler`. Attaches to all
/// currently connected micro gamepads and any that connect later.
pub struct MicroGamepadValueStream {
    inner: BoundedAsyncStream<MicroGamepadValueEvent>,
    _handle: SubscriptionHandle<MicroGamepadValueEvent>,
}

unsafe extern "C" fn micro_gamepad_value_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let _ = kind;
    if payload.is_null() {
        return;
    }
    let push = |sender: &AsyncStreamSender<MicroGamepadValueEvent>| {
        // SAFETY: `payload` points to a stack-allocated `RawMicroGamepadEvent` whose lifetime spans this callback.
        let raw = unsafe { &*payload.cast::<RawMicroGamepadEvent>() };
        sender.push(MicroGamepadValueEvent {
            button_a: raw.button_a,
            button_x: raw.button_x,
            dpad_x: raw.dpad_x,
            dpad_y: raw.dpad_y,
        });
    };
    // SAFETY: `ctx` is the subscription's `CallbackContext`; the Swift subscriber holds a reference to it until it is deallocated.
    let _ = unsafe {
        CallbackContext::<AsyncStreamSender<MicroGamepadValueEvent>>::with(
            ctx,
            "micro_gamepad_value_cb",
            push,
        )
    };
}

impl MicroGamepadValueStream {
    /// Subscribe to micro gamepad value changes.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    #[must_use]
    pub fn subscribe(capacity: usize) -> Self {
        let (inner, handle) = subscribe_stream(
            capacity,
            gc_stream_micro_gamepad_subscribe,
            micro_gamepad_value_cb,
            gc_stream_micro_gamepad_unsubscribe,
        );
        Self {
            inner,
            _handle: handle,
        }
    }

    /// Await the next micro gamepad event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, MicroGamepadValueEvent> {
        self.inner.next()
    }

    /// Non-blocking pop.
    #[must_use]
    pub fn try_next(&self) -> Option<MicroGamepadValueEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::sync::Mutex;

    use super::{
        keyboard_key_cb, subscribe_stream, ContextCallback, KeyboardKeyEvent, RawKeyEvent,
        StreamCallback,
    };

    struct FakeSubscriber {
        context: usize,
        release: ContextCallback,
    }

    static FAKE_SUBSCRIBER: Mutex<Option<FakeSubscriber>> = Mutex::new(None);

    unsafe extern "C" fn fake_subscribe(
        _callback: StreamCallback,
        context: *mut c_void,
        retain: ContextCallback,
        release: ContextCallback,
    ) -> *mut c_void {
        unsafe { retain(context) };
        *FAKE_SUBSCRIBER.lock().unwrap() = Some(FakeSubscriber {
            context: context as usize,
            release,
        });
        context
    }

    unsafe extern "C" fn fake_unsubscribe(_handle: *mut c_void) {}

    fn key_event(keycode: i64) -> RawKeyEvent {
        RawKeyEvent {
            keycode,
            pressed: true,
            value: 1.0,
        }
    }

    #[test]
    fn events_after_drop_are_ignored_while_the_subscriber_keeps_the_sender_alive() {
        let (stream, handle) = subscribe_stream::<KeyboardKeyEvent>(
            4,
            fake_subscribe,
            keyboard_key_cb,
            fake_unsubscribe,
        );
        let subscriber = FAKE_SUBSCRIBER.lock().unwrap().take().expect("subscribed");
        let context = subscriber.context as *mut c_void;

        let first = key_event(4);
        unsafe { keyboard_key_cb(0, (&raw const first).cast(), context) };
        assert_eq!(
            stream.try_next(),
            Some(KeyboardKeyEvent {
                keycode: 4,
                pressed: true,
                value: 1.0,
            })
        );

        drop(handle);
        let late = key_event(5);
        unsafe { keyboard_key_cb(0, (&raw const late).cast(), context) };
        assert_eq!(stream.try_next(), None);
        assert!(!stream.is_closed());

        unsafe { (subscriber.release)(context) };
        assert!(stream.is_closed());
    }
}
