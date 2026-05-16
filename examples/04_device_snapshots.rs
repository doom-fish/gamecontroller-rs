//! Snapshot the keyboard, mouse, event-view-controller defaults, and racing wheels.
//!
//! Run: `cargo run --example 04_device_snapshots`

use gamecontroller::prelude::*;

fn main() -> Result<(), GameControllerError> {
    match keyboard_snapshot()? {
        Some(keyboard) => println!(
            "keyboard: {} ({}) any_key_pressed={} pressed_keys={}",
            keyboard.vendor_name,
            keyboard.product_category,
            keyboard.any_key_pressed,
            keyboard.pressed_keys.len(),
        ),
        None => println!("keyboard: none"),
    }

    match mouse_snapshot()? {
        Some(mouse) => println!(
            "mouse: {} ({}) current={} auxiliary_buttons={}",
            mouse.vendor_name,
            mouse.product_category,
            mouse.is_current,
            mouse.auxiliary_buttons.len(),
        ),
        None => println!("mouse: none"),
    }

    let event_view_controller = event_view_controller_snapshot()?;
    println!(
        "event_view_controller: controller_user_interaction_enabled={}",
        event_view_controller.controller_user_interaction_enabled,
    );

    println!("racing_wheels: {}", connected_racing_wheels()?.len());
    Ok(())
}
