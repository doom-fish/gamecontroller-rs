use gamecontroller::prelude::*;

#[test]
fn controller_surface_smoke() -> Result<(), GameControllerError> {
    let controllers = connected_controller_details()?;
    if let Some(controller) = controllers.first() {
        let _ = &controller.directional_gamepad;
        let _ = &controller.dual_shock;
        let _ = &controller.dual_sense;
        let _ = &controller.xbox;
        let _ = &controller.battery;
        let _ = &controller.light;
        let _ = &controller.haptics;
        let _ = &controller.input;
    }

    let _devices = connected_devices_snapshot()?;
    let _elements = current_controller_elements()?;
    if let Some(physical_elements) = current_controller_physical_input_elements()? {
        let _: GCPhysicalInputElementCollection<ButtonElementDetails> = physical_elements.buttons;
    }

    let _watch_connections = watch_connections(|_| {});
    let _watch_current = watch_current_controller(|_| {});
    let _watch_keyboard = watch_keyboard_connections(|_| {});
    let _watch_mouse = watch_mouse_connections(|_| {});
    let _watch_mouse_current = watch_mouse_current(|_| {});
    let _watch_wheels = watch_racing_wheel_connections(|_| {});
    let _watch_customizations = watch_controller_customizations(|| {});

    let _ = current_controller_input_snapshot()?;
    if let Some(source) = current_controller_input_source()? {
        let _ = source.input_state_queue_depth;
        let _ = &source.capture;
        let _ = &source.next_diff;
    }
    let _ = first_controller_battery()?;
    let _ = first_controller_light()?;
    let _ = first_controller_haptics()?;
    let _ = rumble_first_controller_with_locality(
        haptics_localities::DEFAULT,
        0.0,
        0.0,
        f64::from(HAPTIC_DURATION_INFINITE),
    );
    let _ = set_first_controller_light_color(Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    });
    let _ = dualsense_trigger_feedback_resistive_strengths(
        DualSenseTrigger::Left,
        GCDualSenseAdaptiveTriggerPositionalResistiveStrengths::default(),
    );
    let _ = dualsense_trigger_vibration_amplitudes(
        DualSenseTrigger::Right,
        GCDualSenseAdaptiveTriggerPositionalAmplitudes::default(),
        1.0,
    );
    Ok(())
}
