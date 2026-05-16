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

// MARK: - Rich controller/profile snapshots + discovery (v0.6)

public typealias GCDiscoveryCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class DiscoveryState {
    let callback: GCDiscoveryCallback?
    let userInfo: UnsafeMutableRawPointer?

    init(callback: GCDiscoveryCallback?, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
    }

    func complete() {
        callback?(userInfo)
    }
}

private var discoveryState: DiscoveryState?

private struct ButtonInputStatePayload: Codable {
    let value: Float
    let pressed: Bool
    let touched: Bool
}

private struct AxisInputStatePayload: Codable {
    let value: Float
}

private struct DirectionPadInputStatePayload: Codable {
    let x: Float
    let y: Float
    let up: ButtonInputStatePayload
    let down: ButtonInputStatePayload
    let left: ButtonInputStatePayload
    let right: ButtonInputStatePayload
}

private struct TouchpadDetailsPayload: Codable {
    let button: ButtonInputStatePayload
    let touchSurface: DirectionPadInputStatePayload
    let touchState: String
    let reportsAbsoluteTouchSurfaceValues: Bool
}

private struct GamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonB: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonY: ButtonInputStatePayload
    let leftShoulder: ButtonInputStatePayload
    let rightShoulder: ButtonInputStatePayload
}

private struct MicroGamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonMenu: ButtonInputStatePayload?
    let reportsAbsoluteDpadValues: Bool
    let allowsRotation: Bool
}

private struct ExtendedGamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonB: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonY: ButtonInputStatePayload
    let buttonMenu: ButtonInputStatePayload?
    let buttonOptions: ButtonInputStatePayload?
    let buttonHome: ButtonInputStatePayload?
    let leftThumbstick: DirectionPadInputStatePayload
    let rightThumbstick: DirectionPadInputStatePayload
    let leftShoulder: ButtonInputStatePayload
    let rightShoulder: ButtonInputStatePayload
    let leftTrigger: ButtonInputStatePayload
    let rightTrigger: ButtonInputStatePayload
    let leftThumbstickButton: ButtonInputStatePayload?
    let rightThumbstickButton: ButtonInputStatePayload?
}

private struct DualSenseAdaptiveTriggerStatePayload: Codable {
    let value: Float
    let pressed: Bool
    let touched: Bool
    let mode: String
    let status: String
    let armPosition: Float
}

private struct DualSenseGamepadDetailsPayload: Codable {
    let touchpadButton: ButtonInputStatePayload
    let touchpadPrimary: DirectionPadInputStatePayload
    let touchpadSecondary: DirectionPadInputStatePayload
    let leftTrigger: DualSenseAdaptiveTriggerStatePayload
    let rightTrigger: DualSenseAdaptiveTriggerStatePayload
}

private struct BatteryInfoPayload: Codable {
    let level: Float
    let state: String
}

private struct Vector3Payload: Codable {
    let x: Double
    let y: Double
    let z: Double
}

private struct QuaternionPayload: Codable {
    let x: Double
    let y: Double
    let z: Double
    let w: Double
}

private struct MotionDetailsPayload: Codable {
    let sensorsRequireManualActivation: Bool
    let sensorsActive: Bool
    let hasGravityAndUserAcceleration: Bool
    let gravity: Vector3Payload
    let userAcceleration: Vector3Payload
    let acceleration: Vector3Payload
    let hasAttitude: Bool
    let hasRotationRate: Bool
    let attitude: QuaternionPayload
    let rotationRate: Vector3Payload
}

private struct NamedButtonInputStatePayload: Codable {
    let alias: String
    let value: ButtonInputStatePayload
}

private struct NamedAxisInputStatePayload: Codable {
    let alias: String
    let value: AxisInputStatePayload
}

private struct NamedDirectionPadStatePayload: Codable {
    let alias: String
    let value: DirectionPadInputStatePayload
}

private struct NamedTouchpadStatePayload: Codable {
    let alias: String
    let value: TouchpadDetailsPayload
}

