import AppKit
import Foundation
import GameController

@_cdecl("gc_first_controller_set_light")
public func gc_first_controller_set_light(_ red: Float, _ green: Float, _ blue: Float) -> Bool {
    guard let controller = GCController.controllers().first, let light = controller.light else { return false }
    light.color = GCColor(red: red, green: green, blue: blue)
    return true
}

@_cdecl("gc_first_controller_set_player_index")
public func gc_first_controller_set_player_index(_ index: Int32) -> Bool {
    guard let controller = GCController.controllers().first else { return false }
    guard let playerIndex = GCControllerPlayerIndex(rawValue: Int(index)) else { return false }
    controller.playerIndex = playerIndex
    return true
}

@_cdecl("gc_first_controller_battery_level")
public func gc_first_controller_battery_level() -> Float {
    guard let controller = GCController.controllers().first, let battery = controller.battery else {
        return -1
    }
    return battery.batteryLevel
}

#if canImport(CoreHaptics)
import CoreHaptics

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
        hapticEngines[ObjectIdentifier(engine)] = engine
        DispatchQueue.main.asyncAfter(deadline: .now() + clampedDuration + 0.1) {
            engine.stop(completionHandler: nil)
            hapticEngines.removeValue(forKey: ObjectIdentifier(engine))
        }
        return true
    } catch {
        engine.stop(completionHandler: nil)
        return false
    }
}

@_cdecl("gc_first_controller_rumble")
public func gc_first_controller_rumble(
    _ intensity: Float,
    _ sharpness: Float,
    _ duration: Double
) -> Bool {
    guard #available(macOS 11.0, *), let controller = GCController.controllers().first else {
        return false
    }
    return playRumble(
        controller: controller,
        locality: .default,
        intensity: intensity,
        sharpness: sharpness,
        duration: duration
    )
}

@_cdecl("gc_first_controller_rumble_with_locality")
public func gc_first_controller_rumble_with_locality(
    _ locality: UnsafePointer<CChar>?,
    _ intensity: Float,
    _ sharpness: Float,
    _ duration: Double
) -> Bool {
    guard #available(macOS 11.0, *),
          let controller = GCController.controllers().first,
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
@_cdecl("gc_first_controller_rumble")
public func gc_first_controller_rumble(
    _ intensity: Float,
    _ sharpness: Float,
    _ duration: Double
) -> Bool {
    false
}

@_cdecl("gc_first_controller_rumble_with_locality")
public func gc_first_controller_rumble_with_locality(
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
