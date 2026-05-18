//! High-impact `GameController` constant tables mirrored from `MacOSX26.2.sdk`.
//!
//! These constants cover the large `GCKeyCode*`, `GCKey*`, and `GCInput*`
//! symbol families that dominate the framework's public surface area.

use core::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use serde::Deserialize;

/// Low-level HID-page-7 key code type mirroring Apple's `GCKeyCode`.
pub type GCKeyCode = isize;

/// Typed keyboard-element name mirroring Apple's `GCKey*` string constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyName(&'static str);

impl KeyName {
    /// Create a new static keyboard-element name.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the SDK-defined string value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for KeyName {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<KeyName> for &'static str {
    fn from(value: KeyName) -> Self {
        value.0
    }
}

impl fmt::Display for KeyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Typed alias mirroring Apple's `GCKeyName` constant family.
pub type GCKeyName = KeyName;

/// Typed locality constant mirroring Apple's `GCHapticsLocality` family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HapticsLocality(&'static str);

impl HapticsLocality {
    /// Create a new static haptics-locality name.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the SDK-defined string value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Convert a runtime string into a typed locality when it matches a known SDK constant.
    #[must_use]
    pub fn from_runtime_value(value: &str) -> Option<Self> {
        match value {
            "Default" => Some(Self::new("Default")),
            "All" => Some(Self::new("All")),
            "Handles" => Some(Self::new("Handles")),
            "Left Handle" => Some(Self::new("Left Handle")),
            "Right Handle" => Some(Self::new("Right Handle")),
            "Triggers" => Some(Self::new("Triggers")),
            "Left Trigger" => Some(Self::new("Left Trigger")),
            "Right Trigger" => Some(Self::new("Right Trigger")),
            _ => None,
        }
    }
}

impl AsRef<str> for HapticsLocality {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<HapticsLocality> for &'static str {
    fn from(value: HapticsLocality) -> Self {
        value.0
    }
}

impl fmt::Display for HapticsLocality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Typed alias mirroring Apple's `GCHapticsLocality` constant family.
pub type GCHapticsLocality = HapticsLocality;

/// Typed product-category constant mirroring Apple's `GCProductCategory*` families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProductCategory(&'static str);

impl ProductCategory {
    /// Create a new static product-category string.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the SDK-defined string value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Convert a runtime string into a typed product category when it matches a known SDK constant.
    #[must_use]
    pub fn from_runtime_value(value: &str) -> Option<Self> {
        match value {
            "Arcade Stick" => Some(Self::new("Arcade Stick")),
            "Coalesced Remote" => Some(Self::new("Coalesced Remote")),
            "Control Center Remote" => Some(Self::new("Control Center Remote")),
            "DualSense" => Some(Self::new("DualSense")),
            "DualShock 4" => Some(Self::new("DualShock 4")),
            "HID" => Some(Self::new("HID")),
            "Keyboard" => Some(Self::new("Keyboard")),
            "MFi" => Some(Self::new("MFi")),
            "Mouse" => Some(Self::new("Mouse")),
            "Siri Remote (1st Generation)" => Some(Self::new("Siri Remote (1st Generation)")),
            "Siri Remote (2nd Generation)" => Some(Self::new("Siri Remote (2nd Generation)")),
            "Spatial Controller" => Some(Self::new("Spatial Controller")),
            "Universal Electronics Remote" => Some(Self::new("Universal Electronics Remote")),
            "Xbox One" => Some(Self::new("Xbox One")),
            _ => None,
        }
    }

    /// Check whether a runtime category string matches this SDK-defined category.
    #[must_use]
    pub fn matches(self, value: &str) -> bool {
        self.0 == value
    }
}

impl AsRef<str> for ProductCategory {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<ProductCategory> for &'static str {
    fn from(value: ProductCategory) -> Self {
        value.0
    }
}

impl fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Typed alias mirroring Apple's `GCProductCategory*` constant families.
pub type GCProductCategory = ProductCategory;

/// Two-dimensional value mirroring Apple's `GCPoint2`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Point2 {
/// Mirrors the `GameController` framework property for `x`.
    pub x: f32,
/// Mirrors the `GameController` framework property for `y`.
    pub y: f32,
}

/// Apple-style alias for [`Point2`].
pub type GCPoint2 = Point2;

/// Rust mirror of Apple's `GCPoint2Zero` constant.
pub const POINT2_ZERO: GCPoint2 = GCPoint2 { x: 0.0, y: 0.0 };

/// Euler-angle triple mirroring Apple's `GCEulerAngles`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct EulerAngles {
/// Mirrors the `GameController` framework property for `pitch`.
    pub pitch: f64,
/// Mirrors the `GameController` framework property for `yaw`.
    pub yaw: f64,
/// Mirrors the `GameController` framework property for `roll`.
    pub roll: f64,
}

/// Apple-style alias for [`EulerAngles`].
pub type GCEulerAngles = EulerAngles;

/// The number of discrete adaptive-trigger positions exposed by `DualSense` trigger arrays.
pub const DUALSENSE_ADAPTIVE_TRIGGER_DISCRETE_POSITION_COUNT: usize = 10;

/// Raw positional amplitudes mirroring Apple's `GCDualSenseAdaptiveTriggerPositionalAmplitudes`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct DualSenseAdaptiveTriggerPositionalAmplitudes {
/// Mirrors the `GameController` framework property for `values`.
    pub values: [f32; DUALSENSE_ADAPTIVE_TRIGGER_DISCRETE_POSITION_COUNT],
}

impl Default for DualSenseAdaptiveTriggerPositionalAmplitudes {
    fn default() -> Self {
        Self {
            values: [0.0; DUALSENSE_ADAPTIVE_TRIGGER_DISCRETE_POSITION_COUNT],
        }
    }
}

/// Apple-style alias for [`DualSenseAdaptiveTriggerPositionalAmplitudes`].
pub type GCDualSenseAdaptiveTriggerPositionalAmplitudes =
    DualSenseAdaptiveTriggerPositionalAmplitudes;

/// Raw positional resistive strengths mirroring Apple's `GCDualSenseAdaptiveTriggerPositionalResistiveStrengths`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct DualSenseAdaptiveTriggerPositionalResistiveStrengths {
/// Mirrors the `GameController` framework property for `values`.
    pub values: [f32; DUALSENSE_ADAPTIVE_TRIGGER_DISCRETE_POSITION_COUNT],
}

impl Default for DualSenseAdaptiveTriggerPositionalResistiveStrengths {
    fn default() -> Self {
        Self {
            values: [0.0; DUALSENSE_ADAPTIVE_TRIGGER_DISCRETE_POSITION_COUNT],
        }
    }
}

/// Apple-style alias for [`DualSenseAdaptiveTriggerPositionalResistiveStrengths`].
pub type GCDualSenseAdaptiveTriggerPositionalResistiveStrengths =
    DualSenseAdaptiveTriggerPositionalResistiveStrengths;

/// Rust mirror of Apple's `GCSystemGestureState` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemGestureState {
/// Mirrors the `GameController` framework case `Enabled`.
    Enabled,
/// Mirrors the `GameController` framework case `AlwaysReceive`.
    AlwaysReceive,
/// Mirrors the `GameController` framework case `Disabled`.
    Disabled,
}

/// Apple-style alias for [`SystemGestureState`].
pub type GCSystemGestureState = SystemGestureState;

/// Bitflag wrapper mirroring Apple's `GCPhysicalInputSourceDirection` options type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize)]
#[serde(transparent)]
pub struct PhysicalInputSourceDirection(u8);