private struct PhysicalInputProfileDetailsPayload: Codable {
    let lastEventTimestamp: Double
    let hasRemappedElements: Bool
    let elementAliases: [String]
    let buttonAliases: [String]
    let axisAliases: [String]
    let dpadAliases: [String]
    let touchpadAliases: [String]
    let buttons: [NamedButtonInputStatePayload]
    let axes: [NamedAxisInputStatePayload]
    let dpads: [NamedDirectionPadStatePayload]
    let touchpads: [NamedTouchpadStatePayload]
}

private struct ControllerDetailsPayload: Codable {
    let vendorName: String
    let productCategory: String
    let playerIndex: Int32
    let isAttachedToDevice: Bool
    let isCurrent: Bool
    let supportsBackgroundEvents: Bool
    let hasLiveInput: Bool
    let gamepad: GamepadDetailsPayload?
    let microGamepad: MicroGamepadDetailsPayload?
    let extendedGamepad: ExtendedGamepadDetailsPayload?
    let dualSense: DualSenseGamepadDetailsPayload?
    let battery: BatteryInfoPayload?
    let motion: MotionDetailsPayload?
    let hasLight: Bool
    let hasHaptics: Bool
    let physicalInput: PhysicalInputProfileDetailsPayload?
}

private func jsonString<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()
    guard let data = try? encoder.encode(value),
          let string = String(data: data, encoding: .utf8) else {
        return nil
    }
    return ffiString(string)
}

private func clampUnit(_ value: Float) -> Float {
    max(0, min(1, value))
}

@available(macOS 12.3, *)
private func dualSenseStepIndex(for position: Float) -> Int {
    let maxIndex = max(0, GCDualSenseAdaptiveTrigger.discretePositionCount - 1)
    return Int((Double(clampUnit(position)) * Double(maxIndex)).rounded())
}

@available(macOS 12.3, *)
private func makeResistiveStrengths(_ values: [Float]) -> GCDualSenseAdaptiveTrigger.PositionalResistiveStrengths {
    var strengths = GCDualSenseAdaptiveTrigger.PositionalResistiveStrengths()
    strengths.values = (
        clampUnit(values[0]), clampUnit(values[1]), clampUnit(values[2]), clampUnit(values[3]), clampUnit(values[4]),
        clampUnit(values[5]), clampUnit(values[6]), clampUnit(values[7]), clampUnit(values[8]), clampUnit(values[9])
    )
    return strengths
}

@available(macOS 12.3, *)
private func makeAmplitudes(_ values: [Float]) -> GCDualSenseAdaptiveTrigger.PositionalAmplitudes {
    var amplitudes = GCDualSenseAdaptiveTrigger.PositionalAmplitudes()
    amplitudes.values = (
        clampUnit(values[0]), clampUnit(values[1]), clampUnit(values[2]), clampUnit(values[3]), clampUnit(values[4]),
        clampUnit(values[5]), clampUnit(values[6]), clampUnit(values[7]), clampUnit(values[8]), clampUnit(values[9])
    )
    return amplitudes
}

private func buttonPayload(_ button: GCControllerButtonInput) -> ButtonInputStatePayload {
    ButtonInputStatePayload(value: button.value, pressed: button.isPressed, touched: button.isTouched)
}

private func axisPayload(_ axis: GCControllerAxisInput) -> AxisInputStatePayload {
    AxisInputStatePayload(value: axis.value)
}

private func directionPadPayload(_ dpad: GCControllerDirectionPad) -> DirectionPadInputStatePayload {
    DirectionPadInputStatePayload(
        x: dpad.xAxis.value,
        y: dpad.yAxis.value,
        up: buttonPayload(dpad.up),
        down: buttonPayload(dpad.down),
        left: buttonPayload(dpad.left),
        right: buttonPayload(dpad.right)
    )
}

private func touchStateName(_ state: GCControllerTouchpad.TouchState) -> String {
    switch state {
    case .up: return "up"
    case .down: return "down"
    case .moving: return "moving"
    @unknown default: return "up"
    }
}

