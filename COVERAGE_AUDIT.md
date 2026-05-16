# gamecontroller-rs coverage audit (vs MacOSX26.2.sdk)

This audit compares the public macOS `GameController.framework` surface against the safe/public Rust API and Swift bridge in `gamecontroller-rs`. It filters out symbols unavailable on macOS and treats Apple-deprecated snapshot/profile APIs as exempt per the audit instructions.

Filtered out 7 unavailable symbols (the visionOS-only `GCStylus` family plus spatial-stylus constants).

The raw percentage is heavily dragged down by the SDK's large exported constant tables (`GCKey*`, `GCKeyCode*`, `GCInput*`, and product-category constants), which this crate does not currently mirror as typed Rust exports.

SDK_PUBLIC_SYMBOLS: 444
VERIFIED: 51
GAPS: 375
EXEMPT: 18
COVERAGE_PCT: 11.97%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| GCAcceleration | typedef struct | GCMotion.h | ControllerExtras.gravity/user_acceleration; MotionDetails.gravity/user_acceleration/acceleration |
| GCAxisElement | protocol | GCAxisElement.h | NamedAxisElementState; AxisElementState |
| GCButtonElement | protocol | GCButtonElement.h | NamedButtonElementState; ButtonInputState |
| GCColor | interface | GCColor.h | Color; DeviceLightDetails.color; set_first_controller_light_color() |
| GCController | interface | GCController.h | Controller; connected_controllers(); connected_controller_details(); current_controller_snapshot(); watch_connections(); start_wireless_controller_discovery() |
| GCControllerAxisInput | interface | GCControllerAxisInput.h | AxisInputState; PhysicalInputProfileDetails.axes |
| GCControllerButtonInput | interface | GCControllerButtonInput.h | ButtonInputState; GamepadDetails/MicroGamepadDetails/ExtendedGamepadDetails |
| GCControllerDidConnectNotification | constant | GCController.h | watch_connections() |
| GCControllerDidDisconnectNotification | constant | GCController.h | watch_connections() |
| GCControllerDirectionPad | interface | GCControllerDirectionPad.h | DirectionPadInputState; GamepadDetails/MicroGamepadDetails/ExtendedGamepadDetails |
| GCControllerInputState | interface | GCControllerInput.h | ControllerInputStateDetails; ControllerLiveInputDetails.live/unmapped/next |
| GCControllerLiveInput | interface | GCControllerInput.h | ControllerLiveInputDetails; current_controller_input_snapshot(); ControllerDetails.input |
| GCControllerPlayerIndex | typedef enum | GCController.h | Controller.player_index; ControllerDetails.player_index; set_first_controller_player_index() |
| GCControllerTouchpad | interface | GCControllerTouchpad.h | TouchpadDetails; PhysicalInputProfileDetails.touchpads |
| GCDeviceBattery | interface | GCDeviceBattery.h | BatteryState; ControllerExtras; BatteryInfo; first_controller_battery(); first_controller_battery_level() |
| GCDeviceBatteryState | typedef enum | GCDeviceBattery.h | BatteryState; BatteryInfo.state; ControllerExtras.battery_state |
| GCDeviceHaptics | interface | GCDeviceHaptics.h | DeviceHapticsDetails; first_controller_haptics(); rumble_first_controller() |
| GCDeviceLight | interface | GCDeviceLight.h | DeviceLightDetails; first_controller_light(); set_first_controller_light(); set_first_controller_light_color() |
| GCDirectionPadElement | protocol | GCDirectionPadElement.h | NamedDirectionPadElementState; DirectionPadInputState |
| GCDirectionalGamepad | interface | GCDirectionalGamepad.h | DirectionalGamepadDetails; ControllerDetails.directional_gamepad |
| GCDualSenseAdaptiveTrigger | interface | GCDualSenseAdaptiveTrigger.h | dualsense_trigger_*(); DualSenseAdaptiveTriggerState; DualSenseGamepadDetails.left_trigger/right_trigger |
| GCDualSenseAdaptiveTriggerMode | typedef enum | GCDualSenseAdaptiveTrigger.h | DualSenseTriggerMode |
| GCDualSenseAdaptiveTriggerStatus | typedef enum | GCDualSenseAdaptiveTrigger.h | DualSenseTriggerStatus |
| GCDualSenseGamepad | interface | GCDualSenseGamepad.h | dualsense_is_connected(); dualsense_trigger_*(); DualSenseGamepadDetails; ControllerDetails.dual_sense |
| GCDualShockGamepad | interface | GCDualShockGamepad.h | DualShockGamepadDetails; ControllerDetails.dual_shock |
| GCEventViewController | interface | GCEventViewController.h | event_view_controller_snapshot(); EventViewControllerDetails |
| GCExtendedGamepad | interface | GCExtendedGamepad.h | Controller; ExtendedGamepadDetails; ControllerDetails.extended_gamepad |
| GCGearShifterElement | interface | GCGearShifterElement.h | GearShifterDetails; RacingWheelInputDetails.shifter |
| GCKeyCode | typedef | GCKeyCodes.h | keyboard_is_key_pressed() |
| GCKeyboard | interface | GCKeyboard.h | keyboard_is_connected(); keyboard_snapshot() |
| GCKeyboardInput | interface | GCKeyboardInput.h | keyboard_any_key_pressed(); keyboard_is_key_pressed(); KeyboardSnapshot |
| GCLinearInput | protocol | GCLinearInput.h | ButtonInputState.value; AxisElementState.absolute_value |
| GCMicroGamepad | interface | GCMicroGamepad.h | MicroGamepadDetails; ControllerDetails.micro_gamepad |
| GCMotion | interface | GCMotion.h | ControllerExtras; MotionDetails; ControllerDetails.motion |
| GCMouse | interface | GCMouse.h | mouse_is_connected(); mouse_button_states(); mouse_snapshot() |
| GCMouseInput | interface | GCMouseInput.h | mouse_button_states(); MouseSnapshot |
| GCPhysicalInputElement | protocol | GCPhysicalInputElement.h | NamedButtonElementState/NamedAxisElementState/NamedSwitchElementState/NamedDirectionPadElementState |
| GCPhysicalInputProfile | interface | GCPhysicalInputProfile.h | PhysicalInputProfileDetails; ControllerDetails.physical_input; KeyboardSnapshot.physical_input; MouseSnapshot.physical_input |
| GCPressedStateInput | protocol | GCPressedStateInput.h | ButtonInputState.pressed/value |
| GCQuaternion | typedef struct | GCMotion.h | Quaternion; MotionDetails.attitude |
| GCRacingWheel | interface | GCRacingWheel.h | connected_racing_wheels(); RacingWheelDetails |
| GCRacingWheelInput | interface | GCRacingWheelInput.h | RacingWheelDetails.wheel_input; RacingWheelInputDetails |
| GCRacingWheelInputState | interface | GCRacingWheelInput.h | RacingWheelDetails.wheel_input; RacingWheelInputDetails |
| GCRelativeInput | protocol | GCRelativeInput.h | AxisElementState.relative_delta; SteeringWheelDetails.relative_delta; GearShifterDetails.sequential_delta |
| GCRotationRate | typedef struct | GCMotion.h | MotionDetails.rotation_rate |
| GCSteeringWheelElement | interface | GCSteeringWheelElement.h | SteeringWheelDetails; RacingWheelInputDetails.wheel |
| GCSwitchElement | protocol | GCSwitchElement.h | NamedSwitchElementState; SwitchInputState |
| GCSwitchPositionInput | protocol | GCSwitchPositionInput.h | SwitchInputState; GearShifterDetails.pattern_* |
| GCTouchState | typedef enum | GCControllerTouchpad.h | TouchState |
| GCTouchedStateInput | protocol | GCTouchedStateInput.h | ButtonInputState.touched |
| GCXboxGamepad | interface | GCXboxGamepad.h | XboxGamepadDetails; ControllerDetails.xbox |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| GCAxis2DInput | protocol | GCAxis2DInput.h | No standalone Rust wrapper for the generic axis-input protocol. |
| GCAxisElementName | typedef | GCInputNames.h | Aliases are surfaced as String values, not typed input-name wrappers. |
| GCAxisInput | protocol | GCAxisInput.h | No standalone Rust wrapper for the generic axis-input protocol. |
| GCButtonElementName | typedef | GCInputNames.h | Aliases are surfaced as String values, not typed input-name wrappers. |
| GCControllerDidBecomeCurrentNotification | constant | GCController.h | No Rust notification wrapper for this lifecycle event. |
| GCControllerDidStopBeingCurrentNotification | constant | GCController.h | No Rust notification wrapper for this lifecycle event. |
| GCControllerElement | interface | GCControllerElement.h | No standalone Rust wrapper for controller-element base metadata or system-gesture state. |
| GCControllerUserCustomizationsDidChangeNotification | constant | GCController.h | No Rust notification wrapper for this lifecycle event. |
| GCDevice | protocol | GCDevice.h | No generic Rust device abstraction for handler queues or shared GCDevice properties. |
| GCDeviceCursor | interface | GCDeviceCursor.h | Cursor delta APIs are not wrapped. |
| GCDevicePhysicalInput | protocol | GCDevicePhysicalInput.h | Advanced physical-input queue/diff APIs are not wrapped as standalone Rust types. |
| GCDevicePhysicalInputElementChange | typedef enum | GCDevicePhysicalInputStateDiff.h | Advanced physical-input queue/diff APIs are not wrapped as standalone Rust types. |
| GCDevicePhysicalInputState | protocol | GCDevicePhysicalInputState.h | Advanced physical-input queue/diff APIs are not wrapped as standalone Rust types. |
| GCDevicePhysicalInputStateDiff | protocol | GCDevicePhysicalInputStateDiff.h | Advanced physical-input queue/diff APIs are not wrapped as standalone Rust types. |
| GCDirectionPadElementName | typedef | GCInputNames.h | Aliases are surfaced as String values, not typed input-name wrappers. |
| GCDualSenseAdaptiveTriggerPositionalAmplitudes | typedef struct | GCDualSenseAdaptiveTrigger.h | DualSense helpers expose higher-level trigger modes, not the raw positional-array structs. |
| GCDualSenseAdaptiveTriggerPositionalResistiveStrengths | typedef struct | GCDualSenseAdaptiveTrigger.h | DualSense helpers expose higher-level trigger modes, not the raw positional-array structs. |
| GCEulerAngles | typedef struct | GCMotion.h | Motion snapshots expose quaternions/vector3 values, not Euler-angle structs. |
| GCHapticDurationInfinite | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocality | typedef | GCDeviceHaptics.h | supported_localities is Vec<String>; no typed Rust locality alias is exposed. |
| GCHapticsLocalityAll | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityDefault | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityHandles | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityLeftHandle | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityLeftTrigger | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityRightHandle | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityRightTrigger | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCHapticsLocalityTriggers | constant | GCDeviceHaptics.h | Locality/duration constants are not exported; rumble_first_controller() always uses the default locality. |
| GCInputArcadeButtonName | function | GCInputNames.h | No Rust helper for this generated input-name symbol. |
| GCInputBackLeftButton | function | GCInputNames.h | No Rust helper for this generated input-name symbol. |
| GCInputBackRightButton | function | GCInputNames.h | No Rust helper for this generated input-name symbol. |
| GCInputButtonA | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonB | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonHome | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonMenu | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonOptions | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonShare | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonX | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputButtonY | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputDirectionPad | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputDirectionalCardinalDpad | constant | GCDirectionalGamepad.h | No Rust export for these input-name constants. |
| GCInputDirectionalCenterButton | constant | GCDirectionalGamepad.h | No Rust export for these input-name constants. |
| GCInputDirectionalDpad | constant | GCDirectionalGamepad.h | No Rust export for these input-name constants. |
| GCInputDirectionalTouchSurfaceButton | constant | GCDirectionalGamepad.h | No Rust export for these input-name constants. |
| GCInputDualShockTouchpadButton | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputDualShockTouchpadOne | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputDualShockTouchpadTwo | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputGripButton | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftBumper | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftPaddle | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftShoulder | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftThumbstick | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftThumbstickButton | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputLeftTrigger | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputMicroGamepadButtonA | constant | GCMicroGamepad.h | No Rust export for these input-name constants. |
| GCInputMicroGamepadButtonMenu | constant | GCMicroGamepad.h | No Rust export for these input-name constants. |
| GCInputMicroGamepadButtonX | constant | GCMicroGamepad.h | No Rust export for these input-name constants. |
| GCInputMicroGamepadDpad | constant | GCMicroGamepad.h | No Rust export for these input-name constants. |
| GCInputPedalAccelerator | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputPedalBrake | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputPedalClutch | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightBumper | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightPaddle | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightShoulder | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightThumbstick | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightThumbstickButton | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputRightTrigger | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputShifter | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputSteeringWheel | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputThumbstick | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputThumbstickButton | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputTrigger | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputXboxPaddleFour | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputXboxPaddleOne | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputXboxPaddleThree | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCInputXboxPaddleTwo | constant | GCInputNames.h | No Rust export for this GCInput* alias constant. |
| GCKeyA | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyApplication | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyB | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyBackslash | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyC | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyCapsLock | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyCloseBracket | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyCodeApplication | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeBackslash | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeCapsLock | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeCloseBracket | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeComma | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeDeleteForward | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeDeleteOrBackspace | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeDownArrow | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeEight | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeEnd | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeEqualSign | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeEscape | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF1 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF10 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF11 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF12 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF13 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF14 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF15 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF16 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF17 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF18 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF19 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF2 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF20 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF3 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF4 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF5 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF6 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF7 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF8 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeF9 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeFive | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeFour | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeGraveAccentAndTilde | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeHome | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeHyphen | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInsert | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational1 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational2 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational3 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational4 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational5 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational6 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational7 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational8 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeInternational9 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyA | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyB | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyC | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyD | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyE | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyF | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyG | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyH | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyI | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyJ | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyK | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyL | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyM | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyN | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyO | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyP | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyQ | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyR | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyS | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyT | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyU | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyV | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyW | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyX | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyY | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeyZ | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad0 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad1 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad2 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad3 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad4 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad5 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad6 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad7 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad8 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypad9 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadAsterisk | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadEnter | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadEqualSign | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadHyphen | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadNumLock | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadPeriod | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadPlus | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeKeypadSlash | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG1 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG2 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG3 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG4 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG5 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG6 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG7 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG8 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLANG9 | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLeftAlt | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLeftArrow | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLeftControl | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLeftGUI | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeLeftShift | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeNine | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeNonUSBackslash | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeNonUSPound | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeOne | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeOpenBracket | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePageDown | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePageUp | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePause | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePeriod | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePower | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodePrintScreen | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeQuote | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeReturnOrEnter | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeRightAlt | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeRightArrow | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeRightControl | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeRightGUI | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeRightShift | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeScrollLock | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeSemicolon | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeSeven | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeSix | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeSlash | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeSpacebar | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeTab | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeThree | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeTwo | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeUpArrow | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyCodeZero | constant | GCKeyCodes.h | No Rust constant export for this HID-page-7 keycode. |
| GCKeyComma | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyD | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyDeleteForward | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyDeleteOrBackspace | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyDownArrow | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyE | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyEight | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyEnd | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyEqualSign | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyEscape | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF1 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF10 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF11 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF12 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF13 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF14 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF15 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF16 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF17 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF18 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF19 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF2 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF20 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF3 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF4 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF5 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF6 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF7 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF8 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyF9 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyFive | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyFour | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyG | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyGraveAccentAndTilde | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyH | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyHome | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyHyphen | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyI | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInsert | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational1 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational2 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational3 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational4 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational5 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational6 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational7 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational8 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyInternational9 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyJ | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyK | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad0 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad1 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad2 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad3 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad4 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad5 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad6 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad7 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad8 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypad9 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadAsterisk | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadEnter | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadEqualSign | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadHyphen | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadNumLock | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadPeriod | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadPlus | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyKeypadSlash | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyL | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG1 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG2 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG3 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG4 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG5 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG6 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG7 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG8 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLANG9 | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLeftAlt | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLeftArrow | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLeftControl | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLeftGUI | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyLeftShift | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyM | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyN | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyNine | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyNonUSBackslash | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyNonUSPound | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyO | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyOne | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyOpenBracket | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyP | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPageDown | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPageUp | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPause | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPeriod | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPower | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyPrintScreen | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyQ | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyQuote | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyR | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyReturnOrEnter | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyRightAlt | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyRightArrow | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyRightControl | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyRightGUI | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyRightShift | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyS | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyScrollLock | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeySemicolon | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeySeven | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeySix | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeySlash | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeySpacebar | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyT | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyTab | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyThree | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyTwo | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyU | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyUpArrow | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyV | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyW | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyX | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyY | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyZ | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyZero | constant | GCKeyNames.h | No Rust constant export for this GCKey* alias. |
| GCKeyboardDidConnectNotification | constant | GCKeyboard.h | No Rust notification wrapper for this lifecycle event. |
| GCKeyboardDidDisconnectNotification | constant | GCKeyboard.h | No Rust notification wrapper for this lifecycle event. |
| GCMouseDidBecomeCurrentNotification | constant | GCMouse.h | No Rust notification wrapper for this lifecycle event. |
| GCMouseDidConnectNotification | constant | GCMouse.h | No Rust notification wrapper for this lifecycle event. |
| GCMouseDidDisconnectNotification | constant | GCMouse.h | No Rust notification wrapper for this lifecycle event. |
| GCMouseDidStopBeingCurrentNotification | constant | GCMouse.h | No Rust notification wrapper for this lifecycle event. |
| GCPhysicalInputElementCollection | interface | GCPhysicalInputElement.h | Collections are materialized as Vec<T>, not exposed as GCPhysicalInputElementCollection. |
| GCPhysicalInputElementName | typedef | GCInputNames.h | Aliases are surfaced as String values, not typed input-name wrappers. |
| GCPhysicalInputExtents | protocol | GCPhysicalInputExtents.h | Physical-input extent metadata is not wrapped. |
| GCPhysicalInputSource | protocol | GCPhysicalInputSource.h | Remapping/source metadata is not wrapped. |
| GCPhysicalInputSourceDirection | typedef enum | GCPhysicalInputSource.h | Remapping/source metadata is not wrapped. |
| GCPoint2 | typedef | GCTypes.h | 2D values are flattened to x/y fields, not exposed as GCPoint2 helpers. |
| GCPoint2Zero | constant | GCTypes.h | 2D values are flattened to x/y fields, not exposed as GCPoint2 helpers. |
| GCProductCategoryArcadeStick | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryCoalescedRemote | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryControlCenterRemote | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryDualSense | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryDualShock4 | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryHID | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryKeyboard | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryMFi | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryMouse | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategorySiriRemote1stGen | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategorySiriRemote2ndGen | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategorySpatialController | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryUniversalElectronicsRemote | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCProductCategoryXboxOne | constant | GCProductCategories.h | product_category is surfaced as String; typed category constants are not exported. |
| GCRacingWheelDidConnectNotification | constant | GCRacingWheel.h | No Rust notification wrapper for this lifecycle event. |
| GCRacingWheelDidDisconnectNotification | constant | GCRacingWheel.h | No Rust notification wrapper for this lifecycle event. |
| GCSwitchElementName | typedef | GCInputNames.h | Aliases are surfaced as String values, not typed input-name wrappers. |
| GCSystemGestureState | typedef enum | GCControllerElement.h | No standalone Rust wrapper for controller-element base metadata or system-gesture state. |

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

