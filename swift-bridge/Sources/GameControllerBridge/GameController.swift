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

// MARK: - v0.2: motion / battery / light / haptics quick reads

/// One-shot read of the first connected controller's optional services.
@frozen
public struct GCExtraInfoRaw {
    public var has_motion: Bool
    public var has_haptics: Bool
    public var has_light: Bool
    public var has_battery: Bool
    /// Battery level in 0.0...1.0, or -1 if no battery.
    public var battery_level: Float
    /// 0 = unknown, 1 = discharging, 2 = charging, 3 = full.
    public var battery_state: Int32
    /// Motion gravity vector (x, y, z) in g-units, or 0 if no motion.
    public var gravity_x: Double
    public var gravity_y: Double
    public var gravity_z: Double
    public var user_acceleration_x: Double
    public var user_acceleration_y: Double
    public var user_acceleration_z: Double
}

@_cdecl("gc_first_controller_extra")
public func gc_first_controller_extra(
    _ outInfo: UnsafeMutableRawPointer
) -> Bool {
    guard let c = GCController.controllers().first else {
        return false
    }
    var info = GCExtraInfoRaw(
        has_motion: false, has_haptics: false, has_light: false, has_battery: false,
        battery_level: -1, battery_state: 0,
        gravity_x: 0, gravity_y: 0, gravity_z: 0,
        user_acceleration_x: 0, user_acceleration_y: 0, user_acceleration_z: 0
    )
    if let m = c.motion {
        info.has_motion = true
        info.gravity_x = m.gravity.x
        info.gravity_y = m.gravity.y
        info.gravity_z = m.gravity.z
        info.user_acceleration_x = m.userAcceleration.x
        info.user_acceleration_y = m.userAcceleration.y
        info.user_acceleration_z = m.userAcceleration.z
    }
    if #available(macOS 11.0, *) {
        if c.haptics != nil { info.has_haptics = true }
        if c.light != nil { info.has_light = true }
        if let b = c.battery {
            info.has_battery = true
            info.battery_level = b.batteryLevel
            switch b.batteryState {
            case .unknown:     info.battery_state = 0
            case .discharging: info.battery_state = 1
            case .charging:    info.battery_state = 2
            case .full:        info.battery_state = 3
            @unknown default:  info.battery_state = 0
            }
        }
    }
    outInfo.assumingMemoryBound(to: GCExtraInfoRaw.self).pointee = info
    return true
}

// MARK: - v0.2: connect/disconnect callbacks

/// Callback the Rust side registers. Called when a controller is plugged
/// in (`connected = true`) or unplugged (`connected = false`). The pointer
/// passed is the same `userInfo` you registered the callback with.
public typealias GCNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void

private final class NotifyState {
    let callback: GCNotificationCallback
    let userInfo: UnsafeMutableRawPointer?
    var connectObserver: NSObjectProtocol?
    var disconnectObserver: NSObjectProtocol?
    init(callback: @escaping GCNotificationCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
    }
}

private var notifyStates: [UnsafeMutableRawPointer: NotifyState] = [:]

@_cdecl("gc_register_connection_callback")
public func gc_register_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer {
    let state = NotifyState(callback: callback, userInfo: userInfo)
    let nc = NotificationCenter.default
    state.connectObserver = nc.addObserver(
        forName: .GCControllerDidConnect,
        object: nil,
        queue: .main
    ) { _ in
        state.callback(state.userInfo, true)
    }
    state.disconnectObserver = nc.addObserver(
        forName: .GCControllerDidDisconnect,
        object: nil,
        queue: .main
    ) { _ in
        state.callback(state.userInfo, false)
    }
    let key = Unmanaged.passRetained(state).toOpaque()
    notifyStates[key] = state
    return key
}

@_cdecl("gc_unregister_connection_callback")
public func gc_unregister_connection_callback(_ token: UnsafeMutableRawPointer?) {
    guard let token = token, let state = notifyStates.removeValue(forKey: token) else {
        return
    }
    let nc = NotificationCenter.default
    if let o = state.connectObserver { nc.removeObserver(o) }
    if let o = state.disconnectObserver { nc.removeObserver(o) }
    Unmanaged<NotifyState>.fromOpaque(token).release()
}

// MARK: - Light + simple haptics (v0.3)

@_cdecl("gc_first_controller_set_light")
public func gc_first_controller_set_light(
    _ red: Float, _ green: Float, _ blue: Float
) -> Bool {
    guard let c = GCController.controllers().first else { return false }
    guard let light = c.light else { return false }
    light.color = GCColor(red: red, green: green, blue: blue)
    return true
}

@_cdecl("gc_first_controller_set_player_index")
public func gc_first_controller_set_player_index(_ index: Int32) -> Bool {
    guard let c = GCController.controllers().first else { return false }
    if let pi = GCControllerPlayerIndex(rawValue: Int(index)) {
        c.playerIndex = pi
        return true
    }
    return false
}

@_cdecl("gc_first_controller_battery_level")
public func gc_first_controller_battery_level() -> Float {
    guard let c = GCController.controllers().first, let b = c.battery else { return -1 }
    return b.batteryLevel
}

#if canImport(CoreHaptics)
import CoreHaptics

private var hapticEngines: [ObjectIdentifier: CHHapticEngine] = [:]