private func touchpadPayload(_ touchpad: GCControllerTouchpad) -> TouchpadDetailsPayload {
    TouchpadDetailsPayload(
        button: buttonPayload(touchpad.button),
        touchSurface: directionPadPayload(touchpad.touchSurface),
        touchState: touchStateName(touchpad.touchState),
        reportsAbsoluteTouchSurfaceValues: touchpad.reportsAbsoluteTouchSurfaceValues
    )
}

private func batteryStateName(_ battery: GCDeviceBattery) -> String {
    switch battery.batteryState {
    case .unknown: return "unknown"
    case .discharging: return "discharging"
    case .charging: return "charging"
    case .full: return "full"
    @unknown default: return "unknown"
    }
}

@available(macOS 11.3, *)
private func dualSenseModeName(_ mode: GCDualSenseAdaptiveTrigger.Mode) -> String {
    switch mode.rawValue {
    case 0: return "off"
    case 1: return "feedback"
    case 2: return "weapon"
    case 3: return "vibration"
    case 4: return "slope_feedback"
    default: return "off"
    }
}

@available(macOS 11.3, *)
private func dualSenseStatusName(_ status: GCDualSenseAdaptiveTrigger.Status) -> String {
    switch status.rawValue {
    case -1: return "unknown"
    case 0: return "feedback_no_load"
    case 1: return "feedback_load_applied"
    case 2: return "weapon_ready"
    case 3: return "weapon_firing"
    case 4: return "weapon_fired"
    case 5: return "vibration_not_vibrating"
    case 6: return "vibration_is_vibrating"
    case 7: return "slope_feedback_ready"
    case 8: return "slope_feedback_applying_load"
    case 9: return "slope_feedback_finished"
    default: return "unknown"
    }
}

@available(macOS 11.3, *)
private func dualSenseTriggerPayload(_ trigger: GCDualSenseAdaptiveTrigger) -> DualSenseAdaptiveTriggerStatePayload {
    DualSenseAdaptiveTriggerStatePayload(
        value: trigger.value,
        pressed: trigger.isPressed,
        touched: trigger.isTouched,
        mode: dualSenseModeName(trigger.mode),
        status: dualSenseStatusName(trigger.status),
        armPosition: trigger.armPosition
    )
}

private func gamepadPayload(_ gamepad: GCGamepad) -> GamepadDetailsPayload {
    _ = gamepad.controller
    return GamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: buttonPayload(gamepad.buttonA),
        buttonB: buttonPayload(gamepad.buttonB),
        buttonX: buttonPayload(gamepad.buttonX),
        buttonY: buttonPayload(gamepad.buttonY),
        leftShoulder: buttonPayload(gamepad.leftShoulder),
        rightShoulder: buttonPayload(gamepad.rightShoulder)
    )
}

private func microGamepadPayload(_ microGamepad: GCMicroGamepad) -> MicroGamepadDetailsPayload {
    _ = microGamepad.controller
    return MicroGamepadDetailsPayload(
        dpad: directionPadPayload(microGamepad.dpad),
        buttonA: buttonPayload(microGamepad.buttonA),
        buttonX: buttonPayload(microGamepad.buttonX),
        buttonMenu: buttonPayload(microGamepad.buttonMenu),
        reportsAbsoluteDpadValues: microGamepad.reportsAbsoluteDpadValues,
        allowsRotation: microGamepad.allowsRotation
    )
}

