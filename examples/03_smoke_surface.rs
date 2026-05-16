//! Smoke-test the richer v0.6 surface without requiring a connected controller.
//!
//! Run: `cargo run --example 03_smoke_surface`

use gamecontroller::prelude::*;

fn main() -> Result<(), GameControllerError> {
    let background = should_monitor_background_events();
    set_should_monitor_background_events(background);

    let controllers = connected_controller_details()?;
    println!("controllers: {}", controllers.len());

    if let Some(current) = current_controller_snapshot()? {
        println!(
            "current: {} ({}) player={} physical_buttons={}",
            current.vendor_name,
            current.product_category,
            current.player_index,
            current
                .physical_input
                .as_ref()
                .map_or(0, |profile| profile.buttons.len())
        );
    } else {
        println!("current: none");
    }

    if dualsense_is_connected() {
        let _ = dualsense_trigger_off(DualSenseTrigger::Left);
        let _ = dualsense_trigger_slope_feedback(
            DualSenseTrigger::Right,
            0.2,
            0.8,
            0.3,
            0.9,
        );
    }

    println!("smoke surface OK");
    Ok(())
}
