use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

use gamecontroller::prelude::*;

fn main() {
    set_should_monitor_background_events(true);
    let deadline = Instant::now() + Duration::from_secs(5);
    println!("Polling every controller for 5 seconds. Press buttons and move the sticks.");
    while Instant::now() < deadline {
        let line = ControllerHandle::all()
            .iter()
            .filter_map(|handle| Some((handle.vendor_name()?, handle.extended_gamepad_state()?)))
            .map(|(name, state)| {
                format!(
                    "{name}: A={:.0} L3={:.0} R3={:.0} L=({:+.2},{:+.2})",
                    state.buttons.a,
                    state.left_thumbstick_button,
                    state.right_thumbstick_button,
                    state.thumbsticks.left_x,
                    state.thumbsticks.left_y,
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        print!("\r{:<100}", if line.is_empty() { "No extended gamepad is connected." } else { &line });
        let _ = std::io::stdout().flush();
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}
