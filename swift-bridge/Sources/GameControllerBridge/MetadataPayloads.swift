import Dispatch
import Foundation
import GameController

struct Point2Payload: Codable {
    let x: Float
    let y: Float
}

struct DeviceDetailsPayload: Codable {
    let vendorName: String
    let productCategory: String
    let handlerQueueLabel: String
}

struct ConnectedDevicesSnapshotPayload: Codable {
    let controllers: [DeviceDetailsPayload]
    let keyboard: DeviceDetailsPayload?
    let mouse: DeviceDetailsPayload?
    let racingWheels: [DeviceDetailsPayload]
}

struct ControllerElementDetailsPayload: Codable {
    let analog: Bool
    let collectionPath: [String]
    let isBoundToSystemGesture: Bool
    let preferredSystemGestureState: String
    let sfSymbolsName: String?
    let localizedName: String?
    let unmappedSfSymbolsName: String?
    let unmappedLocalizedName: String?
}

struct NamedControllerElementDetailsPayload: Codable {
    let name: String
    let value: ControllerElementDetailsPayload
}

struct PhysicalInputSourceDetailsPayload: Codable {
    let elementAliases: [String]
    let elementLocalizedName: String?
    let sfSymbolsName: String?
    let direction: UInt8
}

struct PhysicalInputExtentsDetailsPayload: Codable {
    let scaledValue: Double
    let minimumValue: Double
    let maximumValue: Double
}

