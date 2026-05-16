import Foundation
import GameController

import Foundation
import GameController

struct ButtonInputStatePayload: Codable {
    let value: Float
    let pressed: Bool
    let touched: Bool
}

struct AxisInputStatePayload: Codable {
    let value: Float
}

struct DirectionPadInputStatePayload: Codable {
    let x: Float
    let y: Float
    let up: ButtonInputStatePayload
    let down: ButtonInputStatePayload
    let left: ButtonInputStatePayload
    let right: ButtonInputStatePayload
}

struct TouchpadDetailsPayload: Codable {
    let button: ButtonInputStatePayload
    let touchSurface: DirectionPadInputStatePayload
    let touchState: String
    let reportsAbsoluteTouchSurfaceValues: Bool
}

struct GamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonB: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonY: ButtonInputStatePayload
    let leftShoulder: ButtonInputStatePayload
    let rightShoulder: ButtonInputStatePayload
}

struct MicroGamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonMenu: ButtonInputStatePayload?
    let reportsAbsoluteDpadValues: Bool
    let allowsRotation: Bool
}

struct DirectionalGamepadDetailsPayload: Codable {
    let dpad: DirectionPadInputStatePayload
    let buttonA: ButtonInputStatePayload
    let buttonX: ButtonInputStatePayload
    let buttonMenu: ButtonInputStatePayload?
    let reportsAbsoluteDpadValues: Bool
    let allowsRotation: Bool
}

