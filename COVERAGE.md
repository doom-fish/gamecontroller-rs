# GameController v0.7.2 coverage sweep

This sweep audited the requested GameController families against the macOS 26.2 SDK headers and the Rust/Swift surface shipped by this crate.

| Family | Status | Notes |
| --- | --- | --- |
| `GCController` | ✅ | Discovery/current/background events/wireless discovery plus rich controller snapshots are wrapped, including current-controller/customization notifications via watcher helpers. Snapshot-constructor APIs remain intentionally omitted, matching prior crate scope. |
| `GCControllerElement` | ✅ | `current_controller_elements()` now exposes legacy `GCControllerElement` metadata plus typed system-gesture state for the current controller's visible profile elements. |
| `GCKeyboard` | ✅ | `keyboard_is_*` polling helpers remain, `keyboard_snapshot()` exposes richer `GCKeyboardInput` / `GCPhysicalInputProfile` data, and `watch_keyboard_connections()` covers keyboard notifications. |
| `GCMouse` | ✅ | `mouse_is_connected()` / `mouse_button_states()` remain, `mouse_snapshot()` exposes scroll/auxiliary buttons, and dedicated mouse connection/current watchers cover the lifecycle notifications. |
| `GCExtendedGamepad` | ✅ | Detailed snapshots continue to cover the full extended profile surface. |
| `GCMicroGamepad` | ✅ | Detailed snapshots continue to cover the micro profile surface. |
| `GCDirectionalGamepad` | ✅ | `ControllerDetails::directional_gamepad` now reports the directional-gamepad profile when the connected micro gamepad is a `GCDirectionalGamepad`. |
| `GCHaptics` / `GCDeviceHaptics` | ✅ | `rumble_first_controller()` remains, `rumble_first_controller_with_locality()` adds locality-aware playback, and typed `DeviceHapticsDetails` snapshots now expose supported localities. |
| `GCMotion` | ✅ | Motion snapshots remain available both in `ControllerExtras` and `ControllerDetails`, and `Quaternion::to_euler_angles()` exposes the legacy Euler-angle struct when needed. |
| `GCPhysicalInputProfile` | ✅ | Rich JSON-backed `PhysicalInputProfileDetails` coverage remains in controller, keyboard, and mouse snapshots, and `current_controller_physical_input_elements()` now exposes generic physical-input/source/extents metadata. |
| `GCDevice` | ✅ | `connected_devices_snapshot()` exposes generic `GCDevice` vendor/category/handler-queue metadata across controllers, keyboard, mouse, and racing wheels. |
| `GCRacingWheel` | 🟡 | `connected_racing_wheels()` exposes metadata and captured wheel input when the device is already acquired. Exclusive acquire/relinquish APIs are intentionally left out of safe Rust because they require retained handles / ownership beyond the crate's snapshot-first design. |
| `GCDualSenseAdaptiveTrigger` | ✅ | Existing trigger helpers remain and detailed snapshots still report adaptive-trigger state. |
| `GCDualShockGamepad` | ✅ | `ControllerDetails::dual_shock` now snapshots DualShock touchpad controls. |
| `GCXboxGamepad` | ✅ | `ControllerDetails::xbox` now snapshots paddles and Share button state. |
| `GCEventViewController` | 🟡 | `event_view_controller_snapshot()` covers the default `controllerUserInteractionEnabled` state. Window/root-view-controller integration is intentionally not wrapped in safe Rust because examples/tests must stay headless. |
| `GCColor` | ✅ | Typed `Color` snapshots are exposed via `DeviceLightDetails`, and `set_first_controller_light_color()` provides a safe Rust setter path. |
| `GCController.input` | ✅ | `ControllerDetails::input` and `current_controller_input_snapshot()` surface the live-input object snapshot. |
| `GCController.battery` | ✅ | `ControllerDetails::battery`, `first_controller_battery()`, and the legacy scalar helper all expose controller battery data. |
| `GCControllerLiveInput` | 🟡 | Live-input capture / unmapped / next-state snapshot data is exposed. Callback-heavy queue/handler configuration remains intentionally out of scope for the current safe Rust layer. |
| `GCControllerInputState` | ✅ | `ControllerInputStateDetails` models captured state metadata plus button/axis/switch/dpad snapshots. |
| `GCDeviceLight` | ✅ | Typed light snapshots (`DeviceLightDetails`) and the existing/light-color setter APIs are exposed. |
| `GCRemote` | ⏭️ | Skipped: `GCRemote` does not appear in the inspected macOS GameController framework headers, so there is no macOS API surface for this crate to wrap. |
