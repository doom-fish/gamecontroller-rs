//! API-surface coverage harness for `gamecontroller`.
//!
//! The crate now wraps the main `GCController` discovery/current-controller
//! surface plus controller-input snapshots, keyboard/mouse snapshots, legacy
//! and vendor-specific gamepad families, controller light/haptics/battery data,
//! and macOS-only racing-wheel/event-controller helpers.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn read_bridge() -> String {
    let dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("swift-bridge/Sources/GameControllerBridge");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().and_then(std::ffi::OsStr::to_str) == Some("swift")).then_some(path)
        })
        .collect();
    files.sort();
    files.iter().map(read).collect::<Vec<_>>().join("\n")
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/GameController.framework/Headers/{name}.h"
    )))
}

fn extract_interfaces(header: &str, type_name: &str) -> String {
    let needle = regex_lite::Regex::new(&format!(r"@interface\s+{type_name}\b")).unwrap();
    let mut out = String::new();
    for m in needle.find_iter(header) {
        let rest = &header[m.start()..];
        let Some(end_off) = rest.find("@end") else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..end_off]);
        out.push('\n');
    }
    out
}

fn extract_member_surface(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let method_re =
        regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for captures in method_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:NS_|API_|;)",
    )
    .unwrap();
    for captures in prop_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for captures in getter_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    out
}

fn aliases() -> BTreeMap<&'static str, Vec<&'static str>> {
    BTreeMap::from([
        ("acquired", vec!["isAcquired"]),
        ("anyKeyPressed", vec!["isAnyKeyPressed"]),
        ("attachedToDevice", vec!["isAttachedToDevice"]),
        ("buttonForKeyCode", vec!["button(forKeyCode"]),
        ("changeForElement", vec!["change(for"]),
        ("coalescedKeyboard", vec!["coalesced"]),
        (
            "createEngineWithLocality",
            vec!["createEngine(withLocality"],
        ),
        (
            "mappedElementAliasForPhysicalInputName",
            vec!["mappedElementAlias"],
        ),
        (
            "mappedPhysicalInputNamesForElementAlias",
            vec!["mappedPhysicalInputNames"],
        ),
        ("analog", vec!["isAnalog"]),
        ("boundToSystemGesture", vec!["isBoundToSystemGesture"]),
        ("initWithRed", vec!["GCColor(red:"]),
        ("objectForKeyedSubscript", vec!["profile[alias]", "[alias]"]),
        ("snapshot", vec!["isSnapshot"]),
        (
            "setModeFeedbackWithStartPosition",
            vec!["gc_dualsense_set_trigger_mode"],
        ),
        (
            "setModeWeaponWithStartPosition",
            vec!["gc_dualsense_set_trigger_mode"],
        ),
        (
            "setModeVibrationWithStartPosition",
            vec!["gc_dualsense_set_trigger_mode"],
        ),
        (
            "setModeSlopeFeedbackWithStartPosition",
            vec!["gc_dualsense_set_trigger_mode"],
        ),
        (
            "setModeFeedbackWithResistiveStrengths",
            vec!["setModeFeedback"],
        ),
        ("setModeVibrationWithAmplitudes", vec!["setModeVibration"]),
        (
            "startWirelessControllerDiscoveryWithCompletionHandler",
            vec!["startWirelessControllerDiscovery"],
        ),
        ("unmappedInput", vec!["unmapped"]),
    ])
}

fn contains_symbol(haystack: &str, symbol: &str) -> bool {
    if symbol
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        let pattern = format!(r"\b{}\b", regex_lite::escape(symbol));
        regex_lite::Regex::new(&pattern).unwrap().is_match(haystack)
    } else {
        haystack.contains(symbol)
    }
}

fn references_in_bridge(
    symbols: &BTreeSet<String>,
    alias_map: &BTreeMap<&'static str, Vec<&'static str>>,
) -> BTreeSet<String> {
    let bridge = read_bridge();
    symbols
        .iter()
        .filter(|name| {
            contains_symbol(&bridge, name)
                || alias_map
                    .get(name.as_str())
                    .is_some_and(|alts| alts.iter().any(|alt| contains_symbol(&bridge, alt)))
        })
        .cloned()
        .collect()
}

