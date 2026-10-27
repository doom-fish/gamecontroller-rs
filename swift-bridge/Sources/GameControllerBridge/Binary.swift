import Foundation
import GameControllerABI

final class GCArena {
    private var releases: [() -> Void] = []
    func keep<T>(_ value: T) -> UnsafePointer<T> {
        let pointer = UnsafeMutablePointer<T>.allocate(capacity: 1)
        pointer.initialize(to: value)
        releases.append { pointer.deinitialize(count: 1); pointer.deallocate() }
        return UnsafePointer(pointer)
    }
    func array<T>(_ values: [T]) -> GCSliceRaw {
        guard !values.isEmpty else { return GCSliceRaw(data: nil, len: 0) }
        let pointer = UnsafeMutablePointer<T>.allocate(capacity: values.count)
        pointer.initialize(from: values, count: values.count)
        releases.append { pointer.deinitialize(count: values.count); pointer.deallocate() }
        return GCSliceRaw(data: UnsafeRawPointer(pointer), len: values.count)
    }
    func string(_ value: String) -> GCStringRaw {
        let bytes = array(Array(value.utf8))
        return GCStringRaw(data: bytes.data?.assumingMemoryBound(to: UInt8.self), len: bytes.len)
    }
    deinit { for release in releases.reversed() { release() } }
}
protocol GCABIValue { associatedtype Raw; func raw(_ arena: GCArena) -> Raw }
func gcSnapshot<T: GCABIValue>(_ values: [T]) -> GCSnapshotRaw {
    let arena = GCArena()
    let data = arena.array(values.map { $0.raw(arena) })
    return GCSnapshotRaw(data: data.data, len: data.len, owner: Unmanaged.passRetained(arena).toOpaque())
}
@_cdecl("gc_snapshot_free")
public func gc_snapshot_free(_ owner: UnsafeMutableRawPointer?) {
    guard let owner else { return }
    Unmanaged<GCArena>.fromOpaque(owner).release()
}
extension Vector3Payload: GCABIValue {
    func raw(_ arena: GCArena) -> GCVector3Raw {
        GCVector3Raw(
            x: x,
            y: y,
            z: z
        )
    }
}
extension QuaternionPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCQuaternionRaw {
        GCQuaternionRaw(
            x: x,
            y: y,
            z: z,
            w: w
        )
    }
}
extension ColorPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCColorRaw {
        GCColorRaw(
            red: red,
            green: green,
            blue: blue
        )
    }
}
extension ButtonInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCButtonInputStateRaw {
        GCButtonInputStateRaw(
            value: value,
            pressed: pressed ? 1 : 0,
            touched: touched ? 1 : 0
        )
    }
}
extension AxisInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCAxisInputStateRaw {
        GCAxisInputStateRaw(
            value: value
        )
    }
}
extension DirectionPadInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDirectionPadInputStateRaw {
        GCDirectionPadInputStateRaw(
            x: x,
            y: y,
            up: up.raw(arena),
            down: down.raw(arena),
            left: left.raw(arena),
            right: right.raw(arena)
        )
    }
}
extension TouchpadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCTouchpadDetailsRaw {
        GCTouchpadDetailsRaw(
            button: button.raw(arena),
            touch_surface: touchSurface.raw(arena),
            touch_state: ["up": Int32(0), "down": Int32(1), "moving": Int32(2)][touchState] ?? -1,
            reports_absolute_touch_surface_values: reportsAbsoluteTouchSurfaceValues ? 1 : 0
        )
    }
}
extension GamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCGamepadDetailsRaw {
        GCGamepadDetailsRaw(
            dpad: dpad.raw(arena),
            button_a: buttonA.raw(arena),
            button_b: buttonB.raw(arena),
            button_x: buttonX.raw(arena),
            button_y: buttonY.raw(arena),
            left_shoulder: leftShoulder.raw(arena),
            right_shoulder: rightShoulder.raw(arena)
        )
    }
}
extension MicroGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCMicroGamepadDetailsRaw {
        GCMicroGamepadDetailsRaw(
            dpad: dpad.raw(arena),
            button_a: buttonA.raw(arena),
            button_x: buttonX.raw(arena),
            button_menu: buttonMenu.map { arena.keep($0.raw(arena)) },
            reports_absolute_dpad_values: reportsAbsoluteDpadValues ? 1 : 0,
            allows_rotation: allowsRotation ? 1 : 0
        )
    }
}
extension DirectionalGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDirectionalGamepadDetailsRaw {
        GCDirectionalGamepadDetailsRaw(
            dpad: dpad.raw(arena),
            button_a: buttonA.raw(arena),
            button_x: buttonX.raw(arena),
            button_menu: buttonMenu.map { arena.keep($0.raw(arena)) },
            reports_absolute_dpad_values: reportsAbsoluteDpadValues ? 1 : 0,
            allows_rotation: allowsRotation ? 1 : 0
        )
    }
}
extension ExtendedGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCExtendedGamepadDetailsRaw {
        GCExtendedGamepadDetailsRaw(
            dpad: dpad.raw(arena),
            button_a: buttonA.raw(arena),
            button_b: buttonB.raw(arena),
            button_x: buttonX.raw(arena),
            button_y: buttonY.raw(arena),
            button_menu: buttonMenu.map { arena.keep($0.raw(arena)) },
            button_options: buttonOptions.map { arena.keep($0.raw(arena)) },
            button_home: buttonHome.map { arena.keep($0.raw(arena)) },
            left_thumbstick: leftThumbstick.raw(arena),
            right_thumbstick: rightThumbstick.raw(arena),
            left_shoulder: leftShoulder.raw(arena),
            right_shoulder: rightShoulder.raw(arena),
            left_trigger: leftTrigger.raw(arena),
            right_trigger: rightTrigger.raw(arena),
            left_thumbstick_button: leftThumbstickButton.map { arena.keep($0.raw(arena)) },
            right_thumbstick_button: rightThumbstickButton.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension DualShockGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDualShockGamepadDetailsRaw {
        GCDualShockGamepadDetailsRaw(
            touchpad_button: touchpadButton.raw(arena),
            touchpad_primary: touchpadPrimary.raw(arena),
            touchpad_secondary: touchpadSecondary.raw(arena)
        )
    }
}
extension DualSenseAdaptiveTriggerStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDualSenseAdaptiveTriggerStateRaw {
        GCDualSenseAdaptiveTriggerStateRaw(
            value: value,
            pressed: pressed ? 1 : 0,
            touched: touched ? 1 : 0,
            mode: ["off": Int32(0), "feedback": Int32(1), "weapon": Int32(2), "vibration": Int32(3), "slope_feedback": Int32(4)][mode] ?? -1,
            status: ["unknown": Int32(0), "feedback_no_load": Int32(1), "feedback_load_applied": Int32(2), "weapon_ready": Int32(3), "weapon_firing": Int32(4), "weapon_fired": Int32(5), "vibration_not_vibrating": Int32(6), "vibration_is_vibrating": Int32(7), "slope_feedback_ready": Int32(8), "slope_feedback_applying_load": Int32(9), "slope_feedback_finished": Int32(10)][status] ?? -1,
            arm_position: armPosition
        )
    }
}
extension DualSenseGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDualSenseGamepadDetailsRaw {
        GCDualSenseGamepadDetailsRaw(
            touchpad_button: touchpadButton.raw(arena),
            touchpad_primary: touchpadPrimary.raw(arena),
            touchpad_secondary: touchpadSecondary.raw(arena),
            left_trigger: leftTrigger.raw(arena),
            right_trigger: rightTrigger.raw(arena)
        )
    }
}
extension XboxGamepadDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCXboxGamepadDetailsRaw {
        GCXboxGamepadDetailsRaw(
            paddle_button1: paddleButton1.map { arena.keep($0.raw(arena)) },
            paddle_button2: paddleButton2.map { arena.keep($0.raw(arena)) },
            paddle_button3: paddleButton3.map { arena.keep($0.raw(arena)) },
            paddle_button4: paddleButton4.map { arena.keep($0.raw(arena)) },
            button_share: buttonShare.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension BatteryInfoPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCBatteryInfoRaw {
        GCBatteryInfoRaw(
            level: level,
            state: ["unknown": Int32(0), "discharging": Int32(1), "charging": Int32(2), "full": Int32(3)][state] ?? -1
        )
    }
}
extension DeviceLightDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDeviceLightDetailsRaw {
        GCDeviceLightDetailsRaw(
            color: color.raw(arena)
        )
    }
}
extension DeviceHapticsDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDeviceHapticsDetailsRaw {
        GCDeviceHapticsDetailsRaw(
            supported_localities: arena.array(supportedLocalities.map { arena.string($0) })
        )
    }
}
extension DeviceDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDeviceDetailsRaw {
        GCDeviceDetailsRaw(
            vendor_name: arena.string(vendorName),
            product_category: arena.string(productCategory),
            handler_queue_label: arena.string(handlerQueueLabel)
        )
    }
}
extension ConnectedDevicesSnapshotPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCConnectedDevicesSnapshotRaw {
        GCConnectedDevicesSnapshotRaw(
            controllers: arena.array(controllers.map { $0.raw(arena) }),
            keyboard: keyboard.map { arena.keep($0.raw(arena)) },
            mouse: mouse.map { arena.keep($0.raw(arena)) },
            racing_wheels: arena.array(racingWheels.map { $0.raw(arena) })
        )
    }
}
extension MotionDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCMotionDetailsRaw {
        GCMotionDetailsRaw(
            sensors_require_manual_activation: sensorsRequireManualActivation ? 1 : 0,
            sensors_active: sensorsActive ? 1 : 0,
            has_gravity_and_user_acceleration: hasGravityAndUserAcceleration ? 1 : 0,
            gravity: gravity.raw(arena),
            user_acceleration: userAcceleration.raw(arena),
            acceleration: acceleration.raw(arena),
            has_attitude: hasAttitude ? 1 : 0,
            has_rotation_rate: hasRotationRate ? 1 : 0,
            attitude: attitude.raw(arena),
            rotation_rate: rotationRate.raw(arena)
        )
    }
}
extension NamedButtonInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedButtonInputStateRaw {
        GCNamedButtonInputStateRaw(
            alias: arena.string(alias),
            value: value.raw(arena)
        )
    }
}
extension NamedAxisInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedAxisInputStateRaw {
        GCNamedAxisInputStateRaw(
            alias: arena.string(alias),
            value: value.raw(arena)
        )
    }
}
extension NamedDirectionPadStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedDirectionPadStateRaw {
        GCNamedDirectionPadStateRaw(
            alias: arena.string(alias),
            value: value.raw(arena)
        )
    }
}
extension NamedTouchpadStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedTouchpadStateRaw {
        GCNamedTouchpadStateRaw(
            alias: arena.string(alias),
            value: value.raw(arena)
        )
    }
}
extension PhysicalInputProfileDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPhysicalInputProfileDetailsRaw {
        GCPhysicalInputProfileDetailsRaw(
            last_event_timestamp: lastEventTimestamp,
            has_remapped_elements: hasRemappedElements ? 1 : 0,
            element_aliases: arena.array(elementAliases.map { arena.string($0) }),
            button_aliases: arena.array(buttonAliases.map { arena.string($0) }),
            axis_aliases: arena.array(axisAliases.map { arena.string($0) }),
            dpad_aliases: arena.array(dpadAliases.map { arena.string($0) }),
            touchpad_aliases: arena.array(touchpadAliases.map { arena.string($0) }),
            buttons: arena.array(buttons.map { $0.raw(arena) }),
            axes: arena.array(axes.map { $0.raw(arena) }),
            dpads: arena.array(dpads.map { $0.raw(arena) }),
            touchpads: arena.array(touchpads.map { $0.raw(arena) })
        )
    }
}
extension AxisElementStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCAxisElementStateRaw {
        GCAxisElementStateRaw(
            absolute_value: absoluteValue.map { arena.keep($0) },
            relative_delta: relativeDelta
        )
    }
}
extension SwitchInputStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCSwitchInputStateRaw {
        GCSwitchInputStateRaw(
            position: Int32(position),
            position_lower_bound: Int32(positionLowerBound),
            position_count: Int32(positionCount),
            sequential: sequential ? 1 : 0,
            can_wrap: canWrap ? 1 : 0
        )
    }
}
extension NamedButtonElementStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedButtonElementStateRaw {
        GCNamedButtonElementStateRaw(
            primary_alias: arena.string(primaryAlias),
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            value: value.raw(arena)
        )
    }
}
extension NamedAxisElementStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedAxisElementStateRaw {
        GCNamedAxisElementStateRaw(
            primary_alias: arena.string(primaryAlias),
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            value: value.raw(arena)
        )
    }
}
extension NamedSwitchElementStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedSwitchElementStateRaw {
        GCNamedSwitchElementStateRaw(
            primary_alias: arena.string(primaryAlias),
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            value: value.raw(arena)
        )
    }
}
extension NamedDirectionPadElementStatePayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedDirectionPadElementStateRaw {
        GCNamedDirectionPadElementStateRaw(
            primary_alias: arena.string(primaryAlias),
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            value: value.raw(arena)
        )
    }
}
extension InputElementMetadataPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCInputElementMetadataRaw {
        GCInputElementMetadataRaw(
            primary_alias: arena.string(primaryAlias),
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) }
        )
    }
}
extension ControllerElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCControllerElementDetailsRaw {
        GCControllerElementDetailsRaw(
            analog: analog ? 1 : 0,
            collection_path: arena.array(collectionPath.map { arena.string($0) }),
            is_bound_to_system_gesture: isBoundToSystemGesture ? 1 : 0,
            preferred_system_gesture_state: ["enabled": Int32(0), "always_receive": Int32(1), "disabled": Int32(2)][preferredSystemGestureState] ?? -1,
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            unmapped_sf_symbols_name: unmappedSfSymbolsName.map { arena.keep(arena.string($0)) },
            unmapped_localized_name: unmappedLocalizedName.map { arena.keep(arena.string($0)) }
        )
    }
}
extension NamedControllerElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCNamedControllerElementDetailsRaw {
        GCNamedControllerElementDetailsRaw(
            name: arena.string(name),
            value: value.raw(arena)
        )
    }
}
extension PhysicalInputSourceDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPhysicalInputSourceDetailsRaw {
        GCPhysicalInputSourceDetailsRaw(
            element_aliases: arena.array(elementAliases.map { arena.string($0) }),
            element_localized_name: elementLocalizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            direction: direction
        )
    }
}
extension PhysicalInputExtentsDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPhysicalInputExtentsDetailsRaw {
        GCPhysicalInputExtentsDetailsRaw(
            scaled_value: scaledValue,
            minimum_value: minimumValue,
            maximum_value: maximumValue
        )
    }
}
extension LinearInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCLinearInputDetailsRaw {
        GCLinearInputDetailsRaw(
            value: value,
            analog: analog ? 1 : 0,
            can_wrap: canWrap ? 1 : 0,
            last_value_timestamp: lastValueTimestamp,
            last_value_latency: lastValueLatency,
            physical_extents: physicalExtents.map { arena.keep($0.raw(arena)) },
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension PressedStateInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPressedStateInputDetailsRaw {
        GCPressedStateInputDetailsRaw(
            pressed: pressed ? 1 : 0,
            last_pressed_state_timestamp: lastPressedStateTimestamp,
            last_pressed_state_latency: lastPressedStateLatency,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension TouchedStateInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCTouchedStateInputDetailsRaw {
        GCTouchedStateInputDetailsRaw(
            touched: touched ? 1 : 0,
            last_touched_state_timestamp: lastTouchedStateTimestamp,
            last_touched_state_latency: lastTouchedStateLatency,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension RelativeInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCRelativeInputDetailsRaw {
        GCRelativeInputDetailsRaw(
            delta: delta,
            analog: analog ? 1 : 0,
            last_delta_timestamp: lastDeltaTimestamp,
            last_delta_latency: lastDeltaLatency,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension AxisInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCAxisInputDetailsRaw {
        GCAxisInputDetailsRaw(
            value: value,
            analog: analog ? 1 : 0,
            can_wrap: canWrap ? 1 : 0,
            last_value_timestamp: lastValueTimestamp,
            last_value_latency: lastValueLatency,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension Axis2DInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCAxis2DInputDetailsRaw {
        GCAxis2DInputDetailsRaw(
            value: value.raw(arena),
            analog: analog ? 1 : 0,
            can_wrap: canWrap ? 1 : 0,
            last_value_timestamp: lastValueTimestamp,
            last_value_latency: lastValueLatency,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension SwitchPositionInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCSwitchPositionInputDetailsRaw {
        GCSwitchPositionInputDetailsRaw(
            position: position,
            position_lower_bound: positionLowerBound,
            position_count: positionCount,
            sequential: sequential ? 1 : 0,
            can_wrap: canWrap ? 1 : 0,
            sources: arena.array(sources.map { $0.raw(arena) })
        )
    }
}
extension LinearPressedInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCLinearPressedInputDetailsRaw {
        GCLinearPressedInputDetailsRaw(
            linear_input: linearInput.raw(arena),
            pressed_state: pressedState.raw(arena)
        )
    }
}
extension ButtonElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCButtonElementDetailsRaw {
        GCButtonElementDetailsRaw(
            metadata: metadata.raw(arena),
            linear_input: linearInput.raw(arena),
            pressed_state: pressedState.raw(arena),
            touched_state: touchedState.map { arena.keep($0.raw(arena)) },
            force_input: forceInput.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension AxisElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCAxisElementDetailsRaw {
        GCAxisElementDetailsRaw(
            metadata: metadata.raw(arena),
            absolute_input: absoluteInput.map { arena.keep($0.raw(arena)) },
            relative_input: relativeInput.raw(arena)
        )
    }
}
extension SwitchElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCSwitchElementDetailsRaw {
        GCSwitchElementDetailsRaw(
            metadata: metadata.raw(arena),
            position_input: positionInput.raw(arena)
        )
    }
}
extension DirectionPadElementDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDirectionPadElementDetailsRaw {
        GCDirectionPadElementDetailsRaw(
            metadata: metadata.raw(arena),
            xy_axes: xyAxes.map { arena.keep($0.raw(arena)) },
            x_axis: xAxis.raw(arena),
            y_axis: yAxis.raw(arena),
            up: up.raw(arena),
            down: down.raw(arena),
            left: left.raw(arena),
            right: right.raw(arena)
        )
    }
}
extension PhysicalInputElementSnapshotPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPhysicalInputElementSnapshotRaw {
        GCPhysicalInputElementSnapshotRaw(
            buttons: arena.array(buttons.map { $0.raw(arena) }),
            axes: arena.array(axes.map { $0.raw(arena) }),
            switches: arena.array(switches.map { $0.raw(arena) }),
            dpads: arena.array(dpads.map { $0.raw(arena) })
        )
    }
}
extension DevicePhysicalInputStateDiffDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDevicePhysicalInputStateDiffDetailsRaw {
        GCDevicePhysicalInputStateDiffDetailsRaw(
            changed_elements_known: changedElementsKnown ? 1 : 0,
            changed_aliases: arena.array(changedAliases.map { arena.string($0) }),
            changed_elements: arena.array(changedElements.map { $0.raw(arena) })
        )
    }
}
extension ControllerInputStateDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCControllerInputStateDetailsRaw {
        GCControllerInputStateDetailsRaw(
            last_event_timestamp: lastEventTimestamp,
            last_event_latency: lastEventLatency,
            element_count: elementCount,
            button_count: buttonCount,
            axis_count: axisCount,
            switch_count: switchCount,
            dpad_count: dpadCount,
            buttons: arena.array(buttons.map { $0.raw(arena) }),
            axes: arena.array(axes.map { $0.raw(arena) }),
            switches: arena.array(switches.map { $0.raw(arena) }),
            dpads: arena.array(dpads.map { $0.raw(arena) }),
            changed_aliases_known: changedAliasesKnown ? 1 : 0,
            changed_aliases: arena.array(changedAliases.map { arena.string($0) })
        )
    }
}
extension ControllerLiveInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCControllerLiveInputDetailsRaw {
        GCControllerLiveInputDetailsRaw(
            input_state_queue_depth: inputStateQueueDepth,
            live: live.raw(arena),
            unmapped: unmapped.map { arena.keep($0.raw(arena)) },
            next: next.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension DevicePhysicalInputSourceDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCDevicePhysicalInputSourceDetailsRaw {
        GCDevicePhysicalInputSourceDetailsRaw(
            input_state_queue_depth: inputStateQueueDepth,
            live: live.raw(arena),
            capture: capture.raw(arena),
            unmapped: unmapped.map { arena.keep($0.raw(arena)) },
            next: next.map { arena.keep($0.raw(arena)) },
            next_diff: nextDiff.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension ControllerDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCControllerDetailsRaw {
        GCControllerDetailsRaw(
            vendor_name: arena.string(vendorName),
            product_category: arena.string(productCategory),
            player_index: playerIndex,
            is_attached_to_device: isAttachedToDevice ? 1 : 0,
            is_current: isCurrent ? 1 : 0,
            supports_background_events: supportsBackgroundEvents ? 1 : 0,
            has_live_input: hasLiveInput ? 1 : 0,
            gamepad: gamepad.map { arena.keep($0.raw(arena)) },
            micro_gamepad: microGamepad.map { arena.keep($0.raw(arena)) },
            directional_gamepad: directionalGamepad.map { arena.keep($0.raw(arena)) },
            extended_gamepad: extendedGamepad.map { arena.keep($0.raw(arena)) },
            dual_shock: dualShock.map { arena.keep($0.raw(arena)) },
            dual_sense: dualSense.map { arena.keep($0.raw(arena)) },
            xbox: xbox.map { arena.keep($0.raw(arena)) },
            battery: battery.map { arena.keep($0.raw(arena)) },
            motion: motion.map { arena.keep($0.raw(arena)) },
            has_light: hasLight ? 1 : 0,
            light: light.map { arena.keep($0.raw(arena)) },
            has_haptics: hasHaptics ? 1 : 0,
            haptics: haptics.map { arena.keep($0.raw(arena)) },
            physical_input: physicalInput.map { arena.keep($0.raw(arena)) },
            input: input.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension KeyboardSnapshotPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCKeyboardSnapshotRaw {
        GCKeyboardSnapshotRaw(
            vendor_name: arena.string(vendorName),
            product_category: arena.string(productCategory),
            any_key_pressed: anyKeyPressed ? 1 : 0,
            pressed_aliases: arena.array(pressedAliases.map { arena.string($0) }),
            pressed_keys: arena.array(pressedKeys.map { $0.raw(arena) }),
            physical_input: physicalInput.raw(arena)
        )
    }
}
extension MouseSnapshotPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCMouseSnapshotRaw {
        GCMouseSnapshotRaw(
            vendor_name: arena.string(vendorName),
            product_category: arena.string(productCategory),
            is_current: isCurrent ? 1 : 0,
            known_mouse_count: knownMouseCount,
            scroll: scroll.raw(arena),
            left_button: leftButton.raw(arena),
            right_button: rightButton.map { arena.keep($0.raw(arena)) },
            middle_button: middleButton.map { arena.keep($0.raw(arena)) },
            auxiliary_buttons: arena.array(auxiliaryButtons.map { $0.raw(arena) }),
            physical_input: physicalInput.raw(arena)
        )
    }
}
extension EventViewControllerPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCEventViewControllerDetailsRaw {
        GCEventViewControllerDetailsRaw(
            controller_user_interaction_enabled: controllerUserInteractionEnabled ? 1 : 0
        )
    }
}
extension SteeringWheelDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCSteeringWheelDetailsRaw {
        GCSteeringWheelDetailsRaw(
            maximum_degrees_of_rotation: maximumDegreesOfRotation,
            absolute_value: absoluteValue.map { arena.keep($0) },
            relative_delta: relativeDelta
        )
    }
}
extension GearShifterDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCGearShifterDetailsRaw {
        GCGearShifterDetailsRaw(
            aliases: arena.array(aliases.map { arena.string($0) }),
            localized_name: localizedName.map { arena.keep(arena.string($0)) },
            sf_symbols_name: sfSymbolsName.map { arena.keep(arena.string($0)) },
            pattern_position: patternPosition.map { arena.keep(Int32($0)) },
            pattern_lower_bound: patternLowerBound.map { arena.keep(Int32($0)) },
            pattern_count: patternCount.map { arena.keep(Int32($0)) },
            pattern_sequential: patternSequential.map { arena.keep($0 ? 1 : 0) },
            pattern_can_wrap: patternCanWrap.map { arena.keep($0 ? 1 : 0) },
            sequential_delta: sequentialDelta.map { arena.keep($0) }
        )
    }
}
extension RacingWheelInputDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCRacingWheelInputDetailsRaw {
        GCRacingWheelInputDetailsRaw(
            wheel: wheel.raw(arena),
            accelerator_pedal: acceleratorPedal.map { arena.keep($0.raw(arena)) },
            brake_pedal: brakePedal.map { arena.keep($0.raw(arena)) },
            clutch_pedal: clutchPedal.map { arena.keep($0.raw(arena)) },
            shifter: shifter.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension RacingWheelDetailsPayload: GCABIValue {
    func raw(_ arena: GCArena) -> GCRacingWheelDetailsRaw {
        GCRacingWheelDetailsRaw(
            vendor_name: arena.string(vendorName),
            product_category: arena.string(productCategory),
            is_acquired: isAcquired ? 1 : 0,
            is_snapshot: isSnapshot ? 1 : 0,
            wheel_input: wheelInput.map { arena.keep($0.raw(arena)) }
        )
    }
}
extension Point2Payload: GCABIValue {
    func raw(_ arena: GCArena) -> GCPoint2Raw {
        GCPoint2Raw(
            x: x,
            y: y
        )
    }
}
