//! API-surface coverage harness for `gamecontroller`.
//!
//! `GameController` is an Obj-C / Swift framework. v0.1 wraps a *focused
//! subset*: enumeration + extendedGamepad polling. The full framework
//! has 60+ headers covering haptics, light bars, motion sensors, Apple
//! Pencil, etc. — those land in v0.2+.
//!
//! This harness verifies the surface we DO wrap is referenced from the
//! Swift bridge (header-based, Obj-C `@interface` parsing — same
//! pattern as speech-rs / apple-vision / avassetwriter).

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
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
    read(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "swift-bridge/Sources/GameControllerBridge/GameController.swift",
    ))
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/GameController.framework/Headers/{name}.h"
    )))
}

fn extract_interface(header: &str, type_name: &str) -> String {
    let needle = regex_lite::Regex::new(&format!(r"@interface\s+{type_name}\b")).unwrap();
    let Some(start) = needle.find(header) else {
        return String::new();
    };
    let rest = &header[start.start()..];
    let Some(end_off) = rest.find("@end") else {
        return rest.to_string();
    };
    rest[..end_off].to_string()
}

fn extract_member_surface(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let method_re =
        regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in method_re.captures_iter(body) {
        out.insert(c[1].to_string());
    }
    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:NS_|API_|;)",
    )
    .unwrap();
    for c in prop_re.captures_iter(body) {
        out.insert(c[1].to_string());
    }
    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in getter_re.captures_iter(body) {
        out.insert(c[1].to_string());
    }
    out
}

fn references_in_bridge(symbols: &BTreeSet<String>) -> BTreeSet<String> {
    let bridge = read_bridge();
    symbols
        .iter()
        .filter(|name| {
            let pattern = format!(r"\b{}\b", regex_lite::escape(name));
            regex_lite::Regex::new(&pattern).unwrap().is_match(&bridge)
        })
        .cloned()
        .collect()
}

fn report(name: &str, apple: &BTreeSet<String>, ours: &BTreeSet<String>, omitted: &BTreeSet<String>) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|s| !omitted.contains(*s))
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
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

#[test]
fn gc_controller_coverage() {
    let header = read_header("GCController");
    let body = extract_interface(&header, "GCController");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // Connect/disconnect callbacks — v0.2 needs run-loop integration.
        "controllerPausedHandler",
        "shouldMonitorBackgroundEvents",
        "startWirelessControllerDiscoveryWithCompletionHandler",
        "stopWirelessControllerDiscovery",
        "controllers",
        "current",
        // Modern unified `input` / battery / motion / haptics / light /
        // microGamepad / DualSense — v0.2+.
        "input",
        "physicalInputProfile",
        "battery",
        "motion",
        "light",
        "haptics",
        "microGamepad",
        "gamepad",
        // HID device interop — would shadow iohidmanager-rs; skip in v0.1.
        "supportsHIDDevice",
        // The `attachedToDevice` getter is exposed via the `is_attached_to_device`
        // bridge field; the bare property name doesn't appear in the
        // bridge text since Swift uses the `getter=isAttachedToDevice` form.
        "attachedToDevice",
    ]);
    report("GCController", &apple, &ours, &omitted);
}
