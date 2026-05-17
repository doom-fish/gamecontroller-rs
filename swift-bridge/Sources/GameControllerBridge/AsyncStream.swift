import Foundation
import GameController

// MARK: - Common callback type

public typealias GCStreamEventCallback = @convention(c) (
    Int32,
    UnsafeRawPointer?,
    UnsafeMutableRawPointer
) -> Void

// MARK: - Payload structs (must match Rust repr(C) structs exactly)

@frozen
public struct GCKeyEventRaw {
    public var keycode: Int64
    public var pressed: Bool
    public var value: Float
}

@frozen
public struct GCMouseEventRaw {
    public var delta_x: Float
    public var delta_y: Float
    public var pressed: Bool
    public var value: Float
}

@frozen
public struct GCMotionEventRaw {
    public var gravity_x: Double
    public var gravity_y: Double
    public var gravity_z: Double
    public var user_acceleration_x: Double
    public var user_acceleration_y: Double
    public var user_acceleration_z: Double
    public var attitude_x: Double
    public var attitude_y: Double
    public var attitude_z: Double
    public var attitude_w: Double
    public var rotation_rate_x: Double
    public var rotation_rate_y: Double
    public var rotation_rate_z: Double
}

@frozen
public struct GCMicroGamepadEventRaw {
    public var button_a: Float
    public var button_x: Float
    public var dpad_x: Float
    public var dpad_y: Float
}

// MARK: - 1. Controller Connection Stream (NSNotificationCenter-driven)

private final class ControllerConnectionBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        let nc = NotificationCenter.default
        let connectObs = nc.addObserver(
            forName: .GCControllerDidConnect, object: nil, queue: .main
        ) { [weak self] note in
            guard let self else { return }
            let vendorName = (note.object as? GCController)?.vendorName
            if let name = vendorName {
                name.withCString { ptr in
                    self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
                }
            } else {
                self.onEvent(0, nil, self.ctx)
            }
        }
        let disconnectObs = nc.addObserver(
            forName: .GCControllerDidDisconnect, object: nil, queue: .main
        ) { [weak self] note in
            guard let self else { return }
            let vendorName = (note.object as? GCController)?.vendorName
            if let name = vendorName {
                name.withCString { ptr in
                    self.onEvent(1, UnsafeRawPointer(ptr), self.ctx)
                }
            } else {
                self.onEvent(1, nil, self.ctx)
            }
        }
        observers = [connectObs, disconnectObs]
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
    }
}

@_cdecl("gc_stream_controller_connection_subscribe")
public func gc_stream_controller_connection_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(ControllerConnectionBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_controller_connection_unsubscribe")
public func gc_stream_controller_connection_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let bridge = Unmanaged<ControllerConnectionBridge>.fromOpaque(handle)
    let nc = NotificationCenter.default
    for obs in bridge.takeUnretainedValue().observers { nc.removeObserver(obs) }
    bridge.release()
}

// MARK: - 2. Extended Gamepad Value Stream (valueChangedHandler)

private final class GamepadValueBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []
    var handledControllers: Set<ObjectIdentifier> = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        for ctrl in GCController.controllers() { attachHandler(to: ctrl) }
        let obs = NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect, object: nil, queue: .main
        ) { [weak self] note in
            guard let self, let ctrl = note.object as? GCController else { return }
            self.attachHandler(to: ctrl)
        }
        observers.append(obs)
    }

    private func attachHandler(to controller: GCController) {
        guard let gp = controller.extendedGamepad else { return }
        let identifier = ObjectIdentifier(controller)
        guard handledControllers.insert(identifier).inserted else { return }
        gp.valueChangedHandler = { [weak self] (gamepad, _) in
            guard let self else { return }
            var raw = GCControllerInfoRaw(
                vendor_name: nil,
                product_category: nil,
                player_index: 0,
                is_attached_to_device: false,
                has_extended_gamepad: true,
                button_a: gamepad.buttonA.value,
                button_b: gamepad.buttonB.value,
                button_x: gamepad.buttonX.value,
                button_y: gamepad.buttonY.value,
                left_shoulder: gamepad.leftShoulder.value,
                right_shoulder: gamepad.rightShoulder.value,
                left_trigger: gamepad.leftTrigger.value,
                right_trigger: gamepad.rightTrigger.value,
                menu_button: gamepad.buttonMenu.value,
                options_button: gamepad.buttonOptions?.value ?? 0,
                home_button: gamepad.buttonHome?.value ?? 0,
                left_thumbstick_x: gamepad.leftThumbstick.xAxis.value,
                left_thumbstick_y: gamepad.leftThumbstick.yAxis.value,
                right_thumbstick_x: gamepad.rightThumbstick.xAxis.value,
                right_thumbstick_y: gamepad.rightThumbstick.yAxis.value,
                dpad_up: gamepad.dpad.up.value,
                dpad_down: gamepad.dpad.down.value,
                dpad_left: gamepad.dpad.left.value,
                dpad_right: gamepad.dpad.right.value
            )
            withUnsafePointer(to: &raw) { ptr in
                self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
            }
        }
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
        for ctrl in GCController.controllers() { ctrl.extendedGamepad?.valueChangedHandler = nil }
    }
}

