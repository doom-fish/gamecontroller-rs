use core::ffi::CStr;
use std::ffi::c_char;

use serde::{de::DeserializeOwned, Deserialize};

use crate::error::GameControllerError;
use crate::ffi;

use super::{
    BatteryState, EulerAngles, GCHapticsLocality, GCPoint2, HapticsLocality,
    PhysicalInputSourceDirection, SystemGestureState,
};

/// Mirrors the `GameController` framework counterpart for `TouchState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TouchState {
/// Mirrors the `GameController` framework case `Up`.
    Up,
/// Mirrors the `GameController` framework case `Down`.
    Down,
/// Mirrors the `GameController` framework case `Moving`.
    Moving,
}

/// Mirrors the `GameController` framework counterpart for `DualSenseTriggerMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DualSenseTriggerMode {
/// Mirrors the `GameController` framework case `Off`.
    Off,
/// Mirrors the `GameController` framework case `Feedback`.
    Feedback,
/// Mirrors the `GameController` framework case `Weapon`.
    Weapon,
/// Mirrors the `GameController` framework case `Vibration`.
    Vibration,
/// Mirrors the `GameController` framework case `SlopeFeedback`.
    SlopeFeedback,
}

/// Mirrors the `GameController` framework counterpart for `DualSenseTriggerStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DualSenseTriggerStatus {
/// Mirrors the `GameController` framework case `Unknown`.
    Unknown,
/// Mirrors the `GameController` framework case `FeedbackNoLoad`.
    FeedbackNoLoad,
/// Mirrors the `GameController` framework case `FeedbackLoadApplied`.
    FeedbackLoadApplied,
/// Mirrors the `GameController` framework case `WeaponReady`.
    WeaponReady,
/// Mirrors the `GameController` framework case `WeaponFiring`.
    WeaponFiring,
/// Mirrors the `GameController` framework case `WeaponFired`.
    WeaponFired,
/// Mirrors the `GameController` framework case `VibrationNotVibrating`.
    VibrationNotVibrating,
/// Mirrors the `GameController` framework case `VibrationIsVibrating`.
    VibrationIsVibrating,
/// Mirrors the `GameController` framework case `SlopeFeedbackReady`.
    SlopeFeedbackReady,
/// Mirrors the `GameController` framework case `SlopeFeedbackApplyingLoad`.
    SlopeFeedbackApplyingLoad,
/// Mirrors the `GameController` framework case `SlopeFeedbackFinished`.
    SlopeFeedbackFinished,
}

/// Mirrors the `GameController` framework counterpart for `Vector3`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vector3 {
/// Mirrors the `GameController` framework property for `x`.
    pub x: f64,
/// Mirrors the `GameController` framework property for `y`.
    pub y: f64,
/// Mirrors the `GameController` framework property for `z`.
    pub z: f64,
}

/// Mirrors the `GameController` framework counterpart for `Quaternion`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Quaternion {
/// Mirrors the `GameController` framework property for `x`.
    pub x: f64,
/// Mirrors the `GameController` framework property for `y`.
    pub y: f64,
/// Mirrors the `GameController` framework property for `z`.
    pub z: f64,
/// Mirrors the `GameController` framework property for `w`.
    pub w: f64,
}

impl Quaternion {
    /// Convert the quaternion into pitch/yaw/roll Euler angles (radians).
    #[must_use]
    pub fn to_euler_angles(self) -> EulerAngles {
        let sin_pitch = 2.0 * self.w.mul_add(self.x, self.y * self.z);
        let cos_pitch = 2.0f64.mul_add(-(self.x.mul_add(self.x, self.y * self.y)), 1.0);
        let pitch = sin_pitch.atan2(cos_pitch);

        let sin_yaw = 2.0 * self.w.mul_add(self.y, -(self.z * self.x));
        let yaw = if sin_yaw.abs() >= 1.0 {
            sin_yaw.signum() * std::f64::consts::FRAC_PI_2
        } else {
            sin_yaw.asin()
        };

        let sin_roll = 2.0 * self.w.mul_add(self.z, self.x * self.y);
        let cos_roll = 2.0f64.mul_add(-(self.y.mul_add(self.y, self.z * self.z)), 1.0);
        let roll = sin_roll.atan2(cos_roll);

        EulerAngles { pitch, yaw, roll }
    }
}

impl From<Quaternion> for EulerAngles {
    fn from(value: Quaternion) -> Self {
        value.to_euler_angles()
    }
}

/// Mirrors the `GameController` framework counterpart for `Color`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Color {
/// Mirrors the `GameController` framework property for `red`.
    pub red: f32,
/// Mirrors the `GameController` framework property for `green`.
    pub green: f32,
/// Mirrors the `GameController` framework property for `blue`.
    pub blue: f32,
}

/// Mirrors the `GameController` framework counterpart for `ButtonInputState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonInputState {
/// Mirrors the `GameController` framework property for `value`.
    pub value: f32,
/// Mirrors the `GameController` framework property for `pressed`.
    pub pressed: bool,
/// Mirrors the `GameController` framework property for `touched`.
    pub touched: bool,
}

/// Apple-style alias for [`ButtonInputState`].
pub type GCButtonInput = ButtonInputState;

