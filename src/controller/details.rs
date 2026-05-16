use core::ffi::CStr;
use std::ffi::c_char;

use serde::{de::DeserializeOwned, Deserialize};

use crate::error::GameControllerError;
use crate::ffi;

use super::BatteryState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TouchState {
    Up,
    Down,
    Moving,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DualSenseTriggerMode {
    Off,
    Feedback,
    Weapon,
    Vibration,
    SlopeFeedback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DualSenseTriggerStatus {
    Unknown,
    FeedbackNoLoad,
    FeedbackLoadApplied,
    WeaponReady,
    WeaponFiring,
    WeaponFired,
    VibrationNotVibrating,
    VibrationIsVibrating,
    SlopeFeedbackReady,
    SlopeFeedbackApplyingLoad,
    SlopeFeedbackFinished,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonInputState {
    pub value: f32,
    pub pressed: bool,
    pub touched: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisInputState {
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionPadInputState {
    pub x: f32,
    pub y: f32,
    pub up: ButtonInputState,
    pub down: ButtonInputState,
    pub left: ButtonInputState,
    pub right: ButtonInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchpadDetails {
    pub button: ButtonInputState,
    pub touch_surface: DirectionPadInputState,
    pub touch_state: TouchState,
    pub reports_absolute_touch_surface_values: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamepadDetails {
    pub dpad: DirectionPadInputState,
    pub button_a: ButtonInputState,
    pub button_b: ButtonInputState,
    pub button_x: ButtonInputState,
    pub button_y: ButtonInputState,
    pub left_shoulder: ButtonInputState,
    pub right_shoulder: ButtonInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroGamepadDetails {
    pub dpad: DirectionPadInputState,
    pub button_a: ButtonInputState,
    pub button_x: ButtonInputState,
    pub button_menu: Option<ButtonInputState>,
    pub reports_absolute_dpad_values: bool,
    pub allows_rotation: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionalGamepadDetails {
    pub dpad: DirectionPadInputState,
    pub button_a: ButtonInputState,
    pub button_x: ButtonInputState,
    pub button_menu: Option<ButtonInputState>,
    pub reports_absolute_dpad_values: bool,
    pub allows_rotation: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedGamepadDetails {
    pub dpad: DirectionPadInputState,
    pub button_a: ButtonInputState,
    pub button_b: ButtonInputState,
    pub button_x: ButtonInputState,
    pub button_y: ButtonInputState,
    pub button_menu: Option<ButtonInputState>,
    pub button_options: Option<ButtonInputState>,
    pub button_home: Option<ButtonInputState>,
    pub left_thumbstick: DirectionPadInputState,
    pub right_thumbstick: DirectionPadInputState,
    pub left_shoulder: ButtonInputState,
    pub right_shoulder: ButtonInputState,
    pub left_trigger: ButtonInputState,
    pub right_trigger: ButtonInputState,
    pub left_thumbstick_button: Option<ButtonInputState>,
    pub right_thumbstick_button: Option<ButtonInputState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualShockGamepadDetails {
    pub touchpad_button: ButtonInputState,
    pub touchpad_primary: DirectionPadInputState,
    pub touchpad_secondary: DirectionPadInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualSenseAdaptiveTriggerState {
    pub value: f32,
    pub pressed: bool,
    pub touched: bool,
    pub mode: DualSenseTriggerMode,
    pub status: DualSenseTriggerStatus,
    pub arm_position: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualSenseGamepadDetails {
    pub touchpad_button: ButtonInputState,
    pub touchpad_primary: DirectionPadInputState,
    pub touchpad_secondary: DirectionPadInputState,
    pub left_trigger: DualSenseAdaptiveTriggerState,
    pub right_trigger: DualSenseAdaptiveTriggerState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XboxGamepadDetails {
    pub paddle_button1: Option<ButtonInputState>,
    pub paddle_button2: Option<ButtonInputState>,
    pub paddle_button3: Option<ButtonInputState>,
    pub paddle_button4: Option<ButtonInputState>,
    pub button_share: Option<ButtonInputState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryInfo {
    pub level: f32,
    pub state: BatteryState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceLightDetails {
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceHapticsDetails {
    pub supported_localities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct MotionDetails {
    pub sensors_require_manual_activation: bool,
    pub sensors_active: bool,
    pub has_gravity_and_user_acceleration: bool,
    pub gravity: Vector3,
    pub user_acceleration: Vector3,
    pub acceleration: Vector3,
    pub has_attitude: bool,
    pub has_rotation_rate: bool,
    pub attitude: Quaternion,
    pub rotation_rate: Vector3,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedButtonInputState {
    pub alias: String,
    pub value: ButtonInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAxisInputState {
    pub alias: String,
    pub value: AxisInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedDirectionPadState {
    pub alias: String,
    pub value: DirectionPadInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedTouchpadState {
    pub alias: String,
    pub value: TouchpadDetails,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputProfileDetails {
    pub last_event_timestamp: f64,
    pub has_remapped_elements: bool,
    pub element_aliases: Vec<String>,
    pub button_aliases: Vec<String>,
    pub axis_aliases: Vec<String>,
    pub dpad_aliases: Vec<String>,
    pub touchpad_aliases: Vec<String>,
    pub buttons: Vec<NamedButtonInputState>,
    pub axes: Vec<NamedAxisInputState>,
    pub dpads: Vec<NamedDirectionPadState>,
    pub touchpads: Vec<NamedTouchpadState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisElementState {
    pub absolute_value: Option<f32>,
    pub relative_delta: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInputState {
    pub position: i32,
    pub position_lower_bound: i32,
    pub position_count: i32,
    pub sequential: bool,
    pub can_wrap: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedButtonElementState {
    pub primary_alias: String,
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub value: ButtonInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAxisElementState {
    pub primary_alias: String,
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub value: AxisElementState,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSwitchElementState {
    pub primary_alias: String,
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub value: SwitchInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedDirectionPadElementState {
    pub primary_alias: String,
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub value: DirectionPadInputState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerInputStateDetails {
    pub last_event_timestamp: f64,
    pub last_event_latency: f64,
    pub element_count: usize,
    pub button_count: usize,
    pub axis_count: usize,
    pub switch_count: usize,
    pub dpad_count: usize,
    pub buttons: Vec<NamedButtonElementState>,
    pub axes: Vec<NamedAxisElementState>,
    pub switches: Vec<NamedSwitchElementState>,
    pub dpads: Vec<NamedDirectionPadElementState>,
    pub changed_aliases_known: bool,
    pub changed_aliases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerLiveInputDetails {
    pub input_state_queue_depth: isize,
    pub live: ControllerInputStateDetails,
    pub unmapped: Option<ControllerInputStateDetails>,
    pub next: Option<ControllerInputStateDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct ControllerDetails {
    pub vendor_name: String,
    pub product_category: String,
    pub player_index: i32,
    pub is_attached_to_device: bool,
    pub is_current: bool,
    pub supports_background_events: bool,
    pub has_live_input: bool,
    pub gamepad: Option<GamepadDetails>,
    pub micro_gamepad: Option<MicroGamepadDetails>,
    pub directional_gamepad: Option<DirectionalGamepadDetails>,
    pub extended_gamepad: Option<ExtendedGamepadDetails>,
    pub dual_shock: Option<DualShockGamepadDetails>,
    pub dual_sense: Option<DualSenseGamepadDetails>,
    pub xbox: Option<XboxGamepadDetails>,
    pub battery: Option<BatteryInfo>,
    pub motion: Option<MotionDetails>,
    pub has_light: bool,
    pub light: Option<DeviceLightDetails>,
    pub has_haptics: bool,
    pub haptics: Option<DeviceHapticsDetails>,
    pub physical_input: Option<PhysicalInputProfileDetails>,
    pub input: Option<ControllerLiveInputDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardSnapshot {
    pub vendor_name: String,
    pub product_category: String,
    pub any_key_pressed: bool,
    pub pressed_aliases: Vec<String>,
    pub pressed_keys: Vec<NamedButtonInputState>,
    pub physical_input: PhysicalInputProfileDetails,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseSnapshot {
    pub vendor_name: String,
    pub product_category: String,
    pub is_current: bool,
    pub known_mouse_count: usize,
    pub scroll: DirectionPadInputState,
    pub left_button: ButtonInputState,
    pub right_button: Option<ButtonInputState>,
    pub middle_button: Option<ButtonInputState>,
    pub auxiliary_buttons: Vec<NamedButtonInputState>,
    pub physical_input: PhysicalInputProfileDetails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventViewControllerDetails {
    pub controller_user_interaction_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteeringWheelDetails {
    pub maximum_degrees_of_rotation: f32,
    pub absolute_value: Option<f32>,
    pub relative_delta: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearShifterDetails {
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub pattern_position: Option<i32>,
    pub pattern_lower_bound: Option<i32>,
    pub pattern_count: Option<i32>,
    pub pattern_sequential: Option<bool>,
    pub pattern_can_wrap: Option<bool>,
    pub sequential_delta: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacingWheelInputDetails {
    pub wheel: SteeringWheelDetails,
    pub accelerator_pedal: Option<ButtonInputState>,
    pub brake_pedal: Option<ButtonInputState>,
    pub clutch_pedal: Option<ButtonInputState>,
    pub shifter: Option<GearShifterDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacingWheelDetails {
    pub vendor_name: String,
    pub product_category: String,
    pub is_acquired: bool,
    pub is_snapshot: bool,
    pub wheel_input: Option<RacingWheelInputDetails>,
}

/// Snapshot every currently connected controller.
///
/// The detailed payload includes legacy profiles, keyboard/mouse-adjacent
/// physical input collections, `DualSense` trigger state, battery, light,
/// haptics, and controller-input snapshots when the OS exposes them.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn connected_controller_details() -> Result<Vec<ControllerDetails>, GameControllerError> {
    parse_json(unsafe { ffi::gc_controller_details_json(false) })
}

/// Snapshot `GCController.current`, if one exists.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_details() -> Result<Option<ControllerDetails>, GameControllerError> {
    Ok(
        parse_json::<Vec<ControllerDetails>>(unsafe { ffi::gc_controller_details_json(true) })?
            .into_iter()
            .next(),
    )
}

/// Convenience alias for [`current_controller_details`].
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_snapshot() -> Result<Option<ControllerDetails>, GameControllerError> {
    current_controller_details()
}

/// Snapshot the coalesced keyboard, if one is currently connected.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn keyboard_snapshot() -> Result<Option<KeyboardSnapshot>, GameControllerError> {
    parse_json(unsafe { ffi::gc_keyboard_snapshot_json() })
}

/// Snapshot the current mouse, if one is currently connected.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn mouse_snapshot() -> Result<Option<MouseSnapshot>, GameControllerError> {
    parse_json(unsafe { ffi::gc_mouse_snapshot_json() })
}

/// Snapshot the default `GCEventViewController` configuration.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn event_view_controller_snapshot() -> Result<EventViewControllerDetails, GameControllerError> {
    parse_json(unsafe { ffi::gc_event_view_controller_snapshot_json() })
}

/// Snapshot every connected racing wheel visible in the current process.
///
/// The returned entries include wheel-level metadata even when a wheel has not
/// been acquired. `wheel_input` is only populated when the device is already
/// acquired, preserving the crate's snapshot-first / no-handle design.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn connected_racing_wheels() -> Result<Vec<RacingWheelDetails>, GameControllerError> {
    parse_json(unsafe { ffi::gc_racing_wheels_json() })
}

/// Return the first connected controller's battery snapshot, if any.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn first_controller_battery() -> Result<Option<BatteryInfo>, GameControllerError> {
    Ok(connected_controller_details()?
        .into_iter()
        .find_map(|controller| controller.battery))
}

/// Return the first connected controller's light snapshot, if any.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn first_controller_light() -> Result<Option<DeviceLightDetails>, GameControllerError> {
    Ok(connected_controller_details()?
        .into_iter()
        .find_map(|controller| controller.light))
}

/// Return the first connected controller's haptics snapshot, if any.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn first_controller_haptics() -> Result<Option<DeviceHapticsDetails>, GameControllerError> {
    Ok(connected_controller_details()?
        .into_iter()
        .find_map(|controller| controller.haptics))
}

/// Snapshot `GCController.current.input`, if a current controller exists and the
/// OS exposes controller live input.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_input_snapshot(
) -> Result<Option<ControllerLiveInputDetails>, GameControllerError> {
    Ok(current_controller_details()?.and_then(|controller| controller.input))
}

fn parse_json<T: DeserializeOwned>(ptr: *mut c_char) -> Result<T, GameControllerError> {
    let json = take_json_string(ptr)?;
    Ok(serde_json::from_str(&json)?)
}

fn take_json_string(ptr: *mut c_char) -> Result<String, GameControllerError> {
    if ptr.is_null() {
        return Err(GameControllerError::NullBridgeResponse);
    }
    let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned();
    unsafe { ffi::gc_string_free(ptr) };
    Ok(text)
}
