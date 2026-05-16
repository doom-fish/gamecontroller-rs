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

    assert_eq!(key_codes::KEY_A, 4);
    assert_eq!(key_names::SPACEBAR.as_str(), "Spacebar");
    assert_eq!(input_names::BUTTON_A.as_str(), "Button A");
    assert_eq!(input_names::GRIP_BUTTON.as_str(), "Grip");
    assert_eq!(input_names::back_left_button(1), "Back Left Button 1");
    assert_eq!(input_names::back_right_button(0), "Back Right Button 0");
    assert_eq!(input_names::arcade_button_name(1, 2), "Arcade Button 1, 2");
}