struct ExtendedGamepadDetailsPayload: Codable {
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

struct DualShockGamepadDetailsPayload: Codable {
    let touchpadButton: ButtonInputStatePayload
    let touchpadPrimary: DirectionPadInputStatePayload
    let touchpadSecondary: DirectionPadInputStatePayload
}

struct DualSenseAdaptiveTriggerStatePayload: Codable {
    let value: Float
    let pressed: Bool
    let touched: Bool
    let mode: String
    let status: String
    let armPosition: Float
}

struct DualSenseGamepadDetailsPayload: Codable {
    let touchpadButton: ButtonInputStatePayload
    let touchpadPrimary: DirectionPadInputStatePayload
    let touchpadSecondary: DirectionPadInputStatePayload
    let leftTrigger: DualSenseAdaptiveTriggerStatePayload
    let rightTrigger: DualSenseAdaptiveTriggerStatePayload
}

struct XboxGamepadDetailsPayload: Codable {
    let paddleButton1: ButtonInputStatePayload?
    let paddleButton2: ButtonInputStatePayload?
    let paddleButton3: ButtonInputStatePayload?
    let paddleButton4: ButtonInputStatePayload?
    let buttonShare: ButtonInputStatePayload?
}

struct BatteryInfoPayload: Codable {
    let level: Float
    let state: String
}

struct ColorPayload: Codable {
    let red: Float
    let green: Float
    let blue: Float
}

struct DeviceLightDetailsPayload: Codable {
    let color: ColorPayload
}

struct DeviceHapticsDetailsPayload: Codable {
    let supportedLocalities: [String]
}

struct Vector3Payload: Codable {
    let x: Double
    let y: Double
    let z: Double
}

struct QuaternionPayload: Codable {
    let x: Double
    let y: Double
    let z: Double
    let w: Double
}

struct MotionDetailsPayload: Codable {
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

struct NamedButtonInputStatePayload: Codable {
    let alias: String
    let value: ButtonInputStatePayload
}

struct NamedAxisInputStatePayload: Codable {
    let alias: String
    let value: AxisInputStatePayload
}

struct NamedDirectionPadStatePayload: Codable {
    let alias: String
    let value: DirectionPadInputStatePayload
}

struct NamedTouchpadStatePayload: Codable {
    let alias: String
    let value: TouchpadDetailsPayload
}

struct PhysicalInputProfileDetailsPayload: Codable {
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

struct InputElementMetadataPayload: Codable {
    let primaryAlias: String
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
}

struct DevicePhysicalInputStateDiffDetailsPayload: Codable {
    let changedElementsKnown: Bool
    let changedAliases: [String]
    let changedElements: [InputElementMetadataPayload]
}

struct AxisElementStatePayload: Codable {
    let absoluteValue: Float?
    let relativeDelta: Float
}

struct SwitchInputStatePayload: Codable {
    let position: Int
    let positionLowerBound: Int
    let positionCount: Int
    let sequential: Bool
    let canWrap: Bool
}

struct NamedButtonElementStatePayload: Codable {
    let primaryAlias: String
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
    let value: ButtonInputStatePayload
}

struct NamedAxisElementStatePayload: Codable {
    let primaryAlias: String
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
    let value: AxisElementStatePayload
}

struct NamedSwitchElementStatePayload: Codable {
    let primaryAlias: String
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
    let value: SwitchInputStatePayload
}

struct NamedDirectionPadElementStatePayload: Codable {
    let primaryAlias: String
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
    let value: DirectionPadInputStatePayload
}

struct ControllerInputStateDetailsPayload: Codable {
    let lastEventTimestamp: Double
    let lastEventLatency: Double
    let elementCount: Int
    let buttonCount: Int
    let axisCount: Int
    let switchCount: Int
    let dpadCount: Int
    let buttons: [NamedButtonElementStatePayload]
    let axes: [NamedAxisElementStatePayload]
    let switches: [NamedSwitchElementStatePayload]
    let dpads: [NamedDirectionPadElementStatePayload]
    let changedAliasesKnown: Bool
    let changedAliases: [String]
}

struct ControllerLiveInputDetailsPayload: Codable {
    let inputStateQueueDepth: Int
    let live: ControllerInputStateDetailsPayload
    let unmapped: ControllerInputStateDetailsPayload?
    let next: ControllerInputStateDetailsPayload?
}

struct DevicePhysicalInputSourceDetailsPayload: Codable {
    let inputStateQueueDepth: Int
    let live: ControllerInputStateDetailsPayload
    let capture: ControllerInputStateDetailsPayload
    let unmapped: ControllerInputStateDetailsPayload?
    let next: ControllerInputStateDetailsPayload?
    let nextDiff: DevicePhysicalInputStateDiffDetailsPayload?
}

struct ControllerDetailsPayload: Codable {
    let vendorName: String
    let productCategory: String
    let playerIndex: Int32
    let isAttachedToDevice: Bool
    let isCurrent: Bool
    let supportsBackgroundEvents: Bool
    let hasLiveInput: Bool
    let gamepad: GamepadDetailsPayload?
    let microGamepad: MicroGamepadDetailsPayload?
    let directionalGamepad: DirectionalGamepadDetailsPayload?
    let extendedGamepad: ExtendedGamepadDetailsPayload?
    let dualShock: DualShockGamepadDetailsPayload?
    let dualSense: DualSenseGamepadDetailsPayload?
    let xbox: XboxGamepadDetailsPayload?
    let battery: BatteryInfoPayload?
    let motion: MotionDetailsPayload?
    let hasLight: Bool
    let light: DeviceLightDetailsPayload?
    let hasHaptics: Bool
    let haptics: DeviceHapticsDetailsPayload?
    let physicalInput: PhysicalInputProfileDetailsPayload?
    let input: ControllerLiveInputDetailsPayload?
}

struct KeyboardSnapshotPayload: Codable {
    let vendorName: String
    let productCategory: String
    let anyKeyPressed: Bool
    let pressedAliases: [String]
    let pressedKeys: [NamedButtonInputStatePayload]
    let physicalInput: PhysicalInputProfileDetailsPayload
}

struct MouseSnapshotPayload: Codable {
    let vendorName: String
    let productCategory: String
    let isCurrent: Bool
    let knownMouseCount: Int
    let scroll: DirectionPadInputStatePayload
    let leftButton: ButtonInputStatePayload
    let rightButton: ButtonInputStatePayload?
    let middleButton: ButtonInputStatePayload?
    let auxiliaryButtons: [NamedButtonInputStatePayload]
    let physicalInput: PhysicalInputProfileDetailsPayload
}

struct EventViewControllerPayload: Codable {
    let controllerUserInteractionEnabled: Bool
}

struct SteeringWheelDetailsPayload: Codable {
    let maximumDegreesOfRotation: Float
    let absoluteValue: Float?
    let relativeDelta: Float
}

struct GearShifterDetailsPayload: Codable {
    let aliases: [String]
    let localizedName: String?
    let sfSymbolsName: String?
    let patternPosition: Int?
    let patternLowerBound: Int?
    let patternCount: Int?
    let patternSequential: Bool?
    let patternCanWrap: Bool?
    let sequentialDelta: Float?
}

struct RacingWheelInputDetailsPayload: Codable {
    let wheel: SteeringWheelDetailsPayload
    let acceleratorPedal: ButtonInputStatePayload?
    let brakePedal: ButtonInputStatePayload?
    let clutchPedal: ButtonInputStatePayload?
    let shifter: GearShifterDetailsPayload?
}

struct RacingWheelDetailsPayload: Codable {
    let vendorName: String
    let productCategory: String
    let isAcquired: Bool
    let isSnapshot: Bool
    let wheelInput: RacingWheelInputDetailsPayload?
}

