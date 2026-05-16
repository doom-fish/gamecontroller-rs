import Foundation
import GameController

@available(macOS 14.0, *)
func inputElementMetadataPayload(_ element: any GCPhysicalInputElement) -> InputElementMetadataPayload {
    let aliases = Array(element.aliases).sorted()
    return InputElementMetadataPayload(
        primaryAlias: aliases.first ?? element.localizedName ?? "",
        aliases: aliases,
        localizedName: element.localizedName,
        sfSymbolsName: element.sfSymbolsName
    )
}

@available(macOS 14.0, *)
func axisElementPayload(_ axis: any GCAxisElement) -> AxisElementStatePayload {
    AxisElementStatePayload(
        absoluteValue: axis.absoluteInput?.value,
        relativeDelta: axis.relativeInput.delta
    )
}

@available(macOS 14.0, *)
func switchPayload(_ switchElement: any GCSwitchElement) -> SwitchInputStatePayload {
    let positionInput = switchElement.positionInput
    let positionRange = positionInput.positionRange
    return SwitchInputStatePayload(
        position: positionInput.position,
        positionLowerBound: positionRange.location,
        positionCount: positionRange.length,
        sequential: positionInput.isSequential,
        canWrap: positionInput.canWrap
    )
}

@available(macOS 14.0, *)
func namedButtonElementStatePayload(_ button: any GCButtonElement) -> NamedButtonElementStatePayload {
    let metadata = inputElementMetadataPayload(button)
    return NamedButtonElementStatePayload(
        primaryAlias: metadata.primaryAlias,
        aliases: metadata.aliases,
        localizedName: metadata.localizedName,
        sfSymbolsName: metadata.sfSymbolsName,
        value: buttonElementPayload(button)
    )
}

@available(macOS 14.0, *)
func namedAxisElementStatePayload(_ axis: any GCAxisElement) -> NamedAxisElementStatePayload {
    let metadata = inputElementMetadataPayload(axis)
    return NamedAxisElementStatePayload(
        primaryAlias: metadata.primaryAlias,
        aliases: metadata.aliases,
        localizedName: metadata.localizedName,
        sfSymbolsName: metadata.sfSymbolsName,
        value: axisElementPayload(axis)
    )
}

@available(macOS 14.0, *)
func namedSwitchElementStatePayload(_ switchElement: any GCSwitchElement) -> NamedSwitchElementStatePayload {
    let metadata = inputElementMetadataPayload(switchElement)
    return NamedSwitchElementStatePayload(
        primaryAlias: metadata.primaryAlias,
        aliases: metadata.aliases,
        localizedName: metadata.localizedName,
        sfSymbolsName: metadata.sfSymbolsName,
        value: switchPayload(switchElement)
    )
}

@available(macOS 14.0, *)
func directionPadElementPayload(_ dpad: any GCDirectionPadElement) -> DirectionPadInputStatePayload {
    DirectionPadInputStatePayload(
        x: dpad.xAxis.value,
        y: dpad.yAxis.value,
        up: ButtonInputStatePayload(value: dpad.up.value, pressed: dpad.up.isPressed, touched: false),
        down: ButtonInputStatePayload(value: dpad.down.value, pressed: dpad.down.isPressed, touched: false),
        left: ButtonInputStatePayload(value: dpad.left.value, pressed: dpad.left.isPressed, touched: false),
        right: ButtonInputStatePayload(value: dpad.right.value, pressed: dpad.right.isPressed, touched: false)
    )
}

@available(macOS 14.0, *)
func namedDirectionPadElementStatePayload(_ dpad: any GCDirectionPadElement) -> NamedDirectionPadElementStatePayload {
    let metadata = inputElementMetadataPayload(dpad)
    return NamedDirectionPadElementStatePayload(
        primaryAlias: metadata.primaryAlias,
        aliases: metadata.aliases,
        localizedName: metadata.localizedName,
        sfSymbolsName: metadata.sfSymbolsName,
        value: directionPadElementPayload(dpad)
    )
}

@available(macOS 14.0, *)
func devicePhysicalInputStateDiffPayload(
    _ changedElements: NSEnumerator?
) -> DevicePhysicalInputStateDiffDetailsPayload {
    guard let changedElements else {
        return DevicePhysicalInputStateDiffDetailsPayload(
            changedElementsKnown: false,
            changedAliases: [],
            changedElements: []
        )
    }

    var changedAliasSet = Set<String>()
    var changedElementPayloads: [InputElementMetadataPayload] = []
    for case let element as any GCPhysicalInputElement in changedElements {
        changedAliasSet.formUnion(element.aliases)
        changedElementPayloads.append(inputElementMetadataPayload(element))
    }
    changedElementPayloads.sort { $0.primaryAlias < $1.primaryAlias }

    return DevicePhysicalInputStateDiffDetailsPayload(
        changedElementsKnown: true,
        changedAliases: changedAliasSet.sorted(),
        changedElements: changedElementPayloads
    )
}

