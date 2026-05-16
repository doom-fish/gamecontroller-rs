# Changelog

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
