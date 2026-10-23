# gamecontroller

Safe Rust bindings for Apple's [GameController](https://developer.apple.com/documentation/gamecontroller) framework on macOS.

> **Status:** `0.9.1` adds `ControllerHandle::extended_gamepad_state`. `0.9.0` fixes use-after-free and data-race bugs in the watchers and async streams, lets several streams share one controller handler, adds per-controller `ControllerHandle`s for light, player index, rumble and `DualSense` triggers, and adds the macOS 26.4 side-button input names. The coverage audit counts top-level symbols, not methods (see [`COVERAGE.md`](COVERAGE.md)).

## Installation

```toml
[dependencies]
gamecontroller = "0.9"
```

## Requirements

- macOS 13 or newer (the Swift bridge's deployment target).
- `input_names::LEFT_SIDE_BUTTON` / `RIGHT_SIDE_BUTTON` are reported by macOS 26.4 and newer; `input_names::side_buttons_available()` checks the running system.
- Connection notifications, discovery completion and controller input handlers run on the main queue, so the app needs a running main run loop to receive them.

## Quick start

```rust,no_run
use gamecontroller::prelude::*;

fn main() -> Result<(), GameControllerError> {
    for controller in connected_controller_details()? {
        println!(
            "{} ({}) current={} buttons={}",
            controller.vendor_name,
            controller.product_category,
            controller.is_current,
            controller
                .physical_input
                .as_ref()
                .map_or(0, |profile| profile.buttons.len())
        );

        if let Some(extended) = &controller.extended_gamepad {
            println!(
                "  A={} B={} LT={:.2} RT={:.2}",
                extended.button_a.pressed,
                extended.button_b.pressed,
                extended.left_trigger.value,
                extended.right_trigger.value
            );
        }

        if let Some(input) = &controller.input {
            println!(
                "  live_input buttons={} changed_known={}",
                input.live.button_count,
                input.next.as_ref().is_some_and(|next| next.changed_aliases_known)
            );
        }
    }
    Ok(())
}
```

## Surface highlights

- `connected_controllers()` for lightweight polling snapshots.
- `connected_controller_details()` / `current_controller_snapshot()` for richer JSON-backed controller/profile snapshots.
- `current_controller_input_snapshot()` for `GCController.input` / `GCControllerLiveInput` / `GCControllerInputState` capture data.
- `current_controller_input_source()` for direct `GCDevicePhysicalInput` live/captured/queued-diff snapshots.
- `current_controller_elements()` / `current_controller_physical_input_elements()` for legacy `GCControllerElement` metadata plus generic physical-input/source/extents snapshots.
- `connected_devices_snapshot()` for generic `GCDevice` metadata across controllers, keyboard, mouse, and racing wheels.
- `key_codes`, `key_names`, `input_names`, `haptics_localities`, and `product_categories` modules for the audited `GCKeyCode*`, `GCKey*`, `GCInput*`, haptics-locality, and product-category constants.
- `keyboard_snapshot()` / `mouse_snapshot()` for richer `GCKeyboard` / `GCMouse` snapshots.
- `connected_racing_wheels()` for macOS-only `GCRacingWheel` metadata + captured input when already acquired.
- `event_view_controller_snapshot()` for `GCEventViewController` defaults.
- `first_controller_battery()` / `first_controller_light()` / `first_controller_haptics()` for typed device-side snapshots.
- `ControllerHandle::all()` / `current()` / `first()` / `first_dualsense()` return handles to specific controllers with `set_light()`, `set_player_index()`, `battery_level()`, `rumble()` and `set_dualsense_trigger()` (a typed `DualSenseTriggerEffect`).
- `ControllerHandle::extended_gamepad_state()` reads all buttons, triggers, thumbsticks and thumbstick buttons of one controller without JSON. Handles are equal, and hash equally, when they refer to the same controller.
- `set_first_controller_light_color()` plus `set_first_controller_light()` for `GCColor` / `GCDeviceLight` mutation on the first controller.
- `rumble_first_controller_with_locality()` plus `DeviceHapticsDetails::supported_locality_constants()` for typed haptics locality coverage.
- `watch_connections()`, `watch_current_controller()`, `watch_keyboard_connections()`, `watch_mouse_connections()`, `watch_mouse_current()`, `watch_racing_wheel_connections()`, and `watch_controller_customizations()` for notification coverage.
- `should_monitor_background_events()` / `set_should_monitor_background_events()`.
- `start_wireless_controller_discovery()`, `stop_wireless_controller_discovery()`, and `async_api::start_wireless_controller_discovery()`.
- `dualsense_trigger_*` helpers (Apple's feedback, weapon, vibration and slope modes on the first `DualSense`) plus `dualsense_trigger_feedback_resistive_strengths()` / `dualsense_trigger_vibration_amplitudes()` for raw adaptive-trigger array helpers.

## Async API

Enable the `async` feature to await wireless discovery without wiring a callback, or to receive connection, gamepad, keyboard, mouse, motion and micro-gamepad events as bounded async streams:

```toml
[dependencies]
gamecontroller = { version = "0.9", features = ["async"] }
```

```rust,no_run
#[cfg(feature = "async")]
async fn example() -> Result<(), gamecontroller::GameControllerError> {
    gamecontroller::async_api::start_wireless_controller_discovery().await?;
    Ok(())
}
```

## Examples

- `cargo run --example 01_list_controllers`
- `cargo run --example 02_poll_state`
- `cargo run --example 03_smoke_surface`
- `cargo run --example 04_device_snapshots`
- `cargo run --example 05_input_surface`

## Notes

- Wireless-discovery completion callbacks and connection notifications fire on the main run loop.
- `gamepad` support is included for completeness even though Apple prefers `extendedGamepad`.
- The crate is snapshot-first. The async streams are the only code that installs `valueChangedHandler`, `keyChangedHandler` and the mouse handlers. They share one handler per device: several streams can subscribe at once, a handler the app installed earlier keeps being called, and it is restored when the last stream is dropped. Handler-queue mutation and exclusive `GCRacingWheel` acquisition are not wrapped.
- Dropping a watcher or stream stops its callbacks before `drop` returns; the closure or sender is freed once Swift has released it.
- `GCRemote` is not present in the macOS SDK headers inspected for this release and is documented as skipped in [`COVERAGE.md`](COVERAGE.md).

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
