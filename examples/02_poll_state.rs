//! Poll the first connected controller's state every 100ms for 5 seconds.
//!
//! Run: `cargo run --example 02_poll_state`

use gamecontroller::prelude::*;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let deadline = Instant::now() + Duration::from_secs(5);
    println!("Polling for 5 seconds (press buttons / move sticks)...\n");
    while Instant::now() < deadline {
        let controllers = connected_controllers();
        if let Some(c) = controllers.first() {
            print!(
                "\rA={:.0} B={:.0} X={:.0} Y={:.0} | L:({:+.2},{:+.2}) R:({:+.2},{:+.2}) | LT={:.2} RT={:.2}      ",
                c.buttons.a, c.buttons.b, c.buttons.x, c.buttons.y,
                c.thumbsticks.left_x, c.thumbsticks.left_y,
                c.thumbsticks.right_x, c.thumbsticks.right_y,
                c.triggers.left_trigger, c.triggers.right_trigger,
            );
            let _ = std::io::stdout().flush();
        } else {
            println!("\rNo controller connected.       ");
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}
