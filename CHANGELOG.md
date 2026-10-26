# Changelog

All notable changes to `gamecontroller` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.3] - 2026-10-26

### Fixed

- Build the Swift bridge with the default SwiftPM build engine and use its reported product directory.

## [0.9.2] - 2026-10-23

### Fixed

- The `06_async_streams` example requires the `async` feature.
- The Swift bridge now supports macOS 12.0. Slope-feedback triggers return `false` before macOS 12.3.
- Racing-wheel queries return an empty list before macOS 13.0.

## [0.9.1] - 2026-10-23

### Added

- `ControllerHandle::extended_gamepad_state` reads every element of the extended gamepad of one controller: the face, menu, options and home buttons, the shoulders, the triggers, the thumbsticks, the thumbstick buttons and the directional pad. It does not encode JSON, thus it is fast enough to call for each frame.
- `ExtendedGamepadState`, the value of that function.
- `ControllerHandle` implements `Hash`, thus a handle can be a key in a map while the controller is connected.
- The `07_handle_state` example.

## [0.9.0] - 2026-09-24

### Security

- The Swift watcher registries, the pending wireless-discovery state and the
  haptic-engine table are protected by locks. Creating or dropping watchers
  on several threads, or starting discovery while its completion ran, raced
  on them and could corrupt the heap.
- Dropping a `ConnectionWatcher`, `NotificationWatcher` or async stream no
  longer frees its closure or sender while a main-queue callback is still
  using it. They live in a `doom_fish_utils` `CallbackContext` that the Swift
  registration keeps alive until it is deallocated, and drop stops further
  callbacks before it returns.
- Async streams no longer overwrite single-slot handlers. A second stream of
  the same kind took over the events, dropping either stream cleared the
  handlers on every device, and handlers installed by the app were lost.

### Fixed

- `dualsense_trigger_feedback`, `_weapon`, `_vibration` and `_slope_feedback`
  call Apple's start-position trigger modes instead of approximating them
  with positional arrays, so weapon mode snaps as documented. The vibration
  docs said `frequency` was in Hz; it is normalized to `0.0..=1.0`.
- `set_first_controller_player_index` rejects values outside `-1..=3` instead
  of storing an undefined `GCControllerPlayerIndex`.
- Callback panics are contained by the shared `doom_fish_utils::panic_safe`
  helper; the two local copies (one printing a misleading prefix) are gone.

### Changed

- **Breaking:** the raw `ffi` register functions take the context's retain
  and release functions, and the raw `gc_first_controller_*` light,
  player-index, battery and rumble functions and the `gc_dualsense_*`
  functions are replaced by `gc_controller_handle_*` equivalents.
- Streams attach their handlers on the main queue and share one handler per
  device, chaining any handler that was already installed and restoring it
  when the last stream is dropped.
- `doom-fish-utils` requirement is `>=0.4.1, <0.5`; `rust-version` is 1.82.

### Added

- `ControllerHandle` for a specific controller: `all()`, `current()`,
  `first()`, `first_dualsense()`, `vendor_name`, `product_category`,
  `player_index`, `set_player_index`, `set_light`, `battery_level`, `rumble`,
  `is_dualsense` and `set_dualsense_trigger` with `DualSenseTriggerEffect`.
- `input_names::LEFT_SIDE_BUTTON`, `input_names::RIGHT_SIDE_BUTTON`
  (macOS 26.4) and `input_names::side_buttons_available()`.

## [0.8.10] - 2026-06-06

- Contained panics in the async stream trampolines and pinned the raw FFI
  struct layouts with compile-time size and alignment checks.

## [0.8.9] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.8.8] - 2026-05-20

- Phase 32 completeness + async sweep.
- Added an async wireless-controller discovery future in `gamecontroller::async_api`.
- Refreshed the coverage docs against `MacOSX26.5.sdk`.

## [0.8.7] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.8.6] - 2026-05-19

### Added

- Added the Apple-style `GCButtonInput` and `GCSwitchInput` aliases for racing-wheel snapshots, mapping them onto `ButtonInputState` and `SwitchInputState` in the safe Rust API.

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

## [0.5.0] - 2026-05-16

### Added

- Add DualSense adaptive trigger support.

## [0.4.0] - 2026-05-16

### Added

- Add `GCMouse`, `GCKeyboard` and iteration over multiple controllers.

## [0.3.0] - 2026-05-16

### Added

- Add light-bar controls, player indices, battery state and simple haptics.

## [0.2.0] - 2026-05-16

### Added

- Add motion, battery, haptics, light and connection callbacks.

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
