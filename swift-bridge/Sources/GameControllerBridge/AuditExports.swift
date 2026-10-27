import GameControllerABI
import Foundation
import GameController

@_cdecl("gc_connected_devices")
public func gc_connected_devices(_ out: UnsafeMutablePointer<GCSnapshotRaw>) {
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

    out.pointee = gcSnapshot([ConnectedDevicesSnapshotPayload(
            controllers: controllers,
            keyboard: keyboard,
            mouse: mouse,
            racingWheels: racingWheels
        )])
    return
}

@_cdecl("gc_current_controller_elements")
public func gc_current_controller_elements(_ out: UnsafeMutablePointer<GCSnapshotRaw>) {
    out.pointee = gcSnapshot(currentControllerElementPayloads())
    return
}

@_cdecl("gc_current_controller_physical_input_elements")
public func gc_current_controller_physical_input_elements(_ out: UnsafeMutablePointer<GCSnapshotRaw>) {
    if #available(macOS 14.0, *) {
        out.pointee = gcSnapshot((currentControllerPhysicalInputElementSnapshotPayload()).map { [$0] } ?? [])
    return
    }
    out.pointee = gcSnapshot((Optional<PhysicalInputElementSnapshotPayload>.none).map { [$0] } ?? [])
    return
}
