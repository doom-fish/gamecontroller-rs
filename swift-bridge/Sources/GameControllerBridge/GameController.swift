// GameController Bridge
//
// @_cdecl wrappers around Apple's GameController framework. v0.1 surface:
// enumerate connected controllers + snapshot the extendedGamepad's
// button/stick/trigger state.

import Foundation
import GameController

private func ffiString(_ s: String?) -> UnsafeMutablePointer<CChar>? {
    guard let s = s else { return nil }
    return strdup(s)
}

@_cdecl("gc_string_free")
public func gc_string_free(_ s: UnsafeMutablePointer<CChar>?) {
    guard let s = s else { return }
    free(s)
}

// Layout-compatible structs (mirror src/ffi/mod.rs)

public struct GCControllerInfoRaw {
    public var vendor_name: UnsafeMutablePointer<CChar>?
    public var product_category: UnsafeMutablePointer<CChar>?
    public var player_index: Int32
    public var is_attached_to_device: Bool
    public var has_extended_gamepad: Bool

    public var button_a: Float
    public var button_b: Float
    public var button_x: Float
    public var button_y: Float

    public var left_shoulder: Float
    public var right_shoulder: Float
    public var left_trigger: Float
    public var right_trigger: Float

    public var menu_button: Float
    public var options_button: Float
    public var home_button: Float

    public var left_thumbstick_x: Float
    public var left_thumbstick_y: Float
    public var right_thumbstick_x: Float
    public var right_thumbstick_y: Float

    public var dpad_up: Float
    public var dpad_down: Float
    public var dpad_left: Float
    public var dpad_right: Float
}

private func snapshot(_ c: GCController) -> GCControllerInfoRaw {
    var info = GCControllerInfoRaw(
        vendor_name: ffiString(c.vendorName ?? "Unknown"),
        product_category: ffiString(c.productCategory),
        player_index: Int32(c.playerIndex.rawValue),
        is_attached_to_device: c.isAttachedToDevice,
        has_extended_gamepad: c.extendedGamepad != nil,
        button_a: 0, button_b: 0, button_x: 0, button_y: 0,
        left_shoulder: 0, right_shoulder: 0, left_trigger: 0, right_trigger: 0,
        menu_button: 0, options_button: 0, home_button: 0,
        left_thumbstick_x: 0, left_thumbstick_y: 0,
        right_thumbstick_x: 0, right_thumbstick_y: 0,
        dpad_up: 0, dpad_down: 0, dpad_left: 0, dpad_right: 0
    )
    if let g = c.extendedGamepad {
        info.button_a = g.buttonA.value
        info.button_b = g.buttonB.value
        info.button_x = g.buttonX.value
        info.button_y = g.buttonY.value
        info.left_shoulder = g.leftShoulder.value
        info.right_shoulder = g.rightShoulder.value
        info.left_trigger = g.leftTrigger.value
        info.right_trigger = g.rightTrigger.value
        info.menu_button = g.buttonMenu.value
        if #available(macOS 11.0, *) {
            info.options_button = g.buttonOptions?.value ?? 0
            info.home_button = g.buttonHome?.value ?? 0
        }
        info.left_thumbstick_x = g.leftThumbstick.xAxis.value
        info.left_thumbstick_y = g.leftThumbstick.yAxis.value
        info.right_thumbstick_x = g.rightThumbstick.xAxis.value
        info.right_thumbstick_y = g.rightThumbstick.yAxis.value
        info.dpad_up = g.dpad.up.value
        info.dpad_down = g.dpad.down.value
        info.dpad_left = g.dpad.left.value
        info.dpad_right = g.dpad.right.value
    }
    return info
}

@_cdecl("gc_connected_controllers")
public func gc_connected_controllers(
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let controllers = GCController.controllers()
    if controllers.isEmpty {
        outArray.pointee = nil
        outCount.pointee = 0
        return 0
    }
    let buffer = UnsafeMutablePointer<GCControllerInfoRaw>.allocate(capacity: controllers.count)
    for (i, c) in controllers.enumerated() {
        buffer.advanced(by: i).initialize(to: snapshot(c))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = controllers.count
    return 0
}

@_cdecl("gc_controller_infos_free")
public func gc_controller_infos_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array = array else { return }
    let typed = array.assumingMemoryBound(to: GCControllerInfoRaw.self)
    for i in 0..<count {
        let info = typed.advanced(by: i).pointee
        if let p = info.vendor_name { free(p) }
        if let p = info.product_category { free(p) }
    }
    typed.deallocate()
}
