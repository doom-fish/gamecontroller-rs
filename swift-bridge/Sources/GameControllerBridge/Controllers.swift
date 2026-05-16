import Foundation
import GameController

private func snapshot(_ controller: GCController) -> GCControllerInfoRaw {
    var info = GCControllerInfoRaw(
        vendor_name: ffiString(controller.vendorName ?? "Unknown"),
        product_category: ffiString(controller.productCategory),
        player_index: Int32(controller.playerIndex.rawValue),
        is_attached_to_device: controller.isAttachedToDevice,
        has_extended_gamepad: controller.extendedGamepad != nil,
        button_a: 0,
        button_b: 0,
        button_x: 0,
        button_y: 0,
        left_shoulder: 0,
        right_shoulder: 0,
        left_trigger: 0,
        right_trigger: 0,
        menu_button: 0,
        options_button: 0,
        home_button: 0,
        left_thumbstick_x: 0,
        left_thumbstick_y: 0,
        right_thumbstick_x: 0,
        right_thumbstick_y: 0,
        dpad_up: 0,
        dpad_down: 0,
        dpad_left: 0,
        dpad_right: 0
    )
    if let gamepad = controller.extendedGamepad {
        info.button_a = gamepad.buttonA.value
        info.button_b = gamepad.buttonB.value
        info.button_x = gamepad.buttonX.value
        info.button_y = gamepad.buttonY.value
        info.left_shoulder = gamepad.leftShoulder.value
        info.right_shoulder = gamepad.rightShoulder.value
        info.left_trigger = gamepad.leftTrigger.value
        info.right_trigger = gamepad.rightTrigger.value
        info.menu_button = gamepad.buttonMenu.value
        if #available(macOS 11.0, *) {
            info.options_button = gamepad.buttonOptions?.value ?? 0
            info.home_button = gamepad.buttonHome?.value ?? 0
        }
        info.left_thumbstick_x = gamepad.leftThumbstick.xAxis.value
        info.left_thumbstick_y = gamepad.leftThumbstick.yAxis.value
        info.right_thumbstick_x = gamepad.rightThumbstick.xAxis.value
        info.right_thumbstick_y = gamepad.rightThumbstick.yAxis.value
        info.dpad_up = gamepad.dpad.up.value
        info.dpad_down = gamepad.dpad.down.value
        info.dpad_left = gamepad.dpad.left.value
        info.dpad_right = gamepad.dpad.right.value
    }
    return info
}

@_cdecl("gc_connected_controllers")
public func gc_connected_controllers(
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let controllers = GCController.controllers()
    if controllers.isEmpty {
        outArray.pointee = nil
        outCount.pointee = 0
        return 0
    }

    let buffer = UnsafeMutablePointer<GCControllerInfoRaw>.allocate(capacity: controllers.count)
    for (index, controller) in controllers.enumerated() {
        buffer.advanced(by: index).initialize(to: snapshot(controller))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = controllers.count
    return 0
}

@_cdecl("gc_controller_infos_free")
public func gc_controller_infos_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: GCControllerInfoRaw.self)
    for index in 0..<count {
        let info = typed.advanced(by: index).pointee
        if let vendorName = info.vendor_name { free(vendorName) }
        if let productCategory = info.product_category { free(productCategory) }
    }
    typed.deallocate()
}

@_cdecl("gc_first_controller_extra")
public func gc_first_controller_extra(_ outInfo: UnsafeMutableRawPointer) -> Bool {
    guard let controller = GCController.controllers().first else { return false }

    var info = GCExtraInfoRaw(
        has_motion: false,
        has_haptics: false,
        has_light: false,
        has_battery: false,
        battery_level: -1,
        battery_state: 0,
        gravity_x: 0,
        gravity_y: 0,
        gravity_z: 0,
        user_acceleration_x: 0,
        user_acceleration_y: 0,
        user_acceleration_z: 0
    )

    if let motion = controller.motion {
        info.has_motion = true
        info.gravity_x = motion.gravity.x
        info.gravity_y = motion.gravity.y
        info.gravity_z = motion.gravity.z
        info.user_acceleration_x = motion.userAcceleration.x
        info.user_acceleration_y = motion.userAcceleration.y
        info.user_acceleration_z = motion.userAcceleration.z
    }
    if #available(macOS 11.0, *) {
        info.has_haptics = controller.haptics != nil
        info.has_light = controller.light != nil
        if let battery = controller.battery {
            info.has_battery = true
            info.battery_level = battery.batteryLevel
            switch battery.batteryState {
            case .unknown:
                info.battery_state = 0
            case .discharging:
                info.battery_state = 1
            case .charging:
                info.battery_state = 2
            case .full:
                info.battery_state = 3
            @unknown default:
                info.battery_state = 0
            }
        }
    }

    outInfo.assumingMemoryBound(to: GCExtraInfoRaw.self).pointee = info
    return true
}

@_cdecl("gc_all_controllers_extras")
public func gc_all_controllers_extras(_ outBuffer: UnsafeMutableRawPointer, _ max: Int) -> Int {
    let controllers = GCController.controllers()
    let count = min(controllers.count, max)
    let typed = outBuffer.assumingMemoryBound(to: GCExtraInfoRaw.self)

    for index in 0..<count {
        let controller = controllers[index]
        var info = GCExtraInfoRaw(
            has_motion: false,
            has_haptics: false,
            has_light: false,
            has_battery: false,
            battery_level: -1,
            battery_state: 0,
            gravity_x: 0,
            gravity_y: 0,
            gravity_z: 0,
            user_acceleration_x: 0,
            user_acceleration_y: 0,
            user_acceleration_z: 0
        )
        if let battery = controller.battery {
            info.has_battery = true
            info.battery_level = battery.batteryLevel
            info.battery_state = Int32(battery.batteryState.rawValue)
        }
        info.has_haptics = controller.haptics != nil
        info.has_light = controller.light != nil
        if let motion = controller.motion {
            info.has_motion = true
            info.gravity_x = motion.gravity.x
            info.gravity_y = motion.gravity.y
            info.gravity_z = motion.gravity.z
            info.user_acceleration_x = motion.userAcceleration.x
            info.user_acceleration_y = motion.userAcceleration.y
            info.user_acceleration_z = motion.userAcceleration.z
        }
        typed.advanced(by: index).initialize(to: info)
    }

    return count
}

@_cdecl("gc_controller_details_json")
public func gc_controller_details_json(_ currentOnly: Bool) -> UnsafeMutablePointer<CChar>? {
    let controllers: [GCController]
    if currentOnly {
        if #available(macOS 11.0, *), let current = GCController.current {
            controllers = [current]
        } else {
            controllers = []
        }
    } else {
        controllers = GCController.controllers()
    }
    return jsonString(controllers.map(controllerDetailsPayload))
}

@_cdecl("gc_current_controller_input_source_json")
public func gc_current_controller_input_source_json() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 14.0, *), let current = GCController.current {
        return jsonString(Optional(controllerLiveInputSourcePayload(current.input)))
    }
    return jsonString(Optional<DevicePhysicalInputSourceDetailsPayload>.none)
}
