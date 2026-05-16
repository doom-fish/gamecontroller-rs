import Foundation
import GameController

func ffiString(_ string: String?) -> UnsafeMutablePointer<CChar>? {
    guard let string else { return nil }
    return strdup(string)
}

@_cdecl("gc_string_free")
public func gc_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

func jsonString<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()
    guard let data = try? encoder.encode(value),
          let string = String(data: data, encoding: .utf8)
    else {
        return nil
    }
    return ffiString(string)
}

func clampUnit(_ value: Float) -> Float {
    max(0, min(1, value))
}

@frozen
public struct GCControllerInfoRaw {
    public var vendor_name: UnsafeMutablePointer<CChar>?
    public var product_category: UnsafeMutablePointer<CChar>?
    public var player_index: Int32
    public var is_attached_to_device: Bool
    public var has_extended_gamepad: Bool

    public var button_a: Float
    public var button_b: Float
    public var button_x: Float
    public var button_y: Float

    public var left_shoulder: Float
    public var right_shoulder: Float
    public var left_trigger: Float
    public var right_trigger: Float

    public var menu_button: Float
    public var options_button: Float
    public var home_button: Float

    public var left_thumbstick_x: Float
    public var left_thumbstick_y: Float
    public var right_thumbstick_x: Float
    public var right_thumbstick_y: Float

    public var dpad_up: Float
    public var dpad_down: Float
    public var dpad_left: Float
    public var dpad_right: Float
}

@frozen
public struct GCExtraInfoRaw {
    public var has_motion: Bool
    public var has_haptics: Bool
    public var has_light: Bool
    public var has_battery: Bool
    public var battery_level: Float
    public var battery_state: Int32
    public var gravity_x: Double
    public var gravity_y: Double
    public var gravity_z: Double
    public var user_acceleration_x: Double
    public var user_acceleration_y: Double
    public var user_acceleration_z: Double
}
