use gamecontroller::prelude::*;

#[test]
fn constant_surface_smoke() {
    let _: GCKeyCode = key_codes::KEY_A;
    let _: GCKeyName = key_names::SPACEBAR;
    let _: GCInputButtonName = input_names::BUTTON_A;
    let _: GCInputButtonName = input_names::GRIP_BUTTON;
    let _: GCInputAxisName = input_names::STEERING_WHEEL;
    let _: GCInputDirectionPadName = input_names::LEFT_THUMBSTICK;
    let _: GCInputElementName = input_names::SHIFTER;
    let _: GCButtonElementName = input_names::BUTTON_SHARE;
    let _: GCAxisElementName = input_names::STEERING_WHEEL;
    let _: GCDirectionPadElementName = input_names::DIRECTIONAL_DPAD;
    let _: GCHapticsLocality = haptics_localities::DEFAULT;
    let _: GCProductCategory = product_categories::DUAL_SENSE;
    let _: GCPoint2 = POINT2_ZERO;
    let _: GCEulerAngles = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    }
    .into();
    let _: GCDualSenseAdaptiveTriggerPositionalAmplitudes =
        DualSenseAdaptiveTriggerPositionalAmplitudes::default();
    let _: GCDualSenseAdaptiveTriggerPositionalResistiveStrengths =
        DualSenseAdaptiveTriggerPositionalResistiveStrengths::default();

    assert_eq!(key_codes::KEY_A, 4);
    assert_eq!(key_names::SPACEBAR.as_str(), "Spacebar");
    assert_eq!(input_names::BUTTON_A.as_str(), "Button A");
    assert_eq!(input_names::GRIP_BUTTON.as_str(), "Grip");
    assert_eq!(haptics_localities::LEFT_HANDLE.as_str(), "Left Handle");
    assert_eq!(product_categories::DUAL_SHOCK_4.as_str(), "DualShock 4");
    assert!(POINT2_ZERO.x.abs() < f32::EPSILON);
    assert!(POINT2_ZERO.y.abs() < f32::EPSILON);
    assert!((HAPTIC_DURATION_INFINITE - 1_000_000.0).abs() < f32::EPSILON);
    let direction = GCPhysicalInputSourceDirection::UP | GCPhysicalInputSourceDirection::LEFT;
    assert!(direction.contains(GCPhysicalInputSourceDirection::UP));
    assert!(direction.contains(GCPhysicalInputSourceDirection::LEFT));
    assert_eq!(input_names::back_left_button(1), "Back Left Button 1");
    assert_eq!(input_names::back_right_button(0), "Back Right Button 0");
    assert_eq!(input_names::arcade_button_name(1, 2), "Arcade Button 1, 2");
}
