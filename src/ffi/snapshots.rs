#![allow(clippy::struct_field_names)]
use crate::controller::{
    Axis2DInputDetails, AxisElementDetails, AxisElementState, AxisInputDetails, AxisInputState,
    BatteryInfo, BatteryState, ButtonElementDetails, ButtonInputState, Color,
    ConnectedDevicesSnapshot, ControllerDetails, ControllerElementDetails,
    ControllerInputStateDetails, ControllerLiveInputDetails, DeviceCursorState, DeviceDetails,
    DeviceHapticsDetails, DeviceLightDetails, DevicePhysicalInputSourceDetails,
    DevicePhysicalInputState, DevicePhysicalInputStateDiffDetails, DirectionPadElementDetails,
    DirectionPadInputState, DirectionalGamepadDetails, DualSenseAdaptiveTriggerState,
    DualSenseGamepadDetails, DualSenseTriggerMode, DualSenseTriggerStatus, DualShockGamepadDetails,
    EventViewControllerDetails, ExtendedGamepadDetails, GCPoint2, GamepadDetails,
    GearShifterDetails, InputElementMetadata, KeyboardSnapshot, LinearInputDetails,
    LinearPressedInputDetails, MicroGamepadDetails, MotionDetails, MouseSnapshot,
    NamedAxisElementState, NamedAxisInputState, NamedButtonElementState, NamedButtonInputState,
    NamedControllerElementDetails, NamedDirectionPadElementState, NamedDirectionPadState,
    NamedSwitchElementState, NamedTouchpadState, PhysicalInputElementSnapshot,
    PhysicalInputExtentsDetails, PhysicalInputProfileDetails, PhysicalInputSourceDetails,
    PhysicalInputSourceDirection, Point2, PressedStateInputDetails, Quaternion, RacingWheelDetails,
    RacingWheelInputDetails, RelativeInputDetails, SteeringWheelDetails, SwitchElementDetails,
    SwitchInputState, SwitchPositionInputDetails, SystemGestureState, TouchState,
    TouchedStateInputDetails, TouchpadDetails, Vector3, XboxGamepadDetails,
};
use crate::error::GameControllerError;
use core::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCStringRaw {
    pub data: *const u8,
    pub len: usize,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCSliceRaw {
    pub data: *const c_void,
    pub len: usize,
}
#[repr(C)]
pub struct GCSnapshotRaw {
    pub data: *const c_void,
    pub len: usize,
    pub owner: *mut c_void,
}
pub trait FromRaw: Sized {
    type Raw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError>;
}
unsafe fn slice<'a, T>(raw: GCSliceRaw) -> Result<&'a [T], GameControllerError> {
    if raw.len == 0 {
        return Ok(&[]);
    }
    if raw.data.is_null()
        || raw.len > isize::MAX as usize / core::mem::size_of::<T>()
        || (raw.data as usize) % core::mem::align_of::<T>() != 0
    {
        return Err(GameControllerError::InvalidBridgeData);
    }
    Ok(unsafe { core::slice::from_raw_parts(raw.data.cast(), raw.len) })
}
impl FromRaw for String {
    type Raw = GCStringRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        let bytes = unsafe {
            slice::<u8>(GCSliceRaw {
                data: raw.data.cast(),
                len: raw.len,
            })
        }?;
        Ok(core::str::from_utf8(bytes)?.to_owned())
    }
}
unsafe fn array<T: FromRaw>(raw: GCSliceRaw) -> Result<Vec<T>, GameControllerError> {
    unsafe { slice::<T::Raw>(raw) }?
        .iter()
        .map(|value| unsafe { T::from_raw(value) })
        .collect()
}
struct Snapshot(GCSnapshotRaw);
impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe { gc_snapshot_free(self.0.owner) };
    }
}
pub fn read<T: FromRaw>(
    query: impl FnOnce(*mut GCSnapshotRaw),
) -> Result<Vec<T>, GameControllerError> {
    let mut snapshot = Snapshot(GCSnapshotRaw {
        data: core::ptr::null(),
        len: 0,
        owner: core::ptr::null_mut(),
    });
    query(&raw mut snapshot.0);
    if snapshot.0.owner.is_null() {
        return Err(GameControllerError::NullBridgeResponse);
    }
    unsafe {
        array::<T>(GCSliceRaw {
            data: snapshot.0.data,
            len: snapshot.0.len,
        })
    }
}
unsafe extern "C" {
    fn gc_snapshot_free(owner: *mut c_void);
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCVector3Raw {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl FromRaw for Vector3 {
    type Raw = GCVector3Raw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCQuaternionRaw {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl FromRaw for Quaternion {
    type Raw = GCQuaternionRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            x: raw.x,
            y: raw.y,
            z: raw.z,
            w: raw.w,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCColorRaw {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl FromRaw for Color {
    type Raw = GCColorRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            red: raw.red,
            green: raw.green,
            blue: raw.blue,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCButtonInputStateRaw {
    pub value: f32,
    pub pressed: u8,
    pub touched: u8,
}
impl FromRaw for ButtonInputState {
    type Raw = GCButtonInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            value: raw.value,
            pressed: raw.pressed != 0,
            touched: raw.touched != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCAxisInputStateRaw {
    pub value: f32,
}
impl FromRaw for AxisInputState {
    type Raw = GCAxisInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self { value: raw.value })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDirectionPadInputStateRaw {
    pub x: f32,
    pub y: f32,
    pub up: GCButtonInputStateRaw,
    pub down: GCButtonInputStateRaw,
    pub left: GCButtonInputStateRaw,
    pub right: GCButtonInputStateRaw,
}
impl FromRaw for DirectionPadInputState {
    type Raw = GCDirectionPadInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            x: raw.x,
            y: raw.y,
            up: unsafe { ButtonInputState::from_raw(&raw.up) }?,
            down: unsafe { ButtonInputState::from_raw(&raw.down) }?,
            left: unsafe { ButtonInputState::from_raw(&raw.left) }?,
            right: unsafe { ButtonInputState::from_raw(&raw.right) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCTouchpadDetailsRaw {
    pub button: GCButtonInputStateRaw,
    pub touch_surface: GCDirectionPadInputStateRaw,
    pub touch_state: i32,
    pub reports_absolute_touch_surface_values: u8,
}
impl FromRaw for TouchpadDetails {
    type Raw = GCTouchpadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            button: unsafe { ButtonInputState::from_raw(&raw.button) }?,
            touch_surface: unsafe { DirectionPadInputState::from_raw(&raw.touch_surface) }?,
            touch_state: match raw.touch_state {
                0 => TouchState::Up,
                1 => TouchState::Down,
                2 => TouchState::Moving,
                _ => return Err(GameControllerError::InvalidBridgeData),
            },
            reports_absolute_touch_surface_values: raw.reports_absolute_touch_surface_values != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCGamepadDetailsRaw {
    pub dpad: GCDirectionPadInputStateRaw,
    pub button_a: GCButtonInputStateRaw,
    pub button_b: GCButtonInputStateRaw,
    pub button_x: GCButtonInputStateRaw,
    pub button_y: GCButtonInputStateRaw,
    pub left_shoulder: GCButtonInputStateRaw,
    pub right_shoulder: GCButtonInputStateRaw,
}
impl FromRaw for GamepadDetails {
    type Raw = GCGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            dpad: unsafe { DirectionPadInputState::from_raw(&raw.dpad) }?,
            button_a: unsafe { ButtonInputState::from_raw(&raw.button_a) }?,
            button_b: unsafe { ButtonInputState::from_raw(&raw.button_b) }?,
            button_x: unsafe { ButtonInputState::from_raw(&raw.button_x) }?,
            button_y: unsafe { ButtonInputState::from_raw(&raw.button_y) }?,
            left_shoulder: unsafe { ButtonInputState::from_raw(&raw.left_shoulder) }?,
            right_shoulder: unsafe { ButtonInputState::from_raw(&raw.right_shoulder) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCMicroGamepadDetailsRaw {
    pub dpad: GCDirectionPadInputStateRaw,
    pub button_a: GCButtonInputStateRaw,
    pub button_x: GCButtonInputStateRaw,
    pub button_menu: *const GCButtonInputStateRaw,
    pub reports_absolute_dpad_values: u8,
    pub allows_rotation: u8,
}
impl FromRaw for MicroGamepadDetails {
    type Raw = GCMicroGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            dpad: unsafe { DirectionPadInputState::from_raw(&raw.dpad) }?,
            button_a: unsafe { ButtonInputState::from_raw(&raw.button_a) }?,
            button_x: unsafe { ButtonInputState::from_raw(&raw.button_x) }?,
            button_menu: unsafe { raw.button_menu.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            reports_absolute_dpad_values: raw.reports_absolute_dpad_values != 0,
            allows_rotation: raw.allows_rotation != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDirectionalGamepadDetailsRaw {
    pub dpad: GCDirectionPadInputStateRaw,
    pub button_a: GCButtonInputStateRaw,
    pub button_x: GCButtonInputStateRaw,
    pub button_menu: *const GCButtonInputStateRaw,
    pub reports_absolute_dpad_values: u8,
    pub allows_rotation: u8,
}
impl FromRaw for DirectionalGamepadDetails {
    type Raw = GCDirectionalGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            dpad: unsafe { DirectionPadInputState::from_raw(&raw.dpad) }?,
            button_a: unsafe { ButtonInputState::from_raw(&raw.button_a) }?,
            button_x: unsafe { ButtonInputState::from_raw(&raw.button_x) }?,
            button_menu: unsafe { raw.button_menu.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            reports_absolute_dpad_values: raw.reports_absolute_dpad_values != 0,
            allows_rotation: raw.allows_rotation != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCExtendedGamepadDetailsRaw {
    pub dpad: GCDirectionPadInputStateRaw,
    pub button_a: GCButtonInputStateRaw,
    pub button_b: GCButtonInputStateRaw,
    pub button_x: GCButtonInputStateRaw,
    pub button_y: GCButtonInputStateRaw,
    pub button_menu: *const GCButtonInputStateRaw,
    pub button_options: *const GCButtonInputStateRaw,
    pub button_home: *const GCButtonInputStateRaw,
    pub left_thumbstick: GCDirectionPadInputStateRaw,
    pub right_thumbstick: GCDirectionPadInputStateRaw,
    pub left_shoulder: GCButtonInputStateRaw,
    pub right_shoulder: GCButtonInputStateRaw,
    pub left_trigger: GCButtonInputStateRaw,
    pub right_trigger: GCButtonInputStateRaw,
    pub left_thumbstick_button: *const GCButtonInputStateRaw,
    pub right_thumbstick_button: *const GCButtonInputStateRaw,
}
impl FromRaw for ExtendedGamepadDetails {
    type Raw = GCExtendedGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            dpad: unsafe { DirectionPadInputState::from_raw(&raw.dpad) }?,
            button_a: unsafe { ButtonInputState::from_raw(&raw.button_a) }?,
            button_b: unsafe { ButtonInputState::from_raw(&raw.button_b) }?,
            button_x: unsafe { ButtonInputState::from_raw(&raw.button_x) }?,
            button_y: unsafe { ButtonInputState::from_raw(&raw.button_y) }?,
            button_menu: unsafe { raw.button_menu.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            button_options: unsafe { raw.button_options.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            button_home: unsafe { raw.button_home.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            left_thumbstick: unsafe { DirectionPadInputState::from_raw(&raw.left_thumbstick) }?,
            right_thumbstick: unsafe { DirectionPadInputState::from_raw(&raw.right_thumbstick) }?,
            left_shoulder: unsafe { ButtonInputState::from_raw(&raw.left_shoulder) }?,
            right_shoulder: unsafe { ButtonInputState::from_raw(&raw.right_shoulder) }?,
            left_trigger: unsafe { ButtonInputState::from_raw(&raw.left_trigger) }?,
            right_trigger: unsafe { ButtonInputState::from_raw(&raw.right_trigger) }?,
            left_thumbstick_button: unsafe { raw.left_thumbstick_button.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            right_thumbstick_button: unsafe { raw.right_thumbstick_button.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDualShockGamepadDetailsRaw {
    pub touchpad_button: GCButtonInputStateRaw,
    pub touchpad_primary: GCDirectionPadInputStateRaw,
    pub touchpad_secondary: GCDirectionPadInputStateRaw,
}
impl FromRaw for DualShockGamepadDetails {
    type Raw = GCDualShockGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            touchpad_button: unsafe { ButtonInputState::from_raw(&raw.touchpad_button) }?,
            touchpad_primary: unsafe { DirectionPadInputState::from_raw(&raw.touchpad_primary) }?,
            touchpad_secondary: unsafe {
                DirectionPadInputState::from_raw(&raw.touchpad_secondary)
            }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDualSenseAdaptiveTriggerStateRaw {
    pub value: f32,
    pub pressed: u8,
    pub touched: u8,
    pub mode: i32,
    pub status: i32,
    pub arm_position: f32,
}
impl FromRaw for DualSenseAdaptiveTriggerState {
    type Raw = GCDualSenseAdaptiveTriggerStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            value: raw.value,
            pressed: raw.pressed != 0,
            touched: raw.touched != 0,
            mode: match raw.mode {
                0 => DualSenseTriggerMode::Off,
                1 => DualSenseTriggerMode::Feedback,
                2 => DualSenseTriggerMode::Weapon,
                3 => DualSenseTriggerMode::Vibration,
                4 => DualSenseTriggerMode::SlopeFeedback,
                _ => return Err(GameControllerError::InvalidBridgeData),
            },
            status: match raw.status {
                0 => DualSenseTriggerStatus::Unknown,
                1 => DualSenseTriggerStatus::FeedbackNoLoad,
                2 => DualSenseTriggerStatus::FeedbackLoadApplied,
                3 => DualSenseTriggerStatus::WeaponReady,
                4 => DualSenseTriggerStatus::WeaponFiring,
                5 => DualSenseTriggerStatus::WeaponFired,
                6 => DualSenseTriggerStatus::VibrationNotVibrating,
                7 => DualSenseTriggerStatus::VibrationIsVibrating,
                8 => DualSenseTriggerStatus::SlopeFeedbackReady,
                9 => DualSenseTriggerStatus::SlopeFeedbackApplyingLoad,
                10 => DualSenseTriggerStatus::SlopeFeedbackFinished,
                _ => return Err(GameControllerError::InvalidBridgeData),
            },
            arm_position: raw.arm_position,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDualSenseGamepadDetailsRaw {
    pub touchpad_button: GCButtonInputStateRaw,
    pub touchpad_primary: GCDirectionPadInputStateRaw,
    pub touchpad_secondary: GCDirectionPadInputStateRaw,
    pub left_trigger: GCDualSenseAdaptiveTriggerStateRaw,
    pub right_trigger: GCDualSenseAdaptiveTriggerStateRaw,
}
impl FromRaw for DualSenseGamepadDetails {
    type Raw = GCDualSenseGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            touchpad_button: unsafe { ButtonInputState::from_raw(&raw.touchpad_button) }?,
            touchpad_primary: unsafe { DirectionPadInputState::from_raw(&raw.touchpad_primary) }?,
            touchpad_secondary: unsafe {
                DirectionPadInputState::from_raw(&raw.touchpad_secondary)
            }?,
            left_trigger: unsafe { DualSenseAdaptiveTriggerState::from_raw(&raw.left_trigger) }?,
            right_trigger: unsafe { DualSenseAdaptiveTriggerState::from_raw(&raw.right_trigger) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCXboxGamepadDetailsRaw {
    pub paddle_button1: *const GCButtonInputStateRaw,
    pub paddle_button2: *const GCButtonInputStateRaw,
    pub paddle_button3: *const GCButtonInputStateRaw,
    pub paddle_button4: *const GCButtonInputStateRaw,
    pub button_share: *const GCButtonInputStateRaw,
}
impl FromRaw for XboxGamepadDetails {
    type Raw = GCXboxGamepadDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            paddle_button1: unsafe { raw.paddle_button1.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            paddle_button2: unsafe { raw.paddle_button2.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            paddle_button3: unsafe { raw.paddle_button3.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            paddle_button4: unsafe { raw.paddle_button4.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            button_share: unsafe { raw.button_share.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCBatteryInfoRaw {
    pub level: f32,
    pub state: i32,
}
impl FromRaw for BatteryInfo {
    type Raw = GCBatteryInfoRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            level: raw.level,
            state: match raw.state {
                0 => BatteryState::Unknown,
                1 => BatteryState::Discharging,
                2 => BatteryState::Charging,
                3 => BatteryState::Full,
                _ => return Err(GameControllerError::InvalidBridgeData),
            },
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDeviceLightDetailsRaw {
    pub color: GCColorRaw,
}
impl FromRaw for DeviceLightDetails {
    type Raw = GCDeviceLightDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            color: unsafe { Color::from_raw(&raw.color) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDeviceHapticsDetailsRaw {
    pub supported_localities: GCSliceRaw,
}
impl FromRaw for DeviceHapticsDetails {
    type Raw = GCDeviceHapticsDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            supported_localities: unsafe { array::<String>(raw.supported_localities) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDeviceDetailsRaw {
    pub vendor_name: GCStringRaw,
    pub product_category: GCStringRaw,
    pub handler_queue_label: GCStringRaw,
}
impl FromRaw for DeviceDetails {
    type Raw = GCDeviceDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            vendor_name: unsafe { String::from_raw(&raw.vendor_name) }?,
            product_category: unsafe { String::from_raw(&raw.product_category) }?,
            handler_queue_label: unsafe { String::from_raw(&raw.handler_queue_label) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCConnectedDevicesSnapshotRaw {
    pub controllers: GCSliceRaw,
    pub keyboard: *const GCDeviceDetailsRaw,
    pub mouse: *const GCDeviceDetailsRaw,
    pub racing_wheels: GCSliceRaw,
}
impl FromRaw for ConnectedDevicesSnapshot {
    type Raw = GCConnectedDevicesSnapshotRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            controllers: unsafe { array::<DeviceDetails>(raw.controllers) }?,
            keyboard: unsafe { raw.keyboard.as_ref() }
                .map(|value| unsafe { DeviceDetails::from_raw(value) })
                .transpose()?,
            mouse: unsafe { raw.mouse.as_ref() }
                .map(|value| unsafe { DeviceDetails::from_raw(value) })
                .transpose()?,
            racing_wheels: unsafe { array::<DeviceDetails>(raw.racing_wheels) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCMotionDetailsRaw {
    pub sensors_require_manual_activation: u8,
    pub sensors_active: u8,
    pub has_gravity_and_user_acceleration: u8,
    pub gravity: GCVector3Raw,
    pub user_acceleration: GCVector3Raw,
    pub acceleration: GCVector3Raw,
    pub has_attitude: u8,
    pub has_rotation_rate: u8,
    pub attitude: GCQuaternionRaw,
    pub rotation_rate: GCVector3Raw,
}
impl FromRaw for MotionDetails {
    type Raw = GCMotionDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            sensors_require_manual_activation: raw.sensors_require_manual_activation != 0,
            sensors_active: raw.sensors_active != 0,
            has_gravity_and_user_acceleration: raw.has_gravity_and_user_acceleration != 0,
            gravity: unsafe { Vector3::from_raw(&raw.gravity) }?,
            user_acceleration: unsafe { Vector3::from_raw(&raw.user_acceleration) }?,
            acceleration: unsafe { Vector3::from_raw(&raw.acceleration) }?,
            has_attitude: raw.has_attitude != 0,
            has_rotation_rate: raw.has_rotation_rate != 0,
            attitude: unsafe { Quaternion::from_raw(&raw.attitude) }?,
            rotation_rate: unsafe { Vector3::from_raw(&raw.rotation_rate) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedButtonInputStateRaw {
    pub alias: GCStringRaw,
    pub value: GCButtonInputStateRaw,
}
impl FromRaw for NamedButtonInputState {
    type Raw = GCNamedButtonInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            alias: unsafe { String::from_raw(&raw.alias) }?,
            value: unsafe { ButtonInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedAxisInputStateRaw {
    pub alias: GCStringRaw,
    pub value: GCAxisInputStateRaw,
}
impl FromRaw for NamedAxisInputState {
    type Raw = GCNamedAxisInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            alias: unsafe { String::from_raw(&raw.alias) }?,
            value: unsafe { AxisInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedDirectionPadStateRaw {
    pub alias: GCStringRaw,
    pub value: GCDirectionPadInputStateRaw,
}
impl FromRaw for NamedDirectionPadState {
    type Raw = GCNamedDirectionPadStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            alias: unsafe { String::from_raw(&raw.alias) }?,
            value: unsafe { DirectionPadInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedTouchpadStateRaw {
    pub alias: GCStringRaw,
    pub value: GCTouchpadDetailsRaw,
}
impl FromRaw for NamedTouchpadState {
    type Raw = GCNamedTouchpadStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            alias: unsafe { String::from_raw(&raw.alias) }?,
            value: unsafe { TouchpadDetails::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPhysicalInputProfileDetailsRaw {
    pub last_event_timestamp: f64,
    pub has_remapped_elements: u8,
    pub element_aliases: GCSliceRaw,
    pub button_aliases: GCSliceRaw,
    pub axis_aliases: GCSliceRaw,
    pub dpad_aliases: GCSliceRaw,
    pub touchpad_aliases: GCSliceRaw,
    pub buttons: GCSliceRaw,
    pub axes: GCSliceRaw,
    pub dpads: GCSliceRaw,
    pub touchpads: GCSliceRaw,
}
impl FromRaw for PhysicalInputProfileDetails {
    type Raw = GCPhysicalInputProfileDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            last_event_timestamp: raw.last_event_timestamp,
            has_remapped_elements: raw.has_remapped_elements != 0,
            element_aliases: unsafe { array::<String>(raw.element_aliases) }?,
            button_aliases: unsafe { array::<String>(raw.button_aliases) }?,
            axis_aliases: unsafe { array::<String>(raw.axis_aliases) }?,
            dpad_aliases: unsafe { array::<String>(raw.dpad_aliases) }?,
            touchpad_aliases: unsafe { array::<String>(raw.touchpad_aliases) }?,
            buttons: unsafe { array::<NamedButtonInputState>(raw.buttons) }?,
            axes: unsafe { array::<NamedAxisInputState>(raw.axes) }?,
            dpads: unsafe { array::<NamedDirectionPadState>(raw.dpads) }?,
            touchpads: unsafe { array::<NamedTouchpadState>(raw.touchpads) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCAxisElementStateRaw {
    pub absolute_value: *const f32,
    pub relative_delta: f32,
}
impl FromRaw for AxisElementState {
    type Raw = GCAxisElementStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            absolute_value: unsafe { raw.absolute_value.as_ref() }.copied(),
            relative_delta: raw.relative_delta,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCSwitchInputStateRaw {
    pub position: i32,
    pub position_lower_bound: i32,
    pub position_count: i32,
    pub sequential: u8,
    pub can_wrap: u8,
}
impl FromRaw for SwitchInputState {
    type Raw = GCSwitchInputStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            position: raw.position,
            position_lower_bound: raw.position_lower_bound,
            position_count: raw.position_count,
            sequential: raw.sequential != 0,
            can_wrap: raw.can_wrap != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedButtonElementStateRaw {
    pub primary_alias: GCStringRaw,
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub value: GCButtonInputStateRaw,
}
impl FromRaw for NamedButtonElementState {
    type Raw = GCNamedButtonElementStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            primary_alias: unsafe { String::from_raw(&raw.primary_alias) }?,
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            value: unsafe { ButtonInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedAxisElementStateRaw {
    pub primary_alias: GCStringRaw,
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub value: GCAxisElementStateRaw,
}
impl FromRaw for NamedAxisElementState {
    type Raw = GCNamedAxisElementStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            primary_alias: unsafe { String::from_raw(&raw.primary_alias) }?,
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            value: unsafe { AxisElementState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedSwitchElementStateRaw {
    pub primary_alias: GCStringRaw,
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub value: GCSwitchInputStateRaw,
}
impl FromRaw for NamedSwitchElementState {
    type Raw = GCNamedSwitchElementStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            primary_alias: unsafe { String::from_raw(&raw.primary_alias) }?,
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            value: unsafe { SwitchInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedDirectionPadElementStateRaw {
    pub primary_alias: GCStringRaw,
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub value: GCDirectionPadInputStateRaw,
}
impl FromRaw for NamedDirectionPadElementState {
    type Raw = GCNamedDirectionPadElementStateRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            primary_alias: unsafe { String::from_raw(&raw.primary_alias) }?,
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            value: unsafe { DirectionPadInputState::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCInputElementMetadataRaw {
    pub primary_alias: GCStringRaw,
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
}
impl FromRaw for InputElementMetadata {
    type Raw = GCInputElementMetadataRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            primary_alias: unsafe { String::from_raw(&raw.primary_alias) }?,
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCControllerElementDetailsRaw {
    pub analog: u8,
    pub collection_path: GCSliceRaw,
    pub is_bound_to_system_gesture: u8,
    pub preferred_system_gesture_state: i32,
    pub sf_symbols_name: *const GCStringRaw,
    pub localized_name: *const GCStringRaw,
    pub unmapped_sf_symbols_name: *const GCStringRaw,
    pub unmapped_localized_name: *const GCStringRaw,
}
impl FromRaw for ControllerElementDetails {
    type Raw = GCControllerElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            analog: raw.analog != 0,
            collection_path: unsafe { array::<String>(raw.collection_path) }?,
            is_bound_to_system_gesture: raw.is_bound_to_system_gesture != 0,
            preferred_system_gesture_state: match raw.preferred_system_gesture_state {
                0 => SystemGestureState::Enabled,
                1 => SystemGestureState::AlwaysReceive,
                2 => SystemGestureState::Disabled,
                _ => return Err(GameControllerError::InvalidBridgeData),
            },
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            unmapped_sf_symbols_name: unsafe { raw.unmapped_sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            unmapped_localized_name: unsafe { raw.unmapped_localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCNamedControllerElementDetailsRaw {
    pub name: GCStringRaw,
    pub value: GCControllerElementDetailsRaw,
}
impl FromRaw for NamedControllerElementDetails {
    type Raw = GCNamedControllerElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            name: unsafe { String::from_raw(&raw.name) }?,
            value: unsafe { ControllerElementDetails::from_raw(&raw.value) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPhysicalInputSourceDetailsRaw {
    pub element_aliases: GCSliceRaw,
    pub element_localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub direction: u8,
}
impl FromRaw for PhysicalInputSourceDetails {
    type Raw = GCPhysicalInputSourceDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            element_aliases: unsafe { array::<String>(raw.element_aliases) }?,
            element_localized_name: unsafe { raw.element_localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            direction: PhysicalInputSourceDirection::from_bits(raw.direction),
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPhysicalInputExtentsDetailsRaw {
    pub scaled_value: f64,
    pub minimum_value: f64,
    pub maximum_value: f64,
}
impl FromRaw for PhysicalInputExtentsDetails {
    type Raw = GCPhysicalInputExtentsDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            scaled_value: raw.scaled_value,
            minimum_value: raw.minimum_value,
            maximum_value: raw.maximum_value,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCLinearInputDetailsRaw {
    pub value: f32,
    pub analog: u8,
    pub can_wrap: u8,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub physical_extents: *const GCPhysicalInputExtentsDetailsRaw,
    pub sources: GCSliceRaw,
}
impl FromRaw for LinearInputDetails {
    type Raw = GCLinearInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            value: raw.value,
            analog: raw.analog != 0,
            can_wrap: raw.can_wrap != 0,
            last_value_timestamp: raw.last_value_timestamp,
            last_value_latency: raw.last_value_latency,
            physical_extents: unsafe { raw.physical_extents.as_ref() }
                .map(|value| unsafe { PhysicalInputExtentsDetails::from_raw(value) })
                .transpose()?,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPressedStateInputDetailsRaw {
    pub pressed: u8,
    pub last_pressed_state_timestamp: f64,
    pub last_pressed_state_latency: f64,
    pub sources: GCSliceRaw,
}
impl FromRaw for PressedStateInputDetails {
    type Raw = GCPressedStateInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            pressed: raw.pressed != 0,
            last_pressed_state_timestamp: raw.last_pressed_state_timestamp,
            last_pressed_state_latency: raw.last_pressed_state_latency,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCTouchedStateInputDetailsRaw {
    pub touched: u8,
    pub last_touched_state_timestamp: f64,
    pub last_touched_state_latency: f64,
    pub sources: GCSliceRaw,
}
impl FromRaw for TouchedStateInputDetails {
    type Raw = GCTouchedStateInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            touched: raw.touched != 0,
            last_touched_state_timestamp: raw.last_touched_state_timestamp,
            last_touched_state_latency: raw.last_touched_state_latency,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCRelativeInputDetailsRaw {
    pub delta: f32,
    pub analog: u8,
    pub last_delta_timestamp: f64,
    pub last_delta_latency: f64,
    pub sources: GCSliceRaw,
}
impl FromRaw for RelativeInputDetails {
    type Raw = GCRelativeInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            delta: raw.delta,
            analog: raw.analog != 0,
            last_delta_timestamp: raw.last_delta_timestamp,
            last_delta_latency: raw.last_delta_latency,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCAxisInputDetailsRaw {
    pub value: f32,
    pub analog: u8,
    pub can_wrap: u8,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub sources: GCSliceRaw,
}
impl FromRaw for AxisInputDetails {
    type Raw = GCAxisInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            value: raw.value,
            analog: raw.analog != 0,
            can_wrap: raw.can_wrap != 0,
            last_value_timestamp: raw.last_value_timestamp,
            last_value_latency: raw.last_value_latency,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCAxis2DInputDetailsRaw {
    pub value: GCPoint2Raw,
    pub analog: u8,
    pub can_wrap: u8,
    pub last_value_timestamp: f64,
    pub last_value_latency: f64,
    pub sources: GCSliceRaw,
}
impl FromRaw for Axis2DInputDetails {
    type Raw = GCAxis2DInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            value: unsafe { GCPoint2::from_raw(&raw.value) }?,
            analog: raw.analog != 0,
            can_wrap: raw.can_wrap != 0,
            last_value_timestamp: raw.last_value_timestamp,
            last_value_latency: raw.last_value_latency,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCSwitchPositionInputDetailsRaw {
    pub position: i32,
    pub position_lower_bound: i32,
    pub position_count: i32,
    pub sequential: u8,
    pub can_wrap: u8,
    pub sources: GCSliceRaw,
}
impl FromRaw for SwitchPositionInputDetails {
    type Raw = GCSwitchPositionInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            position: raw.position,
            position_lower_bound: raw.position_lower_bound,
            position_count: raw.position_count,
            sequential: raw.sequential != 0,
            can_wrap: raw.can_wrap != 0,
            sources: unsafe { array::<PhysicalInputSourceDetails>(raw.sources) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCLinearPressedInputDetailsRaw {
    pub linear_input: GCLinearInputDetailsRaw,
    pub pressed_state: GCPressedStateInputDetailsRaw,
}
impl FromRaw for LinearPressedInputDetails {
    type Raw = GCLinearPressedInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            linear_input: unsafe { LinearInputDetails::from_raw(&raw.linear_input) }?,
            pressed_state: unsafe { PressedStateInputDetails::from_raw(&raw.pressed_state) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCButtonElementDetailsRaw {
    pub metadata: GCInputElementMetadataRaw,
    pub linear_input: GCLinearInputDetailsRaw,
    pub pressed_state: GCPressedStateInputDetailsRaw,
    pub touched_state: *const GCTouchedStateInputDetailsRaw,
    pub force_input: *const GCLinearInputDetailsRaw,
}
impl FromRaw for ButtonElementDetails {
    type Raw = GCButtonElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            metadata: unsafe { InputElementMetadata::from_raw(&raw.metadata) }?,
            linear_input: unsafe { LinearInputDetails::from_raw(&raw.linear_input) }?,
            pressed_state: unsafe { PressedStateInputDetails::from_raw(&raw.pressed_state) }?,
            touched_state: unsafe { raw.touched_state.as_ref() }
                .map(|value| unsafe { TouchedStateInputDetails::from_raw(value) })
                .transpose()?,
            force_input: unsafe { raw.force_input.as_ref() }
                .map(|value| unsafe { LinearInputDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCAxisElementDetailsRaw {
    pub metadata: GCInputElementMetadataRaw,
    pub absolute_input: *const GCAxisInputDetailsRaw,
    pub relative_input: GCRelativeInputDetailsRaw,
}
impl FromRaw for AxisElementDetails {
    type Raw = GCAxisElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            metadata: unsafe { InputElementMetadata::from_raw(&raw.metadata) }?,
            absolute_input: unsafe { raw.absolute_input.as_ref() }
                .map(|value| unsafe { AxisInputDetails::from_raw(value) })
                .transpose()?,
            relative_input: unsafe { RelativeInputDetails::from_raw(&raw.relative_input) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCSwitchElementDetailsRaw {
    pub metadata: GCInputElementMetadataRaw,
    pub position_input: GCSwitchPositionInputDetailsRaw,
}
impl FromRaw for SwitchElementDetails {
    type Raw = GCSwitchElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            metadata: unsafe { InputElementMetadata::from_raw(&raw.metadata) }?,
            position_input: unsafe { SwitchPositionInputDetails::from_raw(&raw.position_input) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDirectionPadElementDetailsRaw {
    pub metadata: GCInputElementMetadataRaw,
    pub xy_axes: *const GCAxis2DInputDetailsRaw,
    pub x_axis: GCAxisInputDetailsRaw,
    pub y_axis: GCAxisInputDetailsRaw,
    pub up: GCLinearPressedInputDetailsRaw,
    pub down: GCLinearPressedInputDetailsRaw,
    pub left: GCLinearPressedInputDetailsRaw,
    pub right: GCLinearPressedInputDetailsRaw,
}
impl FromRaw for DirectionPadElementDetails {
    type Raw = GCDirectionPadElementDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            metadata: unsafe { InputElementMetadata::from_raw(&raw.metadata) }?,
            xy_axes: unsafe { raw.xy_axes.as_ref() }
                .map(|value| unsafe { Axis2DInputDetails::from_raw(value) })
                .transpose()?,
            x_axis: unsafe { AxisInputDetails::from_raw(&raw.x_axis) }?,
            y_axis: unsafe { AxisInputDetails::from_raw(&raw.y_axis) }?,
            up: unsafe { LinearPressedInputDetails::from_raw(&raw.up) }?,
            down: unsafe { LinearPressedInputDetails::from_raw(&raw.down) }?,
            left: unsafe { LinearPressedInputDetails::from_raw(&raw.left) }?,
            right: unsafe { LinearPressedInputDetails::from_raw(&raw.right) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPhysicalInputElementSnapshotRaw {
    pub buttons: GCSliceRaw,
    pub axes: GCSliceRaw,
    pub switches: GCSliceRaw,
    pub dpads: GCSliceRaw,
}
impl FromRaw for PhysicalInputElementSnapshot {
    type Raw = GCPhysicalInputElementSnapshotRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            buttons: unsafe { array::<ButtonElementDetails>(raw.buttons) }?,
            axes: unsafe { array::<AxisElementDetails>(raw.axes) }?,
            switches: unsafe { array::<SwitchElementDetails>(raw.switches) }?,
            dpads: unsafe { array::<DirectionPadElementDetails>(raw.dpads) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDevicePhysicalInputStateDiffDetailsRaw {
    pub changed_elements_known: u8,
    pub changed_aliases: GCSliceRaw,
    pub changed_elements: GCSliceRaw,
}
impl FromRaw for DevicePhysicalInputStateDiffDetails {
    type Raw = GCDevicePhysicalInputStateDiffDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            changed_elements_known: raw.changed_elements_known != 0,
            changed_aliases: unsafe { array::<String>(raw.changed_aliases) }?,
            changed_elements: unsafe { array::<InputElementMetadata>(raw.changed_elements) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCControllerInputStateDetailsRaw {
    pub last_event_timestamp: f64,
    pub last_event_latency: f64,
    pub element_count: usize,
    pub button_count: usize,
    pub axis_count: usize,
    pub switch_count: usize,
    pub dpad_count: usize,
    pub buttons: GCSliceRaw,
    pub axes: GCSliceRaw,
    pub switches: GCSliceRaw,
    pub dpads: GCSliceRaw,
    pub changed_aliases_known: u8,
    pub changed_aliases: GCSliceRaw,
}
impl FromRaw for ControllerInputStateDetails {
    type Raw = GCControllerInputStateDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            last_event_timestamp: raw.last_event_timestamp,
            last_event_latency: raw.last_event_latency,
            element_count: raw.element_count,
            button_count: raw.button_count,
            axis_count: raw.axis_count,
            switch_count: raw.switch_count,
            dpad_count: raw.dpad_count,
            buttons: unsafe { array::<NamedButtonElementState>(raw.buttons) }?,
            axes: unsafe { array::<NamedAxisElementState>(raw.axes) }?,
            switches: unsafe { array::<NamedSwitchElementState>(raw.switches) }?,
            dpads: unsafe { array::<NamedDirectionPadElementState>(raw.dpads) }?,
            changed_aliases_known: raw.changed_aliases_known != 0,
            changed_aliases: unsafe { array::<String>(raw.changed_aliases) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCControllerLiveInputDetailsRaw {
    pub input_state_queue_depth: isize,
    pub live: GCControllerInputStateDetailsRaw,
    pub unmapped: *const GCControllerInputStateDetailsRaw,
    pub next: *const GCControllerInputStateDetailsRaw,
}
impl FromRaw for ControllerLiveInputDetails {
    type Raw = GCControllerLiveInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            input_state_queue_depth: raw.input_state_queue_depth,
            live: unsafe { ControllerInputStateDetails::from_raw(&raw.live) }?,
            unmapped: unsafe { raw.unmapped.as_ref() }
                .map(|value| unsafe { ControllerInputStateDetails::from_raw(value) })
                .transpose()?,
            next: unsafe { raw.next.as_ref() }
                .map(|value| unsafe { ControllerInputStateDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCDevicePhysicalInputSourceDetailsRaw {
    pub input_state_queue_depth: isize,
    pub live: GCControllerInputStateDetailsRaw,
    pub capture: GCControllerInputStateDetailsRaw,
    pub unmapped: *const GCControllerInputStateDetailsRaw,
    pub next: *const GCControllerInputStateDetailsRaw,
    pub next_diff: *const GCDevicePhysicalInputStateDiffDetailsRaw,
}
impl FromRaw for DevicePhysicalInputSourceDetails {
    type Raw = GCDevicePhysicalInputSourceDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            input_state_queue_depth: raw.input_state_queue_depth,
            live: unsafe { DevicePhysicalInputState::from_raw(&raw.live) }?,
            capture: unsafe { DevicePhysicalInputState::from_raw(&raw.capture) }?,
            unmapped: unsafe { raw.unmapped.as_ref() }
                .map(|value| unsafe { DevicePhysicalInputState::from_raw(value) })
                .transpose()?,
            next: unsafe { raw.next.as_ref() }
                .map(|value| unsafe { DevicePhysicalInputState::from_raw(value) })
                .transpose()?,
            next_diff: unsafe { raw.next_diff.as_ref() }
                .map(|value| unsafe { DevicePhysicalInputStateDiffDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCControllerDetailsRaw {
    pub vendor_name: GCStringRaw,
    pub product_category: GCStringRaw,
    pub player_index: i32,
    pub is_attached_to_device: u8,
    pub is_current: u8,
    pub supports_background_events: u8,
    pub has_live_input: u8,
    pub gamepad: *const GCGamepadDetailsRaw,
    pub micro_gamepad: *const GCMicroGamepadDetailsRaw,
    pub directional_gamepad: *const GCDirectionalGamepadDetailsRaw,
    pub extended_gamepad: *const GCExtendedGamepadDetailsRaw,
    pub dual_shock: *const GCDualShockGamepadDetailsRaw,
    pub dual_sense: *const GCDualSenseGamepadDetailsRaw,
    pub xbox: *const GCXboxGamepadDetailsRaw,
    pub battery: *const GCBatteryInfoRaw,
    pub motion: *const GCMotionDetailsRaw,
    pub has_light: u8,
    pub light: *const GCDeviceLightDetailsRaw,
    pub has_haptics: u8,
    pub haptics: *const GCDeviceHapticsDetailsRaw,
    pub physical_input: *const GCPhysicalInputProfileDetailsRaw,
    pub input: *const GCControllerLiveInputDetailsRaw,
}
impl FromRaw for ControllerDetails {
    type Raw = GCControllerDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            vendor_name: unsafe { String::from_raw(&raw.vendor_name) }?,
            product_category: unsafe { String::from_raw(&raw.product_category) }?,
            player_index: raw.player_index,
            is_attached_to_device: raw.is_attached_to_device != 0,
            is_current: raw.is_current != 0,
            supports_background_events: raw.supports_background_events != 0,
            has_live_input: raw.has_live_input != 0,
            gamepad: unsafe { raw.gamepad.as_ref() }
                .map(|value| unsafe { GamepadDetails::from_raw(value) })
                .transpose()?,
            micro_gamepad: unsafe { raw.micro_gamepad.as_ref() }
                .map(|value| unsafe { MicroGamepadDetails::from_raw(value) })
                .transpose()?,
            directional_gamepad: unsafe { raw.directional_gamepad.as_ref() }
                .map(|value| unsafe { DirectionalGamepadDetails::from_raw(value) })
                .transpose()?,
            extended_gamepad: unsafe { raw.extended_gamepad.as_ref() }
                .map(|value| unsafe { ExtendedGamepadDetails::from_raw(value) })
                .transpose()?,
            dual_shock: unsafe { raw.dual_shock.as_ref() }
                .map(|value| unsafe { DualShockGamepadDetails::from_raw(value) })
                .transpose()?,
            dual_sense: unsafe { raw.dual_sense.as_ref() }
                .map(|value| unsafe { DualSenseGamepadDetails::from_raw(value) })
                .transpose()?,
            xbox: unsafe { raw.xbox.as_ref() }
                .map(|value| unsafe { XboxGamepadDetails::from_raw(value) })
                .transpose()?,
            battery: unsafe { raw.battery.as_ref() }
                .map(|value| unsafe { BatteryInfo::from_raw(value) })
                .transpose()?,
            motion: unsafe { raw.motion.as_ref() }
                .map(|value| unsafe { MotionDetails::from_raw(value) })
                .transpose()?,
            has_light: raw.has_light != 0,
            light: unsafe { raw.light.as_ref() }
                .map(|value| unsafe { DeviceLightDetails::from_raw(value) })
                .transpose()?,
            has_haptics: raw.has_haptics != 0,
            haptics: unsafe { raw.haptics.as_ref() }
                .map(|value| unsafe { DeviceHapticsDetails::from_raw(value) })
                .transpose()?,
            physical_input: unsafe { raw.physical_input.as_ref() }
                .map(|value| unsafe { PhysicalInputProfileDetails::from_raw(value) })
                .transpose()?,
            input: unsafe { raw.input.as_ref() }
                .map(|value| unsafe { ControllerLiveInputDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCKeyboardSnapshotRaw {
    pub vendor_name: GCStringRaw,
    pub product_category: GCStringRaw,
    pub any_key_pressed: u8,
    pub pressed_aliases: GCSliceRaw,
    pub pressed_keys: GCSliceRaw,
    pub physical_input: GCPhysicalInputProfileDetailsRaw,
}
impl FromRaw for KeyboardSnapshot {
    type Raw = GCKeyboardSnapshotRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            vendor_name: unsafe { String::from_raw(&raw.vendor_name) }?,
            product_category: unsafe { String::from_raw(&raw.product_category) }?,
            any_key_pressed: raw.any_key_pressed != 0,
            pressed_aliases: unsafe { array::<String>(raw.pressed_aliases) }?,
            pressed_keys: unsafe { array::<NamedButtonInputState>(raw.pressed_keys) }?,
            physical_input: unsafe { PhysicalInputProfileDetails::from_raw(&raw.physical_input) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCMouseSnapshotRaw {
    pub vendor_name: GCStringRaw,
    pub product_category: GCStringRaw,
    pub is_current: u8,
    pub known_mouse_count: usize,
    pub scroll: GCDirectionPadInputStateRaw,
    pub left_button: GCButtonInputStateRaw,
    pub right_button: *const GCButtonInputStateRaw,
    pub middle_button: *const GCButtonInputStateRaw,
    pub auxiliary_buttons: GCSliceRaw,
    pub physical_input: GCPhysicalInputProfileDetailsRaw,
}
impl FromRaw for MouseSnapshot {
    type Raw = GCMouseSnapshotRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            vendor_name: unsafe { String::from_raw(&raw.vendor_name) }?,
            product_category: unsafe { String::from_raw(&raw.product_category) }?,
            is_current: raw.is_current != 0,
            known_mouse_count: raw.known_mouse_count,
            scroll: unsafe { DeviceCursorState::from_raw(&raw.scroll) }?,
            left_button: unsafe { ButtonInputState::from_raw(&raw.left_button) }?,
            right_button: unsafe { raw.right_button.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            middle_button: unsafe { raw.middle_button.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            auxiliary_buttons: unsafe { array::<NamedButtonInputState>(raw.auxiliary_buttons) }?,
            physical_input: unsafe { PhysicalInputProfileDetails::from_raw(&raw.physical_input) }?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCEventViewControllerDetailsRaw {
    pub controller_user_interaction_enabled: u8,
}
impl FromRaw for EventViewControllerDetails {
    type Raw = GCEventViewControllerDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            controller_user_interaction_enabled: raw.controller_user_interaction_enabled != 0,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCSteeringWheelDetailsRaw {
    pub maximum_degrees_of_rotation: f32,
    pub absolute_value: *const f32,
    pub relative_delta: f32,
}
impl FromRaw for SteeringWheelDetails {
    type Raw = GCSteeringWheelDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            maximum_degrees_of_rotation: raw.maximum_degrees_of_rotation,
            absolute_value: unsafe { raw.absolute_value.as_ref() }.copied(),
            relative_delta: raw.relative_delta,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCGearShifterDetailsRaw {
    pub aliases: GCSliceRaw,
    pub localized_name: *const GCStringRaw,
    pub sf_symbols_name: *const GCStringRaw,
    pub pattern_position: *const i32,
    pub pattern_lower_bound: *const i32,
    pub pattern_count: *const i32,
    pub pattern_sequential: *const u8,
    pub pattern_can_wrap: *const u8,
    pub sequential_delta: *const f32,
}
impl FromRaw for GearShifterDetails {
    type Raw = GCGearShifterDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            aliases: unsafe { array::<String>(raw.aliases) }?,
            localized_name: unsafe { raw.localized_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            sf_symbols_name: unsafe { raw.sf_symbols_name.as_ref() }
                .map(|value| unsafe { String::from_raw(value) })
                .transpose()?,
            pattern_position: unsafe { raw.pattern_position.as_ref() }.copied(),
            pattern_lower_bound: unsafe { raw.pattern_lower_bound.as_ref() }.copied(),
            pattern_count: unsafe { raw.pattern_count.as_ref() }.copied(),
            pattern_sequential: unsafe { raw.pattern_sequential.as_ref() }.map(|value| *value != 0),
            pattern_can_wrap: unsafe { raw.pattern_can_wrap.as_ref() }.map(|value| *value != 0),
            sequential_delta: unsafe { raw.sequential_delta.as_ref() }.copied(),
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCRacingWheelInputDetailsRaw {
    pub wheel: GCSteeringWheelDetailsRaw,
    pub accelerator_pedal: *const GCButtonInputStateRaw,
    pub brake_pedal: *const GCButtonInputStateRaw,
    pub clutch_pedal: *const GCButtonInputStateRaw,
    pub shifter: *const GCGearShifterDetailsRaw,
}
impl FromRaw for RacingWheelInputDetails {
    type Raw = GCRacingWheelInputDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            wheel: unsafe { SteeringWheelDetails::from_raw(&raw.wheel) }?,
            accelerator_pedal: unsafe { raw.accelerator_pedal.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            brake_pedal: unsafe { raw.brake_pedal.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            clutch_pedal: unsafe { raw.clutch_pedal.as_ref() }
                .map(|value| unsafe { ButtonInputState::from_raw(value) })
                .transpose()?,
            shifter: unsafe { raw.shifter.as_ref() }
                .map(|value| unsafe { GearShifterDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCRacingWheelDetailsRaw {
    pub vendor_name: GCStringRaw,
    pub product_category: GCStringRaw,
    pub is_acquired: u8,
    pub is_snapshot: u8,
    pub wheel_input: *const GCRacingWheelInputDetailsRaw,
}
impl FromRaw for RacingWheelDetails {
    type Raw = GCRacingWheelDetailsRaw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self {
            vendor_name: unsafe { String::from_raw(&raw.vendor_name) }?,
            product_category: unsafe { String::from_raw(&raw.product_category) }?,
            is_acquired: raw.is_acquired != 0,
            is_snapshot: raw.is_snapshot != 0,
            wheel_input: unsafe { raw.wheel_input.as_ref() }
                .map(|value| unsafe { RacingWheelInputDetails::from_raw(value) })
                .transpose()?,
        })
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GCPoint2Raw {
    pub x: f32,
    pub y: f32,
}
impl FromRaw for Point2 {
    type Raw = GCPoint2Raw;
    unsafe fn from_raw(raw: &Self::Raw) -> Result<Self, GameControllerError> {
        Ok(Self { x: raw.x, y: raw.y })
    }
}
