# GameController v0.7.0 coverage sweep

This sweep audited the requested GameController families against the macOS 26.2 SDK headers and the Rust/Swift surface shipped by this crate.

| Family | Status | Notes |
| --- | --- | --- |
| `GCController` | ✅ | Discovery/current/background events/wireless discovery plus rich controller snapshots are wrapped. Snapshot-constructor APIs remain intentionally omitted, matching prior crate scope. |
| `GCKeyboard` | ✅ | `keyboard_is_*` polling helpers remain and `keyboard_snapshot()` now exposes richer `GCKeyboardInput` / `GCPhysicalInputProfile` data. |
| `GCMouse` | ✅ | `mouse_is_connected()` / `mouse_button_states()` remain and `mouse_snapshot()` now exposes scroll, auxiliary buttons, and `GCMouseInput` profile data. |
| `GCExtendedGamepad` | ✅ | Detailed snapshots continue to cover the full extended profile surface. |
| `GCMicroGamepad` | ✅ | Detailed snapshots continue to cover the micro profile surface. |
| `GCDirectionalGamepad` | ✅ | `ControllerDetails::directional_gamepad` now reports the directional-gamepad profile when the connected micro gamepad is a `GCDirectionalGamepad`. |
| `GCHaptics` / `GCDeviceHaptics` | ✅ | `rumble_first_controller()` remains, and typed `DeviceHapticsDetails` snapshots now expose supported localities. |
| `GCMotion` | ✅ | Motion snapshots remain available both in `ControllerExtras` and `ControllerDetails`. |
| `GCPhysicalInputProfile` | ✅ | Rich JSON-backed `PhysicalInputProfileDetails` coverage remains in controller, keyboard, and mouse snapshots. |
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