/// Mirrors the `GameController` framework counterpart for `AxisInputState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisInputState {
/// Mirrors the `GameController` framework property for `value`.
    pub value: f32,
}

/// Mirrors the `GameController` framework counterpart for `DirectionPadInputState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionPadInputState {
/// Mirrors the `GameController` framework property for `x`.
    pub x: f32,
/// Mirrors the `GameController` framework property for `y`.
    pub y: f32,
/// Mirrors the `GameController` framework property for `up`.
    pub up: ButtonInputState,
/// Mirrors the `GameController` framework property for `down`.
    pub down: ButtonInputState,
/// Mirrors the `GameController` framework property for `left`.
    pub left: ButtonInputState,
/// Mirrors the `GameController` framework property for `right`.
    pub right: ButtonInputState,
}

/// Typed cursor snapshot mirroring `GCDeviceCursor`.
pub type DeviceCursorState = DirectionPadInputState;
/// Apple-style alias for [`DeviceCursorState`].
pub type GCDeviceCursor = DeviceCursorState;

/// Mirrors the `GameController` framework counterpart for `TouchpadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchpadDetails {
/// Mirrors the `GameController` framework property for `button`.
    pub button: ButtonInputState,
/// Mirrors the `GameController` framework property for `touch_surface`.
    pub touch_surface: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `touch_state`.
    pub touch_state: TouchState,
/// Mirrors the `GameController` framework property for `reports_absolute_touch_surface_values`.
    pub reports_absolute_touch_surface_values: bool,
}

/// Mirrors the `GameController` framework counterpart for `GamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamepadDetails {
/// Mirrors the `GameController` framework property for `dpad`.
    pub dpad: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `button_a`.
    pub button_a: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_b`.
    pub button_b: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_x`.
    pub button_x: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_y`.
    pub button_y: ButtonInputState,
/// Mirrors the `GameController` framework property for `left_shoulder`.
    pub left_shoulder: ButtonInputState,
/// Mirrors the `GameController` framework property for `right_shoulder`.
    pub right_shoulder: ButtonInputState,
}

/// Mirrors the `GameController` framework counterpart for `MicroGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroGamepadDetails {
/// Mirrors the `GameController` framework property for `dpad`.
    pub dpad: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `button_a`.
    pub button_a: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_x`.
    pub button_x: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_menu`.
    pub button_menu: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `reports_absolute_dpad_values`.
    pub reports_absolute_dpad_values: bool,
/// Mirrors the `GameController` framework property for `allows_rotation`.
    pub allows_rotation: bool,
}

/// Mirrors the `GameController` framework counterpart for `DirectionalGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionalGamepadDetails {
/// Mirrors the `GameController` framework property for `dpad`.
    pub dpad: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `button_a`.
    pub button_a: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_x`.
    pub button_x: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_menu`.
    pub button_menu: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `reports_absolute_dpad_values`.
    pub reports_absolute_dpad_values: bool,
/// Mirrors the `GameController` framework property for `allows_rotation`.
    pub allows_rotation: bool,
}

/// Mirrors the `GameController` framework counterpart for `ExtendedGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedGamepadDetails {
/// Mirrors the `GameController` framework property for `dpad`.
    pub dpad: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `button_a`.
    pub button_a: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_b`.
    pub button_b: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_x`.
    pub button_x: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_y`.
    pub button_y: ButtonInputState,
/// Mirrors the `GameController` framework property for `button_menu`.
    pub button_menu: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `button_options`.
    pub button_options: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `button_home`.
    pub button_home: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `left_thumbstick`.
    pub left_thumbstick: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `right_thumbstick`.
    pub right_thumbstick: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `left_shoulder`.
    pub left_shoulder: ButtonInputState,
/// Mirrors the `GameController` framework property for `right_shoulder`.
    pub right_shoulder: ButtonInputState,
/// Mirrors the `GameController` framework property for `left_trigger`.
    pub left_trigger: ButtonInputState,
/// Mirrors the `GameController` framework property for `right_trigger`.
    pub right_trigger: ButtonInputState,
/// Mirrors the `GameController` framework property for `left_thumbstick_button`.
    pub left_thumbstick_button: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `right_thumbstick_button`.
    pub right_thumbstick_button: Option<ButtonInputState>,
}

/// Mirrors the `GameController` framework counterpart for `DualShockGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualShockGamepadDetails {
/// Mirrors the `GameController` framework property for `touchpad_button`.
    pub touchpad_button: ButtonInputState,
/// Mirrors the `GameController` framework property for `touchpad_primary`.
    pub touchpad_primary: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `touchpad_secondary`.
    pub touchpad_secondary: DirectionPadInputState,
}

/// Mirrors the `GameController` framework counterpart for `DualSenseAdaptiveTriggerState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualSenseAdaptiveTriggerState {
/// Mirrors the `GameController` framework property for `value`.
    pub value: f32,
/// Mirrors the `GameController` framework property for `pressed`.
    pub pressed: bool,
/// Mirrors the `GameController` framework property for `touched`.
    pub touched: bool,
/// Mirrors the `GameController` framework property for `mode`.
    pub mode: DualSenseTriggerMode,
/// Mirrors the `GameController` framework property for `status`.
    pub status: DualSenseTriggerStatus,
