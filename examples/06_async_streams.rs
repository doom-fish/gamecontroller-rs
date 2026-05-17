//! Smoke-test the async stream surfaces without hardware.
//!
//! Run: `cargo run --example 06_async_streams --features async`
//!
//! On a headless macOS machine (no controllers connected) every
//! `try_next()` returns `None`, which is the expected result.

use gamecontroller::async_api::{
    ControllerConnectionStream, GamepadValueStream, KeyboardKeyStream, MicroGamepadValueStream,
    MotionStream, MouseInputStream,
};
use std::time::Duration;

fn main() {
    // Subscribe to all 6 stream surfaces.
    let conn_stream = ControllerConnectionStream::subscribe(16);
    let gamepad_stream = GamepadValueStream::subscribe(32);
    let keyboard_stream = KeyboardKeyStream::subscribe(32);
    let mouse_stream = MouseInputStream::subscribe(32);
    let motion_stream = MotionStream::subscribe(32);
    let micro_stream = MicroGamepadValueStream::subscribe(16);

    // Give the framework a moment to deliver any pending events.
    std::thread::sleep(Duration::from_millis(50));

    // On headless CI there are no controllers, so all try_next() return None.
    // That's the correct result — we're testing that the bridge compiles and
    // doesn't crash, not that hardware events arrive.
    println!("conn_event:     {:?}", conn_stream.try_next());
    println!("gamepad_event:  {:?}", gamepad_stream.try_next());
    println!("keyboard_event: {:?}", keyboard_stream.try_next());
    println!("mouse_event:    {:?}", mouse_stream.try_next());
    println!("motion_event:   {:?}", motion_stream.try_next());
    println!("micro_event:    {:?}", micro_stream.try_next());

    // Verify buffered_count is 0 when nothing connected.
    assert_eq!(conn_stream.buffered_count(), 0);
    assert_eq!(gamepad_stream.buffered_count(), 0);
    assert_eq!(keyboard_stream.buffered_count(), 0);
    assert_eq!(mouse_stream.buffered_count(), 0);
    assert_eq!(motion_stream.buffered_count(), 0);
    assert_eq!(micro_stream.buffered_count(), 0);

    // Drop streams — verifies that SubscriptionHandle::drop doesn't crash.
    drop(conn_stream);
    drop(gamepad_stream);
    drop(keyboard_stream);
    drop(mouse_stream);
    drop(motion_stream);
    drop(micro_stream);

    println!("ok — all 6 async stream surfaces exercised");
}
