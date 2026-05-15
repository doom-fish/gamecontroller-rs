//! List every connected MFi-compatible gamepad.
//!
//! Run: `cargo run --example 01_list_controllers`

use gamecontroller::prelude::*;

fn main() {
    let controllers = connected_controllers();
    println!("{} controller(s) connected:", controllers.len());
    for c in &controllers {
        println!("\n  vendor:   {}", c.vendor_name);
        println!("  category: {}", c.product_category);
        println!("  player:   {}", c.player_index);
        println!("  attached: {}", c.is_attached_to_device);
        println!("  extended: {}", c.has_extended_gamepad);
    }
    if controllers.is_empty() {
        println!("\n  (Connect an MFi-compatible gamepad — Xbox, PS4/PS5, or any modern Bluetooth controller — and re-run.)");
    }
}
