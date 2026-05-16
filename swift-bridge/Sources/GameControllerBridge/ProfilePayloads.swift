import Foundation
import GameController

func controllerButtonPayload(_ button: GCControllerButtonInput) -> ButtonInputStatePayload {
    ButtonInputStatePayload(value: button.value, pressed: button.isPressed, touched: button.isTouched)
}

@available(macOS 13.0, *)
func buttonElementPayload(_ button: any GCButtonElement) -> ButtonInputStatePayload {
    ButtonInputStatePayload(
        value: button.pressedInput.value,
        pressed: button.pressedInput.isPressed,
        touched: button.touchedInput?.isTouched ?? false
    )
}

func axisPayload(_ axis: GCControllerAxisInput) -> AxisInputStatePayload {
    AxisInputStatePayload(value: axis.value)
}

func directionPadPayload(_ dpad: GCControllerDirectionPad) -> DirectionPadInputStatePayload {
    DirectionPadInputStatePayload(
        x: dpad.xAxis.value,
        y: dpad.yAxis.value,
        up: controllerButtonPayload(dpad.up),
        down: controllerButtonPayload(dpad.down),
        left: controllerButtonPayload(dpad.left),
        right: controllerButtonPayload(dpad.right)
    )
}

func touchStateName(_ state: GCControllerTouchpad.TouchState) -> String {
    switch state {
    case .up:
        return "up"
    case .down:
        return "down"
    case .moving:
        return "moving"
    @unknown default:
        return "up"
    }
}

func touchpadPayload(_ touchpad: GCControllerTouchpad) -> TouchpadDetailsPayload {
    TouchpadDetailsPayload(
        button: controllerButtonPayload(touchpad.button),
        touchSurface: directionPadPayload(touchpad.touchSurface),
        touchState: touchStateName(touchpad.touchState),
        reportsAbsoluteTouchSurfaceValues: touchpad.reportsAbsoluteTouchSurfaceValues
    )
}

func batteryStateName(_ battery: GCDeviceBattery) -> String {
    switch battery.batteryState {
    case .unknown:
        return "unknown"
    case .discharging:
        return "discharging"
    case .charging:
        return "charging"
    case .full:
        return "full"
    @unknown default:
        return "unknown"
    }
}

@available(macOS 11.0, *)
func colorPayload(_ color: GCColor) -> ColorPayload {
    ColorPayload(red: color.red, green: color.green, blue: color.blue)
}

@available(macOS 11.0, *)
func lightPayload(_ light: GCDeviceLight) -> DeviceLightDetailsPayload {
    DeviceLightDetailsPayload(color: colorPayload(light.color))
}

@available(macOS 11.0, *)
func hapticsPayload(_ haptics: GCDeviceHaptics) -> DeviceHapticsDetailsPayload {
    DeviceHapticsDetailsPayload(
        supportedLocalities: Array(haptics.supportedLocalities)
            .map { String(describing: $0) }
            .sorted()
    )
}

@available(macOS 11.3, *)
func dualSenseModeName(_ mode: GCDualSenseAdaptiveTrigger.Mode) -> String {
    switch mode.rawValue {
    case 0:
        return "off"
    case 1:
        return "feedback"
    case 2:
        return "weapon"
    case 3:
        return "vibration"
    case 4:
        return "slope_feedback"
    default:
        return "off"
    }
}

@available(macOS 11.3, *)
func dualSenseStatusName(_ status: GCDualSenseAdaptiveTrigger.Status) -> String {
    switch status.rawValue {
    case -1:
        return "unknown"
    case 0:
        return "feedback_no_load"
    case 1:
        return "feedback_load_applied"
    case 2:
        return "weapon_ready"
    case 3:
        return "weapon_firing"
    case 4:
        return "weapon_fired"
    case 5:
        return "vibration_not_vibrating"
    case 6:
        return "vibration_is_vibrating"
    case 7:
        return "slope_feedback_ready"
    case 8:
        return "slope_feedback_applying_load"
    case 9:
        return "slope_feedback_finished"
    default:
        return "unknown"
    }
}

@available(macOS 11.3, *)
func dualSenseTriggerPayload(_ trigger: GCDualSenseAdaptiveTrigger) -> DualSenseAdaptiveTriggerStatePayload {
    DualSenseAdaptiveTriggerStatePayload(
        value: trigger.value,
        pressed: trigger.isPressed,
        touched: trigger.isTouched,
        mode: dualSenseModeName(trigger.mode),
        status: dualSenseStatusName(trigger.status),
        armPosition: trigger.armPosition
    )
}

