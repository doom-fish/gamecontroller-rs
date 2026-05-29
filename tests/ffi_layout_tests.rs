//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift bridge.
//!
//! `ControllerInfoRaw` and `ExtraInfoRaw` are read directly from pointers handed
//! across the Rust <-> Swift `@_cdecl` FFI boundary. If their size or alignment
//! ever drifts from what the Swift side (`GCControllerInfoRaw` / `GCExtraInfoRaw`
//! in `Core.swift`) expects, the data marshalling silently corrupts. These tests
//! pin the layout so accidental field reordering / type changes are caught at
//! `cargo test` time rather than as runtime garbage.

use std::mem::{align_of, size_of};

use gamecontroller::ffi::{verify_ffi_layout, ControllerInfoRaw, ExtraInfoRaw};

#[test]
fn controller_info_raw_layout() {
    assert_eq!(
        size_of::<ControllerInfoRaw>(),
        104,
        "ControllerInfoRaw size drifted"
    );
    assert_eq!(
        align_of::<ControllerInfoRaw>(),
        8,
        "ControllerInfoRaw alignment drifted"
    );
}

#[test]
fn extra_info_raw_layout() {
    assert_eq!(size_of::<ExtraInfoRaw>(), 64, "ExtraInfoRaw size drifted");
    assert_eq!(
        align_of::<ExtraInfoRaw>(),
        8,
        "ExtraInfoRaw alignment drifted"
    );
}

#[test]
fn ffi_layout_verified() {
    assert!(
        verify_ffi_layout(),
        "FFI struct layout disagrees with the pinned ABI"
    );
}