fn report(
    name: &str,
    apple: &BTreeSet<String>,
    ours: &BTreeSet<String>,
    omitted: &BTreeSet<String>,
) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|symbol| !omitted.contains(*symbol))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for symbol in &missing {
            println!("  - {symbol}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

fn coverage(type_name: &str, header_name: &str, omitted: &BTreeSet<String>) {
    let header = read_header(header_name);
    let body = extract_interfaces(&header, type_name);
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple, &aliases());
    report(type_name, &apple, &ours, omitted);
}

#[test]
fn gc_controller_coverage() {
    let omitted = omitted_set([
        "controllerPausedHandler",
        "supportsHIDDevice",
        "snapshot",
        "isSnapshot",
        "capture",
        "controllerWithMicroGamepad",
        "controllerWithExtendedGamepad",
    ]);
    coverage("GCController", "GCController", &omitted);
}

#[test]
fn gc_controller_input_state_coverage() {
    let omitted = omitted_set([]);
    coverage("GCControllerInputState", "GCControllerInput", &omitted);
}

#[test]
fn gc_controller_live_input_coverage() {
    let omitted = omitted_set([]);
    coverage("GCControllerLiveInput", "GCControllerInput", &omitted);
}

#[test]
fn gc_keyboard_coverage() {
    let omitted = omitted_set([]);
    coverage("GCKeyboard", "GCKeyboard", &omitted);
}

#[test]
fn gc_keyboard_input_coverage() {
    let omitted = omitted_set(["keyChangedHandler"]);
    coverage("GCKeyboardInput", "GCKeyboardInput", &omitted);
}

#[test]
fn gc_mouse_coverage() {
    let omitted = omitted_set([]);
    coverage("GCMouse", "GCMouse", &omitted);
}

#[test]
fn gc_mouse_input_coverage() {
    let omitted = omitted_set(["mouseMovedHandler"]);
    coverage("GCMouseInput", "GCMouseInput", &omitted);
}

#[test]
fn gc_physical_input_profile_coverage() {
    let omitted = omitted_set(["valueDidChangeHandler", "setStateFromPhysicalInput"]);
    coverage("GCPhysicalInputProfile", "GCPhysicalInputProfile", &omitted);
}

#[test]
fn gc_gamepad_coverage() {
    let omitted = omitted_set(["valueChangedHandler", "saveSnapshot"]);
    coverage("GCGamepad", "GCGamepad", &omitted);
}

#[test]
fn gc_micro_gamepad_coverage() {
    let omitted = omitted_set([
        "valueChangedHandler",
        "saveSnapshot",
        "setStateFromMicroGamepad",
    ]);
    coverage("GCMicroGamepad", "GCMicroGamepad", &omitted);
}

#[test]
fn gc_directional_gamepad_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDirectionalGamepad", "GCDirectionalGamepad", &omitted);
}

#[test]
fn gc_extended_gamepad_coverage() {
    let omitted = omitted_set([
        "valueChangedHandler",
        "saveSnapshot",
        "setStateFromExtendedGamepad",
    ]);
    coverage("GCExtendedGamepad", "GCExtendedGamepad", &omitted);
}

#[test]
fn gc_dualshock_gamepad_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDualShockGamepad", "GCDualShockGamepad", &omitted);
}

#[test]
fn gc_dualsense_gamepad_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDualSenseGamepad", "GCDualSenseGamepad", &omitted);
}

#[test]
fn gc_xbox_gamepad_coverage() {
    let omitted = omitted_set([]);
    coverage("GCXboxGamepad", "GCXboxGamepad", &omitted);
}

#[test]
fn gc_dualsense_adaptive_trigger_coverage() {
    let omitted = omitted_set([]);
    coverage(
        "GCDualSenseAdaptiveTrigger",
        "GCDualSenseAdaptiveTrigger",
        &omitted,
    );
}

#[test]
fn gc_motion_coverage() {
    let omitted = omitted_set([
        "valueChangedHandler",
        "hasAttitudeAndRotationRate",
        "setGravity",
        "setUserAcceleration",
        "setAcceleration",
        "setAttitude",
        "setRotationRate",
        "setStateFromMotion",
    ]);
    coverage("GCMotion", "GCMotion", &omitted);
}

#[test]
fn gc_device_battery_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDeviceBattery", "GCDeviceBattery", &omitted);
}

#[test]
fn gc_device_haptics_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDeviceHaptics", "GCDeviceHaptics", &omitted);
}

#[test]
fn gc_device_light_coverage() {
    let omitted = omitted_set([]);
    coverage("GCDeviceLight", "GCDeviceLight", &omitted);
}

#[test]
fn gc_color_coverage() {
    let omitted = omitted_set([]);
    coverage("GCColor", "GCColor", &omitted);
}

#[test]
fn gc_event_view_controller_coverage() {
    let omitted = omitted_set([]);
    coverage("GCEventViewController", "GCEventViewController", &omitted);
}

#[test]
fn gc_racing_wheel_coverage() {
    let omitted = omitted_set(["acquireDeviceWithError", "relinquishDevice"]);
    coverage("GCRacingWheel", "GCRacingWheel", &omitted);
}

#[test]
fn gc_racing_wheel_input_state_coverage() {
    let omitted = omitted_set([]);
    coverage("GCRacingWheelInputState", "GCRacingWheelInput", &omitted);
}

#[test]
fn gc_racing_wheel_input_coverage() {
    let omitted = omitted_set(["nextInputState"]);
    coverage("GCRacingWheelInput", "GCRacingWheelInput", &omitted);
}

#[test]
fn gc_device_coverage() {
    let omitted = omitted_set(["physicalInputProfile"]);
    coverage("GCDevice", "GCDevice", &omitted);
}

#[test]
fn gc_controller_element_coverage() {
    let omitted = omitted_set([]);
    coverage("GCControllerElement", "GCControllerElement", &omitted);
}

#[test]
fn gc_axis_input_coverage() {
    let omitted = omitted_set(["valueDidChangeHandler"]);
    coverage("GCAxisInput", "GCAxisInput", &omitted);
}

#[test]
fn gc_axis2d_input_coverage() {
    let omitted = omitted_set(["valueDidChangeHandler"]);
    coverage("GCAxis2DInput", "GCAxis2DInput", &omitted);
}

#[test]
fn gc_physical_input_source_coverage() {
    let omitted = omitted_set([]);
    coverage("GCPhysicalInputSource", "GCPhysicalInputSource", &omitted);
}

#[test]
fn gc_physical_input_extents_coverage() {
    let omitted = omitted_set([]);
    coverage("GCPhysicalInputExtents", "GCPhysicalInputExtents", &omitted);
}
