import Foundation
import GameController

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

@available(macOS 12.3, *)
private func triggerValues(_ values: UnsafePointer<Float>?, len: Int) -> [Float]? {
    guard let values, len == GCDualSenseAdaptiveTrigger.discretePositionCount else {
        return nil
    }
    return (0..<len).map { clampUnit(values[$0]) }
}

private func firstDualSense() -> GCDualSenseGamepad? {
    if #unavailable(macOS 11.3) { return nil }
    for controller in GCController.controllers() {
        if let dualSense = controller.extendedGamepad as? GCDualSenseGamepad {
            return dualSense
        }
    }
    return nil
}

@_cdecl("gc_dualsense_is_connected")
public func gc_dualsense_is_connected() -> Bool {
    firstDualSense() != nil
}

@_cdecl("gc_dualsense_set_trigger_mode")
public func gc_dualsense_set_trigger_mode(
    _ which: Int32,
    _ mode: Int32,
    _ startPosition: Float,
    _ endPosition: Float,
    _ strength: Float,
    _ frequency: Float
) -> Bool {
    guard let dualSense = firstDualSense() else { return false }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
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
        for index in startIndex..<values.count {
            values[index] = clampedStrength
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    case 2:
        guard #available(macOS 12.3, *), clampedEnd > clampedStart else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        let endIndex = dualSenseStepIndex(for: clampedEnd)
        var values = Array(repeating: Float(0), count: 10)
        for index in startIndex...endIndex {
            values[index] = clampedStrength
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    case 3:
        guard #available(macOS 12.3, *) else { return false }
        let startIndex = dualSenseStepIndex(for: clampedStart)
        var values = Array(repeating: Float(0), count: 10)
        for index in startIndex..<values.count {
            values[index] = clampedStrength
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
            for position in startIndex...endIndex {
                let numerator = Double(position - startIndex)
                let denominator = Double(endIndex - startIndex)
                let ratio = Float(numerator / denominator)
                values[position] = clampedStrength + ((clampedFrequency - clampedStrength) * ratio)
            }
        }
        trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    default:
        return false
    }

    return true
}

@_cdecl("gc_dualsense_set_trigger_feedback_resistive_strengths")
public func gc_dualsense_set_trigger_feedback_resistive_strengths(
    _ which: Int32,
    _ values: UnsafePointer<Float>?,
    _ len: Int
) -> Bool {
    guard #available(macOS 12.3, *),
          let dualSense = firstDualSense(),
          let values = triggerValues(values, len: len)
    else {
        return false
    }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
    trigger.setModeFeedback(resistiveStrengths: makeResistiveStrengths(values))
    return true
}

@_cdecl("gc_dualsense_set_trigger_vibration_amplitudes")
public func gc_dualsense_set_trigger_vibration_amplitudes(
    _ which: Int32,
    _ values: UnsafePointer<Float>?,
    _ len: Int,
    _ frequency: Float
) -> Bool {
    guard #available(macOS 12.3, *),
          let dualSense = firstDualSense(),
          let values = triggerValues(values, len: len)
    else {
        return false
    }

    let trigger = which == 0 ? dualSense.leftTrigger : dualSense.rightTrigger
    trigger.setModeVibration(amplitudes: makeAmplitudes(values), frequency: clampUnit(frequency))
    return true
}
