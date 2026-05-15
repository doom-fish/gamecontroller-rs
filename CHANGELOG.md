# Changelog

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