struct LinearInputDetailsPayload: Codable {
    let value: Float
    let analog: Bool
    let canWrap: Bool
    let lastValueTimestamp: Double
    let lastValueLatency: Double
    let physicalExtents: PhysicalInputExtentsDetailsPayload?
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct PressedStateInputDetailsPayload: Codable {
    let pressed: Bool
    let lastPressedStateTimestamp: Double
    let lastPressedStateLatency: Double
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct TouchedStateInputDetailsPayload: Codable {
    let touched: Bool
    let lastTouchedStateTimestamp: Double
    let lastTouchedStateLatency: Double
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct RelativeInputDetailsPayload: Codable {
    let delta: Float
    let analog: Bool
    let lastDeltaTimestamp: Double
    let lastDeltaLatency: Double
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct AxisInputDetailsPayload: Codable {
    let value: Float
    let analog: Bool
    let canWrap: Bool
    let lastValueTimestamp: Double
    let lastValueLatency: Double
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct Axis2DInputDetailsPayload: Codable {
    let value: Point2Payload
    let analog: Bool
    let canWrap: Bool
    let lastValueTimestamp: Double
    let lastValueLatency: Double
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct SwitchPositionInputDetailsPayload: Codable {
    let position: Int32
    let positionLowerBound: Int32
    let positionCount: Int32
    let sequential: Bool
    let canWrap: Bool
    let sources: [PhysicalInputSourceDetailsPayload]
}

struct LinearPressedInputDetailsPayload: Codable {
    let linearInput: LinearInputDetailsPayload
    let pressedState: PressedStateInputDetailsPayload
}

struct ButtonElementDetailsPayload: Codable {
    let metadata: InputElementMetadataPayload
    let linearInput: LinearInputDetailsPayload
    let pressedState: PressedStateInputDetailsPayload
    let touchedState: TouchedStateInputDetailsPayload?
    let forceInput: LinearInputDetailsPayload?
}

struct AxisElementDetailsPayload: Codable {
    let metadata: InputElementMetadataPayload
    let absoluteInput: AxisInputDetailsPayload?
    let relativeInput: RelativeInputDetailsPayload
}

struct SwitchElementDetailsPayload: Codable {
    let metadata: InputElementMetadataPayload
    let positionInput: SwitchPositionInputDetailsPayload
}

struct DirectionPadElementDetailsPayload: Codable {
    let metadata: InputElementMetadataPayload
    let xyAxes: Axis2DInputDetailsPayload?
    let xAxis: AxisInputDetailsPayload
    let yAxis: AxisInputDetailsPayload
    let up: LinearPressedInputDetailsPayload
    let down: LinearPressedInputDetailsPayload
    let left: LinearPressedInputDetailsPayload
    let right: LinearPressedInputDetailsPayload
}

struct PhysicalInputElementSnapshotPayload: Codable {
    let buttons: [ButtonElementDetailsPayload]
    let axes: [AxisElementDetailsPayload]
    let switches: [SwitchElementDetailsPayload]
    let dpads: [DirectionPadElementDetailsPayload]
}

@available(macOS 14.3, *)
func point2Payload(_ point: GCPoint2) -> Point2Payload {
    Point2Payload(x: point.x, y: point.y)
}

func systemGestureStateName(_ state: GCControllerElement.SystemGestureState) -> String {
    switch state {
    case .enabled:
        return "enabled"
    case .alwaysReceive:
        return "always_receive"
    case .disabled:
        return "disabled"
    @unknown default:
        return "enabled"
    }
}

func deviceDetailsPayload(_ device: any GCDevice) -> DeviceDetailsPayload {
    DeviceDetailsPayload(
        vendorName: device.vendorName ?? "Unknown",
        productCategory: device.productCategory,
        handlerQueueLabel: device.handlerQueue.label
    )
}

func controllerElementCollectionPath(_ element: GCControllerElement) -> [String] {
    var path: [String] = []
    var current: GCControllerElement? = element.collection
    while let item = current {
        path.append(item.localizedName ?? item.sfSymbolsName ?? String(describing: type(of: item)))
        current = item.collection
    }
    return path.reversed()
}

func controllerElementDetailsPayload(_ element: GCControllerElement) -> ControllerElementDetailsPayload {
    let isBoundToSystemGesture: Bool
    let preferredSystemGestureState: String
    if #available(macOS 11.0, *) {
        isBoundToSystemGesture = element.isBoundToSystemGesture
        preferredSystemGestureState = systemGestureStateName(element.preferredSystemGestureState)
    } else {
        isBoundToSystemGesture = false
        preferredSystemGestureState = "enabled"
    }

    let sfSymbolsName: String?
    let localizedName: String?
    let unmappedSfSymbolsName: String?
    let unmappedLocalizedName: String?
    if #available(macOS 11.0, *) {
        sfSymbolsName = element.sfSymbolsName
        localizedName = element.localizedName
        unmappedSfSymbolsName = element.unmappedSfSymbolsName
        unmappedLocalizedName = element.unmappedLocalizedName
    } else {
        sfSymbolsName = nil
        localizedName = nil
        unmappedSfSymbolsName = nil
        unmappedLocalizedName = nil
    }

    return ControllerElementDetailsPayload(
        analog: element.isAnalog,
        collectionPath: controllerElementCollectionPath(element),
        isBoundToSystemGesture: isBoundToSystemGesture,
        preferredSystemGestureState: preferredSystemGestureState,
        sfSymbolsName: sfSymbolsName,
        localizedName: localizedName,
        unmappedSfSymbolsName: unmappedSfSymbolsName,
        unmappedLocalizedName: unmappedLocalizedName
    )
}

func namedControllerElementDetailsPayload(name: String, element: GCControllerElement) -> NamedControllerElementDetailsPayload {
    NamedControllerElementDetailsPayload(name: name, value: controllerElementDetailsPayload(element))
}

func appendControllerElements(
    _ elements: [(String, GCControllerElement)],
    to out: inout [NamedControllerElementDetailsPayload]
) {
    for (name, element) in elements {
        out.append(namedControllerElementDetailsPayload(name: name, element: element))
    }
}

func appendGamepadControllerElements(
    _ gamepad: GCGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    appendControllerElements([
        ("\(prefix).dpad", gamepad.dpad),
        ("\(prefix).buttonA", gamepad.buttonA),
        ("\(prefix).buttonB", gamepad.buttonB),
        ("\(prefix).buttonX", gamepad.buttonX),
        ("\(prefix).buttonY", gamepad.buttonY),
        ("\(prefix).leftShoulder", gamepad.leftShoulder),
        ("\(prefix).rightShoulder", gamepad.rightShoulder),
    ], to: &out)
}

func appendMicroGamepadControllerElements(
    _ gamepad: GCMicroGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    appendControllerElements([
        ("\(prefix).dpad", gamepad.dpad),
        ("\(prefix).buttonA", gamepad.buttonA),
        ("\(prefix).buttonX", gamepad.buttonX),
        ("\(prefix).buttonMenu", gamepad.buttonMenu),
    ], to: &out)
}

@available(macOS 11.1, *)
func appendDirectionalGamepadControllerElements(
    _ gamepad: GCDirectionalGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    appendControllerElements([
        ("\(prefix).dpad", gamepad.dpad),
        ("\(prefix).buttonA", gamepad.buttonA),
        ("\(prefix).buttonX", gamepad.buttonX),
        ("\(prefix).buttonMenu", gamepad.buttonMenu),
    ], to: &out)
}

func appendExtendedGamepadControllerElements(
    _ gamepad: GCExtendedGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    var elements: [(String, GCControllerElement)] = [
        ("\(prefix).dpad", gamepad.dpad),
        ("\(prefix).buttonA", gamepad.buttonA),
        ("\(prefix).buttonB", gamepad.buttonB),
        ("\(prefix).buttonX", gamepad.buttonX),
        ("\(prefix).buttonY", gamepad.buttonY),
        ("\(prefix).buttonMenu", gamepad.buttonMenu),
        ("\(prefix).leftThumbstick", gamepad.leftThumbstick),
        ("\(prefix).rightThumbstick", gamepad.rightThumbstick),
        ("\(prefix).leftShoulder", gamepad.leftShoulder),
        ("\(prefix).rightShoulder", gamepad.rightShoulder),
        ("\(prefix).leftTrigger", gamepad.leftTrigger),
        ("\(prefix).rightTrigger", gamepad.rightTrigger),
    ]
    if let buttonOptions = gamepad.buttonOptions {
        elements.append(("\(prefix).buttonOptions", buttonOptions))
    }
    if #available(macOS 11.0, *), let buttonHome = gamepad.buttonHome {
        elements.append(("\(prefix).buttonHome", buttonHome))
    }
    if let leftThumbstickButton = gamepad.leftThumbstickButton {
        elements.append(("\(prefix).leftThumbstickButton", leftThumbstickButton))
    }
    if let rightThumbstickButton = gamepad.rightThumbstickButton {
        elements.append(("\(prefix).rightThumbstickButton", rightThumbstickButton))
    }
    appendControllerElements(elements, to: &out)
}

@available(macOS 11.0, *)
func appendDualShockControllerElements(
    _ gamepad: GCDualShockGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    appendControllerElements([
        ("\(prefix).touchpadButton", gamepad.touchpadButton),
        ("\(prefix).touchpadPrimary", gamepad.touchpadPrimary),
        ("\(prefix).touchpadSecondary", gamepad.touchpadSecondary),
    ], to: &out)
}

@available(macOS 11.3, *)
func appendDualSenseControllerElements(
    _ gamepad: GCDualSenseGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    appendControllerElements([
        ("\(prefix).touchpadButton", gamepad.touchpadButton),
        ("\(prefix).touchpadPrimary", gamepad.touchpadPrimary),
        ("\(prefix).touchpadSecondary", gamepad.touchpadSecondary),
        ("\(prefix).leftTrigger", gamepad.leftTrigger),
        ("\(prefix).rightTrigger", gamepad.rightTrigger),
    ], to: &out)
}

@available(macOS 11.0, *)
func appendXboxControllerElements(
    _ gamepad: GCXboxGamepad,
    prefix: String,
    into out: inout [NamedControllerElementDetailsPayload]
) {
    var elements: [(String, GCControllerElement)] = []
    if let paddleButton1 = gamepad.paddleButton1 {
        elements.append(("\(prefix).paddleButton1", paddleButton1))
    }
    if let paddleButton2 = gamepad.paddleButton2 {
        elements.append(("\(prefix).paddleButton2", paddleButton2))
    }
    if let paddleButton3 = gamepad.paddleButton3 {
        elements.append(("\(prefix).paddleButton3", paddleButton3))
    }
    if let paddleButton4 = gamepad.paddleButton4 {
        elements.append(("\(prefix).paddleButton4", paddleButton4))
    }
    if #available(macOS 12.0, *), let buttonShare = gamepad.buttonShare {
        elements.append(("\(prefix).buttonShare", buttonShare))
    }
    appendControllerElements(elements, to: &out)
}

func currentControllerElementPayloads() -> [NamedControllerElementDetailsPayload] {
    guard #available(macOS 11.0, *), let current = GCController.current else {
        return []
    }

    var out: [NamedControllerElementDetailsPayload] = []
    if let gamepad = current.gamepad {
        appendGamepadControllerElements(gamepad, prefix: "gamepad", into: &out)
    }
    if let microGamepad = current.microGamepad {
        appendMicroGamepadControllerElements(microGamepad, prefix: "microGamepad", into: &out)
        if #available(macOS 11.1, *), let directional = microGamepad as? GCDirectionalGamepad {
            appendDirectionalGamepadControllerElements(directional, prefix: "directionalGamepad", into: &out)
        }
    }
    if let extendedGamepad = current.extendedGamepad {
        appendExtendedGamepadControllerElements(extendedGamepad, prefix: "extendedGamepad", into: &out)
        if #available(macOS 11.0, *), let dualShock = extendedGamepad as? GCDualShockGamepad {
            appendDualShockControllerElements(dualShock, prefix: "dualShock", into: &out)
        }
        if #available(macOS 11.3, *), let dualSense = extendedGamepad as? GCDualSenseGamepad {
            appendDualSenseControllerElements(dualSense, prefix: "dualSense", into: &out)
        }
        if #available(macOS 11.0, *), let xbox = extendedGamepad as? GCXboxGamepad {
            appendXboxControllerElements(xbox, prefix: "xbox", into: &out)
        }
    }

    out.sort { $0.name < $1.name }
    return out
}

