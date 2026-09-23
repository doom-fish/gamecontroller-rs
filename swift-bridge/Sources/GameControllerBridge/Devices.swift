import AppKit
import Foundation
import GameController

@_cdecl("gc_controller_handles_copy")
public func gc_controller_handles_copy(
    _ outHandles: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    let controllers = GCController.controllers()
    if let outHandles {
        for (index, controller) in controllers.prefix(max(0, capacity)).enumerated() {
            outHandles[index] = Unmanaged.passRetained(controller).toOpaque()
        }
    }
    return controllers.count
}

@_cdecl("gc_controller_current_handle")
public func gc_controller_current_handle() -> UnsafeMutableRawPointer? {
    guard let current = GCController.current else { return nil }
    return Unmanaged.passRetained(current).toOpaque()
}

@_cdecl("gc_controller_handle_retain")
public func gc_controller_handle_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controller = controllerFromHandle(handle) else { return nil }
    return Unmanaged.passRetained(controller).toOpaque()
}

@_cdecl("gc_controller_handle_release")
public func gc_controller_handle_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<GCController>.fromOpaque(handle).release()
}

@_cdecl("gc_controller_handle_vendor_name")
public func gc_controller_handle_vendor_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    ffiString(controllerFromHandle(handle)?.vendorName)
}

@_cdecl("gc_controller_handle_product_category")
public func gc_controller_handle_product_category(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    ffiString(controllerFromHandle(handle)?.productCategory)
}

@_cdecl("gc_controller_handle_player_index")
public func gc_controller_handle_player_index(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let controller = controllerFromHandle(handle) else { return -1 }
    return Int32(clamping: controller.playerIndex.rawValue)
}

@_cdecl("gc_controller_handle_set_player_index")
public func gc_controller_handle_set_player_index(_ handle: UnsafeMutableRawPointer?, _ index: Int32) -> Bool {
    guard let controller = controllerFromHandle(handle),
          (-1...3).contains(index),
          let playerIndex = GCControllerPlayerIndex(rawValue: Int(index))
    else {
        return false
    }
    controller.playerIndex = playerIndex
    return true
}

@_cdecl("gc_controller_handle_set_light")
public func gc_controller_handle_set_light(
    _ handle: UnsafeMutableRawPointer?,
    _ red: Float,
    _ green: Float,
    _ blue: Float
) -> Bool {
    guard let light = controllerFromHandle(handle)?.light else { return false }
    light.color = GCColor(red: red, green: green, blue: blue)
    return true
}

@_cdecl("gc_controller_handle_battery_level")
public func gc_controller_handle_battery_level(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let battery = controllerFromHandle(handle)?.battery else { return -1 }
    return battery.batteryLevel
}

@_cdecl("gc_controller_handle_is_dualsense")
public func gc_controller_handle_is_dualsense(_ handle: UnsafeMutableRawPointer?) -> Bool {
    controllerFromHandle(handle)?.extendedGamepad is GCDualSenseGamepad
}

#if canImport(CoreHaptics)
import CoreHaptics

private let hapticEnginesLock = NSLock()
private var hapticEngines: [ObjectIdentifier: CHHapticEngine] = [:]

@available(macOS 11.0, *)
private func hapticsLocality(from value: String) -> GCHapticsLocality? {
    switch value {
    case GCHapticsLocality.default.rawValue:
        return .default
    case GCHapticsLocality.all.rawValue:
        return .all
    case GCHapticsLocality.handles.rawValue:
        return .handles
    case GCHapticsLocality.leftHandle.rawValue:
        return .leftHandle
    case GCHapticsLocality.rightHandle.rawValue:
        return .rightHandle
    case GCHapticsLocality.triggers.rawValue:
        return .triggers
    case GCHapticsLocality.leftTrigger.rawValue:
        return .leftTrigger
    case GCHapticsLocality.rightTrigger.rawValue:
        return .rightTrigger
    default:
        return nil
    }
}

@available(macOS 11.0, *)
private func playRumble(
    controller: GCController,
    locality: GCHapticsLocality,
    intensity: Float,
    sharpness: Float,
    duration: Double
) -> Bool {
    guard let haptics = controller.haptics,
          let engine = haptics.createEngine(withLocality: locality)
    else {
        return false
    }

    do {
        try engine.start()
    } catch {
        return false
    }

    let intensityParam = CHHapticEventParameter(
        parameterID: .hapticIntensity,
        value: clampUnit(intensity)
    )
    let sharpnessParam = CHHapticEventParameter(
        parameterID: .hapticSharpness,
        value: clampUnit(sharpness)
    )
    let clampedDuration = max(0.01, duration)
    let event = CHHapticEvent(
        eventType: .hapticContinuous,
        parameters: [intensityParam, sharpnessParam],
        relativeTime: 0,
        duration: clampedDuration
    )

    do {
        let pattern = try CHHapticPattern(events: [event], parameters: [])
        let player = try engine.makePlayer(with: pattern)
        try player.start(atTime: 0)
        hapticEnginesLock.lock()
        hapticEngines[ObjectIdentifier(engine)] = engine
        hapticEnginesLock.unlock()
        DispatchQueue.main.asyncAfter(deadline: .now() + clampedDuration + 0.1) {
            engine.stop(completionHandler: nil)
            hapticEnginesLock.lock()
            hapticEngines.removeValue(forKey: ObjectIdentifier(engine))
            hapticEnginesLock.unlock()
        }
        return true
    } catch {
        engine.stop(completionHandler: nil)
        return false
    }
}