/// Mirrors the `GameController` framework property for `arm_position`.
    pub arm_position: f32,
}

/// Mirrors the `GameController` framework counterpart for `DualSenseGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualSenseGamepadDetails {
/// Mirrors the `GameController` framework property for `touchpad_button`.
    pub touchpad_button: ButtonInputState,
/// Mirrors the `GameController` framework property for `touchpad_primary`.
    pub touchpad_primary: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `touchpad_secondary`.
    pub touchpad_secondary: DirectionPadInputState,
/// Mirrors the `GameController` framework property for `left_trigger`.
    pub left_trigger: DualSenseAdaptiveTriggerState,
/// Mirrors the `GameController` framework property for `right_trigger`.
    pub right_trigger: DualSenseAdaptiveTriggerState,
}

/// Mirrors the `GameController` framework counterpart for `XboxGamepadDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XboxGamepadDetails {
/// Mirrors the `GameController` framework property for `paddle_button1`.
    pub paddle_button1: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `paddle_button2`.
    pub paddle_button2: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `paddle_button3`.
    pub paddle_button3: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `paddle_button4`.
    pub paddle_button4: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `button_share`.
    pub button_share: Option<ButtonInputState>,
}

/// Mirrors the `GameController` framework counterpart for `BatteryInfo`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryInfo {
/// Mirrors the `GameController` framework property for `level`.
    pub level: f32,
/// Mirrors the `GameController` framework property for `state`.
    pub state: BatteryState,
}

/// Mirrors the `GameController` framework counterpart for `DeviceLightDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceLightDetails {
/// Mirrors the `GameController` framework property for `color`.
    pub color: Color,
}

/// Mirrors the `GameController` framework counterpart for `DeviceHapticsDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceHapticsDetails {
/// Mirrors the `GameController` framework property for `supported_localities`.
    pub supported_localities: Vec<String>,
}

impl DeviceHapticsDetails {
    /// Return the supported localities as typed `GCHapticsLocality` constants when possible.
    #[must_use]
    pub fn supported_locality_constants(&self) -> Vec<GCHapticsLocality> {
        self.supported_localities
            .iter()
            .filter_map(|value| HapticsLocality::from_runtime_value(value))
            .collect()
    }

    /// Check whether the device reports support for a given locality.
    #[must_use]
    pub fn supports_locality(&self, locality: GCHapticsLocality) -> bool {
        self.supported_localities
            .iter()
            .any(|value| value == locality.as_ref())
    }
}

/// Mirrors the `GameController` framework counterpart for `DeviceDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDetails {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
/// Mirrors the `GameController` framework property for `handler_queue_label`.
    pub handler_queue_label: String,
}

/// Apple-style alias for the generic `GCDevice` protocol snapshot.
pub type GCDevice = DeviceDetails;

/// Mirrors the `GameController` framework counterpart for `ConnectedDevicesSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevicesSnapshot {
/// Mirrors the `GameController` framework property for `controllers`.
    pub controllers: Vec<DeviceDetails>,
/// Mirrors the `GameController` framework property for `keyboard`.
    pub keyboard: Option<DeviceDetails>,
/// Mirrors the `GameController` framework property for `mouse`.
    pub mouse: Option<DeviceDetails>,
/// Mirrors the `GameController` framework property for `racing_wheels`.
    pub racing_wheels: Vec<DeviceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `MotionDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct MotionDetails {
/// Mirrors the `GameController` framework property for `sensors_require_manual_activation`.
    pub sensors_require_manual_activation: bool,
/// Mirrors the `GameController` framework property for `sensors_active`.
    pub sensors_active: bool,
/// Mirrors the `GameController` framework property for `has_gravity_and_user_acceleration`.
    pub has_gravity_and_user_acceleration: bool,
/// Mirrors the `GameController` framework property for `gravity`.
    pub gravity: Vector3,
/// Mirrors the `GameController` framework property for `user_acceleration`.
    pub user_acceleration: Vector3,
/// Mirrors the `GameController` framework property for `acceleration`.
    pub acceleration: Vector3,
/// Mirrors the `GameController` framework property for `has_attitude`.
    pub has_attitude: bool,
/// Mirrors the `GameController` framework property for `has_rotation_rate`.
    pub has_rotation_rate: bool,
/// Mirrors the `GameController` framework property for `attitude`.
    pub attitude: Quaternion,
/// Mirrors the `GameController` framework property for `rotation_rate`.
    pub rotation_rate: Vector3,
}

/// Mirrors the `GameController` framework counterpart for `NamedButtonInputState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedButtonInputState {
/// Mirrors the `GameController` framework property for `alias`.
    pub alias: String,
/// Mirrors the `GameController` framework property for `value`.
    pub value: ButtonInputState,
}

/// Mirrors the `GameController` framework counterpart for `NamedAxisInputState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAxisInputState {
/// Mirrors the `GameController` framework property for `alias`.
    pub alias: String,
/// Mirrors the `GameController` framework property for `value`.
    pub value: AxisInputState,
}

/// Mirrors the `GameController` framework counterpart for `NamedDirectionPadState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedDirectionPadState {
/// Mirrors the `GameController` framework property for `alias`.
    pub alias: String,