@available(macOS 14.0, *)
func physicalInputSourceDetailsPayload(_ source: any GCPhysicalInputSource) -> PhysicalInputSourceDetailsPayload {
    PhysicalInputSourceDetailsPayload(
        elementAliases: Array(source.elementAliases).sorted(),
        elementLocalizedName: source.elementLocalizedName,
        sfSymbolsName: source.sfSymbolsName,
        direction: UInt8(truncatingIfNeeded: source.direction.rawValue)
    )
}

@available(macOS 14.0, *)
func physicalInputSourceDetailsPayloads(_ rawSources: [AnyHashable]) -> [PhysicalInputSourceDetailsPayload] {
    rawSources
        .compactMap { hashable -> PhysicalInputSourceDetailsPayload? in
            guard let source = hashable.base as? any GCPhysicalInputSource else {
                return nil
            }
            return physicalInputSourceDetailsPayload(source)
        }
        .sorted { lhs, rhs in
            let lhsKey = lhs.elementAliases.first ?? lhs.elementLocalizedName ?? lhs.sfSymbolsName ?? ""
            let rhsKey = rhs.elementAliases.first ?? rhs.elementLocalizedName ?? rhs.sfSymbolsName ?? ""
            return lhsKey < rhsKey
        }
}

@available(macOS 26.2, *)
func physicalInputExtentsDetailsPayload(_ extents: any GCPhysicalInputExtents) -> PhysicalInputExtentsDetailsPayload {
    PhysicalInputExtentsDetailsPayload(
        scaledValue: extents.scaledValue,
        minimumValue: extents.minimumValue,
        maximumValue: extents.maximumValue
    )
}