@_cdecl("gc_controller_handle_rumble")
public func gc_controller_handle_rumble(
    _ handle: UnsafeMutableRawPointer?,
    _ locality: UnsafePointer<CChar>?,
    _ intensity: Float,
    _ sharpness: Float,
    _ duration: Double
) -> Bool {
    guard let controller = controllerFromHandle(handle),
          let locality,
          let typedLocality = hapticsLocality(from: String(cString: locality))
    else {
        return false
    }
    return playRumble(
        controller: controller,
        locality: typedLocality,
        intensity: intensity,
        sharpness: sharpness,
        duration: duration
    )
}
#else
@_cdecl("gc_controller_handle_rumble")
public func gc_controller_handle_rumble(
    _ handle: UnsafeMutableRawPointer?,
    _ locality: UnsafePointer<CChar>?,
    _ intensity: Float,
    _ sharpness: Float,
    _ duration: Double
) -> Bool {
    false
}
#endif

@_cdecl("gc_mouse_is_connected")
public func gc_mouse_is_connected() -> Bool {
    if #unavailable(macOS 11.0) { return false }
    return GCMouse.current != nil
}

@_cdecl("gc_mouse_button_states")
public func gc_mouse_button_states(
    _ outLeft: UnsafeMutablePointer<Bool>,
    _ outRight: UnsafeMutablePointer<Bool>,
    _ outMiddle: UnsafeMutablePointer<Bool>
) -> Bool {
    if #unavailable(macOS 11.0) { return false }
    guard let mouse = GCMouse.current, let input = mouse.mouseInput else { return false }
    outLeft.pointee = input.leftButton.isPressed
    outRight.pointee = input.rightButton?.isPressed ?? false
    outMiddle.pointee = input.middleButton?.isPressed ?? false
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
    guard let keyboard = GCKeyboard.coalesced, let input = keyboard.keyboardInput else { return false }
    return input.isAnyKeyPressed
}

@_cdecl("gc_keyboard_is_key_pressed")
public func gc_keyboard_is_key_pressed(_ keycode: Int) -> Bool {
    if #unavailable(macOS 11.0) { return false }
    guard let keyboard = GCKeyboard.coalesced, let input = keyboard.keyboardInput else { return false }
    let code = GCKeyCode(rawValue: keycode)
    return input.button(forKeyCode: code)?.isPressed ?? false
}

@_cdecl("gc_keyboard_snapshot_json")
public func gc_keyboard_snapshot_json() -> UnsafeMutablePointer<CChar>? {
    if #unavailable(macOS 11.0) {
        return jsonString(Optional<KeyboardSnapshotPayload>.none)
    }
    return jsonString(GCKeyboard.coalesced.flatMap(keyboardSnapshotPayload))
}

@_cdecl("gc_mouse_snapshot_json")
public func gc_mouse_snapshot_json() -> UnsafeMutablePointer<CChar>? {
    if #unavailable(macOS 11.0) {
        return jsonString(Optional<MouseSnapshotPayload>.none)
    }
    return jsonString(GCMouse.current.flatMap(mouseSnapshotPayload))
}

@_cdecl("gc_event_view_controller_snapshot_json")
public func gc_event_view_controller_snapshot_json() -> UnsafeMutablePointer<CChar>? {
    let controller = GCEventViewController()
    let enabled = controller.controllerUserInteractionEnabled
    controller.controllerUserInteractionEnabled = enabled
    return jsonString(EventViewControllerPayload(controllerUserInteractionEnabled: enabled))
}

@_cdecl("gc_racing_wheels_json")
public func gc_racing_wheels_json() -> UnsafeMutablePointer<CChar>? {
    if #unavailable(macOS 13.0) {
        return jsonString([RacingWheelDetailsPayload]())
    }

    let wheels = Array(GCRacingWheel.connectedRacingWheels).sorted {
        let lhsVendor = $0.vendorName ?? ""
        let rhsVendor = $1.vendorName ?? ""
        if lhsVendor != rhsVendor {
            return lhsVendor < rhsVendor
        }
        return $0.productCategory < $1.productCategory
    }
    return jsonString(wheels.map(racingWheelPayload))
}
