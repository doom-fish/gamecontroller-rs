//! Exercise the v0.7.2 input / constant / device-snapshot surface without requiring hardware.
//!
//! Run: `cargo run --example 05_input_surface`

use gamecontroller::prelude::*;

fn main() -> Result<(), GameControllerError> {
    let controllers = connected_controller_details()?;
    println!("controllers: {}", controllers.len());
    if let Some(controller) = controllers.first() {
        println!(
            "first: {} directional={} dualshock={} dualsense={} xbox={} light={} haptics={} live_input={}",
            controller.vendor_name,
            controller.directional_gamepad.is_some(),
            controller.dual_shock.is_some(),
            controller.dual_sense.is_some(),
            controller.xbox.is_some(),
            controller.light.is_some(),
            controller.haptics.is_some(),
            controller.input.is_some(),
        );
    }

    let devices = connected_devices_snapshot()?;
    println!(
        "devices: controllers={} keyboard={} mouse={} racing_wheels={}",
        devices.controllers.len(),
        devices.keyboard.is_some(),
        devices.mouse.is_some(),
        devices.racing_wheels.len(),
    );

    println!(
        "current_input: {}",
        current_controller_input_snapshot()?.is_some()
    );
    println!(
        "current_input_source: {}",
        current_controller_input_source()?.is_some()
    );
    println!("current_elements: {}", current_controller_elements()?.len());
    println!(
        "current_physical_elements: {}",
        current_controller_physical_input_elements()?.is_some()
    );
    println!(
        "constants: key_a={} key_space={} button_a={} left_thumbstick={} dualsense={} locality={} point2_zero=({}, {})",
        key_codes::KEY_A,
        key_names::SPACEBAR,
        input_names::BUTTON_A,
        input_names::LEFT_THUMBSTICK,
        product_categories::DUAL_SENSE,
        haptics_localities::DEFAULT,
        POINT2_ZERO.x,
        POINT2_ZERO.y,
    );
    println!("first_battery: {}", first_controller_battery()?.is_some());
    println!("first_light: {}", first_controller_light()?.is_some());
    println!("first_haptics: {}", first_controller_haptics()?.is_some());
    let _ = rumble_first_controller_with_locality(
        haptics_localities::DEFAULT,
        0.2,
        0.3,
        f64::from(HAPTIC_DURATION_INFINITE),
    );
    let _ = set_first_controller_light_color(Color {
        red: 0.1,
        green: 0.2,
        blue: 0.3,
    });

    Ok(())
}
