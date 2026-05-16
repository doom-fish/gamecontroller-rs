# Changelog

## [0.7.0] - 2026-05-16

### Added

- `GCController.input` / `GCControllerLiveInput` / `GCControllerInputState` snapshots via:
  - `ControllerDetails::input`
  - `current_controller_input_snapshot()`
- Typed device snapshot helpers:
  - `first_controller_battery()`
  - `first_controller_light()`
  - `first_controller_haptics()`
  - `set_first_controller_light_color()`
- Richer `GCKeyboard` / `GCMouse` snapshots via `keyboard_snapshot()` and `mouse_snapshot()`.
- Controller-family coverage for:
  - `GCDirectionalGamepad`
  - `GCDualShockGamepad`
  - `GCXboxGamepad`
  - `GCColor` / `GCDeviceLight`
  - `GCDeviceHaptics`
- macOS-only `connected_racing_wheels()` snapshots for `GCRacingWheel`.
- `event_view_controller_snapshot()` for `GCEventViewController` defaults.
- Examples `04_device_snapshots` and `05_input_surface`.
- Smoke tests for the new snapshot/device surface plus expanded Swift API coverage checks.
- `COVERAGE.md` documenting the audited sweep and intentional partial/skipped areas.

### Changed

- Split the Swift bridge into logical files (`Core`, `Payloads`, `Controllers`, `Discovery`, `Devices`, `DualSense`) so no single bridge file is monolithic.
- `ControllerDetails` now includes vendor-specific gamepad families, typed light/haptics snapshots, and optional live-input data.
- The API-coverage harness now scans every Swift bridge file instead of only the former `GameController.swift` monolith.
- README surface documentation now reflects the v0.7 snapshot/device additions and the explicit `GCRemote` skip note.

### Partial / skipped notes

- `GCRacingWheel` acquisition/relinquish remains intentionally unwrapped in safe Rust because it is exclusive, handle-oriented state that does not fit the crate's existing snapshot-first design.
- `GCRemote` does not appear in the inspected macOS GameController headers and is therefore tracked as skipped in `COVERAGE.md`.

## [0.6.0] - 2026-05-16

### Added

- `connected_controller_details()` and `current_controller_snapshot()` for richer JSON-backed snapshots of:
  - `GCController.current`
  - legacy `gamepad` / `microGamepad` / `extendedGamepad`
  - `GCPhysicalInputProfile`
  - `DualSense` touchpad + adaptive-trigger state
  - battery / motion availability inside the same snapshot
- Wireless discovery helpers:
  - `start_wireless_controller_discovery()`
  - `start_wireless_controller_discovery_with_callback()`
  - `stop_wireless_controller_discovery()`
- Background-event accessors:
  - `should_monitor_background_events()`
  - `set_should_monitor_background_events()`
- `dualsense_trigger_slope_feedback()`.
- `GameControllerError` for the new JSON-backed helpers.
- Example `03_smoke_surface`.

### Changed

- Improved the Swift bridge to expose `GCController.current`, `input`, `physicalInputProfile`, wireless discovery, and richer `DualSense` trigger state.
- `gc_dualsense_set_trigger_mode` now drives distinct feedback / weapon / vibration / slope-style trigger behavior instead of collapsing everything to uniform resistance.
- Expanded the API-coverage harness from a single `GCController` check to the main controller/profile families (`GCController`, `GCPhysicalInputProfile`, `GCGamepad`, `GCMicroGamepad`, `GCExtendedGamepad`, `GCDualSenseGamepad`, `GCDualSenseAdaptiveTrigger`).
- Crate packaging now includes `examples/` and `tests/`.

## [0.1.0] - Initial release

### Added

- `connected_controllers() -> Vec<Controller>` — snapshot every connected
  MFi-compatible gamepad in one call.
- `Controller { vendor_name, product_category, player_index, ... }`.
- `Buttons`, `Triggers`, `Thumbsticks`, `Dpad` snapshot types covering
  Apple's `extendedGamepad` profile (Xbox / DualShock / DualSense / MFi).
- 2 examples (`01_list_controllers`, `02_poll_state`).
- 1 API-coverage test (`GCController`) verifying we reference the
  v0.1-scoped properties + that the rest are explicitly listed in the
  omitted allowlist for v0.2.

### Why a focused subset?

GameController is a 60+-header framework covering haptics, lights,
motion, Apple Pencil, microGamepad (Apple TV), keyboard / mouse
unified-input, etc. v0.1 covers the polling-based gamepad surface most
games actually need; the rest land in v0.2+ as run-loop-integrated
async APIs.