private func extendedGamepadPayload(_ gamepad: GCExtendedGamepad) -> ExtendedGamepadDetailsPayload {
    _ = gamepad.controller
    let buttonHome: ButtonInputStatePayload?
    if #available(macOS 11.0, *) {
        buttonHome = gamepad.buttonHome.map(buttonPayload)
    } else {
        buttonHome = nil
    }

    return ExtendedGamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: buttonPayload(gamepad.buttonA),
        buttonB: buttonPayload(gamepad.buttonB),
        buttonX: buttonPayload(gamepad.buttonX),
        buttonY: buttonPayload(gamepad.buttonY),
        buttonMenu: buttonPayload(gamepad.buttonMenu),
        buttonOptions: gamepad.buttonOptions.map(buttonPayload),
        buttonHome: buttonHome,
        leftThumbstick: directionPadPayload(gamepad.leftThumbstick),
        rightThumbstick: directionPadPayload(gamepad.rightThumbstick),
        leftShoulder: buttonPayload(gamepad.leftShoulder),
        rightShoulder: buttonPayload(gamepad.rightShoulder),
        leftTrigger: buttonPayload(gamepad.leftTrigger),
        rightTrigger: buttonPayload(gamepad.rightTrigger),
        leftThumbstickButton: gamepad.leftThumbstickButton.map(buttonPayload),
        rightThumbstickButton: gamepad.rightThumbstickButton.map(buttonPayload)
    )
}

@available(macOS 11.3, *)
private func dualSensePayload(_ gamepad: GCDualSenseGamepad) -> DualSenseGamepadDetailsPayload {
    DualSenseGamepadDetailsPayload(
        touchpadButton: buttonPayload(gamepad.touchpadButton),
        touchpadPrimary: directionPadPayload(gamepad.touchpadPrimary),
        touchpadSecondary: directionPadPayload(gamepad.touchpadSecondary),
        leftTrigger: dualSenseTriggerPayload(gamepad.leftTrigger),
        rightTrigger: dualSenseTriggerPayload(gamepad.rightTrigger)
    )
}

@available(macOS 11.0, *)
private func motionPayload(_ motion: GCMotion) -> MotionDetailsPayload {
    _ = motion.controller
    return MotionDetailsPayload(
        sensorsRequireManualActivation: motion.sensorsRequireManualActivation,
        sensorsActive: motion.sensorsActive,
        hasGravityAndUserAcceleration: motion.hasGravityAndUserAcceleration,
        gravity: Vector3Payload(x: motion.gravity.x, y: motion.gravity.y, z: motion.gravity.z),
        userAcceleration: Vector3Payload(x: motion.userAcceleration.x, y: motion.userAcceleration.y, z: motion.userAcceleration.z),
        acceleration: Vector3Payload(x: motion.acceleration.x, y: motion.acceleration.y, z: motion.acceleration.z),
        hasAttitude: motion.hasAttitude,
        hasRotationRate: motion.hasRotationRate,
        attitude: QuaternionPayload(x: motion.attitude.x, y: motion.attitude.y, z: motion.attitude.z, w: motion.attitude.w),
        rotationRate: Vector3Payload(x: motion.rotationRate.x, y: motion.rotationRate.y, z: motion.rotationRate.z)
    )
}

@available(macOS 11.0, *)
private func physicalInputPayload(_ profile: GCPhysicalInputProfile) -> PhysicalInputProfileDetailsPayload {
    let elementAliases = profile.elements.keys.sorted()
    let buttonAliases = profile.buttons.keys.sorted()
    let axisAliases = profile.axes.keys.sorted()
    let dpadAliases = profile.dpads.keys.sorted()
    let touchpadAliases = profile.touchpads.keys.sorted()

    let buttons = buttonAliases.compactMap { alias -> NamedButtonInputStatePayload? in
        guard let button = profile.buttons[alias] else { return nil }
        let _ = profile[alias]
        return NamedButtonInputStatePayload(alias: alias, value: buttonPayload(button))
    }
    let axes = axisAliases.compactMap { alias -> NamedAxisInputStatePayload? in
        guard let axis = profile.axes[alias] else { return nil }
        let _ = profile[alias]
        return NamedAxisInputStatePayload(alias: alias, value: axisPayload(axis))
    }
    let dpads = dpadAliases.compactMap { alias -> NamedDirectionPadStatePayload? in
        guard let dpad = profile.dpads[alias] else { return nil }
        let _ = profile[alias]
        return NamedDirectionPadStatePayload(alias: alias, value: directionPadPayload(dpad))
    }
    let touchpads = touchpadAliases.compactMap { alias -> NamedTouchpadStatePayload? in
        guard let touchpad = profile.touchpads[alias] else { return nil }
        let _ = profile[alias]
        return NamedTouchpadStatePayload(alias: alias, value: touchpadPayload(touchpad))
    }

    let hasRemappedElements: Bool
    if #available(macOS 12.0, *) {
        hasRemappedElements = profile.hasRemappedElements
        for alias in elementAliases {
            _ = profile.mappedElementAlias(forPhysicalInputName: alias)
            _ = profile.mappedPhysicalInputNames(forElementAlias: alias)
        }
    } else {
        hasRemappedElements = false
    }

    return PhysicalInputProfileDetailsPayload(
        lastEventTimestamp: profile.lastEventTimestamp,
        hasRemappedElements: hasRemappedElements,
        elementAliases: elementAliases,
        buttonAliases: buttonAliases,
        axisAliases: axisAliases,
        dpadAliases: dpadAliases,
        touchpadAliases: touchpadAliases,
        buttons: buttons,
        axes: axes,
        dpads: dpads,
        touchpads: touchpads
    )
}