/// Mirrors the `GameController` framework property for `value`.
    pub value: DirectionPadInputState,
}

/// Mirrors the `GameController` framework counterpart for `NamedTouchpadState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedTouchpadState {
/// Mirrors the `GameController` framework property for `alias`.
    pub alias: String,
/// Mirrors the `GameController` framework property for `value`.
    pub value: TouchpadDetails,
}

/// Mirrors the `GameController` framework counterpart for `PhysicalInputProfileDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputProfileDetails {
/// Mirrors the `GameController` framework property for `last_event_timestamp`.
    pub last_event_timestamp: f64,
/// Mirrors the `GameController` framework property for `has_remapped_elements`.
    pub has_remapped_elements: bool,
/// Mirrors the `GameController` framework property for `element_aliases`.
    pub element_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `button_aliases`.
    pub button_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `axis_aliases`.
    pub axis_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `dpad_aliases`.
    pub dpad_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `touchpad_aliases`.
    pub touchpad_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `buttons`.
    pub buttons: Vec<NamedButtonInputState>,
/// Mirrors the `GameController` framework property for `axes`.
    pub axes: Vec<NamedAxisInputState>,
/// Mirrors the `GameController` framework property for `dpads`.
    pub dpads: Vec<NamedDirectionPadState>,
/// Mirrors the `GameController` framework property for `touchpads`.
    pub touchpads: Vec<NamedTouchpadState>,
}

/// Mirrors the `GameController` framework counterpart for `AxisElementState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisElementState {
/// Mirrors the `GameController` framework property for `absolute_value`.
    pub absolute_value: Option<f32>,
/// Mirrors the `GameController` framework property for `relative_delta`.
    pub relative_delta: f32,
}

/// Mirrors the `GameController` framework counterpart for `SwitchInputState`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInputState {
/// Mirrors the `GameController` framework property for `position`.
    pub position: i32,
/// Mirrors the `GameController` framework property for `position_lower_bound`.
    pub position_lower_bound: i32,
/// Mirrors the `GameController` framework property for `position_count`.
    pub position_count: i32,
/// Mirrors the `GameController` framework property for `sequential`.
    pub sequential: bool,
/// Mirrors the `GameController` framework property for `can_wrap`.
    pub can_wrap: bool,
}

/// Apple-style alias for [`SwitchInputState`].
pub type GCSwitchInput = SwitchInputState;

/// Mirrors the `GameController` framework counterpart for `NamedButtonElementState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedButtonElementState {
/// Mirrors the `GameController` framework property for `primary_alias`.
    pub primary_alias: String,
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `value`.
    pub value: ButtonInputState,
}

/// Mirrors the `GameController` framework counterpart for `NamedAxisElementState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAxisElementState {
/// Mirrors the `GameController` framework property for `primary_alias`.
    pub primary_alias: String,
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `value`.
    pub value: AxisElementState,
}

/// Mirrors the `GameController` framework counterpart for `NamedSwitchElementState`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedSwitchElementState {
/// Mirrors the `GameController` framework property for `primary_alias`.
    pub primary_alias: String,
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `value`.
    pub value: SwitchInputState,
}

/// Mirrors the `GameController` framework counterpart for `NamedDirectionPadElementState`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedDirectionPadElementState {
/// Mirrors the `GameController` framework property for `primary_alias`.
    pub primary_alias: String,
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `value`.
    pub value: DirectionPadInputState,
}

/// Mirrors the `GameController` framework counterpart for `InputElementMetadata`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputElementMetadata {
/// Mirrors the `GameController` framework property for `primary_alias`.
    pub primary_alias: String,
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
}

/// Mirrors the `GameController` framework counterpart for `ControllerElementDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerElementDetails {
/// Mirrors the `GameController` framework property for `analog`.
    pub analog: bool,
/// Mirrors the `GameController` framework property for `collection_path`.
    pub collection_path: Vec<String>,
/// Mirrors the `GameController` framework property for `is_bound_to_system_gesture`.
    pub is_bound_to_system_gesture: bool,
/// Mirrors the `GameController` framework property for `preferred_system_gesture_state`.
    pub preferred_system_gesture_state: SystemGestureState,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `unmapped_sf_symbols_name`.
    pub unmapped_sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `unmapped_localized_name`.
    pub unmapped_localized_name: Option<String>,
}

/// Apple-style alias for the legacy `GCControllerElement` base class.
pub type GCControllerElement = ControllerElementDetails;

/// Mirrors the `GameController` framework counterpart for `NamedControllerElementDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedControllerElementDetails {
/// Mirrors the `GameController` framework property for `name`.
    pub name: String,
/// Mirrors the `GameController` framework property for `value`.
    pub value: ControllerElementDetails,
}

/// Mirrors the `GameController` framework counterpart for `PhysicalInputSourceDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputSourceDetails {
/// Mirrors the `GameController` framework property for `element_aliases`.
    pub element_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `element_localized_name`.
    pub element_localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `direction`.
    pub direction: PhysicalInputSourceDirection,
}

/// Apple-style alias for `GCPhysicalInputSource` snapshots.
pub type GCPhysicalInputSource = PhysicalInputSourceDetails;

/// Mirrors the `GameController` framework counterpart for `PhysicalInputExtentsDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputExtentsDetails {
/// Mirrors the `GameController` framework property for `scaled_value`.
    pub scaled_value: f64,
