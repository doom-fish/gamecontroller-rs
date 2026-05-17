# gamecontroller-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 444
VERIFIED: 426
GAPS: 0
EXEMPT: 18
COVERAGE_PCT: 100.00%

Verification of gamecontroller-rs against MacOSX26.2.sdk GameController framework. The v2 audit re-confirms that all 426 previously-verified public macOS symbols remain present in v26.2 and are still wrapped by the crate's safe Rust API. No breaking symbol removals detected; framework compatibility is maintained at 100% for non-exempt surface.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| GCAcceleration | typedef struct | GCMotion.h | ControllerExtras.gravity/user_acceleration; MotionDetails.gravity/user_acceleration/acceleration |
| GCAxis2DInput | protocol | GCAxis2DInput.h | Axis2DInputDetails; current_controller_physical_input_elements() |
| GCAxisElement | protocol | GCAxisElement.h | NamedAxisElementState; AxisElementState |
| GCAxisElementName | typedef | GCInputNames.h | GCAxisElementName / GCInputAxisName aliases; input_names::STEERING_WHEEL |
| GCAxisInput | protocol | GCAxisInput.h | AxisInputDetails; current_controller_physical_input_elements() |
| GCButtonElement | protocol | GCButtonElement.h | NamedButtonElementState; ButtonInputState |
| GCButtonElementName | typedef | GCInputNames.h | GCButtonElementName / GCInputButtonName aliases; input_names::BUTTON_A |
| GCColor | interface | GCColor.h | Color; DeviceLightDetails.color; set_first_controller_light_color() |
| GCController | interface | GCController.h | Controller; connected_controllers(); connected_controller_details(); current_controller_snapshot(); watch_connections(); start_wireless_controller_discovery() |
| GCControllerAxisInput | interface | GCControllerAxisInput.h | AxisInputState; PhysicalInputProfileDetails.axes |
| GCControllerButtonInput | interface | GCControllerButtonInput.h | ButtonInputState; GamepadDetails/MicroGamepadDetails/ExtendedGamepadDetails |
| GCControllerDidBecomeCurrentNotification | constant | GCController.h | watch_current_controller() |
| GCControllerDidConnectNotification | constant | GCController.h | watch_connections() |
| GCControllerDidDisconnectNotification | constant | GCController.h | watch_connections() |
| GCControllerDidStopBeingCurrentNotification | constant | GCController.h | watch_current_controller() |
| GCControllerDirectionPad | interface | GCControllerDirectionPad.h | DirectionPadInputState; GamepadDetails/MicroGamepadDetails/ExtendedGamepadDetails |
| GCControllerElement | interface | GCControllerElement.h | GCControllerElement / NamedControllerElementDetails; current_controller_elements() |
| GCControllerInputState | interface | GCControllerInput.h | ControllerInputStateDetails; ControllerLiveInputDetails.live/unmapped/next |
| GCControllerLiveInput | interface | GCControllerInput.h | ControllerLiveInputDetails; current_controller_input_snapshot(); ControllerDetails.input |
| GCControllerPlayerIndex | typedef enum | GCController.h | Controller.player_index; ControllerDetails.player_index; set_first_controller_player_index() |
| GCControllerTouchpad | interface | GCControllerTouchpad.h | TouchpadDetails; PhysicalInputProfileDetails.touchpads |
| GCControllerUserCustomizationsDidChangeNotification | constant | GCController.h | watch_controller_customizations() |
| GCDevice | protocol | GCDevice.h | GCDevice / ConnectedDevicesSnapshot; connected_devices_snapshot() |
| GCDeviceBattery | interface | GCDeviceBattery.h | BatteryState; ControllerExtras; BatteryInfo; first_controller_battery(); first_controller_battery_level() |
| GCDeviceBatteryState | typedef enum | GCDeviceBattery.h | BatteryState; BatteryInfo.state; ControllerExtras.battery_state |
| GCDeviceCursor | interface | GCDeviceCursor.h | DeviceCursorState; MouseSnapshot.scroll |
| GCDeviceHaptics | interface | GCDeviceHaptics.h | DeviceHapticsDetails; first_controller_haptics(); rumble_first_controller() |
| GCDeviceLight | interface | GCDeviceLight.h | DeviceLightDetails; first_controller_light(); set_first_controller_light(); set_first_controller_light_color() |
| GCDevicePhysicalInput | protocol | GCDevicePhysicalInput.h | DevicePhysicalInputSourceDetails; current_controller_input_source() |
| GCDevicePhysicalInputElementChange | typedef enum | GCDevicePhysicalInputStateDiff.h | DevicePhysicalInputElementChange; DevicePhysicalInputStateDiffDetails::change_for_alias() |
| GCDevicePhysicalInputState | protocol | GCDevicePhysicalInputState.h | DevicePhysicalInputState alias; DevicePhysicalInputSourceDetails.live/capture/next |
| GCDevicePhysicalInputStateDiff | protocol | GCDevicePhysicalInputStateDiff.h | DevicePhysicalInputStateDiffDetails; current_controller_input_source() |
| GCDirectionPadElement | protocol | GCDirectionPadElement.h | NamedDirectionPadElementState; DirectionPadInputState |
| GCDirectionPadElementName | typedef | GCInputNames.h | GCDirectionPadElementName / GCInputDirectionPadName aliases; input_names::DIRECTION_PAD |
| GCDirectionalGamepad | interface | GCDirectionalGamepad.h | DirectionalGamepadDetails; ControllerDetails.directional_gamepad |
| GCDualSenseAdaptiveTrigger | interface | GCDualSenseAdaptiveTrigger.h | dualsense_trigger_*(); DualSenseAdaptiveTriggerState; DualSenseGamepadDetails.left_trigger/right_trigger |
| GCDualSenseAdaptiveTriggerMode | typedef enum | GCDualSenseAdaptiveTrigger.h | DualSenseTriggerMode |
| GCDualSenseAdaptiveTriggerPositionalAmplitudes | typedef struct | GCDualSenseAdaptiveTrigger.h | DualSenseAdaptiveTriggerPositionalAmplitudes; dualsense_trigger_vibration_amplitudes() |
| GCDualSenseAdaptiveTriggerPositionalResistiveStrengths | typedef struct | GCDualSenseAdaptiveTrigger.h | DualSenseAdaptiveTriggerPositionalResistiveStrengths; dualsense_trigger_feedback_resistive_strengths() |
| GCDualSenseAdaptiveTriggerStatus | typedef enum | GCDualSenseAdaptiveTrigger.h | DualSenseTriggerStatus |
| GCDualSenseGamepad | interface | GCDualSenseGamepad.h | dualsense_is_connected(); dualsense_trigger_*(); DualSenseGamepadDetails; ControllerDetails.dual_sense |
| GCDualShockGamepad | interface | GCDualShockGamepad.h | DualShockGamepadDetails; ControllerDetails.dual_shock |
| GCEulerAngles | typedef struct | GCMotion.h | EulerAngles; Quaternion::to_euler_angles() |
| GCEventViewController | interface | GCEventViewController.h | event_view_controller_snapshot(); EventViewControllerDetails |
| GCExtendedGamepad | interface | GCExtendedGamepad.h | Controller; ExtendedGamepadDetails; ControllerDetails.extended_gamepad |
| GCGearShifterElement | interface | GCGearShifterElement.h | GearShifterDetails; RacingWheelInputDetails.shifter |
| GCHapticDurationInfinite | constant | GCDeviceHaptics.h | HAPTIC_DURATION_INFINITE; rumble_first_controller_with_locality() |
| GCHapticsLocality | typedef | GCDeviceHaptics.h | GCHapticsLocality / haptics_localities::*; rumble_first_controller_with_locality() |
| GCHapticsLocalityAll | constant | GCDeviceHaptics.h | haptics_localities::ALL |
| GCHapticsLocalityDefault | constant | GCDeviceHaptics.h | haptics_localities::DEFAULT; rumble_first_controller_with_locality() |
| GCHapticsLocalityHandles | constant | GCDeviceHaptics.h | haptics_localities::HANDLES |
| GCHapticsLocalityLeftHandle | constant | GCDeviceHaptics.h | haptics_localities::LEFT_HANDLE |
| GCHapticsLocalityLeftTrigger | constant | GCDeviceHaptics.h | haptics_localities::LEFT_TRIGGER |
| GCHapticsLocalityRightHandle | constant | GCDeviceHaptics.h | haptics_localities::RIGHT_HANDLE |
| GCHapticsLocalityRightTrigger | constant | GCDeviceHaptics.h | haptics_localities::RIGHT_TRIGGER |
| GCHapticsLocalityTriggers | constant | GCDeviceHaptics.h | haptics_localities::TRIGGERS |
| GCInputArcadeButtonName | function | GCInputNames.h | input_names::arcade_button_name() |
| GCInputBackLeftButton | function | GCInputNames.h | input_names::back_left_button() |
| GCInputBackRightButton | function | GCInputNames.h | input_names::back_right_button() |
| GCInputButtonA | constant | GCInputNames.h | input_names::BUTTON_A |
| GCInputButtonB | constant | GCInputNames.h | input_names::BUTTON_B |
| GCInputButtonHome | constant | GCInputNames.h | input_names::BUTTON_HOME |
| GCInputButtonMenu | constant | GCInputNames.h | input_names::BUTTON_MENU |
| GCInputButtonOptions | constant | GCInputNames.h | input_names::BUTTON_OPTIONS |
| GCInputButtonShare | constant | GCInputNames.h | input_names::BUTTON_SHARE |
| GCInputButtonX | constant | GCInputNames.h | input_names::BUTTON_X |
| GCInputButtonY | constant | GCInputNames.h | input_names::BUTTON_Y |
| GCInputDirectionPad | constant | GCInputNames.h | input_names::DIRECTION_PAD |
| GCInputDirectionalCardinalDpad | constant | GCDirectionalGamepad.h | input_names::DIRECTIONAL_CARDINAL_DPAD |
| GCInputDirectionalCenterButton | constant | GCDirectionalGamepad.h | input_names::DIRECTIONAL_CENTER_BUTTON |
| GCInputDirectionalDpad | constant | GCDirectionalGamepad.h | input_names::DIRECTIONAL_DPAD |
| GCInputDirectionalTouchSurfaceButton | constant | GCDirectionalGamepad.h | input_names::DIRECTIONAL_TOUCH_SURFACE_BUTTON |
| GCInputDualShockTouchpadButton | constant | GCInputNames.h | input_names::DUAL_SHOCK_TOUCHPAD_BUTTON |
| GCInputDualShockTouchpadOne | constant | GCInputNames.h | input_names::DUAL_SHOCK_TOUCHPAD_ONE |
| GCInputDualShockTouchpadTwo | constant | GCInputNames.h | input_names::DUAL_SHOCK_TOUCHPAD_TWO |
| GCInputGripButton | constant | GCInputNames.h | input_names::GRIP_BUTTON |
| GCInputLeftBumper | constant | GCInputNames.h | input_names::LEFT_BUMPER |
| GCInputLeftPaddle | constant | GCInputNames.h | input_names::LEFT_PADDLE |
| GCInputLeftShoulder | constant | GCInputNames.h | input_names::LEFT_SHOULDER |
| GCInputLeftThumbstick | constant | GCInputNames.h | input_names::LEFT_THUMBSTICK |
| GCInputLeftThumbstickButton | constant | GCInputNames.h | input_names::LEFT_THUMBSTICK_BUTTON |
| GCInputLeftTrigger | constant | GCInputNames.h | input_names::LEFT_TRIGGER |
| GCInputMicroGamepadButtonA | constant | GCMicroGamepad.h | input_names::MICRO_GAMEPAD_BUTTON_A |
| GCInputMicroGamepadButtonMenu | constant | GCMicroGamepad.h | input_names::MICRO_GAMEPAD_BUTTON_MENU |
| GCInputMicroGamepadButtonX | constant | GCMicroGamepad.h | input_names::MICRO_GAMEPAD_BUTTON_X |
| GCInputMicroGamepadDpad | constant | GCMicroGamepad.h | input_names::MICRO_GAMEPAD_DPAD |
| GCInputPedalAccelerator | constant | GCInputNames.h | input_names::PEDAL_ACCELERATOR |
| GCInputPedalBrake | constant | GCInputNames.h | input_names::PEDAL_BRAKE |
| GCInputPedalClutch | constant | GCInputNames.h | input_names::PEDAL_CLUTCH |
| GCInputRightBumper | constant | GCInputNames.h | input_names::RIGHT_BUMPER |
| GCInputRightPaddle | constant | GCInputNames.h | input_names::RIGHT_PADDLE |
| GCInputRightShoulder | constant | GCInputNames.h | input_names::RIGHT_SHOULDER |
| GCInputRightThumbstick | constant | GCInputNames.h | input_names::RIGHT_THUMBSTICK |
| GCInputRightThumbstickButton | constant | GCInputNames.h | input_names::RIGHT_THUMBSTICK_BUTTON |
| GCInputRightTrigger | constant | GCInputNames.h | input_names::RIGHT_TRIGGER |
| GCInputShifter | constant | GCInputNames.h | input_names::SHIFTER |
| GCInputSteeringWheel | constant | GCInputNames.h | input_names::STEERING_WHEEL |
| GCInputThumbstick | constant | GCInputNames.h | input_names::THUMBSTICK |
| GCInputThumbstickButton | constant | GCInputNames.h | input_names::THUMBSTICK_BUTTON |
| GCInputTrigger | constant | GCInputNames.h | input_names::TRIGGER |
| GCInputXboxPaddleFour | constant | GCInputNames.h | input_names::XBOX_PADDLE_FOUR |
| GCInputXboxPaddleOne | constant | GCInputNames.h | input_names::XBOX_PADDLE_ONE |
| GCInputXboxPaddleThree | constant | GCInputNames.h | input_names::XBOX_PADDLE_THREE |
| GCInputXboxPaddleTwo | constant | GCInputNames.h | input_names::XBOX_PADDLE_TWO |
| GCKeyA | constant | GCKeyNames.h | key_names::A |
| GCKeyApplication | constant | GCKeyNames.h | key_names::APPLICATION |
| GCKeyB | constant | GCKeyNames.h | key_names::B |
| GCKeyBackslash | constant | GCKeyNames.h | key_names::BACKSLASH |
| GCKeyC | constant | GCKeyNames.h | key_names::C |
| GCKeyCapsLock | constant | GCKeyNames.h | key_names::CAPS_LOCK |
| GCKeyCloseBracket | constant | GCKeyNames.h | key_names::CLOSE_BRACKET |
| GCKeyCode | typedef | GCKeyCodes.h | keyboard_is_key_pressed() |
| GCKeyCodeApplication | constant | GCKeyCodes.h | key_codes::APPLICATION |
| GCKeyCodeBackslash | constant | GCKeyCodes.h | key_codes::BACKSLASH |
| GCKeyCodeCapsLock | constant | GCKeyCodes.h | key_codes::CAPS_LOCK |
| GCKeyCodeCloseBracket | constant | GCKeyCodes.h | key_codes::CLOSE_BRACKET |
| GCKeyCodeComma | constant | GCKeyCodes.h | key_codes::COMMA |
| GCKeyCodeDeleteForward | constant | GCKeyCodes.h | key_codes::DELETE_FORWARD |
| GCKeyCodeDeleteOrBackspace | constant | GCKeyCodes.h | key_codes::DELETE_OR_BACKSPACE |
| GCKeyCodeDownArrow | constant | GCKeyCodes.h | key_codes::DOWN_ARROW |
| GCKeyCodeEight | constant | GCKeyCodes.h | key_codes::EIGHT |
| GCKeyCodeEnd | constant | GCKeyCodes.h | key_codes::END |
| GCKeyCodeEqualSign | constant | GCKeyCodes.h | key_codes::EQUAL_SIGN |
| GCKeyCodeEscape | constant | GCKeyCodes.h | key_codes::ESCAPE |
| GCKeyCodeF1 | constant | GCKeyCodes.h | key_codes::F_1 |
| GCKeyCodeF10 | constant | GCKeyCodes.h | key_codes::F_10 |
| GCKeyCodeF11 | constant | GCKeyCodes.h | key_codes::F_11 |
| GCKeyCodeF12 | constant | GCKeyCodes.h | key_codes::F_12 |
| GCKeyCodeF13 | constant | GCKeyCodes.h | key_codes::F_13 |
| GCKeyCodeF14 | constant | GCKeyCodes.h | key_codes::F_14 |
| GCKeyCodeF15 | constant | GCKeyCodes.h | key_codes::F_15 |
| GCKeyCodeF16 | constant | GCKeyCodes.h | key_codes::F_16 |
| GCKeyCodeF17 | constant | GCKeyCodes.h | key_codes::F_17 |
| GCKeyCodeF18 | constant | GCKeyCodes.h | key_codes::F_18 |
| GCKeyCodeF19 | constant | GCKeyCodes.h | key_codes::F_19 |
| GCKeyCodeF2 | constant | GCKeyCodes.h | key_codes::F_2 |
| GCKeyCodeF20 | constant | GCKeyCodes.h | key_codes::F_20 |
| GCKeyCodeF3 | constant | GCKeyCodes.h | key_codes::F_3 |
| GCKeyCodeF4 | constant | GCKeyCodes.h | key_codes::F_4 |
| GCKeyCodeF5 | constant | GCKeyCodes.h | key_codes::F_5 |
| GCKeyCodeF6 | constant | GCKeyCodes.h | key_codes::F_6 |
| GCKeyCodeF7 | constant | GCKeyCodes.h | key_codes::F_7 |
| GCKeyCodeF8 | constant | GCKeyCodes.h | key_codes::F_8 |
| GCKeyCodeF9 | constant | GCKeyCodes.h | key_codes::F_9 |
| GCKeyCodeFive | constant | GCKeyCodes.h | key_codes::FIVE |
| GCKeyCodeFour | constant | GCKeyCodes.h | key_codes::FOUR |
| GCKeyCodeGraveAccentAndTilde | constant | GCKeyCodes.h | key_codes::GRAVE_ACCENT_AND_TILDE |
| GCKeyCodeHome | constant | GCKeyCodes.h | key_codes::HOME |
| GCKeyCodeHyphen | constant | GCKeyCodes.h | key_codes::HYPHEN |
| GCKeyCodeInsert | constant | GCKeyCodes.h | key_codes::INSERT |
| GCKeyCodeInternational1 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_1 |
| GCKeyCodeInternational2 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_2 |
| GCKeyCodeInternational3 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_3 |
| GCKeyCodeInternational4 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_4 |
| GCKeyCodeInternational5 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_5 |
| GCKeyCodeInternational6 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_6 |
| GCKeyCodeInternational7 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_7 |
| GCKeyCodeInternational8 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_8 |
| GCKeyCodeInternational9 | constant | GCKeyCodes.h | key_codes::INTERNATIONAL_9 |
| GCKeyCodeKeyA | constant | GCKeyCodes.h | key_codes::KEY_A |
| GCKeyCodeKeyB | constant | GCKeyCodes.h | key_codes::KEY_B |
| GCKeyCodeKeyC | constant | GCKeyCodes.h | key_codes::KEY_C |
| GCKeyCodeKeyD | constant | GCKeyCodes.h | key_codes::KEY_D |
| GCKeyCodeKeyE | constant | GCKeyCodes.h | key_codes::KEY_E |
| GCKeyCodeKeyF | constant | GCKeyCodes.h | key_codes::KEY_F |
| GCKeyCodeKeyG | constant | GCKeyCodes.h | key_codes::KEY_G |
| GCKeyCodeKeyH | constant | GCKeyCodes.h | key_codes::KEY_H |
| GCKeyCodeKeyI | constant | GCKeyCodes.h | key_codes::KEY_I |
| GCKeyCodeKeyJ | constant | GCKeyCodes.h | key_codes::KEY_J |
| GCKeyCodeKeyK | constant | GCKeyCodes.h | key_codes::KEY_K |
| GCKeyCodeKeyL | constant | GCKeyCodes.h | key_codes::KEY_L |
| GCKeyCodeKeyM | constant | GCKeyCodes.h | key_codes::KEY_M |
| GCKeyCodeKeyN | constant | GCKeyCodes.h | key_codes::KEY_N |
| GCKeyCodeKeyO | constant | GCKeyCodes.h | key_codes::KEY_O |
| GCKeyCodeKeyP | constant | GCKeyCodes.h | key_codes::KEY_P |
| GCKeyCodeKeyQ | constant | GCKeyCodes.h | key_codes::KEY_Q |
| GCKeyCodeKeyR | constant | GCKeyCodes.h | key_codes::KEY_R |
| GCKeyCodeKeyS | constant | GCKeyCodes.h | key_codes::KEY_S |
| GCKeyCodeKeyT | constant | GCKeyCodes.h | key_codes::KEY_T |
| GCKeyCodeKeyU | constant | GCKeyCodes.h | key_codes::KEY_U |
| GCKeyCodeKeyV | constant | GCKeyCodes.h | key_codes::KEY_V |
| GCKeyCodeKeyW | constant | GCKeyCodes.h | key_codes::KEY_W |
| GCKeyCodeKeyX | constant | GCKeyCodes.h | key_codes::KEY_X |
| GCKeyCodeKeyY | constant | GCKeyCodes.h | key_codes::KEY_Y |
| GCKeyCodeKeyZ | constant | GCKeyCodes.h | key_codes::KEY_Z |
| GCKeyCodeKeypad0 | constant | GCKeyCodes.h | key_codes::KEYPAD_0 |
| GCKeyCodeKeypad1 | constant | GCKeyCodes.h | key_codes::KEYPAD_1 |
| GCKeyCodeKeypad2 | constant | GCKeyCodes.h | key_codes::KEYPAD_2 |
| GCKeyCodeKeypad3 | constant | GCKeyCodes.h | key_codes::KEYPAD_3 |
| GCKeyCodeKeypad4 | constant | GCKeyCodes.h | key_codes::KEYPAD_4 |
| GCKeyCodeKeypad5 | constant | GCKeyCodes.h | key_codes::KEYPAD_5 |
| GCKeyCodeKeypad6 | constant | GCKeyCodes.h | key_codes::KEYPAD_6 |
| GCKeyCodeKeypad7 | constant | GCKeyCodes.h | key_codes::KEYPAD_7 |
| GCKeyCodeKeypad8 | constant | GCKeyCodes.h | key_codes::KEYPAD_8 |
| GCKeyCodeKeypad9 | constant | GCKeyCodes.h | key_codes::KEYPAD_9 |
| GCKeyCodeKeypadAsterisk | constant | GCKeyCodes.h | key_codes::KEYPAD_ASTERISK |
| GCKeyCodeKeypadEnter | constant | GCKeyCodes.h | key_codes::KEYPAD_ENTER |
| GCKeyCodeKeypadEqualSign | constant | GCKeyCodes.h | key_codes::KEYPAD_EQUAL_SIGN |
| GCKeyCodeKeypadHyphen | constant | GCKeyCodes.h | key_codes::KEYPAD_HYPHEN |
| GCKeyCodeKeypadNumLock | constant | GCKeyCodes.h | key_codes::KEYPAD_NUM_LOCK |
| GCKeyCodeKeypadPeriod | constant | GCKeyCodes.h | key_codes::KEYPAD_PERIOD |
| GCKeyCodeKeypadPlus | constant | GCKeyCodes.h | key_codes::KEYPAD_PLUS |
| GCKeyCodeKeypadSlash | constant | GCKeyCodes.h | key_codes::KEYPAD_SLASH |
| GCKeyCodeLANG1 | constant | GCKeyCodes.h | key_codes::LANG_1 |
| GCKeyCodeLANG2 | constant | GCKeyCodes.h | key_codes::LANG_2 |
| GCKeyCodeLANG3 | constant | GCKeyCodes.h | key_codes::LANG_3 |
| GCKeyCodeLANG4 | constant | GCKeyCodes.h | key_codes::LANG_4 |
| GCKeyCodeLANG5 | constant | GCKeyCodes.h | key_codes::LANG_5 |
| GCKeyCodeLANG6 | constant | GCKeyCodes.h | key_codes::LANG_6 |
| GCKeyCodeLANG7 | constant | GCKeyCodes.h | key_codes::LANG_7 |
| GCKeyCodeLANG8 | constant | GCKeyCodes.h | key_codes::LANG_8 |
| GCKeyCodeLANG9 | constant | GCKeyCodes.h | key_codes::LANG_9 |
| GCKeyCodeLeftAlt | constant | GCKeyCodes.h | key_codes::LEFT_ALT |
| GCKeyCodeLeftArrow | constant | GCKeyCodes.h | key_codes::LEFT_ARROW |
| GCKeyCodeLeftControl | constant | GCKeyCodes.h | key_codes::LEFT_CONTROL |
| GCKeyCodeLeftGUI | constant | GCKeyCodes.h | key_codes::LEFT_GUI |
| GCKeyCodeLeftShift | constant | GCKeyCodes.h | key_codes::LEFT_SHIFT |
| GCKeyCodeNine | constant | GCKeyCodes.h | key_codes::NINE |
| GCKeyCodeNonUSBackslash | constant | GCKeyCodes.h | key_codes::NON_US_BACKSLASH |
| GCKeyCodeNonUSPound | constant | GCKeyCodes.h | key_codes::NON_US_POUND |
| GCKeyCodeOne | constant | GCKeyCodes.h | key_codes::ONE |
| GCKeyCodeOpenBracket | constant | GCKeyCodes.h | key_codes::OPEN_BRACKET |
| GCKeyCodePageDown | constant | GCKeyCodes.h | key_codes::PAGE_DOWN |
| GCKeyCodePageUp | constant | GCKeyCodes.h | key_codes::PAGE_UP |
| GCKeyCodePause | constant | GCKeyCodes.h | key_codes::PAUSE |
| GCKeyCodePeriod | constant | GCKeyCodes.h | key_codes::PERIOD |
| GCKeyCodePower | constant | GCKeyCodes.h | key_codes::POWER |
| GCKeyCodePrintScreen | constant | GCKeyCodes.h | key_codes::PRINT_SCREEN |
| GCKeyCodeQuote | constant | GCKeyCodes.h | key_codes::QUOTE |
| GCKeyCodeReturnOrEnter | constant | GCKeyCodes.h | key_codes::RETURN_OR_ENTER |
| GCKeyCodeRightAlt | constant | GCKeyCodes.h | key_codes::RIGHT_ALT |
| GCKeyCodeRightArrow | constant | GCKeyCodes.h | key_codes::RIGHT_ARROW |
| GCKeyCodeRightControl | constant | GCKeyCodes.h | key_codes::RIGHT_CONTROL |
| GCKeyCodeRightGUI | constant | GCKeyCodes.h | key_codes::RIGHT_GUI |
| GCKeyCodeRightShift | constant | GCKeyCodes.h | key_codes::RIGHT_SHIFT |
| GCKeyCodeScrollLock | constant | GCKeyCodes.h | key_codes::SCROLL_LOCK |
| GCKeyCodeSemicolon | constant | GCKeyCodes.h | key_codes::SEMICOLON |
| GCKeyCodeSeven | constant | GCKeyCodes.h | key_codes::SEVEN |
| GCKeyCodeSix | constant | GCKeyCodes.h | key_codes::SIX |
| GCKeyCodeSlash | constant | GCKeyCodes.h | key_codes::SLASH |
| GCKeyCodeSpacebar | constant | GCKeyCodes.h | key_codes::SPACEBAR |
| GCKeyCodeTab | constant | GCKeyCodes.h | key_codes::TAB |
| GCKeyCodeThree | constant | GCKeyCodes.h | key_codes::THREE |
| GCKeyCodeTwo | constant | GCKeyCodes.h | key_codes::TWO |
| GCKeyCodeUpArrow | constant | GCKeyCodes.h | key_codes::UP_ARROW |
| GCKeyCodeZero | constant | GCKeyCodes.h | key_codes::ZERO |
| GCKeyComma | constant | GCKeyNames.h | key_names::COMMA |
| GCKeyD | constant | GCKeyNames.h | key_names::D |
| GCKeyDeleteForward | constant | GCKeyNames.h | key_names::DELETE_FORWARD |
| GCKeyDeleteOrBackspace | constant | GCKeyNames.h | key_names::DELETE_OR_BACKSPACE |
| GCKeyDownArrow | constant | GCKeyNames.h | key_names::DOWN_ARROW |
| GCKeyE | constant | GCKeyNames.h | key_names::E |
| GCKeyEight | constant | GCKeyNames.h | key_names::EIGHT |
| GCKeyEnd | constant | GCKeyNames.h | key_names::END |
| GCKeyEqualSign | constant | GCKeyNames.h | key_names::EQUAL_SIGN |
| GCKeyEscape | constant | GCKeyNames.h | key_names::ESCAPE |
| GCKeyF | constant | GCKeyNames.h | key_names::F |
| GCKeyF1 | constant | GCKeyNames.h | key_names::F_1 |
| GCKeyF10 | constant | GCKeyNames.h | key_names::F_10 |
| GCKeyF11 | constant | GCKeyNames.h | key_names::F_11 |
| GCKeyF12 | constant | GCKeyNames.h | key_names::F_12 |
| GCKeyF13 | constant | GCKeyNames.h | key_names::F_13 |
| GCKeyF14 | constant | GCKeyNames.h | key_names::F_14 |
| GCKeyF15 | constant | GCKeyNames.h | key_names::F_15 |
| GCKeyF16 | constant | GCKeyNames.h | key_names::F_16 |
| GCKeyF17 | constant | GCKeyNames.h | key_names::F_17 |
| GCKeyF18 | constant | GCKeyNames.h | key_names::F_18 |
| GCKeyF19 | constant | GCKeyNames.h | key_names::F_19 |
| GCKeyF2 | constant | GCKeyNames.h | key_names::F_2 |
| GCKeyF20 | constant | GCKeyNames.h | key_names::F_20 |
| GCKeyF3 | constant | GCKeyNames.h | key_names::F_3 |
| GCKeyF4 | constant | GCKeyNames.h | key_names::F_4 |
| GCKeyF5 | constant | GCKeyNames.h | key_names::F_5 |
| GCKeyF6 | constant | GCKeyNames.h | key_names::F_6 |
| GCKeyF7 | constant | GCKeyNames.h | key_names::F_7 |
| GCKeyF8 | constant | GCKeyNames.h | key_names::F_8 |
| GCKeyF9 | constant | GCKeyNames.h | key_names::F_9 |
| GCKeyFive | constant | GCKeyNames.h | key_names::FIVE |
| GCKeyFour | constant | GCKeyNames.h | key_names::FOUR |
| GCKeyG | constant | GCKeyNames.h | key_names::G |
| GCKeyGraveAccentAndTilde | constant | GCKeyNames.h | key_names::GRAVE_ACCENT_AND_TILDE |
| GCKeyH | constant | GCKeyNames.h | key_names::H |
| GCKeyHome | constant | GCKeyNames.h | key_names::HOME |
| GCKeyHyphen | constant | GCKeyNames.h | key_names::HYPHEN |
| GCKeyI | constant | GCKeyNames.h | key_names::I |
| GCKeyInsert | constant | GCKeyNames.h | key_names::INSERT |
| GCKeyInternational1 | constant | GCKeyNames.h | key_names::INTERNATIONAL_1 |
| GCKeyInternational2 | constant | GCKeyNames.h | key_names::INTERNATIONAL_2 |
| GCKeyInternational3 | constant | GCKeyNames.h | key_names::INTERNATIONAL_3 |
| GCKeyInternational4 | constant | GCKeyNames.h | key_names::INTERNATIONAL_4 |
| GCKeyInternational5 | constant | GCKeyNames.h | key_names::INTERNATIONAL_5 |
| GCKeyInternational6 | constant | GCKeyNames.h | key_names::INTERNATIONAL_6 |
| GCKeyInternational7 | constant | GCKeyNames.h | key_names::INTERNATIONAL_7 |
| GCKeyInternational8 | constant | GCKeyNames.h | key_names::INTERNATIONAL_8 |
| GCKeyInternational9 | constant | GCKeyNames.h | key_names::INTERNATIONAL_9 |
| GCKeyJ | constant | GCKeyNames.h | key_names::J |
| GCKeyK | constant | GCKeyNames.h | key_names::K |
| GCKeyKeypad0 | constant | GCKeyNames.h | key_names::KEYPAD_0 |
| GCKeyKeypad1 | constant | GCKeyNames.h | key_names::KEYPAD_1 |
| GCKeyKeypad2 | constant | GCKeyNames.h | key_names::KEYPAD_2 |
| GCKeyKeypad3 | constant | GCKeyNames.h | key_names::KEYPAD_3 |
| GCKeyKeypad4 | constant | GCKeyNames.h | key_names::KEYPAD_4 |
| GCKeyKeypad5 | constant | GCKeyNames.h | key_names::KEYPAD_5 |
| GCKeyKeypad6 | constant | GCKeyNames.h | key_names::KEYPAD_6 |
| GCKeyKeypad7 | constant | GCKeyNames.h | key_names::KEYPAD_7 |
| GCKeyKeypad8 | constant | GCKeyNames.h | key_names::KEYPAD_8 |
| GCKeyKeypad9 | constant | GCKeyNames.h | key_names::KEYPAD_9 |
| GCKeyKeypadAsterisk | constant | GCKeyNames.h | key_names::KEYPAD_ASTERISK |
| GCKeyKeypadEnter | constant | GCKeyNames.h | key_names::KEYPAD_ENTER |
| GCKeyKeypadEqualSign | constant | GCKeyNames.h | key_names::KEYPAD_EQUAL_SIGN |
| GCKeyKeypadHyphen | constant | GCKeyNames.h | key_names::KEYPAD_HYPHEN |
| GCKeyKeypadNumLock | constant | GCKeyNames.h | key_names::KEYPAD_NUM_LOCK |
| GCKeyKeypadPeriod | constant | GCKeyNames.h | key_names::KEYPAD_PERIOD |
| GCKeyKeypadPlus | constant | GCKeyNames.h | key_names::KEYPAD_PLUS |
| GCKeyKeypadSlash | constant | GCKeyNames.h | key_names::KEYPAD_SLASH |
| GCKeyL | constant | GCKeyNames.h | key_names::L |
| GCKeyLANG1 | constant | GCKeyNames.h | key_names::LANG_1 |
| GCKeyLANG2 | constant | GCKeyNames.h | key_names::LANG_2 |
| GCKeyLANG3 | constant | GCKeyNames.h | key_names::LANG_3 |
| GCKeyLANG4 | constant | GCKeyNames.h | key_names::LANG_4 |
| GCKeyLANG5 | constant | GCKeyNames.h | key_names::LANG_5 |
| GCKeyLANG6 | constant | GCKeyNames.h | key_names::LANG_6 |
| GCKeyLANG7 | constant | GCKeyNames.h | key_names::LANG_7 |
| GCKeyLANG8 | constant | GCKeyNames.h | key_names::LANG_8 |
| GCKeyLANG9 | constant | GCKeyNames.h | key_names::LANG_9 |
| GCKeyLeftAlt | constant | GCKeyNames.h | key_names::LEFT_ALT |
| GCKeyLeftArrow | constant | GCKeyNames.h | key_names::LEFT_ARROW |
| GCKeyLeftControl | constant | GCKeyNames.h | key_names::LEFT_CONTROL |
| GCKeyLeftGUI | constant | GCKeyNames.h | key_names::LEFT_GUI |
| GCKeyLeftShift | constant | GCKeyNames.h | key_names::LEFT_SHIFT |
| GCKeyM | constant | GCKeyNames.h | key_names::M |
| GCKeyN | constant | GCKeyNames.h | key_names::N |
| GCKeyNine | constant | GCKeyNames.h | key_names::NINE |
| GCKeyNonUSBackslash | constant | GCKeyNames.h | key_names::NON_US_BACKSLASH |
| GCKeyNonUSPound | constant | GCKeyNames.h | key_names::NON_US_POUND |
| GCKeyO | constant | GCKeyNames.h | key_names::O |
| GCKeyOne | constant | GCKeyNames.h | key_names::ONE |
| GCKeyOpenBracket | constant | GCKeyNames.h | key_names::OPEN_BRACKET |
| GCKeyP | constant | GCKeyNames.h | key_names::P |
| GCKeyPageDown | constant | GCKeyNames.h | key_names::PAGE_DOWN |
| GCKeyPageUp | constant | GCKeyNames.h | key_names::PAGE_UP |
| GCKeyPause | constant | GCKeyNames.h | key_names::PAUSE |
| GCKeyPeriod | constant | GCKeyNames.h | key_names::PERIOD |
| GCKeyPower | constant | GCKeyNames.h | key_names::POWER |
| GCKeyPrintScreen | constant | GCKeyNames.h | key_names::PRINT_SCREEN |
| GCKeyQ | constant | GCKeyNames.h | key_names::Q |
| GCKeyQuote | constant | GCKeyNames.h | key_names::QUOTE |
| GCKeyR | constant | GCKeyNames.h | key_names::R |
| GCKeyReturnOrEnter | constant | GCKeyNames.h | key_names::RETURN_OR_ENTER |
| GCKeyRightAlt | constant | GCKeyNames.h | key_names::RIGHT_ALT |
| GCKeyRightArrow | constant | GCKeyNames.h | key_names::RIGHT_ARROW |
| GCKeyRightControl | constant | GCKeyNames.h | key_names::RIGHT_CONTROL |
| GCKeyRightGUI | constant | GCKeyNames.h | key_names::RIGHT_GUI |
| GCKeyRightShift | constant | GCKeyNames.h | key_names::RIGHT_SHIFT |
| GCKeyS | constant | GCKeyNames.h | key_names::S |
| GCKeyScrollLock | constant | GCKeyNames.h | key_names::SCROLL_LOCK |
| GCKeySemicolon | constant | GCKeyNames.h | key_names::SEMICOLON |
| GCKeySeven | constant | GCKeyNames.h | key_names::SEVEN |
| GCKeySix | constant | GCKeyNames.h | key_names::SIX |
| GCKeySlash | constant | GCKeyNames.h | key_names::SLASH |
| GCKeySpacebar | constant | GCKeyNames.h | key_names::SPACEBAR |
| GCKeyT | constant | GCKeyNames.h | key_names::T |
| GCKeyTab | constant | GCKeyNames.h | key_names::TAB |
| GCKeyThree | constant | GCKeyNames.h | key_names::THREE |
| GCKeyTwo | constant | GCKeyNames.h | key_names::TWO |
| GCKeyU | constant | GCKeyNames.h | key_names::U |
| GCKeyUpArrow | constant | GCKeyNames.h | key_names::UP_ARROW |
| GCKeyV | constant | GCKeyNames.h | key_names::V |
| GCKeyW | constant | GCKeyNames.h | key_names::W |
| GCKeyX | constant | GCKeyNames.h | key_names::X |
| GCKeyY | constant | GCKeyNames.h | key_names::Y |
| GCKeyZ | constant | GCKeyNames.h | key_names::Z |
| GCKeyZero | constant | GCKeyNames.h | key_names::ZERO |
| GCKeyboard | interface | GCKeyboard.h | keyboard_is_connected(); keyboard_snapshot() |
| GCKeyboardDidConnectNotification | constant | GCKeyboard.h | watch_keyboard_connections() |
| GCKeyboardDidDisconnectNotification | constant | GCKeyboard.h | watch_keyboard_connections() |
| GCKeyboardInput | interface | GCKeyboardInput.h | keyboard_any_key_pressed(); keyboard_is_key_pressed(); KeyboardSnapshot |
| GCLinearInput | protocol | GCLinearInput.h | ButtonInputState.value; AxisElementState.absolute_value |
| GCMicroGamepad | interface | GCMicroGamepad.h | MicroGamepadDetails; ControllerDetails.micro_gamepad |
| GCMotion | interface | GCMotion.h | ControllerExtras; MotionDetails; ControllerDetails.motion |
| GCMouse | interface | GCMouse.h | mouse_is_connected(); mouse_button_states(); mouse_snapshot() |
| GCMouseDidBecomeCurrentNotification | constant | GCMouse.h | watch_mouse_current() |
| GCMouseDidConnectNotification | constant | GCMouse.h | watch_mouse_connections() |
| GCMouseDidDisconnectNotification | constant | GCMouse.h | watch_mouse_connections() |
| GCMouseDidStopBeingCurrentNotification | constant | GCMouse.h | watch_mouse_current() |
| GCMouseInput | interface | GCMouseInput.h | mouse_button_states(); MouseSnapshot |
| GCPhysicalInputElement | protocol | GCPhysicalInputElement.h | NamedButtonElementState/NamedAxisElementState/NamedSwitchElementState/NamedDirectionPadElementState |
| GCPhysicalInputElementCollection | interface | GCPhysicalInputElement.h | GCPhysicalInputElementCollection<T>; PhysicalInputElementSnapshot |
| GCPhysicalInputElementName | typedef | GCInputNames.h | GCPhysicalInputElementName / GCInputElementName aliases; input_names::SHIFTER |
| GCPhysicalInputExtents | protocol | GCPhysicalInputExtents.h | PhysicalInputExtentsDetails; current_controller_physical_input_elements() |
| GCPhysicalInputProfile | interface | GCPhysicalInputProfile.h | PhysicalInputProfileDetails; ControllerDetails.physical_input; KeyboardSnapshot.physical_input; MouseSnapshot.physical_input |
| GCPhysicalInputSource | protocol | GCPhysicalInputSource.h | PhysicalInputSourceDetails; current_controller_physical_input_elements() |
| GCPhysicalInputSourceDirection | typedef enum | GCPhysicalInputSource.h | PhysicalInputSourceDirection; PhysicalInputSourceDetails.direction |
| GCPoint2 | typedef | GCTypes.h | Point2 / GCPoint2; Axis2DInputDetails.value |
| GCPoint2Zero | constant | GCTypes.h | POINT2_ZERO |
| GCPressedStateInput | protocol | GCPressedStateInput.h | ButtonInputState.pressed/value |
| GCProductCategoryArcadeStick | constant | GCProductCategories.h | product_categories::ARCADE_STICK |
| GCProductCategoryCoalescedRemote | constant | GCProductCategories.h | product_categories::COALESCED_REMOTE |
| GCProductCategoryControlCenterRemote | constant | GCProductCategories.h | product_categories::CONTROL_CENTER_REMOTE |
| GCProductCategoryDualSense | constant | GCProductCategories.h | product_categories::DUAL_SENSE |
| GCProductCategoryDualShock4 | constant | GCProductCategories.h | product_categories::DUAL_SHOCK_4 |
| GCProductCategoryHID | constant | GCProductCategories.h | product_categories::HID |
| GCProductCategoryKeyboard | constant | GCProductCategories.h | product_categories::KEYBOARD |
| GCProductCategoryMFi | constant | GCProductCategories.h | product_categories::MFI |
| GCProductCategoryMouse | constant | GCProductCategories.h | product_categories::MOUSE |
| GCProductCategorySiriRemote1stGen | constant | GCProductCategories.h | product_categories::SIRI_REMOTE_1ST_GEN |
| GCProductCategorySiriRemote2ndGen | constant | GCProductCategories.h | product_categories::SIRI_REMOTE_2ND_GEN |
| GCProductCategorySpatialController | constant | GCProductCategories.h | product_categories::SPATIAL_CONTROLLER |
| GCProductCategoryUniversalElectronicsRemote | constant | GCProductCategories.h | product_categories::UNIVERSAL_ELECTRONICS_REMOTE |
| GCProductCategoryXboxOne | constant | GCProductCategories.h | product_categories::XBOX_ONE |
| GCQuaternion | typedef struct | GCMotion.h | Quaternion; MotionDetails.attitude |
| GCRacingWheel | interface | GCRacingWheel.h | connected_racing_wheels(); RacingWheelDetails |
| GCRacingWheelDidConnectNotification | constant | GCRacingWheel.h | watch_racing_wheel_connections() |
| GCRacingWheelDidDisconnectNotification | constant | GCRacingWheel.h | watch_racing_wheel_connections() |
| GCRacingWheelInput | interface | GCRacingWheelInput.h | RacingWheelDetails.wheel_input; RacingWheelInputDetails |
| GCRacingWheelInputState | interface | GCRacingWheelInput.h | RacingWheelDetails.wheel_input; RacingWheelInputDetails |
| GCRelativeInput | protocol | GCRelativeInput.h | AxisElementState.relative_delta; SteeringWheelDetails.relative_delta; GearShifterDetails.sequential_delta |
| GCRotationRate | typedef struct | GCMotion.h | MotionDetails.rotation_rate |
| GCSteeringWheelElement | interface | GCSteeringWheelElement.h | SteeringWheelDetails; RacingWheelInputDetails.wheel |
| GCSwitchElement | protocol | GCSwitchElement.h | NamedSwitchElementState; SwitchInputState |
| GCSwitchElementName | typedef | GCInputNames.h | GCSwitchElementName / GCInputSwitchName aliases |
| GCSwitchPositionInput | protocol | GCSwitchPositionInput.h | SwitchInputState; GearShifterDetails.pattern_* |
| GCSystemGestureState | typedef enum | GCControllerElement.h | SystemGestureState; ControllerElementDetails.preferred_system_gesture_state |
| GCTouchState | typedef enum | GCControllerTouchpad.h | TouchState |
| GCTouchedStateInput | protocol | GCTouchedStateInput.h | ButtonInputState.touched |
| GCXboxGamepad | interface | GCXboxGamepad.h | XboxGamepadDetails; ControllerDetails.xbox |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| GCCurrentExtendedGamepadSnapshotDataVersion | constant | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.11, 10.15), ios(9.0, 13.0), tvos(9.0, 13.0)); |
| GCCurrentMicroGamepadSnapshotDataVersion | constant | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCExtendedGamepadSnapShotDataV100 | typedef struct | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)); |
| GCExtendedGamepadSnapShotDataV100FromNSData | function | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)); |
| GCExtendedGamepadSnapshot | interface | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)) @interface GCExtendedGamepadSnapshot : GCExtendedGamepad |
| GCExtendedGamepadSnapshotData | typedef struct | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCExtendedGamepadSnapshotDataFromNSData | function | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCExtendedGamepadSnapshotDataVersion | typedef enum | GCExtendedGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithExtendedGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCGamepad | interface | GCGamepad.h | Deprecated standard-gamepad profile; excluded per audit instructions. | API_DEPRECATED_WITH_REPLACEMENT("GCExtendedGamepad", macos(10.9, 10.12), ios(7.0, 10.0), tvos(9.0, 10.0)) @interface GCGamepad : GCPhysicalInputProfile |
| GCGamepadSnapShotDataV100 | typedef struct | GCGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)); |
| GCGamepadSnapShotDataV100FromNSData | function | GCGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)); |
| GCGamepadSnapshot | interface | GCGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(9.0, 13.0)) @interface GCGamepadSnapshot : GCGamepad |
| GCMicroGamepadSnapShotDataV100 | typedef struct | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.11, 10.15), ios(9.0, 13.0), tvos(9.0, 13.0)); |
| GCMicroGamepadSnapShotDataV100FromNSData | function | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.11, 10.15), ios(9.0, 13.0), tvos(9.0, 13.0)); |
| GCMicroGamepadSnapshot | interface | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.11, 10.15), ios(9.0, 13.0), tvos(9.0, 13.0)) @interface GCMicroGamepadSnapshot : GCMicroGamepad |
| GCMicroGamepadSnapshotData | typedef struct | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCMicroGamepadSnapshotDataFromNSData | function | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
| GCMicroGamepadSnapshotDataVersion | typedef enum | GCMicroGamepadSnapshot.h | Deprecated snapshot family; excluded per audit instructions. | API_DEPRECATED("Use the -[GCController controllerWithMicroGamepad] method instead", macos(10.15, 10.15), ios(13.0, 13.0), tvos(13.0, 13.0)); |
