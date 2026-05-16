import Foundation
import GameController

func keyboardSnapshotPayload(_ keyboard: GCKeyboard) -> KeyboardSnapshotPayload? {
    guard let input = keyboard.keyboardInput else { return nil }
    let pressedKeys = input.buttons.keys.sorted().compactMap { alias -> NamedButtonInputStatePayload? in
        guard let button = input.buttons[alias], button.isPressed else { return nil }
        return NamedButtonInputStatePayload(alias: alias, value: controllerButtonPayload(button))
    }
    return KeyboardSnapshotPayload(
        vendorName: keyboard.vendorName ?? "Unknown",
        productCategory: keyboard.productCategory,
        anyKeyPressed: input.isAnyKeyPressed,
        pressedAliases: pressedKeys.map(\.alias),
        pressedKeys: pressedKeys,
        physicalInput: physicalInputPayload(input)
    )
}

@available(macOS 11.0, *)
func mouseSnapshotPayload(_ mouse: GCMouse) -> MouseSnapshotPayload? {
    guard let input = mouse.mouseInput else { return nil }
    let knownMice = GCMouse.mice()
    let auxiliaryButtons = (input.auxiliaryButtons ?? []).enumerated().map { index, button in
        NamedButtonInputStatePayload(alias: "auxiliary_\(index)", value: controllerButtonPayload(button))
    }
    return MouseSnapshotPayload(
        vendorName: mouse.vendorName ?? "Unknown",
        productCategory: mouse.productCategory,
        isCurrent: GCMouse.current === mouse,
        knownMouseCount: knownMice.count,
        scroll: directionPadPayload(input.scroll),
        leftButton: controllerButtonPayload(input.leftButton),
        rightButton: input.rightButton.map(controllerButtonPayload),
        middleButton: input.middleButton.map(controllerButtonPayload),
        auxiliaryButtons: auxiliaryButtons,
        physicalInput: physicalInputPayload(input)
    )
}

@available(macOS 13.0, *)
func steeringWheelPayload(_ wheel: GCSteeringWheelElement) -> SteeringWheelDetailsPayload {
    SteeringWheelDetailsPayload(
        maximumDegreesOfRotation: wheel.maximumDegreesOfRotation,
        absoluteValue: wheel.absoluteInput?.value,
        relativeDelta: wheel.relativeInput.delta
    )
}

@available(macOS 13.0, *)
func gearShifterPayload(_ shifter: GCGearShifterElement) -> GearShifterDetailsPayload {
    let aliases = Array(shifter.aliases).sorted()
    let patternRange = shifter.patternInput?.positionRange
    return GearShifterDetailsPayload(
        aliases: aliases,
        localizedName: shifter.localizedName,
        sfSymbolsName: shifter.sfSymbolsName,
        patternPosition: shifter.patternInput?.position,
        patternLowerBound: patternRange?.location,
        patternCount: patternRange?.length,
        patternSequential: shifter.patternInput?.isSequential,
        patternCanWrap: shifter.patternInput?.canWrap,
        sequentialDelta: shifter.sequentialInput?.delta
    )
}

@available(macOS 13.0, *)
func racingWheelInputPayload(_ input: GCRacingWheelInputState) -> RacingWheelInputDetailsPayload {
    RacingWheelInputDetailsPayload(
        wheel: steeringWheelPayload(input.wheel),
        acceleratorPedal: input.acceleratorPedal.map(buttonElementPayload),
        brakePedal: input.brakePedal.map(buttonElementPayload),
        clutchPedal: input.clutchPedal.map(buttonElementPayload),
        shifter: input.shifter.map(gearShifterPayload)
    )
}

@available(macOS 13.0, *)
func racingWheelPayload(_ wheel: GCRacingWheel) -> RacingWheelDetailsPayload {
    let sourceWheel = wheel.isAcquired ? wheel.capture() : wheel
    let wheelInput = sourceWheel.isAcquired ? racingWheelInputPayload(sourceWheel.wheelInput.capture()) : nil
    return RacingWheelDetailsPayload(
        vendorName: sourceWheel.vendorName ?? "Unknown",
        productCategory: sourceWheel.productCategory,
        isAcquired: sourceWheel.isAcquired,
        isSnapshot: sourceWheel.isSnapshot,
        wheelInput: wheelInput
    )
}