/// Mirrors the `GameController` framework property for `minimum_value`.
    pub minimum_value: f64,
/// Mirrors the `GameController` framework property for `maximum_value`.
    pub maximum_value: f64,
}

/// Apple-style alias for `GCPhysicalInputExtents` snapshots.
pub type GCPhysicalInputExtents = PhysicalInputExtentsDetails;

/// Mirrors the `GameController` framework counterpart for `LinearInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearInputDetails {
/// Mirrors the `GameController` framework property for `value`.
    pub value: f32,
/// Mirrors the `GameController` framework property for `analog`.
    pub analog: bool,
/// Mirrors the `GameController` framework property for `can_wrap`.
    pub can_wrap: bool,
/// Mirrors the `GameController` framework property for `last_value_timestamp`.
    pub last_value_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_value_latency`.
    pub last_value_latency: f64,
/// Mirrors the `GameController` framework property for `physical_extents`.
    pub physical_extents: Option<PhysicalInputExtentsDetails>,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `PressedStateInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressedStateInputDetails {
/// Mirrors the `GameController` framework property for `pressed`.
    pub pressed: bool,
/// Mirrors the `GameController` framework property for `last_pressed_state_timestamp`.
    pub last_pressed_state_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_pressed_state_latency`.
    pub last_pressed_state_latency: f64,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `TouchedStateInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchedStateInputDetails {
/// Mirrors the `GameController` framework property for `touched`.
    pub touched: bool,
/// Mirrors the `GameController` framework property for `last_touched_state_timestamp`.
    pub last_touched_state_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_touched_state_latency`.
    pub last_touched_state_latency: f64,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `RelativeInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeInputDetails {
/// Mirrors the `GameController` framework property for `delta`.
    pub delta: f32,
/// Mirrors the `GameController` framework property for `analog`.
    pub analog: bool,
/// Mirrors the `GameController` framework property for `last_delta_timestamp`.
    pub last_delta_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_delta_latency`.
    pub last_delta_latency: f64,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `AxisInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisInputDetails {
/// Mirrors the `GameController` framework property for `value`.
    pub value: f32,
/// Mirrors the `GameController` framework property for `analog`.
    pub analog: bool,
/// Mirrors the `GameController` framework property for `can_wrap`.
    pub can_wrap: bool,
/// Mirrors the `GameController` framework property for `last_value_timestamp`.
    pub last_value_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_value_latency`.
    pub last_value_latency: f64,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Apple-style alias for the generic `GCAxisInput` protocol snapshot.
pub type GCAxisInput = AxisInputDetails;

/// Mirrors the `GameController` framework counterpart for `Axis2DInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis2DInputDetails {
/// Mirrors the `GameController` framework property for `value`.
    pub value: GCPoint2,
/// Mirrors the `GameController` framework property for `analog`.
    pub analog: bool,
/// Mirrors the `GameController` framework property for `can_wrap`.
    pub can_wrap: bool,
/// Mirrors the `GameController` framework property for `last_value_timestamp`.
    pub last_value_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_value_latency`.
    pub last_value_latency: f64,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Apple-style alias for the generic `GCAxis2DInput` protocol snapshot.
pub type GCAxis2DInput = Axis2DInputDetails;

/// Mirrors the `GameController` framework counterpart for `SwitchPositionInputDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionInputDetails {
/// Mirrors the `GameController` framework property for `position`.
    pub position: i32,
/// Mirrors the `GameController` framework property for `position_lower_bound`.
    pub position_lower_bound: i32,
/// Mirrors the `GameController` framework property for `position_count`.
    pub position_count: i32,
/// Mirrors the `GameController` framework property for `sequential`.
    pub sequential: bool,
/// Mirrors the `GameController` framework property for `can_wrap`.
    pub can_wrap: bool,
/// Mirrors the `GameController` framework property for `sources`.
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Mirrors the `GameController` framework counterpart for `LinearPressedInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearPressedInputDetails {
/// Mirrors the `GameController` framework property for `linear_input`.
    pub linear_input: LinearInputDetails,
/// Mirrors the `GameController` framework property for `pressed_state`.
    pub pressed_state: PressedStateInputDetails,
}

/// Mirrors the `GameController` framework counterpart for `ButtonElementDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonElementDetails {
/// Mirrors the `GameController` framework property for `metadata`.
    pub metadata: InputElementMetadata,
/// Mirrors the `GameController` framework property for `linear_input`.
    pub linear_input: LinearInputDetails,
/// Mirrors the `GameController` framework property for `pressed_state`.
    pub pressed_state: PressedStateInputDetails,
/// Mirrors the `GameController` framework property for `touched_state`.
    pub touched_state: Option<TouchedStateInputDetails>,
/// Mirrors the `GameController` framework property for `force_input`.
    pub force_input: Option<LinearInputDetails>,
}

/// Mirrors the `GameController` framework counterpart for `AxisElementDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisElementDetails {
/// Mirrors the `GameController` framework property for `metadata`.
    pub metadata: InputElementMetadata,
/// Mirrors the `GameController` framework property for `absolute_input`.
    pub absolute_input: Option<AxisInputDetails>,
/// Mirrors the `GameController` framework property for `relative_input`.
    pub relative_input: RelativeInputDetails,
}

/// Mirrors the `GameController` framework counterpart for `SwitchElementDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchElementDetails {
/// Mirrors the `GameController` framework property for `metadata`.
    pub metadata: InputElementMetadata,
/// Mirrors the `GameController` framework property for `position_input`.
    pub position_input: SwitchPositionInputDetails,
}

/// Mirrors the `GameController` framework counterpart for `DirectionPadElementDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionPadElementDetails {
/// Mirrors the `GameController` framework property for `metadata`.
    pub metadata: InputElementMetadata,
/// Mirrors the `GameController` framework property for `xy_axes`.
    pub xy_axes: Option<Axis2DInputDetails>,
/// Mirrors the `GameController` framework property for `x_axis`.
    pub x_axis: AxisInputDetails,
/// Mirrors the `GameController` framework property for `y_axis`.
    pub y_axis: AxisInputDetails,
/// Mirrors the `GameController` framework property for `up`.
    pub up: LinearPressedInputDetails,
/// Mirrors the `GameController` framework property for `down`.
    pub down: LinearPressedInputDetails,
/// Mirrors the `GameController` framework property for `left`.
    pub left: LinearPressedInputDetails,
/// Mirrors the `GameController` framework property for `right`.
    pub right: LinearPressedInputDetails,
}

/// Rust alias mirroring the `GCPhysicalInputElementCollection` concept with `Vec<T>` semantics.
pub type GCPhysicalInputElementCollection<T> = Vec<T>;

/// Mirrors the `GameController` framework counterpart for `PhysicalInputElementSnapshot`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputElementSnapshot {
/// Mirrors the `GameController` framework property for `buttons`.
    pub buttons: GCPhysicalInputElementCollection<ButtonElementDetails>,
/// Mirrors the `GameController` framework property for `axes`.
    pub axes: GCPhysicalInputElementCollection<AxisElementDetails>,
/// Mirrors the `GameController` framework property for `switches`.
    pub switches: GCPhysicalInputElementCollection<SwitchElementDetails>,
/// Mirrors the `GameController` framework property for `dpads`.
    pub dpads: GCPhysicalInputElementCollection<DirectionPadElementDetails>,
}

/// Mirrors the `GameController` framework counterpart for `DevicePhysicalInputElementChange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DevicePhysicalInputElementChange {
/// Mirrors the `GameController` framework case `Unknown`.
    Unknown,
/// Mirrors the `GameController` framework case `NoChange`.
    NoChange,
/// Mirrors the `GameController` framework case `Changed`.
    Changed,
}

/// Apple-style alias for [`DevicePhysicalInputElementChange`].
pub type GCDevicePhysicalInputElementChange = DevicePhysicalInputElementChange;

/// Mirrors the `GameController` framework counterpart for `DevicePhysicalInputStateDiffDetails`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePhysicalInputStateDiffDetails {
/// Mirrors the `GameController` framework property for `changed_elements_known`.
    pub changed_elements_known: bool,
/// Mirrors the `GameController` framework property for `changed_aliases`.
    pub changed_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `changed_elements`.
    pub changed_elements: Vec<InputElementMetadata>,
}

