import Foundation
import GameController

@_cdecl("gc_connected_devices_json")
public func gc_connected_devices_json() -> UnsafeMutablePointer<CChar>? {
    let controllers = GCController.controllers().map(deviceDetailsPayload)

    let keyboard: DeviceDetailsPayload?
    if #available(macOS 11.0, *) {
        keyboard = GCKeyboard.coalesced.map(deviceDetailsPayload)
    } else {
        keyboard = nil
    }

    let mouse: DeviceDetailsPayload?
    if #available(macOS 11.0, *) {
        mouse = GCMouse.current.map(deviceDetailsPayload)
    } else {
        mouse = nil
    }

    let racingWheels: [DeviceDetailsPayload]
    if #available(macOS 13.0, *) {
        racingWheels = Array(GCRacingWheel.connectedRacingWheels).map(deviceDetailsPayload)
    } else {
        racingWheels = []
    }

    return jsonString(
        ConnectedDevicesSnapshotPayload(
            controllers: controllers,
            keyboard: keyboard,
            mouse: mouse,
            racingWheels: racingWheels
        )
    )
}

@_cdecl("gc_current_controller_elements_json")
public func gc_current_controller_elements_json() -> UnsafeMutablePointer<CChar>? {
    return jsonString(currentControllerElementPayloads())
}

@_cdecl("gc_current_controller_physical_input_elements_json")
public func gc_current_controller_physical_input_elements_json() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 14.0, *) {
        return jsonString(currentControllerPhysicalInputElementSnapshotPayload())
    }
    return jsonString(Optional<PhysicalInputElementSnapshotPayload>.none)
}