private func controllerDetailsPayload(_ controller: GCController) -> ControllerDetailsPayload {
    let currentController: GCController?
    if #available(macOS 11.0, *) {
        currentController = GCController.current
    } else {
        currentController = nil
    }

    let supportsBackgroundEvents: Bool
    if #available(macOS 11.3, *) {
        supportsBackgroundEvents = GCController.shouldMonitorBackgroundEvents
    } else {
        supportsBackgroundEvents = false
    }

    let hasLiveInput: Bool
    if #available(macOS 14.0, *) {
        _ = controller.input.elements.count
        hasLiveInput = true
    } else {
        hasLiveInput = false
    }

    let battery: BatteryInfoPayload?
    let motion: MotionDetailsPayload?
    let hasLight: Bool
    let hasHaptics: Bool
    let physicalInput: PhysicalInputProfileDetailsPayload?
    let dualSense: DualSenseGamepadDetailsPayload?
    if #available(macOS 11.0, *) {
        battery = controller.battery.map {
            BatteryInfoPayload(level: $0.batteryLevel, state: batteryStateName($0))
        }
        motion = controller.motion.map(motionPayload)
        hasLight = controller.light != nil
        hasHaptics = controller.haptics != nil

        let profile = controller.physicalInputProfile
        _ = profile.device
        _ = profile.allElements
        _ = profile.allButtons
        _ = profile.allAxes
        _ = profile.allDpads
        _ = profile.allTouchpads
        _ = profile.capture()
        physicalInput = physicalInputPayload(profile)
    } else {
        battery = nil
        motion = nil
        hasLight = false
        hasHaptics = false
        physicalInput = nil
    }

    if #available(macOS 11.3, *) {
        dualSense = (controller.extendedGamepad as? GCDualSenseGamepad).map(dualSensePayload)
    } else {
        dualSense = nil
    }

    return ControllerDetailsPayload(
        vendorName: controller.vendorName ?? "Unknown",
        productCategory: controller.productCategory,
        playerIndex: Int32(controller.playerIndex.rawValue),
        isAttachedToDevice: controller.isAttachedToDevice,
        isCurrent: currentController.map { $0 === controller } ?? false,
        supportsBackgroundEvents: supportsBackgroundEvents,
        hasLiveInput: hasLiveInput,
        gamepad: controller.gamepad.map(gamepadPayload),
        microGamepad: controller.microGamepad.map(microGamepadPayload),
        extendedGamepad: controller.extendedGamepad.map(extendedGamepadPayload),
        dualSense: dualSense,
        battery: battery,
        motion: motion,
        hasLight: hasLight,
        hasHaptics: hasHaptics,
        physicalInput: physicalInput
    )
}

@_cdecl("gc_controller_details_json")
public func gc_controller_details_json(_ currentOnly: Bool) -> UnsafeMutablePointer<CChar>? {
    let controllers: [GCController]
    if currentOnly {
        if #available(macOS 11.0, *), let current = GCController.current {
            controllers = [current]
        } else {
            controllers = []
        }
    } else {
        controllers = GCController.controllers()
    }
    return jsonString(controllers.map(controllerDetailsPayload))
}