@_cdecl("gc_stream_gamepad_value_subscribe")
public func gc_stream_gamepad_value_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(GamepadValueBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_gamepad_value_unsubscribe")
public func gc_stream_gamepad_value_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<GamepadValueBridge>.fromOpaque(handle).release()
}

// MARK: - 3. Keyboard Key Stream (keyChangedHandler)

private final class KeyboardKeyBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        attachKeyboardHandler()
        if #available(macOS 11.0, *) {
            let obs = NotificationCenter.default.addObserver(
                forName: .GCKeyboardDidConnect, object: nil, queue: .main
            ) { [weak self] _ in self?.attachKeyboardHandler() }
            observers.append(obs)
        }
    }

    private func attachKeyboardHandler() {
        if #available(macOS 11.0, *) {
            GCKeyboard.coalesced?.keyboardInput?.keyChangedHandler = {
                [weak self] (_, element, keyCode, pressed) in
                guard let self else { return }
                var raw = GCKeyEventRaw(
                    keycode: Int64(keyCode.rawValue),
                    pressed: pressed,
                    value: element.value
                )
                withUnsafePointer(to: &raw) { ptr in
                    self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
                }
            }
        }
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
        if #available(macOS 11.0, *) {
            GCKeyboard.coalesced?.keyboardInput?.keyChangedHandler = nil
        }
    }
}

@_cdecl("gc_stream_keyboard_key_subscribe")
public func gc_stream_keyboard_key_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(KeyboardKeyBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_keyboard_key_unsubscribe")
public func gc_stream_keyboard_key_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<KeyboardKeyBridge>.fromOpaque(handle).release()
}

// MARK: - 4. Mouse Input Stream (mouseMovedHandler / pressedChangedHandler)

private final class MouseInputBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        if #available(macOS 11.0, *) {
            attachMouseHandlers()
            let obs = NotificationCenter.default.addObserver(
                forName: .GCMouseDidConnect, object: nil, queue: .main
            ) { [weak self] _ in self?.attachMouseHandlers() }
            observers.append(obs)
        }
    }

    private func attachMouseHandlers() {
        if #available(macOS 11.0, *) {
            guard let mouse = GCMouse.current, let input = mouse.mouseInput else { return }
            input.mouseMovedHandler = { [weak self] (_, dx, dy) in
                guard let self else { return }
                var raw = GCMouseEventRaw(delta_x: dx, delta_y: dy, pressed: false, value: 0)
                withUnsafePointer(to: &raw) { ptr in
                    self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
                }
            }
            input.leftButton.pressedChangedHandler = { [weak self] (_, value, pressed) in
                guard let self else { return }
                var raw = GCMouseEventRaw(delta_x: 0, delta_y: 0, pressed: pressed, value: value)
                withUnsafePointer(to: &raw) { ptr in
                    self.onEvent(1, UnsafeRawPointer(ptr), self.ctx)
                }
            }
            input.rightButton?.pressedChangedHandler = { [weak self] (_, value, pressed) in
                guard let self else { return }
                var raw = GCMouseEventRaw(delta_x: 0, delta_y: 0, pressed: pressed, value: value)
                withUnsafePointer(to: &raw) { ptr in
                    self.onEvent(2, UnsafeRawPointer(ptr), self.ctx)
                }
            }
            input.middleButton?.pressedChangedHandler = { [weak self] (_, value, pressed) in
                guard let self else { return }
                var raw = GCMouseEventRaw(delta_x: 0, delta_y: 0, pressed: pressed, value: value)
                withUnsafePointer(to: &raw) { ptr in
                    self.onEvent(3, UnsafeRawPointer(ptr), self.ctx)
                }
            }
        }
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
        if #available(macOS 11.0, *) {
            if let mouse = GCMouse.current, let input = mouse.mouseInput {
                input.mouseMovedHandler = nil
                input.leftButton.pressedChangedHandler = nil
                input.rightButton?.pressedChangedHandler = nil
                input.middleButton?.pressedChangedHandler = nil
            }
        }
    }
}