impl PhysicalInputSourceDirection {
/// Mirrors the `GameController` framework constant `NOT_APPLICABLE`.
    pub const NOT_APPLICABLE: Self = Self(0);
/// Mirrors the `GameController` framework constant `UP`.
    pub const UP: Self = Self(1 << 0);
/// Mirrors the `GameController` framework constant `RIGHT`.
    pub const RIGHT: Self = Self(1 << 1);
/// Mirrors the `GameController` framework constant `DOWN`.
    pub const DOWN: Self = Self(1 << 2);
/// Mirrors the `GameController` framework constant `LEFT`.
    pub const LEFT: Self = Self(1 << 3);

    /// Return the raw bitset value.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Check whether all bits in `other` are set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for PhysicalInputSourceDirection {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PhysicalInputSourceDirection {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for PhysicalInputSourceDirection {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for PhysicalInputSourceDirection {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Apple-style alias for [`PhysicalInputSourceDirection`].
pub type GCPhysicalInputSourceDirection = PhysicalInputSourceDirection;

/// Typed physical-input element name mirroring Apple's `GCInput*` string constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InputName(&'static str);

impl InputName {
    /// Create a new static physical-input element name.
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Borrow the SDK-defined string value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for InputName {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<InputName> for &'static str {
    fn from(value: InputName) -> Self {
        value.0
    }
}

impl fmt::Display for InputName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Typed alias mirroring Apple's `GCPhysicalInputElementName`.
pub type GCPhysicalInputElementName = InputName;
/// Typed alias mirroring Apple's `GCInputElementName`.
pub type GCInputElementName = InputName;
/// Typed alias mirroring Apple's `GCButtonElementName`.
pub type GCButtonElementName = InputName;
/// Typed alias mirroring Apple's `GCAxisElementName`.
pub type GCAxisElementName = InputName;
/// Typed alias mirroring Apple's `GCSwitchElementName`.
pub type GCSwitchElementName = InputName;
/// Typed alias mirroring Apple's `GCDirectionPadElementName`.
pub type GCDirectionPadElementName = InputName;
/// Typed alias mirroring Apple's `GCInputButtonName`.
pub type GCInputButtonName = InputName;
/// Typed alias mirroring Apple's `GCInputAxisName`.
pub type GCInputAxisName = InputName;
/// Typed alias mirroring Apple's `GCInputSwitchName`.
pub type GCInputSwitchName = InputName;
/// Typed alias mirroring Apple's `GCInputDirectionPadName`.
pub type GCInputDirectionPadName = InputName;
/// Mirrors Apple's `GCKeyCode*` constants.
pub mod key_codes {
    use super::GCKeyCode;
/// Mirrors the `GameController` framework constant `KEY_A`.
    pub const KEY_A: GCKeyCode = 4;
/// Mirrors the `GameController` framework constant `KEY_B`.
    pub const KEY_B: GCKeyCode = 5;
/// Mirrors the `GameController` framework constant `KEY_C`.
    pub const KEY_C: GCKeyCode = 6;
/// Mirrors the `GameController` framework constant `KEY_D`.
    pub const KEY_D: GCKeyCode = 7;
/// Mirrors the `GameController` framework constant `KEY_E`.
    pub const KEY_E: GCKeyCode = 8;
/// Mirrors the `GameController` framework constant `KEY_F`.
    pub const KEY_F: GCKeyCode = 9;
/// Mirrors the `GameController` framework constant `KEY_G`.
    pub const KEY_G: GCKeyCode = 10;
/// Mirrors the `GameController` framework constant `KEY_H`.
    pub const KEY_H: GCKeyCode = 11;
/// Mirrors the `GameController` framework constant `KEY_I`.
    pub const KEY_I: GCKeyCode = 12;
/// Mirrors the `GameController` framework constant `KEY_J`.
    pub const KEY_J: GCKeyCode = 13;
/// Mirrors the `GameController` framework constant `KEY_K`.
    pub const KEY_K: GCKeyCode = 14;
/// Mirrors the `GameController` framework constant `KEY_L`.
    pub const KEY_L: GCKeyCode = 15;
/// Mirrors the `GameController` framework constant `KEY_M`.
    pub const KEY_M: GCKeyCode = 16;
/// Mirrors the `GameController` framework constant `KEY_N`.
    pub const KEY_N: GCKeyCode = 17;
/// Mirrors the `GameController` framework constant `KEY_O`.
    pub const KEY_O: GCKeyCode = 18;
/// Mirrors the `GameController` framework constant `KEY_P`.
    pub const KEY_P: GCKeyCode = 19;
/// Mirrors the `GameController` framework constant `KEY_Q`.
    pub const KEY_Q: GCKeyCode = 20;
/// Mirrors the `GameController` framework constant `KEY_R`.
    pub const KEY_R: GCKeyCode = 21;
/// Mirrors the `GameController` framework constant `KEY_S`.
    pub const KEY_S: GCKeyCode = 22;
/// Mirrors the `GameController` framework constant `KEY_T`.
    pub const KEY_T: GCKeyCode = 23;
/// Mirrors the `GameController` framework constant `KEY_U`.
    pub const KEY_U: GCKeyCode = 24;
/// Mirrors the `GameController` framework constant `KEY_V`.
    pub const KEY_V: GCKeyCode = 25;
/// Mirrors the `GameController` framework constant `KEY_W`.
    pub const KEY_W: GCKeyCode = 26;
/// Mirrors the `GameController` framework constant `KEY_X`.
    pub const KEY_X: GCKeyCode = 27;
/// Mirrors the `GameController` framework constant `KEY_Y`.
    pub const KEY_Y: GCKeyCode = 28;
/// Mirrors the `GameController` framework constant `KEY_Z`.
    pub const KEY_Z: GCKeyCode = 29;
/// Mirrors the `GameController` framework constant `ONE`.
    pub const ONE: GCKeyCode = 30;
/// Mirrors the `GameController` framework constant `TWO`.
    pub const TWO: GCKeyCode = 31;
/// Mirrors the `GameController` framework constant `THREE`.
    pub const THREE: GCKeyCode = 32;
/// Mirrors the `GameController` framework constant `FOUR`.
    pub const FOUR: GCKeyCode = 33;
/// Mirrors the `GameController` framework constant `FIVE`.
    pub const FIVE: GCKeyCode = 34;
/// Mirrors the `GameController` framework constant `SIX`.
    pub const SIX: GCKeyCode = 35;
/// Mirrors the `GameController` framework constant `SEVEN`.
    pub const SEVEN: GCKeyCode = 36;
/// Mirrors the `GameController` framework constant `EIGHT`.
    pub const EIGHT: GCKeyCode = 37;
/// Mirrors the `GameController` framework constant `NINE`.
    pub const NINE: GCKeyCode = 38;
/// Mirrors the `GameController` framework constant `ZERO`.
    pub const ZERO: GCKeyCode = 39;
/// Mirrors the `GameController` framework constant `RETURN_OR_ENTER`.
    pub const RETURN_OR_ENTER: GCKeyCode = 40;
/// Mirrors the `GameController` framework constant `ESCAPE`.
    pub const ESCAPE: GCKeyCode = 41;
/// Mirrors the `GameController` framework constant `DELETE_OR_BACKSPACE`.
    pub const DELETE_OR_BACKSPACE: GCKeyCode = 42;
/// Mirrors the `GameController` framework constant `TAB`.
    pub const TAB: GCKeyCode = 43;
/// Mirrors the `GameController` framework constant `SPACEBAR`.
    pub const SPACEBAR: GCKeyCode = 44;
/// Mirrors the `GameController` framework constant `HYPHEN`.
    pub const HYPHEN: GCKeyCode = 45;
/// Mirrors the `GameController` framework constant `EQUAL_SIGN`.
    pub const EQUAL_SIGN: GCKeyCode = 46;
/// Mirrors the `GameController` framework constant `OPEN_BRACKET`.
    pub const OPEN_BRACKET: GCKeyCode = 47;
/// Mirrors the `GameController` framework constant `CLOSE_BRACKET`.
    pub const CLOSE_BRACKET: GCKeyCode = 48;
/// Mirrors the `GameController` framework constant `BACKSLASH`.
    pub const BACKSLASH: GCKeyCode = 49;
/// Mirrors the `GameController` framework constant `NON_US_POUND`.
    pub const NON_US_POUND: GCKeyCode = 50;
/// Mirrors the `GameController` framework constant `SEMICOLON`.
    pub const SEMICOLON: GCKeyCode = 51;
/// Mirrors the `GameController` framework constant `QUOTE`.
    pub const QUOTE: GCKeyCode = 52;
/// Mirrors the `GameController` framework constant `GRAVE_ACCENT_AND_TILDE`.
    pub const GRAVE_ACCENT_AND_TILDE: GCKeyCode = 53;
/// Mirrors the `GameController` framework constant `COMMA`.
    pub const COMMA: GCKeyCode = 54;
/// Mirrors the `GameController` framework constant `PERIOD`.
    pub const PERIOD: GCKeyCode = 55;
/// Mirrors the `GameController` framework constant `SLASH`.
    pub const SLASH: GCKeyCode = 56;
/// Mirrors the `GameController` framework constant `CAPS_LOCK`.
    pub const CAPS_LOCK: GCKeyCode = 57;
/// Mirrors the `GameController` framework constant `F_1`.
    pub const F_1: GCKeyCode = 58;
/// Mirrors the `GameController` framework constant `F_2`.
    pub const F_2: GCKeyCode = 59;
/// Mirrors the `GameController` framework constant `F_3`.
    pub const F_3: GCKeyCode = 60;
/// Mirrors the `GameController` framework constant `F_4`.
    pub const F_4: GCKeyCode = 61;
/// Mirrors the `GameController` framework constant `F_5`.
    pub const F_5: GCKeyCode = 62;
/// Mirrors the `GameController` framework constant `F_6`.
    pub const F_6: GCKeyCode = 63;
/// Mirrors the `GameController` framework constant `F_7`.
    pub const F_7: GCKeyCode = 64;
/// Mirrors the `GameController` framework constant `F_8`.
    pub const F_8: GCKeyCode = 65;
/// Mirrors the `GameController` framework constant `F_9`.
    pub const F_9: GCKeyCode = 66;
/// Mirrors the `GameController` framework constant `F_10`.
    pub const F_10: GCKeyCode = 67;
/// Mirrors the `GameController` framework constant `F_11`.
    pub const F_11: GCKeyCode = 68;
/// Mirrors the `GameController` framework constant `F_12`.
    pub const F_12: GCKeyCode = 69;
/// Mirrors the `GameController` framework constant `F_13`.
    pub const F_13: GCKeyCode = 104;
/// Mirrors the `GameController` framework constant `F_14`.
    pub const F_14: GCKeyCode = 105;
/// Mirrors the `GameController` framework constant `F_15`.
    pub const F_15: GCKeyCode = 106;
/// Mirrors the `GameController` framework constant `F_16`.
    pub const F_16: GCKeyCode = 107;
/// Mirrors the `GameController` framework constant `F_17`.
    pub const F_17: GCKeyCode = 108;
/// Mirrors the `GameController` framework constant `F_18`.
    pub const F_18: GCKeyCode = 109;
/// Mirrors the `GameController` framework constant `F_19`.
    pub const F_19: GCKeyCode = 110;
/// Mirrors the `GameController` framework constant `F_20`.
    pub const F_20: GCKeyCode = 111;
/// Mirrors the `GameController` framework constant `PRINT_SCREEN`.
    pub const PRINT_SCREEN: GCKeyCode = 70;
/// Mirrors the `GameController` framework constant `SCROLL_LOCK`.
    pub const SCROLL_LOCK: GCKeyCode = 71;
/// Mirrors the `GameController` framework constant `PAUSE`.
    pub const PAUSE: GCKeyCode = 72;
/// Mirrors the `GameController` framework constant `INSERT`.
    pub const INSERT: GCKeyCode = 73;
/// Mirrors the `GameController` framework constant `HOME`.
    pub const HOME: GCKeyCode = 74;
/// Mirrors the `GameController` framework constant `PAGE_UP`.
    pub const PAGE_UP: GCKeyCode = 75;
/// Mirrors the `GameController` framework constant `DELETE_FORWARD`.
    pub const DELETE_FORWARD: GCKeyCode = 76;
/// Mirrors the `GameController` framework constant `END`.
    pub const END: GCKeyCode = 77;
/// Mirrors the `GameController` framework constant `PAGE_DOWN`.
    pub const PAGE_DOWN: GCKeyCode = 78;
/// Mirrors the `GameController` framework constant `RIGHT_ARROW`.
    pub const RIGHT_ARROW: GCKeyCode = 79;
/// Mirrors the `GameController` framework constant `LEFT_ARROW`.
    pub const LEFT_ARROW: GCKeyCode = 80;
/// Mirrors the `GameController` framework constant `DOWN_ARROW`.
    pub const DOWN_ARROW: GCKeyCode = 81;
/// Mirrors the `GameController` framework constant `UP_ARROW`.
    pub const UP_ARROW: GCKeyCode = 82;
/// Mirrors the `GameController` framework constant `KEYPAD_NUM_LOCK`.
    pub const KEYPAD_NUM_LOCK: GCKeyCode = 83;
/// Mirrors the `GameController` framework constant `KEYPAD_SLASH`.
    pub const KEYPAD_SLASH: GCKeyCode = 84;
/// Mirrors the `GameController` framework constant `KEYPAD_ASTERISK`.
    pub const KEYPAD_ASTERISK: GCKeyCode = 85;
/// Mirrors the `GameController` framework constant `KEYPAD_HYPHEN`.
    pub const KEYPAD_HYPHEN: GCKeyCode = 86;
/// Mirrors the `GameController` framework constant `KEYPAD_PLUS`.
    pub const KEYPAD_PLUS: GCKeyCode = 87;
/// Mirrors the `GameController` framework constant `KEYPAD_ENTER`.
    pub const KEYPAD_ENTER: GCKeyCode = 88;
/// Mirrors the `GameController` framework constant `KEYPAD_1`.
    pub const KEYPAD_1: GCKeyCode = 89;
/// Mirrors the `GameController` framework constant `KEYPAD_2`.
    pub const KEYPAD_2: GCKeyCode = 90;
/// Mirrors the `GameController` framework constant `KEYPAD_3`.
    pub const KEYPAD_3: GCKeyCode = 91;
/// Mirrors the `GameController` framework constant `KEYPAD_4`.
    pub const KEYPAD_4: GCKeyCode = 92;
/// Mirrors the `GameController` framework constant `KEYPAD_5`.
    pub const KEYPAD_5: GCKeyCode = 93;
/// Mirrors the `GameController` framework constant `KEYPAD_6`.
    pub const KEYPAD_6: GCKeyCode = 94;
/// Mirrors the `GameController` framework constant `KEYPAD_7`.
    pub const KEYPAD_7: GCKeyCode = 95;
/// Mirrors the `GameController` framework constant `KEYPAD_8`.
    pub const KEYPAD_8: GCKeyCode = 96;
/// Mirrors the `GameController` framework constant `KEYPAD_9`.
    pub const KEYPAD_9: GCKeyCode = 97;
/// Mirrors the `GameController` framework constant `KEYPAD_0`.
    pub const KEYPAD_0: GCKeyCode = 98;
/// Mirrors the `GameController` framework constant `KEYPAD_PERIOD`.
    pub const KEYPAD_PERIOD: GCKeyCode = 99;
/// Mirrors the `GameController` framework constant `KEYPAD_EQUAL_SIGN`.
    pub const KEYPAD_EQUAL_SIGN: GCKeyCode = 103;
/// Mirrors the `GameController` framework constant `NON_US_BACKSLASH`.
    pub const NON_US_BACKSLASH: GCKeyCode = 100;
/// Mirrors the `GameController` framework constant `APPLICATION`.
    pub const APPLICATION: GCKeyCode = 101;
/// Mirrors the `GameController` framework constant `POWER`.
    pub const POWER: GCKeyCode = 102;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_1`.
    pub const INTERNATIONAL_1: GCKeyCode = 135;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_2`.
    pub const INTERNATIONAL_2: GCKeyCode = 136;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_3`.
    pub const INTERNATIONAL_3: GCKeyCode = 137;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_4`.
    pub const INTERNATIONAL_4: GCKeyCode = 138;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_5`.
    pub const INTERNATIONAL_5: GCKeyCode = 139;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_6`.
    pub const INTERNATIONAL_6: GCKeyCode = 140;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_7`.
    pub const INTERNATIONAL_7: GCKeyCode = 141;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_8`.
    pub const INTERNATIONAL_8: GCKeyCode = 142;
/// Mirrors the `GameController` framework constant `INTERNATIONAL_9`.
    pub const INTERNATIONAL_9: GCKeyCode = 143;
/// Mirrors the `GameController` framework constant `LANG_1`.
    pub const LANG_1: GCKeyCode = 144;
/// Mirrors the `GameController` framework constant `LANG_2`.
    pub const LANG_2: GCKeyCode = 145;
/// Mirrors the `GameController` framework constant `LANG_3`.
    pub const LANG_3: GCKeyCode = 146;
/// Mirrors the `GameController` framework constant `LANG_4`.
    pub const LANG_4: GCKeyCode = 147;
/// Mirrors the `GameController` framework constant `LANG_5`.
    pub const LANG_5: GCKeyCode = 148;
/// Mirrors the `GameController` framework constant `LANG_6`.
    pub const LANG_6: GCKeyCode = 149;
/// Mirrors the `GameController` framework constant `LANG_7`.
    pub const LANG_7: GCKeyCode = 150;
/// Mirrors the `GameController` framework constant `LANG_8`.
    pub const LANG_8: GCKeyCode = 151;
/// Mirrors the `GameController` framework constant `LANG_9`.
    pub const LANG_9: GCKeyCode = 152;
/// Mirrors the `GameController` framework constant `LEFT_CONTROL`.
    pub const LEFT_CONTROL: GCKeyCode = 224;
/// Mirrors the `GameController` framework constant `LEFT_SHIFT`.
    pub const LEFT_SHIFT: GCKeyCode = 225;
/// Mirrors the `GameController` framework constant `LEFT_ALT`.
    pub const LEFT_ALT: GCKeyCode = 226;
/// Mirrors the `GameController` framework constant `LEFT_GUI`.
    pub const LEFT_GUI: GCKeyCode = 227;
/// Mirrors the `GameController` framework constant `RIGHT_CONTROL`.
    pub const RIGHT_CONTROL: GCKeyCode = 228;
/// Mirrors the `GameController` framework constant `RIGHT_SHIFT`.
    pub const RIGHT_SHIFT: GCKeyCode = 229;
/// Mirrors the `GameController` framework constant `RIGHT_ALT`.
    pub const RIGHT_ALT: GCKeyCode = 230;
/// Mirrors the `GameController` framework constant `RIGHT_GUI`.
    pub const RIGHT_GUI: GCKeyCode = 231;
}
/// Mirrors Apple's `GCKey*` string constants.
pub mod key_names {
    use super::GCKeyName;
/// Mirrors the `GameController` framework constant `A`.
    pub const A: GCKeyName = GCKeyName::new("A");
/// Mirrors the `GameController` framework constant `B`.
    pub const B: GCKeyName = GCKeyName::new("B");
/// Mirrors the `GameController` framework constant `C`.
    pub const C: GCKeyName = GCKeyName::new("C");
/// Mirrors the `GameController` framework constant `D`.
    pub const D: GCKeyName = GCKeyName::new("D");
/// Mirrors the `GameController` framework constant `E`.
    pub const E: GCKeyName = GCKeyName::new("E");
/// Mirrors the `GameController` framework constant `F`.
    pub const F: GCKeyName = GCKeyName::new("F");
/// Mirrors the `GameController` framework constant `G`.
    pub const G: GCKeyName = GCKeyName::new("G");
/// Mirrors the `GameController` framework constant `H`.
    pub const H: GCKeyName = GCKeyName::new("H");
/// Mirrors the `GameController` framework constant `I`.
    pub const I: GCKeyName = GCKeyName::new("I");
/// Mirrors the `GameController` framework constant `J`.
    pub const J: GCKeyName = GCKeyName::new("J");
/// Mirrors the `GameController` framework constant `K`.
    pub const K: GCKeyName = GCKeyName::new("K");
/// Mirrors the `GameController` framework constant `L`.
    pub const L: GCKeyName = GCKeyName::new("L");
/// Mirrors the `GameController` framework constant `M`.
    pub const M: GCKeyName = GCKeyName::new("M");
/// Mirrors the `GameController` framework constant `N`.
    pub const N: GCKeyName = GCKeyName::new("N");
/// Mirrors the `GameController` framework constant `O`.
    pub const O: GCKeyName = GCKeyName::new("O");
/// Mirrors the `GameController` framework constant `P`.
    pub const P: GCKeyName = GCKeyName::new("P");
/// Mirrors the `GameController` framework constant `Q`.
    pub const Q: GCKeyName = GCKeyName::new("Q");
/// Mirrors the `GameController` framework constant `R`.
    pub const R: GCKeyName = GCKeyName::new("R");
/// Mirrors the `GameController` framework constant `S`.
    pub const S: GCKeyName = GCKeyName::new("S");
/// Mirrors the `GameController` framework constant `T`.
    pub const T: GCKeyName = GCKeyName::new("T");
/// Mirrors the `GameController` framework constant `U`.
    pub const U: GCKeyName = GCKeyName::new("U");
/// Mirrors the `GameController` framework constant `V`.
    pub const V: GCKeyName = GCKeyName::new("V");
/// Mirrors the `GameController` framework constant `W`.
    pub const W: GCKeyName = GCKeyName::new("W");
/// Mirrors the `GameController` framework constant `X`.
    pub const X: GCKeyName = GCKeyName::new("X");
/// Mirrors the `GameController` framework constant `Y`.
    pub const Y: GCKeyName = GCKeyName::new("Y");
/// Mirrors the `GameController` framework constant `Z`.
    pub const Z: GCKeyName = GCKeyName::new("Z");
/// Mirrors the `GameController` framework constant `ONE`.
    pub const ONE: GCKeyName = GCKeyName::new("One");
/// Mirrors the `GameController` framework constant `TWO`.
    pub const TWO: GCKeyName = GCKeyName::new("Two");
/// Mirrors the `GameController` framework constant `THREE`.
    pub const THREE: GCKeyName = GCKeyName::new("Three");
/// Mirrors the `GameController` framework constant `FOUR`.
    pub const FOUR: GCKeyName = GCKeyName::new("Four");
/// Mirrors the `GameController` framework constant `FIVE`.
    pub const FIVE: GCKeyName = GCKeyName::new("Five");
/// Mirrors the `GameController` framework constant `SIX`.
    pub const SIX: GCKeyName = GCKeyName::new("Six");
/// Mirrors the `GameController` framework constant `SEVEN`.
    pub const SEVEN: GCKeyName = GCKeyName::new("Seven");
/// Mirrors the `GameController` framework constant `EIGHT`.
    pub const EIGHT: GCKeyName = GCKeyName::new("Eight");
/// Mirrors the `GameController` framework constant `NINE`.
    pub const NINE: GCKeyName = GCKeyName::new("Nine");
/// Mirrors the `GameController` framework constant `ZERO`.
    pub const ZERO: GCKeyName = GCKeyName::new("Zero");
/// Mirrors the `GameController` framework constant `RETURN_OR_ENTER`.
    pub const RETURN_OR_ENTER: GCKeyName = GCKeyName::new("ReturnOrEnter");
/// Mirrors the `GameController` framework constant `ESCAPE`.
    pub const ESCAPE: GCKeyName = GCKeyName::new("Escape");
/// Mirrors the `GameController` framework constant `DELETE_OR_BACKSPACE`.
    pub const DELETE_OR_BACKSPACE: GCKeyName = GCKeyName::new("DeleteOrBackspace");
/// Mirrors the `GameController` framework constant `TAB`.
    pub const TAB: GCKeyName = GCKeyName::new("Tab");
/// Mirrors the `GameController` framework constant `SPACEBAR`.
    pub const SPACEBAR: GCKeyName = GCKeyName::new("Spacebar");
/// Mirrors the `GameController` framework constant `HYPHEN`.
    pub const HYPHEN: GCKeyName = GCKeyName::new("Hyphen");
/// Mirrors the `GameController` framework constant `EQUAL_SIGN`.
    pub const EQUAL_SIGN: GCKeyName = GCKeyName::new("EqualSign");
/// Mirrors the `GameController` framework constant `OPEN_BRACKET`.
    pub const OPEN_BRACKET: GCKeyName = GCKeyName::new("OpenBracket");
/// Mirrors the `GameController` framework constant `CLOSE_BRACKET`.
    pub const CLOSE_BRACKET: GCKeyName = GCKeyName::new("CloseBracket");
/// Mirrors the `GameController` framework constant `BACKSLASH`.
    pub const BACKSLASH: GCKeyName = GCKeyName::new("Backslash");
/// Mirrors the `GameController` framework constant `NON_US_POUND`.
    pub const NON_US_POUND: GCKeyName = GCKeyName::new("NonUSPound");
/// Mirrors the `GameController` framework constant `SEMICOLON`.
    pub const SEMICOLON: GCKeyName = GCKeyName::new("Semicolon");
/// Mirrors the `GameController` framework constant `QUOTE`.
    pub const QUOTE: GCKeyName = GCKeyName::new("Quote");
/// Mirrors the `GameController` framework constant `GRAVE_ACCENT_AND_TILDE`.
    pub const GRAVE_ACCENT_AND_TILDE: GCKeyName = GCKeyName::new("GraveAccentAndTilde");
/// Mirrors the `GameController` framework constant `COMMA`.
    pub const COMMA: GCKeyName = GCKeyName::new("Comma");
/// Mirrors the `GameController` framework constant `PERIOD`.
    pub const PERIOD: GCKeyName = GCKeyName::new("Period");
/// Mirrors the `GameController` framework constant `SLASH`.
    pub const SLASH: GCKeyName = GCKeyName::new("Slash");
/// Mirrors the `GameController` framework constant `CAPS_LOCK`.
    pub const CAPS_LOCK: GCKeyName = GCKeyName::new("CapsLock");
/// Mirrors the `GameController` framework constant `F_1`.
    pub const F_1: GCKeyName = GCKeyName::new("F1");
/// Mirrors the `GameController` framework constant `F_2`.
    pub const F_2: GCKeyName = GCKeyName::new("F2");
/// Mirrors the `GameController` framework constant `F_3`.
    pub const F_3: GCKeyName = GCKeyName::new("F3");
/// Mirrors the `GameController` framework constant `F_4`.
    pub const F_4: GCKeyName = GCKeyName::new("F4");
/// Mirrors the `GameController` framework constant `F_5`.
    pub const F_5: GCKeyName = GCKeyName::new("F5");
/// Mirrors the `GameController` framework constant `F_6`.
    pub const F_6: GCKeyName = GCKeyName::new("F6");
/// Mirrors the `GameController` framework constant `F_7`.
    pub const F_7: GCKeyName = GCKeyName::new("F7");
/// Mirrors the `GameController` framework constant `F_8`.
    pub const F_8: GCKeyName = GCKeyName::new("F8");
/// Mirrors the `GameController` framework constant `F_9`.
    pub const F_9: GCKeyName = GCKeyName::new("F9");
/// Mirrors the `GameController` framework constant `F_10`.
    pub const F_10: GCKeyName = GCKeyName::new("F10");
/// Mirrors the `GameController` framework constant `F_11`.
    pub const F_11: GCKeyName = GCKeyName::new("F11");
/// Mirrors the `GameController` framework constant `F_12`.
    pub const F_12: GCKeyName = GCKeyName::new("F12");
/// Mirrors the `GameController` framework constant `F_13`.
    pub const F_13: GCKeyName = GCKeyName::new("F13");
/// Mirrors the `GameController` framework constant `F_14`.
    pub const F_14: GCKeyName = GCKeyName::new("F14");
/// Mirrors the `GameController` framework constant `F_15`.
    pub const F_15: GCKeyName = GCKeyName::new("F15");
/// Mirrors the `GameController` framework constant `F_16`.
    pub const F_16: GCKeyName = GCKeyName::new("F16");
/// Mirrors the `GameController` framework constant `F_17`.
    pub const F_17: GCKeyName = GCKeyName::new("F17");
/// Mirrors the `GameController` framework constant `F_18`.
    pub const F_18: GCKeyName = GCKeyName::new("F18");
/// Mirrors the `GameController` framework constant `F_19`.
    pub const F_19: GCKeyName = GCKeyName::new("F19");
/// Mirrors the `GameController` framework constant `F_20`.
    pub const F_20: GCKeyName = GCKeyName::new("F20");
/// Mirrors the `GameController` framework constant `PRINT_SCREEN`.
    pub const PRINT_SCREEN: GCKeyName = GCKeyName::new("PrintScreen");
/// Mirrors the `GameController` framework constant `SCROLL_LOCK`.
    pub const SCROLL_LOCK: GCKeyName = GCKeyName::new("ScrollLock");
/// Mirrors the `GameController` framework constant `PAUSE`.
    pub const PAUSE: GCKeyName = GCKeyName::new("Pause");
/// Mirrors the `GameController` framework constant `INSERT`.
    pub const INSERT: GCKeyName = GCKeyName::new("Insert");
/// Mirrors the `GameController` framework constant `HOME`.
    pub const HOME: GCKeyName = GCKeyName::new("Home");
/// Mirrors the `GameController` framework constant `PAGE_UP`.
    pub const PAGE_UP: GCKeyName = GCKeyName::new("PageUp");
/// Mirrors the `GameController` framework constant `DELETE_FORWARD`.
    pub const DELETE_FORWARD: GCKeyName = GCKeyName::new("DeleteForward");
/// Mirrors the `GameController` framework constant `END`.
    pub const END: GCKeyName = GCKeyName::new("End");
/// Mirrors the `GameController` framework constant `PAGE_DOWN`.
    pub const PAGE_DOWN: GCKeyName = GCKeyName::new("PageDown");
/// Mirrors the `GameController` framework constant `RIGHT_ARROW`.
    pub const RIGHT_ARROW: GCKeyName = GCKeyName::new("RightArrow");
/// Mirrors the `GameController` framework constant `LEFT_ARROW`.
    pub const LEFT_ARROW: GCKeyName = GCKeyName::new("LeftArrow");
/// Mirrors the `GameController` framework constant `DOWN_ARROW`.
    pub const DOWN_ARROW: GCKeyName = GCKeyName::new("DownArrow");
/// Mirrors the `GameController` framework constant `UP_ARROW`.
    pub const UP_ARROW: GCKeyName = GCKeyName::new("UpArrow");
/// Mirrors the `GameController` framework constant `KEYPAD_NUM_LOCK`.
    pub const KEYPAD_NUM_LOCK: GCKeyName = GCKeyName::new("KeypadNumLock");
/// Mirrors the `GameController` framework constant `KEYPAD_SLASH`.
    pub const KEYPAD_SLASH: GCKeyName = GCKeyName::new("KeypadSlash");
/// Mirrors the `GameController` framework constant `KEYPAD_ASTERISK`.
    pub const KEYPAD_ASTERISK: GCKeyName = GCKeyName::new("KeypadAsterisk");
/// Mirrors the `GameController` framework constant `KEYPAD_HYPHEN`.
    pub const KEYPAD_HYPHEN: GCKeyName = GCKeyName::new("KeypadHyphen");
/// Mirrors the `GameController` framework constant `KEYPAD_PLUS`.
    pub const KEYPAD_PLUS: GCKeyName = GCKeyName::new("KeypadPlus");
/// Mirrors the `GameController` framework constant `KEYPAD_ENTER`.
    pub const KEYPAD_ENTER: GCKeyName = GCKeyName::new("KeypadEnter");
/// Mirrors the `GameController` framework constant `KEYPAD_1`.
    pub const KEYPAD_1: GCKeyName = GCKeyName::new("Keypad1");
/// Mirrors the `GameController` framework constant `KEYPAD_2`.
    pub const KEYPAD_2: GCKeyName = GCKeyName::new("Keypad2");
/// Mirrors the `GameController` framework constant `KEYPAD_3`.
    pub const KEYPAD_3: GCKeyName = GCKeyName::new("Keypad3");
/// Mirrors the `GameController` framework constant `KEYPAD_4`.
    pub const KEYPAD_4: GCKeyName = GCKeyName::new("Keypad4");
/// Mirrors the `GameController` framework constant `KEYPAD_5`.
    pub const KEYPAD_5: GCKeyName = GCKeyName::new("Keypad5");
/// Mirrors the `GameController` framework constant `KEYPAD_6`.
    pub const KEYPAD_6: GCKeyName = GCKeyName::new("Keypad6");
/// Mirrors the `GameController` framework constant `KEYPAD_7`.
    pub const KEYPAD_7: GCKeyName = GCKeyName::new("Keypad7");
/// Mirrors the `GameController` framework constant `KEYPAD_8`.
    pub const KEYPAD_8: GCKeyName = GCKeyName::new("Keypad8");
/// Mirrors the `GameController` framework constant `KEYPAD_9`.
    pub const KEYPAD_9: GCKeyName = GCKeyName::new("Keypad9");
/// Mirrors the `GameController` framework constant `KEYPAD_0`.
    pub const KEYPAD_0: GCKeyName = GCKeyName::new("Keypad0");
/// Mirrors the `GameController` framework constant `KEYPAD_PERIOD`.
    pub const KEYPAD_PERIOD: GCKeyName = GCKeyName::new("KeypadPeriod");
/// Mirrors the `GameController` framework constant `KEYPAD_EQUAL_SIGN`.
    pub const KEYPAD_EQUAL_SIGN: GCKeyName = GCKeyName::new("KeypadEqualSign");
/// Mirrors the `GameController` framework constant `NON_US_BACKSLASH`.
    pub const NON_US_BACKSLASH: GCKeyName = GCKeyName::new("NonUSBackslash");
/// Mirrors the `GameController` framework constant `APPLICATION`.
    pub const APPLICATION: GCKeyName = GCKeyName::new("Application");
/// Mirrors the `GameController` framework constant `POWER`.
    pub const POWER: GCKeyName = GCKeyName::new("Power");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_1`.
    pub const INTERNATIONAL_1: GCKeyName = GCKeyName::new("International1");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_2`.
    pub const INTERNATIONAL_2: GCKeyName = GCKeyName::new("International2");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_3`.
    pub const INTERNATIONAL_3: GCKeyName = GCKeyName::new("International3");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_4`.
    pub const INTERNATIONAL_4: GCKeyName = GCKeyName::new("International4");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_5`.
    pub const INTERNATIONAL_5: GCKeyName = GCKeyName::new("International5");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_6`.
    pub const INTERNATIONAL_6: GCKeyName = GCKeyName::new("International6");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_7`.
    pub const INTERNATIONAL_7: GCKeyName = GCKeyName::new("International7");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_8`.
    pub const INTERNATIONAL_8: GCKeyName = GCKeyName::new("International8");
/// Mirrors the `GameController` framework constant `INTERNATIONAL_9`.
    pub const INTERNATIONAL_9: GCKeyName = GCKeyName::new("International9");
/// Mirrors the `GameController` framework constant `LANG_1`.
    pub const LANG_1: GCKeyName = GCKeyName::new("LANG1");
/// Mirrors the `GameController` framework constant `LANG_2`.
    pub const LANG_2: GCKeyName = GCKeyName::new("LANG2");
/// Mirrors the `GameController` framework constant `LANG_3`.
    pub const LANG_3: GCKeyName = GCKeyName::new("LANG3");
/// Mirrors the `GameController` framework constant `LANG_4`.
    pub const LANG_4: GCKeyName = GCKeyName::new("LANG4");
/// Mirrors the `GameController` framework constant `LANG_5`.
    pub const LANG_5: GCKeyName = GCKeyName::new("LANG5");
/// Mirrors the `GameController` framework constant `LANG_6`.
    pub const LANG_6: GCKeyName = GCKeyName::new("LANG6");
/// Mirrors the `GameController` framework constant `LANG_7`.
    pub const LANG_7: GCKeyName = GCKeyName::new("LANG7");
/// Mirrors the `GameController` framework constant `LANG_8`.
    pub const LANG_8: GCKeyName = GCKeyName::new("LANG8");
/// Mirrors the `GameController` framework constant `LANG_9`.
    pub const LANG_9: GCKeyName = GCKeyName::new("LANG9");
/// Mirrors the `GameController` framework constant `LEFT_CONTROL`.
    pub const LEFT_CONTROL: GCKeyName = GCKeyName::new("LeftControl");
/// Mirrors the `GameController` framework constant `LEFT_SHIFT`.
    pub const LEFT_SHIFT: GCKeyName = GCKeyName::new("LeftShift");
/// Mirrors the `GameController` framework constant `LEFT_ALT`.
    pub const LEFT_ALT: GCKeyName = GCKeyName::new("LeftAlt");
/// Mirrors the `GameController` framework constant `LEFT_GUI`.
    pub const LEFT_GUI: GCKeyName = GCKeyName::new("LeftGUI");
/// Mirrors the `GameController` framework constant `RIGHT_CONTROL`.
    pub const RIGHT_CONTROL: GCKeyName = GCKeyName::new("RightControl");
/// Mirrors the `GameController` framework constant `RIGHT_SHIFT`.
    pub const RIGHT_SHIFT: GCKeyName = GCKeyName::new("RightShift");
/// Mirrors the `GameController` framework constant `RIGHT_ALT`.
    pub const RIGHT_ALT: GCKeyName = GCKeyName::new("RightAlt");
/// Mirrors the `GameController` framework constant `RIGHT_GUI`.
    pub const RIGHT_GUI: GCKeyName = GCKeyName::new("RightGUI");
}
/// Mirrors Apple's `GCInput*` string constants.
pub mod input_names {
    use super::{
        GCButtonElementName, GCDirectionPadElementName, GCInputAxisName, GCInputButtonName,
        GCInputDirectionPadName, GCInputElementName,
    };
/// Mirrors the `GameController` framework constant `BUTTON_A`.
    pub const BUTTON_A: GCInputButtonName = GCInputButtonName::new("Button A");
/// Mirrors the `GameController` framework constant `BUTTON_B`.
    pub const BUTTON_B: GCInputButtonName = GCInputButtonName::new("Button B");
/// Mirrors the `GameController` framework constant `BUTTON_X`.
    pub const BUTTON_X: GCInputButtonName = GCInputButtonName::new("Button X");
/// Mirrors the `GameController` framework constant `BUTTON_Y`.
    pub const BUTTON_Y: GCInputButtonName = GCInputButtonName::new("Button Y");
/// Mirrors the `GameController` framework constant `DIRECTION_PAD`.
    pub const DIRECTION_PAD: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Direction Pad");
/// Mirrors the `GameController` framework constant `THUMBSTICK`.
    pub const THUMBSTICK: GCInputDirectionPadName = GCInputDirectionPadName::new("Thumbstick");
/// Mirrors the `GameController` framework constant `LEFT_THUMBSTICK`.
    pub const LEFT_THUMBSTICK: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Left Thumbstick");
/// Mirrors the `GameController` framework constant `RIGHT_THUMBSTICK`.
    pub const RIGHT_THUMBSTICK: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Right Thumbstick");
/// Mirrors the `GameController` framework constant `THUMBSTICK_BUTTON`.
    pub const THUMBSTICK_BUTTON: GCInputButtonName = GCInputButtonName::new("Thumbstick Button");
/// Mirrors the `GameController` framework constant `LEFT_THUMBSTICK_BUTTON`.
    pub const LEFT_THUMBSTICK_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Left Thumbstick Button");
/// Mirrors the `GameController` framework constant `RIGHT_THUMBSTICK_BUTTON`.
    pub const RIGHT_THUMBSTICK_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Right Thumbstick Button");
/// Mirrors the `GameController` framework constant `GRIP_BUTTON`.
    pub const GRIP_BUTTON: GCInputButtonName = GCInputButtonName::new("Grip");
/// Mirrors the `GameController` framework constant `LEFT_SHOULDER`.
    pub const LEFT_SHOULDER: GCInputButtonName = GCInputButtonName::new("Left Shoulder");
/// Mirrors the `GameController` framework constant `RIGHT_SHOULDER`.
    pub const RIGHT_SHOULDER: GCInputButtonName = GCInputButtonName::new("Right Shoulder");
/// Mirrors the `GameController` framework constant `LEFT_BUMPER`.
    pub const LEFT_BUMPER: GCInputButtonName = GCInputButtonName::new("Left Bumper");
/// Mirrors the `GameController` framework constant `RIGHT_BUMPER`.
    pub const RIGHT_BUMPER: GCInputButtonName = GCInputButtonName::new("Right Bumper");
/// Mirrors the `GameController` framework constant `TRIGGER`.
    pub const TRIGGER: GCInputButtonName = GCInputButtonName::new("Trigger");
/// Mirrors the `GameController` framework constant `LEFT_TRIGGER`.
    pub const LEFT_TRIGGER: GCInputButtonName = GCInputButtonName::new("Left Trigger");
/// Mirrors the `GameController` framework constant `RIGHT_TRIGGER`.
    pub const RIGHT_TRIGGER: GCInputButtonName = GCInputButtonName::new("Right Trigger");
/// Mirrors the `GameController` framework constant `BUTTON_HOME`.
    pub const BUTTON_HOME: GCInputButtonName = GCInputButtonName::new("Button Home");
/// Mirrors the `GameController` framework constant `BUTTON_MENU`.
    pub const BUTTON_MENU: GCInputButtonName = GCInputButtonName::new("Button Menu");
/// Mirrors the `GameController` framework constant `BUTTON_OPTIONS`.
    pub const BUTTON_OPTIONS: GCInputButtonName = GCInputButtonName::new("Button Options");
/// Mirrors the `GameController` framework constant `BUTTON_SHARE`.
    pub const BUTTON_SHARE: GCInputButtonName = GCInputButtonName::new("Button Share");
/// Mirrors the `GameController` framework constant `XBOX_PADDLE_ONE`.
    pub const XBOX_PADDLE_ONE: GCInputButtonName = GCInputButtonName::new("Paddle 1");
/// Mirrors the `GameController` framework constant `XBOX_PADDLE_TWO`.
    pub const XBOX_PADDLE_TWO: GCInputButtonName = GCInputButtonName::new("Paddle 2");
/// Mirrors the `GameController` framework constant `XBOX_PADDLE_THREE`.
    pub const XBOX_PADDLE_THREE: GCInputButtonName = GCInputButtonName::new("Paddle 3");
/// Mirrors the `GameController` framework constant `XBOX_PADDLE_FOUR`.
    pub const XBOX_PADDLE_FOUR: GCInputButtonName = GCInputButtonName::new("Paddle 4");
/// Mirrors the `GameController` framework constant `DUAL_SHOCK_TOUCHPAD_ONE`.
    pub const DUAL_SHOCK_TOUCHPAD_ONE: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Touchpad 1");
/// Mirrors the `GameController` framework constant `DUAL_SHOCK_TOUCHPAD_TWO`.
    pub const DUAL_SHOCK_TOUCHPAD_TWO: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Touchpad 2");
/// Mirrors the `GameController` framework constant `DUAL_SHOCK_TOUCHPAD_BUTTON`.
    pub const DUAL_SHOCK_TOUCHPAD_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Touchpad Button");
/// Mirrors the `GameController` framework constant `STEERING_WHEEL`.
    pub const STEERING_WHEEL: GCInputAxisName = GCInputAxisName::new("Steering Wheel");
/// Mirrors the `GameController` framework constant `SHIFTER`.
    pub const SHIFTER: GCInputElementName = GCInputElementName::new("Shifter");
/// Mirrors the `GameController` framework constant `PEDAL_ACCELERATOR`.
    pub const PEDAL_ACCELERATOR: GCInputButtonName = GCInputButtonName::new("Accelerator Pedal");
/// Mirrors the `GameController` framework constant `PEDAL_BRAKE`.
    pub const PEDAL_BRAKE: GCInputButtonName = GCInputButtonName::new("Brake Pedal");
/// Mirrors the `GameController` framework constant `PEDAL_CLUTCH`.
    pub const PEDAL_CLUTCH: GCInputButtonName = GCInputButtonName::new("Clutch Pedal");
/// Mirrors the `GameController` framework constant `LEFT_PADDLE`.
    pub const LEFT_PADDLE: GCInputButtonName = GCInputButtonName::new("Left Paddle");
/// Mirrors the `GameController` framework constant `RIGHT_PADDLE`.
    pub const RIGHT_PADDLE: GCInputButtonName = GCInputButtonName::new("Right Paddle");
/// Mirrors the `GameController` framework constant `DIRECTIONAL_DPAD`.
    pub const DIRECTIONAL_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Direction Pad");
/// Mirrors the `GameController` framework constant `DIRECTIONAL_TOUCH_SURFACE_BUTTON`.
    pub const DIRECTIONAL_TOUCH_SURFACE_BUTTON: GCButtonElementName =
        GCButtonElementName::new("Button A");
/// Mirrors the `GameController` framework constant `DIRECTIONAL_CARDINAL_DPAD`.
    pub const DIRECTIONAL_CARDINAL_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Cardinal Direction Pad");
/// Mirrors the `GameController` framework constant `DIRECTIONAL_CENTER_BUTTON`.
    pub const DIRECTIONAL_CENTER_BUTTON: GCButtonElementName =
        GCButtonElementName::new("Button Center");
/// Mirrors the `GameController` framework constant `MICRO_GAMEPAD_DPAD`.
    pub const MICRO_GAMEPAD_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Direction Pad");
/// Mirrors the `GameController` framework constant `MICRO_GAMEPAD_BUTTON_A`.
    pub const MICRO_GAMEPAD_BUTTON_A: GCButtonElementName = GCButtonElementName::new("Button A");
/// Mirrors the `GameController` framework constant `MICRO_GAMEPAD_BUTTON_X`.
    pub const MICRO_GAMEPAD_BUTTON_X: GCButtonElementName = GCButtonElementName::new("Button X");
/// Mirrors the `GameController` framework constant `MICRO_GAMEPAD_BUTTON_MENU`.
    pub const MICRO_GAMEPAD_BUTTON_MENU: GCButtonElementName =
        GCButtonElementName::new("Button Menu");