@available(macOS 14.0, *)
func controllerInputStatePayload(
    _ state: GCControllerInputState,
    changedAliasesKnown: Bool = false,
    changedAliases: [String] = []
) -> ControllerInputStateDetailsPayload {
    var buttons: [NamedButtonElementStatePayload] = []
    for button in state.buttons {
        buttons.append(namedButtonElementStatePayload(button))
    }
    buttons.sort { $0.primaryAlias < $1.primaryAlias }

    var axes: [NamedAxisElementStatePayload] = []
    for axis in state.axes {
        axes.append(namedAxisElementStatePayload(axis))
    }
    axes.sort { $0.primaryAlias < $1.primaryAlias }

    var switches: [NamedSwitchElementStatePayload] = []
    for switchElement in state.switches {
        switches.append(namedSwitchElementStatePayload(switchElement))
    }
    switches.sort { $0.primaryAlias < $1.primaryAlias }

    var dpads: [NamedDirectionPadElementStatePayload] = []
    for dpad in state.dpads {
        dpads.append(namedDirectionPadElementStatePayload(dpad))
    }
    dpads.sort { $0.primaryAlias < $1.primaryAlias }

    return ControllerInputStateDetailsPayload(
        lastEventTimestamp: state.lastEventTimestamp,
        lastEventLatency: state.lastEventLatency,
        elementCount: state.elements.count,
        buttonCount: state.buttons.count,
        axisCount: state.axes.count,
        switchCount: state.switches.count,
        dpadCount: state.dpads.count,
        buttons: buttons,
        axes: axes,
        switches: switches,
        dpads: dpads,
        changedAliasesKnown: changedAliasesKnown,
        changedAliases: changedAliases
    )
}

@available(macOS 14.0, *)
func controllerLiveInputPayload(_ input: GCControllerLiveInput) -> ControllerLiveInputDetailsPayload {
    let live = controllerInputStatePayload(input.capture())
    let unmapped = input.unmapped.map { controllerInputStatePayload($0.capture()) }

    let nextState = input.nextInputState()
    let diff = devicePhysicalInputStateDiffPayload(nextState?.changedElements())
    let next = nextState.map {
        controllerInputStatePayload(
            $0,
            changedAliasesKnown: diff.changedElementsKnown,
            changedAliases: diff.changedAliases
        )
    }

    return ControllerLiveInputDetailsPayload(
        inputStateQueueDepth: input.inputStateQueueDepth,
        live: live,
        unmapped: unmapped,
        next: next
    )
}

@available(macOS 14.0, *)
func controllerLiveInputSourcePayload(_ input: GCControllerLiveInput) -> DevicePhysicalInputSourceDetailsPayload {
    let live = controllerInputStatePayload(input)
    let capture = controllerInputStatePayload(input.capture())
    let unmapped = input.unmapped.map { controllerInputStatePayload($0.capture()) }

    let nextState = input.nextInputState()
    let nextDiff = nextState.map { devicePhysicalInputStateDiffPayload($0.changedElements()) }
    let next = nextState.map {
        controllerInputStatePayload(
            $0,
            changedAliasesKnown: nextDiff?.changedElementsKnown ?? false,
            changedAliases: nextDiff?.changedAliases ?? []
        )
    }

    return DevicePhysicalInputSourceDetailsPayload(
        inputStateQueueDepth: input.inputStateQueueDepth,
        live: live,
        capture: capture,
        unmapped: unmapped,
        next: next,
        nextDiff: nextDiff
    )
}

func controllerDetailsPayload(_ controller: GCController) -> ControllerDetailsPayload {
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

    let directionalGamepad: DirectionalGamepadDetailsPayload?
    if #available(macOS 11.1, *) {
        directionalGamepad = (controller.microGamepad as? GCDirectionalGamepad).map(directionalGamepadPayload)
    } else {
        directionalGamepad = nil
    }

    let dualShock: DualShockGamepadDetailsPayload?
    let dualSense: DualSenseGamepadDetailsPayload?
    let xbox: XboxGamepadDetailsPayload?
    let battery: BatteryInfoPayload?
    let motion: MotionDetailsPayload?
    let light: DeviceLightDetailsPayload?
    let haptics: DeviceHapticsDetailsPayload?
    let physicalInput: PhysicalInputProfileDetailsPayload?
    if #available(macOS 11.0, *) {
        dualShock = (controller.extendedGamepad as? GCDualShockGamepad).map(dualShockPayload)
        battery = controller.battery.map {
            BatteryInfoPayload(level: $0.batteryLevel, state: batteryStateName($0))
        }
        motion = controller.motion.map(motionPayload)
        light = controller.light.map(lightPayload)
        haptics = controller.haptics.map(hapticsPayload)

        let profile = controller.physicalInputProfile
        _ = profile.device
        _ = profile.allElements
        _ = profile.allButtons
        _ = profile.allAxes
        _ = profile.allDpads
        _ = profile.allTouchpads
        _ = profile.capture()
        physicalInput = physicalInputPayload(profile)

        if let extendedGamepad = controller.extendedGamepad as? GCXboxGamepad {
            xbox = xboxPayload(extendedGamepad)
        } else {
            xbox = nil
        }
    } else {
        dualShock = nil
        battery = nil
        motion = nil
        light = nil
        haptics = nil
        physicalInput = nil
        xbox = nil
    }

    if #available(macOS 11.3, *) {
        dualSense = (controller.extendedGamepad as? GCDualSenseGamepad).map(dualSensePayload)
    } else {
        dualSense = nil
    }

    let input: ControllerLiveInputDetailsPayload?
    let hasLiveInput: Bool
    if #available(macOS 14.0, *) {
        input = controllerLiveInputPayload(controller.input)
        hasLiveInput = true
    } else {
        input = nil
        hasLiveInput = false
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
        directionalGamepad: directionalGamepad,
        extendedGamepad: controller.extendedGamepad.map(extendedGamepadPayload),
        dualShock: dualShock,
        dualSense: dualSense,
        xbox: xbox,
        battery: battery,
        motion: motion,
        hasLight: light != nil,
        light: light,
        hasHaptics: haptics != nil,
        haptics: haptics,
        physicalInput: physicalInput,
        input: input
    )
}