@_cdecl("gc_start_wireless_controller_discovery")
public func gc_start_wireless_controller_discovery(
    _ callback: GCDiscoveryCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) {
    discoveryState = callback.map { DiscoveryState(callback: $0, userInfo: userInfo) }
    GCController.startWirelessControllerDiscovery {
        let state = discoveryState
        discoveryState = nil
        state?.complete()
    }
}

@_cdecl("gc_stop_wireless_controller_discovery")
public func gc_stop_wireless_controller_discovery() {
    discoveryState = nil
    GCController.stopWirelessControllerDiscovery()
}

@_cdecl("gc_should_monitor_background_events")
public func gc_should_monitor_background_events() -> Bool {
    if #unavailable(macOS 11.3) { return false }
    return GCController.shouldMonitorBackgroundEvents
}

@_cdecl("gc_set_should_monitor_background_events")
public func gc_set_should_monitor_background_events(_ enabled: Bool) {
    if #unavailable(macOS 11.3) { return }
    GCController.shouldMonitorBackgroundEvents = enabled
}

// MARK: - DualSense adaptive triggers (v0.5)

private func firstDualSense() -> Any? {
    if #unavailable(macOS 11.0) { return nil }
    for c in GCController.controllers() {
        if let ds = c.extendedGamepad as? GCDualSenseGamepad { return ds }
    }
    return nil
}

@_cdecl("gc_dualsense_is_connected")
public func gc_dualsense_is_connected() -> Bool {
    return firstDualSense() != nil
}

/// Modes: 0 = off, 1 = feedback (resistive), 2 = weapon (resist+snap),
/// 3 = vibration, 4 = slope feedback. `which` selects left (0) or right (1)
/// trigger.
@_cdecl("gc_dualsense_set_trigger_mode")
public func gc_dualsense_set_trigger_mode(
    _ which: Int32,
    _ mode: Int32,
    _ startPosition: Float,
    _ endPosition: Float,
    _ strength: Float,
    _ frequency: Float
) -> Bool {
    guard let ds = firstDualSense() as? GCDualSenseGamepad else { return false }
    let trigger = which == 0 ? ds.leftTrigger : ds.rightTrigger
    let clampedStart = clampUnit(startPosition)
    let clampedEnd = clampUnit(endPosition)
    let clampedStrength = clampUnit(strength)
    let clampedFrequency = clampUnit(frequency)
    switch mode {
    case 0:
        trigger.setModeOff()
    case 1:
        guard #available(macOS 12.3, *) else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        var values = Array(repeating: Float(0), count: 10)
        for idx in startIndex..<values.count {
            values[idx] = clampedStrength
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    case 2:
        guard #available(macOS 12.3, *), clampedEnd > clampedStart else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        let endIndex = dualSenseStepIndex(for: clampedEnd)
        var values = Array(repeating: Float(0), count: 10)
        for idx in startIndex...endIndex {
            values[idx] = clampedStrength
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    case 3:
        guard #available(macOS 12.3, *) else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        var values = Array(repeating: Float(0), count: 10)
        for idx in startIndex..<values.count {
            values[idx] = clampedStrength
        }
        trigger.setModeVibration(amplitudes: makeAmplitudes(values), frequency: clampedFrequency)
    case 4:
        guard #available(macOS 12.3, *), clampedEnd > clampedStart else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        let endIndex = dualSenseStepIndex(for: clampedEnd)
        var values = Array(repeating: Float(0), count: 10)
        if startIndex == endIndex {
            values[startIndex] = clampedFrequency
        } else {
            for idx in startIndex...endIndex {
                let ratio = Float(idx - startIndex) / Float(endIndex - startIndex)
                values[idx] = clampedStrength + ((clampedFrequency - clampedStrength) * ratio)
            }
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    default:
        return false
    }
    return true
}
