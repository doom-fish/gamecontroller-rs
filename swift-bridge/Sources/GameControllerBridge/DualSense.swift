import Foundation
import GameController

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

@available(macOS 12.3, *)
private func triggerValues(_ values: UnsafePointer<Float>?, len: Int) -> [Float]? {
    guard let values, len == GCDualSenseAdaptiveTrigger.discretePositionCount else {
        return nil
    }
    return (0..<len).map { clampUnit(values[$0]) }
}

private func dualSenseGamepad(_ handle: UnsafeMutableRawPointer?) -> GCDualSenseGamepad? {
    controllerFromHandle(handle)?.extendedGamepad as? GCDualSenseGamepad
}

@_cdecl("gc_controller_handle_dualsense_set_trigger_mode")
public func gc_controller_handle_dualsense_set_trigger_mode(
    _ handle: UnsafeMutableRawPointer?,
    _ which: Int32,
    _ mode: Int32,
    _ startPosition: Float,
    _ endPosition: Float,
    _ strength: Float,
    _ frequency: Float
) -> Bool {
    guard let dualSense = dualSenseGamepad(handle) else { return false }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
    let clampedStart = clampUnit(startPosition)
    let clampedEnd = clampUnit(endPosition)
    let clampedStrength = clampUnit(strength)
    let clampedFrequency = clampUnit(frequency)

    switch mode {
    case 0:
        trigger.setModeOff()
    case 1:
        trigger.setModeFeedbackWithStartPosition(clampedStart, resistiveStrength: clampedStrength)
    case 2:
        guard clampedEnd > clampedStart else { return false }
        trigger.setModeWeaponWithStartPosition(
            clampedStart,
            endPosition: clampedEnd,
            resistiveStrength: clampedStrength
        )
    case 3:
        trigger.setModeVibrationWithStartPosition(
            clampedStart,
            amplitude: clampedStrength,
            frequency: clampedFrequency
        )
    case 4:
        guard clampedEnd > clampedStart else { return false }
        trigger.setModeSlopeFeedback(
            startPosition: clampedStart,
            endPosition: clampedEnd,
            startStrength: clampedStrength,
            endStrength: clampedFrequency
        )
    default:
        return false
    }

    return true
}

@_cdecl("gc_controller_handle_dualsense_set_trigger_feedback_resistive_strengths")
public func gc_controller_handle_dualsense_set_trigger_feedback_resistive_strengths(
    _ handle: UnsafeMutableRawPointer?,
    _ which: Int32,
    _ values: UnsafePointer<Float>?,
    _ len: Int
) -> Bool {
    guard #available(macOS 12.3, *),
          let dualSense = dualSenseGamepad(handle),
          let values = triggerValues(values, len: len)
    else {
        return false
    }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
    trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    return true
}

@_cdecl("gc_controller_handle_dualsense_set_trigger_vibration_amplitudes")
public func gc_controller_handle_dualsense_set_trigger_vibration_amplitudes(
    _ handle: UnsafeMutableRawPointer?,
    _ which: Int32,
    _ values: UnsafePointer<Float>?,
    _ len: Int,
    _ frequency: Float
) -> Bool {
    guard #available(macOS 12.3, *),
          let dualSense = dualSenseGamepad(handle),
          let values = triggerValues(values, len: len)
    else {
        return false
    }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
    trigger.setModeVibration(amplitudes: makeAmplitudes(values), frequency: clampUnit(frequency))
    return true
}
