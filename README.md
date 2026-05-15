# gamecontroller

Safe Rust bindings for Apple's [GameController](https://developer.apple.com/documentation/gamecontroller) framework on macOS — enumerate connected gamepads and snapshot their button / stick / trigger / D-pad state.

> **Status:** experimental. v0.1 ships polling-based snapshots from `extendedGamepad` (Xbox, DualShock 4, DualSense, MFi). v0.2 adds connect/disconnect callbacks via run-loop integration, haptic-engine support, light-bar control, motion sensors, and the `microGamepad` (Apple TV remote) profile.

## Quick start

```rust,no_run
use gamecontroller::prelude::*;

fn main() {
    for c in connected_controllers() {
        println!("{} ({}) — player {}",
            c.vendor_name, c.product_category, c.player_index);
        println!("  A={} B={} X={} Y={}",
            c.buttons.a, c.buttons.b, c.buttons.x, c.buttons.y);
        println!("  L-stick = ({:.2}, {:.2})",
            c.thumbsticks.left_x, c.thumbsticks.left_y);
        println!("  triggers = L:{:.2} R:{:.2}",
            c.triggers.left_trigger, c.triggers.right_trigger);
    }
}
```

## Pipeline composition

```text
gamecontroller (poll state) ──► your game loop / mapping engine
                                   │
                                   ├─► cgevents (synth keyboard/mouse from gamepad)
                                   └─► iohidmanager (raw HID for non-MFi controllers)
```

## Roadmap

- [x] `connected_controllers() -> Vec<Controller>`
- [x] `Controller { vendor_name, product_category, player_index, ... }`
- [x] Buttons, Triggers, Thumbsticks, Dpad snapshot types
- [x] `extendedGamepad` profile (Xbox / `DualShock` / `DualSense` / `MFi`)
- [ ] Connect / disconnect notification callbacks (run-loop)
- [ ] `GCDeviceHaptics` — rumble / trigger feedback
- [ ] `GCDeviceLight` — `DualSense` light-bar / Joy-Con LED control
- [ ] `GCMotion` — accelerometer / gyro
- [ ] `microGamepad` (Apple TV / Siri Remote) profile
- [ ] `GCDualSenseGamepad`-specific features (touchpad, adaptive triggers)
- [ ] `GCKeyboard` + `GCMouse` (modern Apple unified-input wrappers)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