@_cdecl("gc_stream_mouse_input_subscribe")
public func gc_stream_mouse_input_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(MouseInputBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_mouse_input_unsubscribe")
public func gc_stream_mouse_input_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<MouseInputBridge>.fromOpaque(handle).release()
}

// MARK: - 5. Motion Stream (GCMotion.valueChangedHandler)

private final class MotionBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []
    var handledControllers: Set<ObjectIdentifier> = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        for ctrl in GCController.controllers() { attachHandler(to: ctrl) }
        let obs = NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect, object: nil, queue: .main
        ) { [weak self] note in
            guard let self, let ctrl = note.object as? GCController else { return }
            self.attachHandler(to: ctrl)
        }
        observers.append(obs)
    }

    private func attachHandler(to controller: GCController) {
        guard let motion = controller.motion else { return }
        let identifier = ObjectIdentifier(controller)
        guard handledControllers.insert(identifier).inserted else { return }
        motion.valueChangedHandler = { [weak self] m in
            guard let self else { return }
            var raw = GCMotionEventRaw(
                gravity_x: m.gravity.x,
                gravity_y: m.gravity.y,
                gravity_z: m.gravity.z,
                user_acceleration_x: m.userAcceleration.x,
                user_acceleration_y: m.userAcceleration.y,
                user_acceleration_z: m.userAcceleration.z,
                attitude_x: m.attitude.x,
                attitude_y: m.attitude.y,
                attitude_z: m.attitude.z,
                attitude_w: m.attitude.w,
                rotation_rate_x: m.rotationRate.x,
                rotation_rate_y: m.rotationRate.y,
                rotation_rate_z: m.rotationRate.z
            )
            withUnsafePointer(to: &raw) { ptr in
                self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
            }
        }
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
        for ctrl in GCController.controllers() { ctrl.motion?.valueChangedHandler = nil }
    }
}

@_cdecl("gc_stream_motion_subscribe")
public func gc_stream_motion_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(MotionBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_motion_unsubscribe")
public func gc_stream_motion_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<MotionBridge>.fromOpaque(handle).release()
}

// MARK: - 6. Micro Gamepad Value Stream (GCMicroGamepad.valueChangedHandler)

private final class MicroGamepadValueBridge {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observers: [NSObjectProtocol] = []
    var handledControllers: Set<ObjectIdentifier> = []

    init(onEvent: @escaping GCStreamEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
        for ctrl in GCController.controllers() { attachHandler(to: ctrl) }
        let obs = NotificationCenter.default.addObserver(
            forName: .GCControllerDidConnect, object: nil, queue: .main
        ) { [weak self] note in
            guard let self, let ctrl = note.object as? GCController else { return }
            self.attachHandler(to: ctrl)
        }
        observers.append(obs)
    }

    private func attachHandler(to controller: GCController) {
        guard let gamepad = controller.microGamepad else { return }
        let identifier = ObjectIdentifier(controller)
        guard handledControllers.insert(identifier).inserted else { return }
        gamepad.valueChangedHandler = { [weak self] (gp, _) in
            guard let self else { return }
            var raw = GCMicroGamepadEventRaw(
                button_a: gp.buttonA.value,
                button_x: gp.buttonX.value,
                dpad_x: gp.dpad.xAxis.value,
                dpad_y: gp.dpad.yAxis.value
            )
            withUnsafePointer(to: &raw) { ptr in
                self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
            }
        }
    }

    deinit {
        let nc = NotificationCenter.default
        for obs in observers { nc.removeObserver(obs) }
        for ctrl in GCController.controllers() { ctrl.microGamepad?.valueChangedHandler = nil }
    }
}

@_cdecl("gc_stream_micro_gamepad_subscribe")
public func gc_stream_micro_gamepad_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(MicroGamepadValueBridge(onEvent: onEvent, ctx: ctx)).toOpaque()
}

@_cdecl("gc_stream_micro_gamepad_unsubscribe")
public func gc_stream_micro_gamepad_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<MicroGamepadValueBridge>.fromOpaque(handle).release()
}