@_cdecl("gc_first_controller_rumble")
public func gc_first_controller_rumble(
    _ intensity: Float, _ sharpness: Float, _ duration: Double
) -> Bool {
    guard let c = GCController.controllers().first else { return false }
    guard let haptics = c.haptics else { return false }
    guard let engine = haptics.createEngine(withLocality: .default) else { return false }
    do {
        try engine.start()
    } catch {
        return false
    }
    let intensityParam = CHHapticEventParameter(
        parameterID: .hapticIntensity,
        value: max(0, min(1, intensity))
    )
    let sharpnessParam = CHHapticEventParameter(
        parameterID: .hapticSharpness,
        value: max(0, min(1, sharpness))
    )
    let event = CHHapticEvent(
        eventType: .hapticContinuous,
        parameters: [intensityParam, sharpnessParam],
        relativeTime: 0,
        duration: max(0.01, duration)
    )
    do {
        let pattern = try CHHapticPattern(events: [event], parameters: [])
        let player = try engine.makePlayer(with: pattern)
        try player.start(atTime: 0)
        hapticEngines[ObjectIdentifier(engine)] = engine
        DispatchQueue.main.asyncAfter(deadline: .now() + duration + 0.1) {
            engine.stop(completionHandler: nil)
            hapticEngines.removeValue(forKey: ObjectIdentifier(engine))
        }
        return true
    } catch {
        engine.stop(completionHandler: nil)
        return false
    }
}
#else
@_cdecl("gc_first_controller_rumble")
public func gc_first_controller_rumble(
    _ intensity: Float, _ sharpness: Float, _ duration: Double
) -> Bool {
    return false
}
#endif

// MARK: - Mouse + Keyboard + multi-controller iteration (v0.4)

@_cdecl("gc_mouse_is_connected")
public func gc_mouse_is_connected() -> Bool {
    if #unavailable(macOS 11.0) { return false }
    return GCMouse.current != nil
}

@_cdecl("gc_mouse_button_states")
public func gc_mouse_button_states(
    _ out_left: UnsafeMutablePointer<Bool>,
    _ out_right: UnsafeMutablePointer<Bool>,
    _ out_middle: UnsafeMutablePointer<Bool>
) -> Bool {
    if #unavailable(macOS 11.0) { return false }
    guard let mouse = GCMouse.current, let input = mouse.mouseInput else { return false }
    out_left.pointee = input.leftButton.isPressed
    out_right.pointee = input.rightButton?.isPressed ?? false
    out_middle.pointee = input.middleButton?.isPressed ?? false
    return true
}

@_cdecl("gc_keyboard_is_connected")
public func gc_keyboard_is_connected() -> Bool {
    if #unavailable(macOS 11.0) { return false }
    return GCKeyboard.coalesced != nil
}

@_cdecl("gc_keyboard_any_key_pressed")
public func gc_keyboard_any_key_pressed() -> Bool {
    if #unavailable(macOS 11.0) { return false }
    guard let kb = GCKeyboard.coalesced, let input = kb.keyboardInput else { return false }
    return input.isAnyKeyPressed
}

/// `keycode` is a `GCKeyCode.rawValue` (HID page-7 usage code).
/// E.g. `4 = "a"`, `40 = enter`, `44 = space`.
@_cdecl("gc_keyboard_is_key_pressed")
public func gc_keyboard_is_key_pressed(_ keycode: Int) -> Bool {
    if #unavailable(macOS 11.0) { return false }
    guard let kb = GCKeyboard.coalesced, let input = kb.keyboardInput else { return false }
    let code = GCKeyCode(rawValue: keycode)
    return input.button(forKeyCode: code)?.isPressed ?? false
}

/// Snapshot extras for ALL connected controllers into an
/// already-allocated buffer of `max` `GCExtraInfoRaw` slots. Returns
/// the actual number written.
@_cdecl("gc_all_controllers_extras")
public func gc_all_controllers_extras(
    _ out_buf: UnsafeMutableRawPointer,
    _ max: Int
) -> Int {
    let controllers = GCController.controllers()
    let n = min(controllers.count, max)
    let typed = out_buf.assumingMemoryBound(to: GCExtraInfoRaw.self)
    for i in 0..<n {
        let c = controllers[i]
        var info = GCExtraInfoRaw(
            has_motion: false,
            has_haptics: false,
            has_light: false,
            has_battery: false,
            battery_level: -1.0,
            battery_state: 0,
            gravity_x: 0.0,
            gravity_y: 0.0,
            gravity_z: 0.0,
            user_acceleration_x: 0.0,
            user_acceleration_y: 0.0,
            user_acceleration_z: 0.0
        )
        if let b = c.battery {
            info.has_battery = true
            info.battery_state = Int32(b.batteryState.rawValue)
            info.battery_level = b.batteryLevel
        }
        if c.haptics != nil { info.has_haptics = true }
        if c.light != nil { info.has_light = true }
        if let m = c.motion {
            info.has_motion = true
            info.gravity_x = m.gravity.x
            info.gravity_y = m.gravity.y
            info.gravity_z = m.gravity.z
            info.user_acceleration_x = m.userAcceleration.x
            info.user_acceleration_y = m.userAcceleration.y
            info.user_acceleration_z = m.userAcceleration.z
        }
        typed.advanced(by: i).initialize(to: info)
    }
    return n
}
