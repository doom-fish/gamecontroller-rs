# Changelog

## [0.8.4] - 2026-05-18

- Add one-line docs across the public safe and FFI surfaces, raising public-item rustdoc coverage to 99.2%.

## [0.8.3] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## [0.8.2] - 2026-05-18

### Fixed

- **Panic safety**: `connection_trampoline`, `notification_trampoline`, and
  `discovery_trampoline` now wrap user-closure calls in `catch_unwind` via a
  private `catch_cb_panic` helper, preventing UB from panics unwinding across
  the `extern "C"` ABI boundary.
- **SAFETY comments**: added `// SAFETY:` annotations to every `unsafe { }`
  block and `unsafe impl` in `src/controller/mod.rs` (42 sites) and
  `src/async_api.rs` (20 sites).

## [0.8.1] - 2026-05-17

### Changed

- Added `@available(macOS 26.2, *)` guard on `physicalInputExtentsDetailsPayload`
  and `if #available(macOS 26.2, *)` guard around `GCLinearInput.physicalExtents`
  access in `linearInputDetailsPayload` so the Swift bridge compiles against
  SDKs that predate macOS 26 (Tahoe).
- Added `if #available(macOS 26.0, *)` guard around `GCButtonElement.forceInput`
  access in `buttonElementDetailsPayload` for the same reason.
- All `@_cdecl` thunks that transitively reach the macOS 26+ code paths already
  have matching `#available` fallbacks returning safe defaults (`nil` / empty
  payload) on older OS versions.

## [0.8.0] - 2026-05-17

### Added

- `async` Cargo feature gate and `src/async_api.rs` module with six
  `BoundedAsyncStream<T>` event-stream surfaces:
  - `ControllerConnectionStream` — `GCControllerDidConnect` /
    `GCControllerDidDisconnect` notifications
  - `GamepadValueStream` — `GCExtendedGamepad.valueChangedHandler`
  - `KeyboardKeyStream` — `GCKeyboard.keyboardInput.keyChangedHandler`
    (macOS 11.0+)
  - `MouseInputStream` — `GCMouse.mouseInput` moved / button handlers
    (macOS 11.0+)
  - `MotionStream` — `GCMotion.valueChangedHandler`
  - `MicroGamepadValueStream` — `GCMicroGamepad.valueChangedHandler`
- Swift bridge thunks in `AsyncStream.swift` backing each stream surface.
- Example `examples/06_async_streams.rs` demonstrating all six surfaces.
- Integration tests in `tests/async_stream_tests.rs`.

## [0.7.2] - 2026-05-17

### Added

- Generic `GCDevice` / `GCControllerElement` snapshot helpers:
  - `connected_devices_snapshot()`
  - `current_controller_elements()`
  - `current_controller_physical_input_elements()`
- Typed constant/type coverage for the remaining audit tables:
  - `haptics_localities::*` and `GCHapticsLocality`
  - `product_categories::*` and `GCProductCategory`
  - `POINT2_ZERO`, `GCPoint2`, and `GCEulerAngles`
- Extra notification watchers:
  - `watch_current_controller()`
  - `watch_keyboard_connections()`
  - `watch_mouse_connections()` / `watch_mouse_current()`
  - `watch_racing_wheel_connections()`
  - `watch_controller_customizations()`
- Locality-aware haptics plus raw DualSense positional-array helpers:
  - `rumble_first_controller_with_locality()`
  - `dualsense_trigger_feedback_resistive_strengths()`
  - `dualsense_trigger_vibration_amplitudes()`

### Changed

- `DeviceHapticsDetails` now offers typed locality helpers via `supported_locality_constants()` / `supports_locality()`.
- `Quaternion` now converts to the legacy Euler-angle struct with `to_euler_angles()`.
- `COVERAGE_AUDIT.md` now reports 100% non-exempt public-symbol coverage (426 verified / 0 gaps).
- README and coverage docs now call out the new device/element snapshots, constants, and notification watchers.

## [0.7.1] - 2026-05-17

### Added

- High-impact constant coverage for the large GameController tables:
  - `key_codes::*` for `GCKeyCode*`
  - `key_names::*` for `GCKey*`
  - `input_names::*` for `GCInput*`
  - typed name aliases such as `GCKeyName`, `GCInputButtonName`, `GCButtonElementName`, and `GCPhysicalInputElementName`
- `current_controller_input_source()` plus `DevicePhysicalInputSourceDetails` / `DevicePhysicalInputStateDiffDetails` for `GCDevicePhysicalInput` live/capture/queued-diff snapshots.
- `GCDeviceCursor` alias coverage via `DeviceCursorState` / `MouseSnapshot::scroll`.
- Smoke coverage for the new constant/input-source surface.

### Changed

- README surface docs now call out the constant modules and `GCDevicePhysicalInput` source snapshots.
- `COVERAGE_AUDIT.md` now reflects the v0.7.1 gap-closure sweep and documents the smaller set of still-pending areas.

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
