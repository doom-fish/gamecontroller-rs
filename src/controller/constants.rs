//! High-impact `GameController` constant tables mirrored from `MacOSX26.2.sdk`.
//!
//! These constants cover the large `GCKeyCode*`, `GCKey*`, and `GCInput*`
//! symbol families that dominate the framework's public surface area.

use core::fmt;

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
    pub const KEY_A: GCKeyCode = 4;
    pub const KEY_B: GCKeyCode = 5;
    pub const KEY_C: GCKeyCode = 6;
    pub const KEY_D: GCKeyCode = 7;
    pub const KEY_E: GCKeyCode = 8;
    pub const KEY_F: GCKeyCode = 9;
    pub const KEY_G: GCKeyCode = 10;
    pub const KEY_H: GCKeyCode = 11;
    pub const KEY_I: GCKeyCode = 12;
    pub const KEY_J: GCKeyCode = 13;
    pub const KEY_K: GCKeyCode = 14;
    pub const KEY_L: GCKeyCode = 15;
    pub const KEY_M: GCKeyCode = 16;
    pub const KEY_N: GCKeyCode = 17;
    pub const KEY_O: GCKeyCode = 18;
    pub const KEY_P: GCKeyCode = 19;
    pub const KEY_Q: GCKeyCode = 20;
    pub const KEY_R: GCKeyCode = 21;
    pub const KEY_S: GCKeyCode = 22;
    pub const KEY_T: GCKeyCode = 23;
    pub const KEY_U: GCKeyCode = 24;
    pub const KEY_V: GCKeyCode = 25;
    pub const KEY_W: GCKeyCode = 26;
    pub const KEY_X: GCKeyCode = 27;
    pub const KEY_Y: GCKeyCode = 28;
    pub const KEY_Z: GCKeyCode = 29;
    pub const ONE: GCKeyCode = 30;
    pub const TWO: GCKeyCode = 31;
    pub const THREE: GCKeyCode = 32;
    pub const FOUR: GCKeyCode = 33;
    pub const FIVE: GCKeyCode = 34;
    pub const SIX: GCKeyCode = 35;
    pub const SEVEN: GCKeyCode = 36;
    pub const EIGHT: GCKeyCode = 37;
    pub const NINE: GCKeyCode = 38;
    pub const ZERO: GCKeyCode = 39;
    pub const RETURN_OR_ENTER: GCKeyCode = 40;
    pub const ESCAPE: GCKeyCode = 41;
    pub const DELETE_OR_BACKSPACE: GCKeyCode = 42;
    pub const TAB: GCKeyCode = 43;
    pub const SPACEBAR: GCKeyCode = 44;
    pub const HYPHEN: GCKeyCode = 45;
    pub const EQUAL_SIGN: GCKeyCode = 46;
    pub const OPEN_BRACKET: GCKeyCode = 47;
    pub const CLOSE_BRACKET: GCKeyCode = 48;
    pub const BACKSLASH: GCKeyCode = 49;
    pub const NON_US_POUND: GCKeyCode = 50;
    pub const SEMICOLON: GCKeyCode = 51;
    pub const QUOTE: GCKeyCode = 52;
    pub const GRAVE_ACCENT_AND_TILDE: GCKeyCode = 53;
    pub const COMMA: GCKeyCode = 54;
    pub const PERIOD: GCKeyCode = 55;
    pub const SLASH: GCKeyCode = 56;
    pub const CAPS_LOCK: GCKeyCode = 57;
    pub const F_1: GCKeyCode = 58;
    pub const F_2: GCKeyCode = 59;
    pub const F_3: GCKeyCode = 60;
    pub const F_4: GCKeyCode = 61;
    pub const F_5: GCKeyCode = 62;
    pub const F_6: GCKeyCode = 63;
    pub const F_7: GCKeyCode = 64;
    pub const F_8: GCKeyCode = 65;
    pub const F_9: GCKeyCode = 66;
    pub const F_10: GCKeyCode = 67;
    pub const F_11: GCKeyCode = 68;
    pub const F_12: GCKeyCode = 69;
    pub const F_13: GCKeyCode = 104;
    pub const F_14: GCKeyCode = 105;
    pub const F_15: GCKeyCode = 106;
    pub const F_16: GCKeyCode = 107;
    pub const F_17: GCKeyCode = 108;
    pub const F_18: GCKeyCode = 109;
    pub const F_19: GCKeyCode = 110;
    pub const F_20: GCKeyCode = 111;
    pub const PRINT_SCREEN: GCKeyCode = 70;
    pub const SCROLL_LOCK: GCKeyCode = 71;
    pub const PAUSE: GCKeyCode = 72;
    pub const INSERT: GCKeyCode = 73;
    pub const HOME: GCKeyCode = 74;
    pub const PAGE_UP: GCKeyCode = 75;
    pub const DELETE_FORWARD: GCKeyCode = 76;
    pub const END: GCKeyCode = 77;
    pub const PAGE_DOWN: GCKeyCode = 78;
    pub const RIGHT_ARROW: GCKeyCode = 79;
    pub const LEFT_ARROW: GCKeyCode = 80;
    pub const DOWN_ARROW: GCKeyCode = 81;
    pub const UP_ARROW: GCKeyCode = 82;
    pub const KEYPAD_NUM_LOCK: GCKeyCode = 83;
    pub const KEYPAD_SLASH: GCKeyCode = 84;
    pub const KEYPAD_ASTERISK: GCKeyCode = 85;
    pub const KEYPAD_HYPHEN: GCKeyCode = 86;
    pub const KEYPAD_PLUS: GCKeyCode = 87;
    pub const KEYPAD_ENTER: GCKeyCode = 88;
    pub const KEYPAD_1: GCKeyCode = 89;
    pub const KEYPAD_2: GCKeyCode = 90;
    pub const KEYPAD_3: GCKeyCode = 91;
    pub const KEYPAD_4: GCKeyCode = 92;
    pub const KEYPAD_5: GCKeyCode = 93;
    pub const KEYPAD_6: GCKeyCode = 94;
    pub const KEYPAD_7: GCKeyCode = 95;
    pub const KEYPAD_8: GCKeyCode = 96;
    pub const KEYPAD_9: GCKeyCode = 97;
    pub const KEYPAD_0: GCKeyCode = 98;
    pub const KEYPAD_PERIOD: GCKeyCode = 99;
    pub const KEYPAD_EQUAL_SIGN: GCKeyCode = 103;
    pub const NON_US_BACKSLASH: GCKeyCode = 100;
    pub const APPLICATION: GCKeyCode = 101;
    pub const POWER: GCKeyCode = 102;
    pub const INTERNATIONAL_1: GCKeyCode = 135;
    pub const INTERNATIONAL_2: GCKeyCode = 136;
    pub const INTERNATIONAL_3: GCKeyCode = 137;
    pub const INTERNATIONAL_4: GCKeyCode = 138;
    pub const INTERNATIONAL_5: GCKeyCode = 139;
    pub const INTERNATIONAL_6: GCKeyCode = 140;
    pub const INTERNATIONAL_7: GCKeyCode = 141;
    pub const INTERNATIONAL_8: GCKeyCode = 142;
    pub const INTERNATIONAL_9: GCKeyCode = 143;
    pub const LANG_1: GCKeyCode = 144;
    pub const LANG_2: GCKeyCode = 145;
    pub const LANG_3: GCKeyCode = 146;
    pub const LANG_4: GCKeyCode = 147;
    pub const LANG_5: GCKeyCode = 148;
    pub const LANG_6: GCKeyCode = 149;
    pub const LANG_7: GCKeyCode = 150;
    pub const LANG_8: GCKeyCode = 151;
    pub const LANG_9: GCKeyCode = 152;
    pub const LEFT_CONTROL: GCKeyCode = 224;
    pub const LEFT_SHIFT: GCKeyCode = 225;
    pub const LEFT_ALT: GCKeyCode = 226;
    pub const LEFT_GUI: GCKeyCode = 227;
    pub const RIGHT_CONTROL: GCKeyCode = 228;
    pub const RIGHT_SHIFT: GCKeyCode = 229;
    pub const RIGHT_ALT: GCKeyCode = 230;
    pub const RIGHT_GUI: GCKeyCode = 231;
}
/// Mirrors Apple's `GCKey*` string constants.
pub mod key_names {
    use super::GCKeyName;
    pub const A: GCKeyName = GCKeyName::new("A");
    pub const B: GCKeyName = GCKeyName::new("B");
    pub const C: GCKeyName = GCKeyName::new("C");
    pub const D: GCKeyName = GCKeyName::new("D");
    pub const E: GCKeyName = GCKeyName::new("E");
    pub const F: GCKeyName = GCKeyName::new("F");
    pub const G: GCKeyName = GCKeyName::new("G");
    pub const H: GCKeyName = GCKeyName::new("H");
    pub const I: GCKeyName = GCKeyName::new("I");
    pub const J: GCKeyName = GCKeyName::new("J");
    pub const K: GCKeyName = GCKeyName::new("K");
    pub const L: GCKeyName = GCKeyName::new("L");
    pub const M: GCKeyName = GCKeyName::new("M");
    pub const N: GCKeyName = GCKeyName::new("N");
    pub const O: GCKeyName = GCKeyName::new("O");
    pub const P: GCKeyName = GCKeyName::new("P");
    pub const Q: GCKeyName = GCKeyName::new("Q");
    pub const R: GCKeyName = GCKeyName::new("R");
    pub const S: GCKeyName = GCKeyName::new("S");
    pub const T: GCKeyName = GCKeyName::new("T");
    pub const U: GCKeyName = GCKeyName::new("U");
    pub const V: GCKeyName = GCKeyName::new("V");
    pub const W: GCKeyName = GCKeyName::new("W");
    pub const X: GCKeyName = GCKeyName::new("X");
    pub const Y: GCKeyName = GCKeyName::new("Y");
    pub const Z: GCKeyName = GCKeyName::new("Z");
    pub const ONE: GCKeyName = GCKeyName::new("One");
    pub const TWO: GCKeyName = GCKeyName::new("Two");
    pub const THREE: GCKeyName = GCKeyName::new("Three");
    pub const FOUR: GCKeyName = GCKeyName::new("Four");
    pub const FIVE: GCKeyName = GCKeyName::new("Five");
    pub const SIX: GCKeyName = GCKeyName::new("Six");
    pub const SEVEN: GCKeyName = GCKeyName::new("Seven");
    pub const EIGHT: GCKeyName = GCKeyName::new("Eight");
    pub const NINE: GCKeyName = GCKeyName::new("Nine");
    pub const ZERO: GCKeyName = GCKeyName::new("Zero");
    pub const RETURN_OR_ENTER: GCKeyName = GCKeyName::new("ReturnOrEnter");
    pub const ESCAPE: GCKeyName = GCKeyName::new("Escape");
    pub const DELETE_OR_BACKSPACE: GCKeyName = GCKeyName::new("DeleteOrBackspace");
    pub const TAB: GCKeyName = GCKeyName::new("Tab");
    pub const SPACEBAR: GCKeyName = GCKeyName::new("Spacebar");
    pub const HYPHEN: GCKeyName = GCKeyName::new("Hyphen");
    pub const EQUAL_SIGN: GCKeyName = GCKeyName::new("EqualSign");
    pub const OPEN_BRACKET: GCKeyName = GCKeyName::new("OpenBracket");
    pub const CLOSE_BRACKET: GCKeyName = GCKeyName::new("CloseBracket");
    pub const BACKSLASH: GCKeyName = GCKeyName::new("Backslash");
    pub const NON_US_POUND: GCKeyName = GCKeyName::new("NonUSPound");
    pub const SEMICOLON: GCKeyName = GCKeyName::new("Semicolon");
    pub const QUOTE: GCKeyName = GCKeyName::new("Quote");
    pub const GRAVE_ACCENT_AND_TILDE: GCKeyName = GCKeyName::new("GraveAccentAndTilde");
    pub const COMMA: GCKeyName = GCKeyName::new("Comma");
    pub const PERIOD: GCKeyName = GCKeyName::new("Period");
    pub const SLASH: GCKeyName = GCKeyName::new("Slash");
    pub const CAPS_LOCK: GCKeyName = GCKeyName::new("CapsLock");
    pub const F_1: GCKeyName = GCKeyName::new("F1");
    pub const F_2: GCKeyName = GCKeyName::new("F2");
    pub const F_3: GCKeyName = GCKeyName::new("F3");
    pub const F_4: GCKeyName = GCKeyName::new("F4");
    pub const F_5: GCKeyName = GCKeyName::new("F5");
    pub const F_6: GCKeyName = GCKeyName::new("F6");
    pub const F_7: GCKeyName = GCKeyName::new("F7");
    pub const F_8: GCKeyName = GCKeyName::new("F8");
    pub const F_9: GCKeyName = GCKeyName::new("F9");
    pub const F_10: GCKeyName = GCKeyName::new("F10");
    pub const F_11: GCKeyName = GCKeyName::new("F11");
    pub const F_12: GCKeyName = GCKeyName::new("F12");
    pub const F_13: GCKeyName = GCKeyName::new("F13");
    pub const F_14: GCKeyName = GCKeyName::new("F14");
    pub const F_15: GCKeyName = GCKeyName::new("F15");
    pub const F_16: GCKeyName = GCKeyName::new("F16");
    pub const F_17: GCKeyName = GCKeyName::new("F17");
    pub const F_18: GCKeyName = GCKeyName::new("F18");
    pub const F_19: GCKeyName = GCKeyName::new("F19");
    pub const F_20: GCKeyName = GCKeyName::new("F20");
    pub const PRINT_SCREEN: GCKeyName = GCKeyName::new("PrintScreen");
    pub const SCROLL_LOCK: GCKeyName = GCKeyName::new("ScrollLock");
    pub const PAUSE: GCKeyName = GCKeyName::new("Pause");
    pub const INSERT: GCKeyName = GCKeyName::new("Insert");
    pub const HOME: GCKeyName = GCKeyName::new("Home");
    pub const PAGE_UP: GCKeyName = GCKeyName::new("PageUp");
    pub const DELETE_FORWARD: GCKeyName = GCKeyName::new("DeleteForward");
    pub const END: GCKeyName = GCKeyName::new("End");
    pub const PAGE_DOWN: GCKeyName = GCKeyName::new("PageDown");
    pub const RIGHT_ARROW: GCKeyName = GCKeyName::new("RightArrow");
    pub const LEFT_ARROW: GCKeyName = GCKeyName::new("LeftArrow");
    pub const DOWN_ARROW: GCKeyName = GCKeyName::new("DownArrow");
    pub const UP_ARROW: GCKeyName = GCKeyName::new("UpArrow");
    pub const KEYPAD_NUM_LOCK: GCKeyName = GCKeyName::new("KeypadNumLock");
    pub const KEYPAD_SLASH: GCKeyName = GCKeyName::new("KeypadSlash");
    pub const KEYPAD_ASTERISK: GCKeyName = GCKeyName::new("KeypadAsterisk");
    pub const KEYPAD_HYPHEN: GCKeyName = GCKeyName::new("KeypadHyphen");
    pub const KEYPAD_PLUS: GCKeyName = GCKeyName::new("KeypadPlus");
    pub const KEYPAD_ENTER: GCKeyName = GCKeyName::new("KeypadEnter");
    pub const KEYPAD_1: GCKeyName = GCKeyName::new("Keypad1");
    pub const KEYPAD_2: GCKeyName = GCKeyName::new("Keypad2");
    pub const KEYPAD_3: GCKeyName = GCKeyName::new("Keypad3");
    pub const KEYPAD_4: GCKeyName = GCKeyName::new("Keypad4");
    pub const KEYPAD_5: GCKeyName = GCKeyName::new("Keypad5");
    pub const KEYPAD_6: GCKeyName = GCKeyName::new("Keypad6");
    pub const KEYPAD_7: GCKeyName = GCKeyName::new("Keypad7");
    pub const KEYPAD_8: GCKeyName = GCKeyName::new("Keypad8");
    pub const KEYPAD_9: GCKeyName = GCKeyName::new("Keypad9");
    pub const KEYPAD_0: GCKeyName = GCKeyName::new("Keypad0");
    pub const KEYPAD_PERIOD: GCKeyName = GCKeyName::new("KeypadPeriod");
    pub const KEYPAD_EQUAL_SIGN: GCKeyName = GCKeyName::new("KeypadEqualSign");
    pub const NON_US_BACKSLASH: GCKeyName = GCKeyName::new("NonUSBackslash");
    pub const APPLICATION: GCKeyName = GCKeyName::new("Application");
    pub const POWER: GCKeyName = GCKeyName::new("Power");
    pub const INTERNATIONAL_1: GCKeyName = GCKeyName::new("International1");
    pub const INTERNATIONAL_2: GCKeyName = GCKeyName::new("International2");
    pub const INTERNATIONAL_3: GCKeyName = GCKeyName::new("International3");
    pub const INTERNATIONAL_4: GCKeyName = GCKeyName::new("International4");
    pub const INTERNATIONAL_5: GCKeyName = GCKeyName::new("International5");
    pub const INTERNATIONAL_6: GCKeyName = GCKeyName::new("International6");
    pub const INTERNATIONAL_7: GCKeyName = GCKeyName::new("International7");
    pub const INTERNATIONAL_8: GCKeyName = GCKeyName::new("International8");
    pub const INTERNATIONAL_9: GCKeyName = GCKeyName::new("International9");
    pub const LANG_1: GCKeyName = GCKeyName::new("LANG1");
    pub const LANG_2: GCKeyName = GCKeyName::new("LANG2");
    pub const LANG_3: GCKeyName = GCKeyName::new("LANG3");
    pub const LANG_4: GCKeyName = GCKeyName::new("LANG4");
    pub const LANG_5: GCKeyName = GCKeyName::new("LANG5");
    pub const LANG_6: GCKeyName = GCKeyName::new("LANG6");
    pub const LANG_7: GCKeyName = GCKeyName::new("LANG7");
    pub const LANG_8: GCKeyName = GCKeyName::new("LANG8");
    pub const LANG_9: GCKeyName = GCKeyName::new("LANG9");
    pub const LEFT_CONTROL: GCKeyName = GCKeyName::new("LeftControl");
    pub const LEFT_SHIFT: GCKeyName = GCKeyName::new("LeftShift");
    pub const LEFT_ALT: GCKeyName = GCKeyName::new("LeftAlt");
    pub const LEFT_GUI: GCKeyName = GCKeyName::new("LeftGUI");
    pub const RIGHT_CONTROL: GCKeyName = GCKeyName::new("RightControl");
    pub const RIGHT_SHIFT: GCKeyName = GCKeyName::new("RightShift");
    pub const RIGHT_ALT: GCKeyName = GCKeyName::new("RightAlt");
    pub const RIGHT_GUI: GCKeyName = GCKeyName::new("RightGUI");
}
/// Mirrors Apple's `GCInput*` string constants.
pub mod input_names {
    use super::{
        GCButtonElementName, GCDirectionPadElementName, GCInputAxisName, GCInputButtonName,
        GCInputDirectionPadName, GCInputElementName,
    };
    pub const BUTTON_A: GCInputButtonName = GCInputButtonName::new("Button A");
    pub const BUTTON_B: GCInputButtonName = GCInputButtonName::new("Button B");
    pub const BUTTON_X: GCInputButtonName = GCInputButtonName::new("Button X");
    pub const BUTTON_Y: GCInputButtonName = GCInputButtonName::new("Button Y");
    pub const DIRECTION_PAD: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Direction Pad");
    pub const THUMBSTICK: GCInputDirectionPadName = GCInputDirectionPadName::new("Thumbstick");
    pub const LEFT_THUMBSTICK: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Left Thumbstick");
    pub const RIGHT_THUMBSTICK: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Right Thumbstick");
    pub const THUMBSTICK_BUTTON: GCInputButtonName = GCInputButtonName::new("Thumbstick Button");
    pub const LEFT_THUMBSTICK_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Left Thumbstick Button");
    pub const RIGHT_THUMBSTICK_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Right Thumbstick Button");
    pub const GRIP_BUTTON: GCInputButtonName = GCInputButtonName::new("Grip");
    pub const LEFT_SHOULDER: GCInputButtonName = GCInputButtonName::new("Left Shoulder");
    pub const RIGHT_SHOULDER: GCInputButtonName = GCInputButtonName::new("Right Shoulder");
    pub const LEFT_BUMPER: GCInputButtonName = GCInputButtonName::new("Left Bumper");
    pub const RIGHT_BUMPER: GCInputButtonName = GCInputButtonName::new("Right Bumper");
    pub const TRIGGER: GCInputButtonName = GCInputButtonName::new("Trigger");
    pub const LEFT_TRIGGER: GCInputButtonName = GCInputButtonName::new("Left Trigger");
    pub const RIGHT_TRIGGER: GCInputButtonName = GCInputButtonName::new("Right Trigger");
    pub const BUTTON_HOME: GCInputButtonName = GCInputButtonName::new("Button Home");
    pub const BUTTON_MENU: GCInputButtonName = GCInputButtonName::new("Button Menu");
    pub const BUTTON_OPTIONS: GCInputButtonName = GCInputButtonName::new("Button Options");
    pub const BUTTON_SHARE: GCInputButtonName = GCInputButtonName::new("Button Share");
    pub const XBOX_PADDLE_ONE: GCInputButtonName = GCInputButtonName::new("Paddle 1");
    pub const XBOX_PADDLE_TWO: GCInputButtonName = GCInputButtonName::new("Paddle 2");
    pub const XBOX_PADDLE_THREE: GCInputButtonName = GCInputButtonName::new("Paddle 3");
    pub const XBOX_PADDLE_FOUR: GCInputButtonName = GCInputButtonName::new("Paddle 4");
    pub const DUAL_SHOCK_TOUCHPAD_ONE: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Touchpad 1");
    pub const DUAL_SHOCK_TOUCHPAD_TWO: GCInputDirectionPadName =
        GCInputDirectionPadName::new("Touchpad 2");
    pub const DUAL_SHOCK_TOUCHPAD_BUTTON: GCInputButtonName =
        GCInputButtonName::new("Touchpad Button");
    pub const STEERING_WHEEL: GCInputAxisName = GCInputAxisName::new("Steering Wheel");
    pub const SHIFTER: GCInputElementName = GCInputElementName::new("Shifter");
    pub const PEDAL_ACCELERATOR: GCInputButtonName = GCInputButtonName::new("Accelerator Pedal");
    pub const PEDAL_BRAKE: GCInputButtonName = GCInputButtonName::new("Brake Pedal");
    pub const PEDAL_CLUTCH: GCInputButtonName = GCInputButtonName::new("Clutch Pedal");
    pub const LEFT_PADDLE: GCInputButtonName = GCInputButtonName::new("Left Paddle");
    pub const RIGHT_PADDLE: GCInputButtonName = GCInputButtonName::new("Right Paddle");
    pub const DIRECTIONAL_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Direction Pad");
    pub const DIRECTIONAL_TOUCH_SURFACE_BUTTON: GCButtonElementName =
        GCButtonElementName::new("Button A");
    pub const DIRECTIONAL_CARDINAL_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Cardinal Direction Pad");
    pub const DIRECTIONAL_CENTER_BUTTON: GCButtonElementName =
        GCButtonElementName::new("Button Center");
    pub const MICRO_GAMEPAD_DPAD: GCDirectionPadElementName =
        GCDirectionPadElementName::new("Direction Pad");
    pub const MICRO_GAMEPAD_BUTTON_A: GCButtonElementName = GCButtonElementName::new("Button A");
    pub const MICRO_GAMEPAD_BUTTON_X: GCButtonElementName = GCButtonElementName::new("Button X");
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