impl DevicePhysicalInputStateDiffDetails {
    /// Mirror `GCDevicePhysicalInputStateDiff.changeForElement(_:)` using aliases.
    #[must_use]
    pub fn change_for_alias(&self, alias: &str) -> DevicePhysicalInputElementChange {
        if !self.changed_elements_known {
            return DevicePhysicalInputElementChange::Unknown;
        }
        if self
            .changed_aliases
            .iter()
            .any(|candidate| candidate == alias)
        {
            DevicePhysicalInputElementChange::Changed
        } else {
            DevicePhysicalInputElementChange::NoChange
        }
    }
}

/// Apple-style alias for [`DevicePhysicalInputStateDiffDetails`].
pub type GCDevicePhysicalInputStateDiff = DevicePhysicalInputStateDiffDetails;

/// Mirrors the `GameController` framework counterpart for `ControllerInputStateDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerInputStateDetails {
/// Mirrors the `GameController` framework property for `last_event_timestamp`.
    pub last_event_timestamp: f64,
/// Mirrors the `GameController` framework property for `last_event_latency`.
    pub last_event_latency: f64,
/// Mirrors the `GameController` framework property for `element_count`.
    pub element_count: usize,
/// Mirrors the `GameController` framework property for `button_count`.
    pub button_count: usize,
/// Mirrors the `GameController` framework property for `axis_count`.
    pub axis_count: usize,
/// Mirrors the `GameController` framework property for `switch_count`.
    pub switch_count: usize,
/// Mirrors the `GameController` framework property for `dpad_count`.
    pub dpad_count: usize,
/// Mirrors the `GameController` framework property for `buttons`.
    pub buttons: Vec<NamedButtonElementState>,
/// Mirrors the `GameController` framework property for `axes`.
    pub axes: Vec<NamedAxisElementState>,
/// Mirrors the `GameController` framework property for `switches`.
    pub switches: Vec<NamedSwitchElementState>,
/// Mirrors the `GameController` framework property for `dpads`.
    pub dpads: Vec<NamedDirectionPadElementState>,