@available(macOS 14.0, *)
func linearInputDetailsPayload(_ input: any GCLinearInput) -> LinearInputDetailsPayload {
    let physicalExtents: PhysicalInputExtentsDetailsPayload?
    if #available(macOS 26.2, *) {
        physicalExtents = input.physicalExtents.map(physicalInputExtentsDetailsPayload)
    } else {
        physicalExtents = nil
    }

    return LinearInputDetailsPayload(
        value: input.value,
        analog: input.isAnalog,
        canWrap: input.canWrap,
        lastValueTimestamp: input.lastValueTimestamp,
        lastValueLatency: input.lastValueLatency,
        physicalExtents: physicalExtents,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func pressedStateInputDetailsPayload(_ input: any GCPressedStateInput) -> PressedStateInputDetailsPayload {
    PressedStateInputDetailsPayload(
        pressed: input.isPressed,
        lastPressedStateTimestamp: input.lastPressedStateTimestamp,
        lastPressedStateLatency: input.lastPressedStateLatency,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func touchedStateInputDetailsPayload(_ input: any GCTouchedStateInput) -> TouchedStateInputDetailsPayload {
    TouchedStateInputDetailsPayload(
        touched: input.isTouched,
        lastTouchedStateTimestamp: input.lastTouchedStateTimestamp,
        lastTouchedStateLatency: input.lastTouchedStateLatency,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func relativeInputDetailsPayload(_ input: any GCRelativeInput) -> RelativeInputDetailsPayload {
    RelativeInputDetailsPayload(
        delta: input.delta,
        analog: input.isAnalog,
        lastDeltaTimestamp: input.lastDeltaTimestamp,
        lastDeltaLatency: input.lastDeltaLatency,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func axisInputDetailsPayload(_ input: any GCAxisInput) -> AxisInputDetailsPayload {
    AxisInputDetailsPayload(
        value: input.value,
        analog: input.isAnalog,
        canWrap: input.canWrap,
        lastValueTimestamp: input.lastValueTimestamp,
        lastValueLatency: input.lastValueLatency,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.3, *)
func axis2DInputDetailsPayload(_ input: any GCAxis2DInput) -> Axis2DInputDetailsPayload {
    Axis2DInputDetailsPayload(
        value: point2Payload(input.value),
        analog: input.isAnalog,
        canWrap: input.canWrap,
        lastValueTimestamp: input.lastValueTimestamp,
        lastValueLatency: input.lastValueLatency,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func switchPositionInputDetailsPayload(_ input: any GCSwitchPositionInput) -> SwitchPositionInputDetailsPayload {
    let range = input.positionRange
    return SwitchPositionInputDetailsPayload(
        position: Int32(input.position),
        positionLowerBound: Int32(range.location),
        positionCount: Int32(range.length),
        sequential: input.isSequential,
        canWrap: input.canWrap,
        sources: physicalInputSourceDetailsPayloads(Array(input.sources))
    )
}

@available(macOS 14.0, *)
func linearPressedInputDetailsPayload(_ input: any GCLinearInput & GCPressedStateInput) -> LinearPressedInputDetailsPayload {
    LinearPressedInputDetailsPayload(
        linearInput: linearInputDetailsPayload(input),
        pressedState: pressedStateInputDetailsPayload(input)
    )
}

@available(macOS 14.0, *)
func buttonElementDetailsPayload(_ button: any GCButtonElement) -> ButtonElementDetailsPayload {
    let forceInput: LinearInputDetailsPayload?
    if #available(macOS 26.0, *) {
        forceInput = button.forceInput.map(linearInputDetailsPayload)
    } else {
        forceInput = nil
    }

    return ButtonElementDetailsPayload(
        metadata: inputElementMetadataPayload(button),
        linearInput: linearInputDetailsPayload(button.pressedInput),
        pressedState: pressedStateInputDetailsPayload(button.pressedInput),
        touchedState: button.touchedInput.map(touchedStateInputDetailsPayload),
        forceInput: forceInput
    )
}

@available(macOS 14.0, *)
func axisElementDetailsPayload(_ axis: any GCAxisElement) -> AxisElementDetailsPayload {
    AxisElementDetailsPayload(
        metadata: inputElementMetadataPayload(axis),
        absoluteInput: axis.absoluteInput.map(axisInputDetailsPayload),
        relativeInput: relativeInputDetailsPayload(axis.relativeInput)
    )
}

@available(macOS 14.0, *)
func switchElementDetailsPayload(_ switchElement: any GCSwitchElement) -> SwitchElementDetailsPayload {
    SwitchElementDetailsPayload(
        metadata: inputElementMetadataPayload(switchElement),
        positionInput: switchPositionInputDetailsPayload(switchElement.positionInput)
    )
}

@available(macOS 14.0, *)
func directionPadElementDetailsPayload(_ dpad: any GCDirectionPadElement) -> DirectionPadElementDetailsPayload {
    let xyAxes: Axis2DInputDetailsPayload?
    if #available(macOS 14.3, *) {
        xyAxes = axis2DInputDetailsPayload(dpad.xyAxes)
    } else {
        xyAxes = nil
    }

    return DirectionPadElementDetailsPayload(
        metadata: inputElementMetadataPayload(dpad),
        xyAxes: xyAxes,
        xAxis: axisInputDetailsPayload(dpad.xAxis),
        yAxis: axisInputDetailsPayload(dpad.yAxis),
        up: linearPressedInputDetailsPayload(dpad.up),
        down: linearPressedInputDetailsPayload(dpad.down),
        left: linearPressedInputDetailsPayload(dpad.left),
        right: linearPressedInputDetailsPayload(dpad.right)
    )
}

@available(macOS 14.0, *)
func currentControllerPhysicalInputElementSnapshotPayload() -> PhysicalInputElementSnapshotPayload? {
    guard let current = GCController.current else { return nil }

    let state = current.input.capture()

    var buttons: [ButtonElementDetailsPayload] = []
    for button in state.buttons {
        buttons.append(buttonElementDetailsPayload(button))
    }
    buttons.sort { $0.metadata.primaryAlias < $1.metadata.primaryAlias }

    var axes: [AxisElementDetailsPayload] = []
    for axis in state.axes {
        axes.append(axisElementDetailsPayload(axis))
    }
    axes.sort { $0.metadata.primaryAlias < $1.metadata.primaryAlias }

    var switches: [SwitchElementDetailsPayload] = []
    for switchElement in state.switches {
        switches.append(switchElementDetailsPayload(switchElement))
    }
    switches.sort { $0.metadata.primaryAlias < $1.metadata.primaryAlias }

    var dpads: [DirectionPadElementDetailsPayload] = []
    for dpad in state.dpads {
        dpads.append(directionPadElementDetailsPayload(dpad))
    }
    dpads.sort { $0.metadata.primaryAlias < $1.metadata.primaryAlias }

    return PhysicalInputElementSnapshotPayload(buttons: buttons, axes: axes, switches: switches, dpads: dpads)
}