func gamepadPayload(_ gamepad: GCGamepad) -> GamepadDetailsPayload {
    _ = gamepad.controller
    return GamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: controllerButtonPayload(gamepad.buttonA),
        buttonB: controllerButtonPayload(gamepad.buttonB),
        buttonX: controllerButtonPayload(gamepad.buttonX),
        buttonY: controllerButtonPayload(gamepad.buttonY),
        leftShoulder: controllerButtonPayload(gamepad.leftShoulder),
        rightShoulder: controllerButtonPayload(gamepad.rightShoulder)
    )
}

func microGamepadPayload(_ gamepad: GCMicroGamepad) -> MicroGamepadDetailsPayload {
    _ = gamepad.controller
    return MicroGamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: controllerButtonPayload(gamepad.buttonA),
        buttonX: controllerButtonPayload(gamepad.buttonX),
        buttonMenu: controllerButtonPayload(gamepad.buttonMenu),
        reportsAbsoluteDpadValues: gamepad.reportsAbsoluteDpadValues,
        allowsRotation: gamepad.allowsRotation
    )
}

@available(macOS 11.1, *)
func directionalGamepadPayload(_ gamepad: GCDirectionalGamepad) -> DirectionalGamepadDetailsPayload {
    DirectionalGamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: controllerButtonPayload(gamepad.buttonA),
        buttonX: controllerButtonPayload(gamepad.buttonX),
        buttonMenu: controllerButtonPayload(gamepad.buttonMenu),
        reportsAbsoluteDpadValues: gamepad.reportsAbsoluteDpadValues,
        allowsRotation: gamepad.allowsRotation
    )
}

func extendedGamepadPayload(_ gamepad: GCExtendedGamepad) -> ExtendedGamepadDetailsPayload {
    _ = gamepad.controller
    let buttonHome: ButtonInputStatePayload?
    if #available(macOS 11.0, *) {
        buttonHome = gamepad.buttonHome.map(controllerButtonPayload)
    } else {
        buttonHome = nil
    }

    return ExtendedGamepadDetailsPayload(
        dpad: directionPadPayload(gamepad.dpad),
        buttonA: controllerButtonPayload(gamepad.buttonA),
        buttonB: controllerButtonPayload(gamepad.buttonB),
        buttonX: controllerButtonPayload(gamepad.buttonX),
        buttonY: controllerButtonPayload(gamepad.buttonY),
        buttonMenu: controllerButtonPayload(gamepad.buttonMenu),
        buttonOptions: gamepad.buttonOptions.map(controllerButtonPayload),
        buttonHome: buttonHome,
        leftThumbstick: directionPadPayload(gamepad.leftThumbstick),
        rightThumbstick: directionPadPayload(gamepad.rightThumbstick),
        leftShoulder: controllerButtonPayload(gamepad.leftShoulder),
        rightShoulder: controllerButtonPayload(gamepad.rightShoulder),
        leftTrigger: controllerButtonPayload(gamepad.leftTrigger),
        rightTrigger: controllerButtonPayload(gamepad.rightTrigger),
        leftThumbstickButton: gamepad.leftThumbstickButton.map(controllerButtonPayload),
        rightThumbstickButton: gamepad.rightThumbstickButton.map(controllerButtonPayload)
    )
}

@available(macOS 11.0, *)
func dualShockPayload(_ gamepad: GCDualShockGamepad) -> DualShockGamepadDetailsPayload {
    DualShockGamepadDetailsPayload(
        touchpadButton: controllerButtonPayload(gamepad.touchpadButton),
        touchpadPrimary: directionPadPayload(gamepad.touchpadPrimary),
        touchpadSecondary: directionPadPayload(gamepad.touchpadSecondary)
    )
}

@available(macOS 11.3, *)
func dualSensePayload(_ gamepad: GCDualSenseGamepad) -> DualSenseGamepadDetailsPayload {
    DualSenseGamepadDetailsPayload(
        touchpadButton: controllerButtonPayload(gamepad.touchpadButton),
        touchpadPrimary: directionPadPayload(gamepad.touchpadPrimary),
        touchpadSecondary: directionPadPayload(gamepad.touchpadSecondary),
        leftTrigger: dualSenseTriggerPayload(gamepad.leftTrigger),
        rightTrigger: dualSenseTriggerPayload(gamepad.rightTrigger)
    )
}