/// Mirrors the `GameController` framework property for `changed_aliases_known`.
    pub changed_aliases_known: bool,
/// Mirrors the `GameController` framework property for `changed_aliases`.
    pub changed_aliases: Vec<String>,
}

/// Mirrors the `GameController` framework counterpart for `ControllerLiveInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerLiveInputDetails {
/// Mirrors the `GameController` framework property for `input_state_queue_depth`.
    pub input_state_queue_depth: isize,
/// Mirrors the `GameController` framework property for `live`.
    pub live: ControllerInputStateDetails,
/// Mirrors the `GameController` framework property for `unmapped`.
    pub unmapped: Option<ControllerInputStateDetails>,
/// Mirrors the `GameController` framework property for `next`.
    pub next: Option<ControllerInputStateDetails>,
}

/// Generic `GCDevicePhysicalInputState` view reusing the controller-input snapshot.
pub type DevicePhysicalInputState = ControllerInputStateDetails;
/// Apple-style alias for [`DevicePhysicalInputState`].
pub type GCDevicePhysicalInputState = DevicePhysicalInputState;

/// Mirrors the `GameController` framework counterpart for `DevicePhysicalInputSourceDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePhysicalInputSourceDetails {
/// Mirrors the `GameController` framework property for `input_state_queue_depth`.
    pub input_state_queue_depth: isize,
/// Mirrors the `GameController` framework property for `live`.
    pub live: DevicePhysicalInputState,
/// Mirrors the `GameController` framework property for `capture`.
    pub capture: DevicePhysicalInputState,
/// Mirrors the `GameController` framework property for `unmapped`.
    pub unmapped: Option<DevicePhysicalInputState>,
/// Mirrors the `GameController` framework property for `next`.
    pub next: Option<DevicePhysicalInputState>,
/// Mirrors the `GameController` framework property for `next_diff`.
    pub next_diff: Option<DevicePhysicalInputStateDiffDetails>,
}

/// Generic `GCDevicePhysicalInput` view for the current controller input source.
pub type DevicePhysicalInput = DevicePhysicalInputSourceDetails;
/// Apple-style alias for [`DevicePhysicalInput`].
pub type GCDevicePhysicalInput = DevicePhysicalInput;

/// Mirrors the `GameController` framework counterpart for `ControllerDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct ControllerDetails {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
/// Mirrors the `GameController` framework property for `player_index`.
    pub player_index: i32,
/// Mirrors the `GameController` framework property for `is_attached_to_device`.
    pub is_attached_to_device: bool,
/// Mirrors the `GameController` framework property for `is_current`.
    pub is_current: bool,
/// Mirrors the `GameController` framework property for `supports_background_events`.
    pub supports_background_events: bool,
/// Mirrors the `GameController` framework property for `has_live_input`.
    pub has_live_input: bool,
/// Mirrors the `GameController` framework property for `gamepad`.
    pub gamepad: Option<GamepadDetails>,
/// Mirrors the `GameController` framework property for `micro_gamepad`.
    pub micro_gamepad: Option<MicroGamepadDetails>,
/// Mirrors the `GameController` framework property for `directional_gamepad`.
    pub directional_gamepad: Option<DirectionalGamepadDetails>,
/// Mirrors the `GameController` framework property for `extended_gamepad`.
    pub extended_gamepad: Option<ExtendedGamepadDetails>,
/// Mirrors the `GameController` framework property for `dual_shock`.
    pub dual_shock: Option<DualShockGamepadDetails>,
/// Mirrors the `GameController` framework property for `dual_sense`.
    pub dual_sense: Option<DualSenseGamepadDetails>,
/// Mirrors the `GameController` framework property for `xbox`.
    pub xbox: Option<XboxGamepadDetails>,
/// Mirrors the `GameController` framework property for `battery`.
    pub battery: Option<BatteryInfo>,
/// Mirrors the `GameController` framework property for `motion`.
    pub motion: Option<MotionDetails>,
/// Mirrors the `GameController` framework property for `has_light`.
    pub has_light: bool,
/// Mirrors the `GameController` framework property for `light`.
    pub light: Option<DeviceLightDetails>,
/// Mirrors the `GameController` framework property for `has_haptics`.
    pub has_haptics: bool,
/// Mirrors the `GameController` framework property for `haptics`.
    pub haptics: Option<DeviceHapticsDetails>,
/// Mirrors the `GameController` framework property for `physical_input`.
    pub physical_input: Option<PhysicalInputProfileDetails>,
/// Mirrors the `GameController` framework property for `input`.
    pub input: Option<ControllerLiveInputDetails>,
}

/// Mirrors the `GameController` framework counterpart for `KeyboardSnapshot`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardSnapshot {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
/// Mirrors the `GameController` framework property for `any_key_pressed`.
    pub any_key_pressed: bool,
/// Mirrors the `GameController` framework property for `pressed_aliases`.
    pub pressed_aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `pressed_keys`.
    pub pressed_keys: Vec<NamedButtonInputState>,
/// Mirrors the `GameController` framework property for `physical_input`.
    pub physical_input: PhysicalInputProfileDetails,
}