    /// Mirrors `GCInputBackLeftButton(position)`.
    #[must_use]
    pub fn back_left_button(position: usize) -> String {
        format!("Back Left Button {position}")
    }

    /// Mirrors `GCInputBackRightButton(position)`.
    #[must_use]
    pub fn back_right_button(position: usize) -> String {
        format!("Back Right Button {position}")
    }

    /// Mirrors `GCInputArcadeButtonName(row, column)`.
    #[must_use]
    pub fn arcade_button_name(row: usize, column: usize) -> String {
        format!("Arcade Button {row}, {column}")
    }
}

/// Mirrors Apple's `GCHapticsLocality*` string constants.
pub mod haptics_localities {
    use super::GCHapticsLocality;

/// Mirrors the `GameController` framework constant `DEFAULT`.
    pub const DEFAULT: GCHapticsLocality = GCHapticsLocality::new("Default");
/// Mirrors the `GameController` framework constant `ALL`.
    pub const ALL: GCHapticsLocality = GCHapticsLocality::new("All");
/// Mirrors the `GameController` framework constant `HANDLES`.
    pub const HANDLES: GCHapticsLocality = GCHapticsLocality::new("Handles");
/// Mirrors the `GameController` framework constant `LEFT_HANDLE`.
    pub const LEFT_HANDLE: GCHapticsLocality = GCHapticsLocality::new("Left Handle");
/// Mirrors the `GameController` framework constant `RIGHT_HANDLE`.
    pub const RIGHT_HANDLE: GCHapticsLocality = GCHapticsLocality::new("Right Handle");
/// Mirrors the `GameController` framework constant `TRIGGERS`.
    pub const TRIGGERS: GCHapticsLocality = GCHapticsLocality::new("Triggers");
/// Mirrors the `GameController` framework constant `LEFT_TRIGGER`.
    pub const LEFT_TRIGGER: GCHapticsLocality = GCHapticsLocality::new("Left Trigger");
/// Mirrors the `GameController` framework constant `RIGHT_TRIGGER`.
    pub const RIGHT_TRIGGER: GCHapticsLocality = GCHapticsLocality::new("Right Trigger");
}

/// Mirrors Apple's `GCHapticDurationInfinite` constant.
pub const HAPTIC_DURATION_INFINITE: f32 = 1_000_000.0;

/// Mirrors Apple's `GCProductCategory*` string constants.
pub mod product_categories {
    use super::GCProductCategory;

/// Mirrors the `GameController` framework constant `ARCADE_STICK`.
    pub const ARCADE_STICK: GCProductCategory = GCProductCategory::new("Arcade Stick");
/// Mirrors the `GameController` framework constant `COALESCED_REMOTE`.
    pub const COALESCED_REMOTE: GCProductCategory = GCProductCategory::new("Coalesced Remote");
/// Mirrors the `GameController` framework constant `CONTROL_CENTER_REMOTE`.
    pub const CONTROL_CENTER_REMOTE: GCProductCategory =
        GCProductCategory::new("Control Center Remote");
/// Mirrors the `GameController` framework constant `DUAL_SENSE`.
    pub const DUAL_SENSE: GCProductCategory = GCProductCategory::new("DualSense");
/// Mirrors the `GameController` framework constant `DUAL_SHOCK_4`.
    pub const DUAL_SHOCK_4: GCProductCategory = GCProductCategory::new("DualShock 4");
/// Mirrors the `GameController` framework constant `HID`.
    pub const HID: GCProductCategory = GCProductCategory::new("HID");
/// Mirrors the `GameController` framework constant `KEYBOARD`.
    pub const KEYBOARD: GCProductCategory = GCProductCategory::new("Keyboard");
/// Mirrors the `GameController` framework constant `MFI`.
    pub const MFI: GCProductCategory = GCProductCategory::new("MFi");
/// Mirrors the `GameController` framework constant `MOUSE`.
    pub const MOUSE: GCProductCategory = GCProductCategory::new("Mouse");
/// Mirrors the `GameController` framework constant `SIRI_REMOTE_1ST_GEN`.
    pub const SIRI_REMOTE_1ST_GEN: GCProductCategory =
        GCProductCategory::new("Siri Remote (1st Generation)");
/// Mirrors the `GameController` framework constant `SIRI_REMOTE_2ND_GEN`.
    pub const SIRI_REMOTE_2ND_GEN: GCProductCategory =
        GCProductCategory::new("Siri Remote (2nd Generation)");
/// Mirrors the `GameController` framework constant `SPATIAL_CONTROLLER`.
    pub const SPATIAL_CONTROLLER: GCProductCategory = GCProductCategory::new("Spatial Controller");
/// Mirrors the `GameController` framework constant `UNIVERSAL_ELECTRONICS_REMOTE`.
    pub const UNIVERSAL_ELECTRONICS_REMOTE: GCProductCategory =
        GCProductCategory::new("Universal Electronics Remote");
/// Mirrors the `GameController` framework constant `XBOX_ONE`.
    pub const XBOX_ONE: GCProductCategory = GCProductCategory::new("Xbox One");
}
