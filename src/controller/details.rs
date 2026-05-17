use core::ffi::CStr;
use std::ffi::c_char;

use serde::{de::DeserializeOwned, Deserialize};

use crate::error::GameControllerError;
use crate::ffi;

use super::{
    BatteryState, EulerAngles, GCHapticsLocality, GCPoint2, HapticsLocality,
    PhysicalInputSourceDirection, SystemGestureState,
};

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

/// Typed cursor snapshot mirroring `GCDeviceCursor`.
pub type DeviceCursorState = DirectionPadInputState;
/// Apple-style alias for [`DeviceCursorState`].
pub type GCDeviceCursor = DeviceCursorState;

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDetails {
    pub vendor_name: String,
    pub product_category: String,
    pub handler_queue_label: String,
}

/// Apple-style alias for the generic `GCDevice` protocol snapshot.
pub type GCDevice = DeviceDetails;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedDevicesSnapshot {
    pub controllers: Vec<DeviceDetails>,
    pub keyboard: Option<DeviceDetails>,
    pub mouse: Option<DeviceDetails>,
    pub racing_wheels: Vec<DeviceDetails>,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputElementMetadata {
    pub primary_alias: String,
    pub aliases: Vec<String>,
    pub localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerElementDetails {
    pub analog: bool,
    pub collection_path: Vec<String>,
    pub is_bound_to_system_gesture: bool,
    pub preferred_system_gesture_state: SystemGestureState,
    pub sf_symbols_name: Option<String>,
    pub localized_name: Option<String>,
    pub unmapped_sf_symbols_name: Option<String>,
    pub unmapped_localized_name: Option<String>,
}

/// Apple-style alias for the legacy `GCControllerElement` base class.
pub type GCControllerElement = ControllerElementDetails;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedControllerElementDetails {
    pub name: String,
    pub value: ControllerElementDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputSourceDetails {
    pub element_aliases: Vec<String>,
    pub element_localized_name: Option<String>,
    pub sf_symbols_name: Option<String>,
    pub direction: PhysicalInputSourceDirection,
}

/// Apple-style alias for `GCPhysicalInputSource` snapshots.
pub type GCPhysicalInputSource = PhysicalInputSourceDetails;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputExtentsDetails {
    pub scaled_value: f64,
    pub minimum_value: f64,
    pub maximum_value: f64,
}

/// Apple-style alias for `GCPhysicalInputExtents` snapshots.
pub type GCPhysicalInputExtents = PhysicalInputExtentsDetails;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearInputDetails {
    pub value: f32,
    pub analog: bool,
    pub can_wrap: bool,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub physical_extents: Option<PhysicalInputExtentsDetails>,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressedStateInputDetails {
    pub pressed: bool,
    pub last_pressed_state_timestamp: f64,
    pub last_pressed_state_latency: f64,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchedStateInputDetails {
    pub touched: bool,
    pub last_touched_state_timestamp: f64,
    pub last_touched_state_latency: f64,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeInputDetails {
    pub delta: f32,
    pub analog: bool,
    pub last_delta_timestamp: f64,
    pub last_delta_latency: f64,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisInputDetails {
    pub value: f32,
    pub analog: bool,
    pub can_wrap: bool,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Apple-style alias for the generic `GCAxisInput` protocol snapshot.
pub type GCAxisInput = AxisInputDetails;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis2DInputDetails {
    pub value: GCPoint2,
    pub analog: bool,
    pub can_wrap: bool,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

/// Apple-style alias for the generic `GCAxis2DInput` protocol snapshot.
pub type GCAxis2DInput = Axis2DInputDetails;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionInputDetails {
    pub position: i32,
    pub position_lower_bound: i32,
    pub position_count: i32,
    pub sequential: bool,
    pub can_wrap: bool,
    pub sources: Vec<PhysicalInputSourceDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearPressedInputDetails {
    pub linear_input: LinearInputDetails,
    pub pressed_state: PressedStateInputDetails,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonElementDetails {
    pub metadata: InputElementMetadata,
    pub linear_input: LinearInputDetails,
    pub pressed_state: PressedStateInputDetails,
    pub touched_state: Option<TouchedStateInputDetails>,
    pub force_input: Option<LinearInputDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisElementDetails {
    pub metadata: InputElementMetadata,
    pub absolute_input: Option<AxisInputDetails>,
    pub relative_input: RelativeInputDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchElementDetails {
    pub metadata: InputElementMetadata,
    pub position_input: SwitchPositionInputDetails,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionPadElementDetails {
    pub metadata: InputElementMetadata,
    pub xy_axes: Option<Axis2DInputDetails>,
    pub x_axis: AxisInputDetails,
    pub y_axis: AxisInputDetails,
    pub up: LinearPressedInputDetails,
    pub down: LinearPressedInputDetails,
    pub left: LinearPressedInputDetails,
    pub right: LinearPressedInputDetails,
}

/// Rust alias mirroring the `GCPhysicalInputElementCollection` concept with `Vec<T>` semantics.
pub type GCPhysicalInputElementCollection<T> = Vec<T>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInputElementSnapshot {
    pub buttons: GCPhysicalInputElementCollection<ButtonElementDetails>,
    pub axes: GCPhysicalInputElementCollection<AxisElementDetails>,
    pub switches: GCPhysicalInputElementCollection<SwitchElementDetails>,
    pub dpads: GCPhysicalInputElementCollection<DirectionPadElementDetails>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DevicePhysicalInputElementChange {
    Unknown,
    NoChange,
    Changed,
}

/// Apple-style alias for [`DevicePhysicalInputElementChange`].
pub type GCDevicePhysicalInputElementChange = DevicePhysicalInputElementChange;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePhysicalInputStateDiffDetails {
    pub changed_elements_known: bool,
    pub changed_aliases: Vec<String>,
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

/// Generic `GCDevicePhysicalInputState` view reusing the controller-input snapshot.
pub type DevicePhysicalInputState = ControllerInputStateDetails;
/// Apple-style alias for [`DevicePhysicalInputState`].
pub type GCDevicePhysicalInputState = DevicePhysicalInputState;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePhysicalInputSourceDetails {
    pub input_state_queue_depth: isize,
    pub live: DevicePhysicalInputState,
    pub capture: DevicePhysicalInputState,
    pub unmapped: Option<DevicePhysicalInputState>,
    pub next: Option<DevicePhysicalInputState>,
    pub next_diff: Option<DevicePhysicalInputStateDiffDetails>,
}

/// Generic `GCDevicePhysicalInput` view for the current controller input source.
pub type DevicePhysicalInput = DevicePhysicalInputSourceDetails;
/// Apple-style alias for [`DevicePhysicalInput`].
pub type GCDevicePhysicalInput = DevicePhysicalInput;

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
    pub scroll: DeviceCursorState,
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