/// Mirrors the `GameController` framework counterpart for `MouseSnapshot`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseSnapshot {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
/// Mirrors the `GameController` framework property for `is_current`.
    pub is_current: bool,
/// Mirrors the `GameController` framework property for `known_mouse_count`.
    pub known_mouse_count: usize,
/// Mirrors the `GameController` framework property for `scroll`.
    pub scroll: DeviceCursorState,
/// Mirrors the `GameController` framework property for `left_button`.
    pub left_button: ButtonInputState,
/// Mirrors the `GameController` framework property for `right_button`.
    pub right_button: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `middle_button`.
    pub middle_button: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `auxiliary_buttons`.
    pub auxiliary_buttons: Vec<NamedButtonInputState>,
/// Mirrors the `GameController` framework property for `physical_input`.
    pub physical_input: PhysicalInputProfileDetails,
}

/// Mirrors the `GameController` framework counterpart for `EventViewControllerDetails`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventViewControllerDetails {
/// Mirrors the `GameController` framework property for `controller_user_interaction_enabled`.
    pub controller_user_interaction_enabled: bool,
}

/// Mirrors the `GameController` framework counterpart for `SteeringWheelDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteeringWheelDetails {
/// Mirrors the `GameController` framework property for `maximum_degrees_of_rotation`.
    pub maximum_degrees_of_rotation: f32,
/// Mirrors the `GameController` framework property for `absolute_value`.
    pub absolute_value: Option<f32>,
/// Mirrors the `GameController` framework property for `relative_delta`.
    pub relative_delta: f32,
}

/// Mirrors the `GameController` framework counterpart for `GearShifterDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearShifterDetails {
/// Mirrors the `GameController` framework property for `aliases`.
    pub aliases: Vec<String>,
/// Mirrors the `GameController` framework property for `localized_name`.
    pub localized_name: Option<String>,
/// Mirrors the `GameController` framework property for `sf_symbols_name`.
    pub sf_symbols_name: Option<String>,
/// Mirrors the `GameController` framework property for `pattern_position`.
    pub pattern_position: Option<i32>,
/// Mirrors the `GameController` framework property for `pattern_lower_bound`.
    pub pattern_lower_bound: Option<i32>,
/// Mirrors the `GameController` framework property for `pattern_count`.
    pub pattern_count: Option<i32>,
/// Mirrors the `GameController` framework property for `pattern_sequential`.
    pub pattern_sequential: Option<bool>,
/// Mirrors the `GameController` framework property for `pattern_can_wrap`.
    pub pattern_can_wrap: Option<bool>,
/// Mirrors the `GameController` framework property for `sequential_delta`.
    pub sequential_delta: Option<f32>,
}

/// Mirrors the `GameController` framework counterpart for `RacingWheelInputDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacingWheelInputDetails {
/// Mirrors the `GameController` framework property for `wheel`.
    pub wheel: SteeringWheelDetails,
/// Mirrors the `GameController` framework property for `accelerator_pedal`.
    pub accelerator_pedal: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `brake_pedal`.
    pub brake_pedal: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `clutch_pedal`.
    pub clutch_pedal: Option<ButtonInputState>,
/// Mirrors the `GameController` framework property for `shifter`.
    pub shifter: Option<GearShifterDetails>,
}

/// Mirrors the `GameController` framework counterpart for `RacingWheelDetails`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacingWheelDetails {
/// Mirrors the `GameController` framework property for `vendor_name`.
    pub vendor_name: String,
/// Mirrors the `GameController` framework property for `product_category`.
    pub product_category: String,
/// Mirrors the `GameController` framework property for `is_acquired`.
    pub is_acquired: bool,
/// Mirrors the `GameController` framework property for `is_snapshot`.
    pub is_snapshot: bool,
/// Mirrors the `GameController` framework property for `wheel_input`.
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

/// Snapshot the current controller's `GCDevicePhysicalInput` source, including a
/// direct live read, a captured snapshot, and the next queued diff if one is
/// pending.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_input_source(
) -> Result<Option<DevicePhysicalInputSourceDetails>, GameControllerError> {
    parse_json(unsafe { ffi::gc_current_controller_input_source_json() })
}

/// Snapshot every connected `GCDevice`-conforming object that the crate can see.
///
/// This includes controllers, the coalesced keyboard, the current mouse, and
/// connected racing wheels.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn connected_devices_snapshot() -> Result<ConnectedDevicesSnapshot, GameControllerError> {
    parse_json(unsafe { ffi::gc_connected_devices_json() })
}

/// Snapshot legacy `GCControllerElement` metadata for the current controller's
/// visible profile elements.
///
/// The returned list is empty when there is no current controller.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_elements(
) -> Result<Vec<NamedControllerElementDetails>, GameControllerError> {
    parse_json(unsafe { ffi::gc_current_controller_elements_json() })
}

/// Snapshot the current controller's physical-input element collections, including
/// axis/source/extents metadata when the OS exposes it.
///
/// # Errors
///
/// Returns an error if the Swift bridge returns invalid UTF-8 or malformed JSON.
pub fn current_controller_physical_input_elements(
) -> Result<Option<PhysicalInputElementSnapshot>, GameControllerError> {
    parse_json(unsafe { ffi::gc_current_controller_physical_input_elements_json() })
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
