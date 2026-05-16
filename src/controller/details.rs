use core::ffi::CStr;
use std::ffi::c_char;

use serde::Deserialize;

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
pub struct BatteryInfo {
    pub level: f32,
    pub state: BatteryState,
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
    pub extended_gamepad: Option<ExtendedGamepadDetails>,
    pub dual_sense: Option<DualSenseGamepadDetails>,
    pub battery: Option<BatteryInfo>,
    pub motion: Option<MotionDetails>,
    pub has_light: bool,
    pub has_haptics: bool,
    pub physical_input: Option<PhysicalInputProfileDetails>,
}

/// Snapshot every currently connected controller, including legacy profiles,
/// physical input collections, `DualSense` trigger state, battery, and motion.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn connected_controller_details() -> Result<Vec<ControllerDetails>, GameControllerError> {
    parse_details_json(false)
}

/// Snapshot `GCController.current`, if one exists.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_details() -> Result<Option<ControllerDetails>, GameControllerError> {
    Ok(parse_details_json(true)?.into_iter().next())
}

/// Convenience alias for [`current_controller_details`].
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_snapshot() -> Result<Option<ControllerDetails>, GameControllerError> {
    current_controller_details()
}

fn parse_details_json(current_only: bool) -> Result<Vec<ControllerDetails>, GameControllerError> {
    let json = take_json_string(unsafe { ffi::gc_controller_details_json(current_only) })?;
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
