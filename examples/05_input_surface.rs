//! Exercise the v0.7 input / device-snapshot surface without requiring hardware.
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

    println!(
        "current_input: {}",
        current_controller_input_snapshot()?.is_some()
    );
    println!("first_battery: {}", first_controller_battery()?.is_some());
    println!("first_light: {}", first_controller_light()?.is_some());
    println!("first_haptics: {}", first_controller_haptics()?.is_some());
    let _ = set_first_controller_light_color(Color {
        red: 0.1,
        green: 0.2,
        blue: 0.3,
    });

    Ok(())
}
