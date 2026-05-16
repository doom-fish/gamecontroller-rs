# gamecontroller

Safe Rust bindings for Apple's [GameController](https://developer.apple.com/documentation/gamecontroller) framework on macOS.

> **Status:** `0.6.0` covers connected-controller polling, connection watchers, keyboard/mouse helpers, battery/light/haptics/motion reads, `GCController.current`, background-event control, wireless discovery, legacy `gamepad` / `microGamepad` / `extendedGamepad` snapshots, `GCPhysicalInputProfile` snapshots, and `DualSense` touchpad/trigger state.

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
    }
    Ok(())
}
```

## Surface highlights

- `connected_controllers()` for lightweight polling snapshots.
- `connected_controller_details()` / `current_controller_snapshot()` for richer JSON-backed controller/profile snapshots.
- `watch_connections()` for connect/disconnect notifications.
- `should_monitor_background_events()` / `set_should_monitor_background_events()`.
- `start_wireless_controller_discovery()` and `stop_wireless_controller_discovery()`.
- `first_controller_extras()` / `all_controller_extras()` for battery, motion, light, and haptics availability.
- `mouse_*` and `keyboard_*` helpers for `GCMouse` / `GCKeyboard`.
- `dualsense_trigger_*` helpers plus `DualSenseAdaptiveTriggerState` readback in detailed snapshots.

## Examples

- `cargo run --example 01_list_controllers`
- `cargo run --example 02_poll_state`
- `cargo run --example 03_smoke_surface`

## Notes

- Wireless-discovery completion callbacks and connection notifications fire on the main run loop.
- `gamepad` support is included for completeness even though Apple prefers `extendedGamepad`.
- The crate ships additive wrappers only; snapshot-mutation APIs such as `controllerWithExtendedGamepad()` remain out of scope for now.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