@available(macOS 11.0, *)
func xboxPayload(_ gamepad: GCXboxGamepad) -> XboxGamepadDetailsPayload {
    let buttonShare: ButtonInputStatePayload?
    if #available(macOS 12.0, *) {
        buttonShare = gamepad.buttonShare.map(controllerButtonPayload)
    } else {
        buttonShare = nil
    }

    return XboxGamepadDetailsPayload(
        paddleButton1: gamepad.paddleButton1.map(controllerButtonPayload),
        paddleButton2: gamepad.paddleButton2.map(controllerButtonPayload),
        paddleButton3: gamepad.paddleButton3.map(controllerButtonPayload),
        paddleButton4: gamepad.paddleButton4.map(controllerButtonPayload),
        buttonShare: buttonShare
    )
}

@available(macOS 11.0, *)
func motionPayload(_ motion: GCMotion) -> MotionDetailsPayload {
    _ = motion.controller
    return MotionDetailsPayload(
        sensorsRequireManualActivation: motion.sensorsRequireManualActivation,
        sensorsActive: motion.sensorsActive,
        hasGravityAndUserAcceleration: motion.hasGravityAndUserAcceleration,
        gravity: Vector3Payload(x: motion.gravity.x, y: motion.gravity.y, z: motion.gravity.z),
        userAcceleration: Vector3Payload(
            x: motion.userAcceleration.x,
            y: motion.userAcceleration.y,
            z: motion.userAcceleration.z
        ),
        acceleration: Vector3Payload(
            x: motion.acceleration.x,
            y: motion.acceleration.y,
            z: motion.acceleration.z
        ),
        hasAttitude: motion.hasAttitude,
        hasRotationRate: motion.hasRotationRate,
        attitude: QuaternionPayload(
            x: motion.attitude.x,
            y: motion.attitude.y,
            z: motion.attitude.z,
            w: motion.attitude.w
        ),
        rotationRate: Vector3Payload(
            x: motion.rotationRate.x,
            y: motion.rotationRate.y,
            z: motion.rotationRate.z
        )
    )
}

@available(macOS 11.0, *)
func physicalInputPayload(_ profile: GCPhysicalInputProfile) -> PhysicalInputProfileDetailsPayload {
    let elementAliases = profile.elements.keys.sorted()
    let buttonAliases = profile.buttons.keys.sorted()
    let axisAliases = profile.axes.keys.sorted()
    let dpadAliases = profile.dpads.keys.sorted()
    let touchpadAliases = profile.touchpads.keys.sorted()

    let buttons = buttonAliases.compactMap { alias -> NamedButtonInputStatePayload? in
        guard let button = profile.buttons[alias] else { return nil }
        let _ = profile[alias]
        return NamedButtonInputStatePayload(alias: alias, value: controllerButtonPayload(button))
    }
    let axes = axisAliases.compactMap { alias -> NamedAxisInputStatePayload? in
        guard let axis = profile.axes[alias] else { return nil }
        let _ = profile[alias]
        return NamedAxisInputStatePayload(alias: alias, value: axisPayload(axis))
    }
    let dpads = dpadAliases.compactMap { alias -> NamedDirectionPadStatePayload? in
        guard let dpad = profile.dpads[alias] else { return nil }
        let _ = profile[alias]
        return NamedDirectionPadStatePayload(alias: alias, value: directionPadPayload(dpad))
    }
    let touchpads = touchpadAliases.compactMap { alias -> NamedTouchpadStatePayload? in
        guard let touchpad = profile.touchpads[alias] else { return nil }
        let _ = profile[alias]
        return NamedTouchpadStatePayload(alias: alias, value: touchpadPayload(touchpad))
    }

    let hasRemappedElements: Bool
    if #available(macOS 12.0, *) {
        hasRemappedElements = profile.hasRemappedElements
        for alias in elementAliases {
            _ = profile.mappedElementAlias(forPhysicalInputName: alias)
            _ = profile.mappedPhysicalInputNames(forElementAlias: alias)
        }
    } else {
        hasRemappedElements = false
    }

    return PhysicalInputProfileDetailsPayload(
        lastEventTimestamp: profile.lastEventTimestamp,
        hasRemappedElements: hasRemappedElements,
        elementAliases: elementAliases,
        buttonAliases: buttonAliases,
        axisAliases: axisAliases,
        dpadAliases: dpadAliases,
        touchpadAliases: touchpadAliases,
        buttons: buttons,
        axes: axes,
        dpads: dpads,
        touchpads: touchpads
    )
}
