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

private final class StreamSubscriber {
    let onEvent: GCStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let releaseContext: GCContextCallback

    init(
        onEvent: @escaping GCStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        retainContext: GCContextCallback,
        releaseContext: @escaping GCContextCallback
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
        retainContext(ctx)
    }

    deinit {
        releaseContext(ctx)
    }

    func send(_ kind: Int32, _ payload: UnsafeRawPointer?) {
        onEvent(kind, payload, ctx)
    }
}

private protocol StreamSubscription: AnyObject {
    func cancel()
}

private func retainSubscription(_ subscription: StreamSubscription) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(subscription as AnyObject).toOpaque()
}

private func releaseSubscription(_ handle: UnsafeMutableRawPointer) {
    autoreleasepool {
        let unmanaged = Unmanaged<AnyObject>.fromOpaque(handle)
        (unmanaged.takeUnretainedValue() as? StreamSubscription)?.cancel()
        unmanaged.release()
    }
}

private func onMainQueue(_ work: @escaping () -> Void) {
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.async(execute: work)
    }
}

private final class HandlerSlot {
    weak var target: AnyObject?
    let restore: () -> Void

    init(target: AnyObject, restore: @escaping () -> Void) {
        self.target = target
        self.restore = restore
    }
}

private final class StreamHub {
    private let lock = NSLock()
    private var subscribers: [StreamSubscriber] = []
    private var slots: [ObjectIdentifier: HandlerSlot] = [:]
    private var observers: [NSObjectProtocol] = []
    private var installed = false
    private let connectNotifications: [Notification.Name]
    private let installAll: (StreamHub) -> Void
    private let installConnected: (StreamHub, Notification) -> Void

    init(
        connectNotifications: [Notification.Name],
        installAll: @escaping (StreamHub) -> Void,
        installConnected: @escaping (StreamHub, Notification) -> Void
    ) {
        self.connectNotifications = connectNotifications
        self.installAll = installAll
        self.installConnected = installConnected
    }

    func add(_ subscriber: StreamSubscriber) {
        lock.lock()
        subscribers.append(subscriber)
        lock.unlock()
        onMainQueue { self.reconcile() }
    }

    func remove(_ subscriber: StreamSubscriber) {
        lock.lock()
        subscribers.removeAll { $0 === subscriber }
        lock.unlock()
        onMainQueue { self.reconcile() }
    }

    func emit(_ kind: Int32, _ payload: UnsafeRawPointer?) {
        lock.lock()
        let current = subscribers
        lock.unlock()
        for subscriber in current {
            subscriber.send(kind, payload)
        }
    }

    func install(on target: AnyObject, _ attach: () -> () -> Void) {
        let key = ObjectIdentifier(target)
        if let slot = slots[key], slot.target === target {
            return
        }
        slots[key] = HandlerSlot(target: target, restore: attach())
    }

    private func reconcile() {
        lock.lock()
        let wanted = !subscribers.isEmpty
        lock.unlock()
        if wanted && !installed {
            installed = true
            let center = NotificationCenter.default
            observers = connectNotifications.map { name in
                center.addObserver(forName: name, object: nil, queue: .main) { [weak self] note in
                    guard let self, self.installed else { return }
                    self.installConnected(self, note)
                }
            }
            installAll(self)
        } else if !wanted && installed {
            installed = false
            let center = NotificationCenter.default
            for observer in observers {
                center.removeObserver(observer)
            }
            observers = []
            for slot in slots.values where slot.target != nil {
                slot.restore()
            }
            slots = [:]
        }
    }
}

private final class HubSubscription: StreamSubscription {
    let hub: StreamHub
    let subscriber: StreamSubscriber

    init(hub: StreamHub, subscriber: StreamSubscriber) {
        self.hub = hub
        self.subscriber = subscriber
        hub.add(subscriber)
    }

    func cancel() {
        hub.remove(subscriber)
    }
}

private func subscribeToHub(
    _ hub: StreamHub,
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    let subscriber = StreamSubscriber(
        onEvent: onEvent,
        ctx: ctx,
        retainContext: retainContext,
        releaseContext: releaseContext
    )
    return retainSubscription(HubSubscription(hub: hub, subscriber: subscriber))
}

// MARK: - 1. Controller Connection Stream (NSNotificationCenter-driven)

private func sendConnectionEvent(_ subscriber: StreamSubscriber, _ kind: Int32, _ note: Notification) {
    if let name = (note.object as? GCController)?.vendorName {
        name.withCString { ptr in
            subscriber.send(kind, UnsafeRawPointer(ptr))
        }
    } else {
        subscriber.send(kind, nil)
    }
}

private final class ControllerConnectionBridge: StreamSubscription {
    private let lock = NSLock()
    private var observers: [NSObjectProtocol] = []

    init(subscriber: StreamSubscriber) {
        let nc = NotificationCenter.default
        let connectObs = nc.addObserver(
            forName: .GCControllerDidConnect, object: nil, queue: .main
        ) { note in
            sendConnectionEvent(subscriber, 0, note)
        }
        let disconnectObs = nc.addObserver(
            forName: .GCControllerDidDisconnect, object: nil, queue: .main
        ) { note in
            sendConnectionEvent(subscriber, 1, note)
        }
        observers = [connectObs, disconnectObs]
    }

    func cancel() {
        lock.lock()
        let current = observers
        observers = []
        lock.unlock()
        let nc = NotificationCenter.default
        for obs in current { nc.removeObserver(obs) }
    }
}

@_cdecl("gc_stream_controller_connection_subscribe")
public func gc_stream_controller_connection_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    let subscriber = StreamSubscriber(
        onEvent: onEvent,
        ctx: ctx,
        retainContext: retainContext,
        releaseContext: releaseContext
    )
    return autoreleasepool {
        retainSubscription(ControllerConnectionBridge(subscriber: subscriber))
    }
}

@_cdecl("gc_stream_controller_connection_unsubscribe")
public func gc_stream_controller_connection_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}

// MARK: - 2. Extended Gamepad Value Stream (valueChangedHandler)

private func installGamepadHandler(_ hub: StreamHub, _ controller: GCController) {
    guard let gamepad = controller.extendedGamepad else { return }
    hub.install(on: gamepad) {
        let previous = gamepad.valueChangedHandler
        gamepad.valueChangedHandler = { [weak hub] gamepad, element in
            previous?(gamepad, element)
            guard let hub else { return }
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
                hub.emit(0, UnsafeRawPointer(ptr))
            }
        }
        return { [weak gamepad] in gamepad?.valueChangedHandler = previous }
    }
}

private let gamepadValueHub = StreamHub(
    connectNotifications: [.GCControllerDidConnect],
    installAll: { hub in
        for controller in GCController.controllers() { installGamepadHandler(hub, controller) }
    },
    installConnected: { hub, note in
        if let controller = note.object as? GCController { installGamepadHandler(hub, controller) }
    }
)

@_cdecl("gc_stream_gamepad_value_subscribe")
public func gc_stream_gamepad_value_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    subscribeToHub(gamepadValueHub, onEvent, ctx, retainContext, releaseContext)
}

@_cdecl("gc_stream_gamepad_value_unsubscribe")
public func gc_stream_gamepad_value_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}

// MARK: - 3. Keyboard Key Stream (keyChangedHandler)

private func installKeyboardHandler(_ hub: StreamHub) {
    guard let input = GCKeyboard.coalesced?.keyboardInput else { return }
    hub.install(on: input) {
        let previous = input.keyChangedHandler
        input.keyChangedHandler = { [weak hub] keyboardInput, element, keyCode, pressed in
            previous?(keyboardInput, element, keyCode, pressed)
            guard let hub else { return }
            var raw = GCKeyEventRaw(
                keycode: Int64(keyCode.rawValue),
                pressed: pressed,
                value: element.value
            )
            withUnsafePointer(to: &raw) { ptr in
                hub.emit(0, UnsafeRawPointer(ptr))
            }
        }
        return { [weak input] in input?.keyChangedHandler = previous }
    }
}

private let keyboardKeyHub = StreamHub(
    connectNotifications: [.GCKeyboardDidConnect],
    installAll: { hub in installKeyboardHandler(hub) },
    installConnected: { hub, _ in installKeyboardHandler(hub) }
)

@_cdecl("gc_stream_keyboard_key_subscribe")
public func gc_stream_keyboard_key_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    subscribeToHub(keyboardKeyHub, onEvent, ctx, retainContext, releaseContext)
}

@_cdecl("gc_stream_keyboard_key_unsubscribe")
public func gc_stream_keyboard_key_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}

// MARK: - 4. Mouse Input Stream (mouseMovedHandler / pressedChangedHandler)

private func installMouseHandlers(_ hub: StreamHub, _ mouse: GCMouse) {
    guard let input = mouse.mouseInput else { return }
    hub.install(on: input) {
        let previousMoved = input.mouseMovedHandler
        input.mouseMovedHandler = { [weak hub] mouseInput, dx, dy in
            previousMoved?(mouseInput, dx, dy)
            guard let hub else { return }
            var raw = GCMouseEventRaw(delta_x: dx, delta_y: dy, pressed: false, value: 0)
            withUnsafePointer(to: &raw) { ptr in
                hub.emit(0, UnsafeRawPointer(ptr))
            }
        }
        let buttons: [(GCControllerButtonInput?, Int32)] = [
            (input.leftButton, 1),
            (input.rightButton, 2),
            (input.middleButton, 3),
        ]
        var restores: [() -> Void] = []
        for (candidate, kind) in buttons {
            guard let button = candidate else { continue }
            let previous = button.pressedChangedHandler
            button.pressedChangedHandler = { [weak hub] element, value, pressed in
                previous?(element, value, pressed)
                guard let hub else { return }
                var raw = GCMouseEventRaw(delta_x: 0, delta_y: 0, pressed: pressed, value: value)
                withUnsafePointer(to: &raw) { ptr in
                    hub.emit(kind, UnsafeRawPointer(ptr))
                }
            }
            restores.append { [weak button] in button?.pressedChangedHandler = previous }
        }
        return { [weak input] in
            input?.mouseMovedHandler = previousMoved
            for restore in restores { restore() }
        }
    }
}

private let mouseInputHub = StreamHub(
    connectNotifications: [.GCMouseDidConnect],
    installAll: { hub in
        for mouse in GCMouse.mice() { installMouseHandlers(hub, mouse) }
    },
    installConnected: { hub, note in
        if let mouse = note.object as? GCMouse { installMouseHandlers(hub, mouse) }
    }
)

@_cdecl("gc_stream_mouse_input_subscribe")
public func gc_stream_mouse_input_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    subscribeToHub(mouseInputHub, onEvent, ctx, retainContext, releaseContext)
}

@_cdecl("gc_stream_mouse_input_unsubscribe")
public func gc_stream_mouse_input_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}

// MARK: - 5. Motion Stream (GCMotion.valueChangedHandler)

private func installMotionHandler(_ hub: StreamHub, _ controller: GCController) {
    guard let motion = controller.motion else { return }
    hub.install(on: motion) {
        let previous = motion.valueChangedHandler
        motion.valueChangedHandler = { [weak hub] m in
            previous?(m)
            guard let hub else { return }
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
                hub.emit(0, UnsafeRawPointer(ptr))
            }
        }
        return { [weak motion] in motion?.valueChangedHandler = previous }
    }
}

private let motionHub = StreamHub(
    connectNotifications: [.GCControllerDidConnect],
    installAll: { hub in
        for controller in GCController.controllers() { installMotionHandler(hub, controller) }
    },
    installConnected: { hub, note in
        if let controller = note.object as? GCController { installMotionHandler(hub, controller) }
    }
)

@_cdecl("gc_stream_motion_subscribe")
public func gc_stream_motion_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    subscribeToHub(motionHub, onEvent, ctx, retainContext, releaseContext)
}

@_cdecl("gc_stream_motion_unsubscribe")
public func gc_stream_motion_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}

// MARK: - 6. Micro Gamepad Value Stream (GCMicroGamepad.valueChangedHandler)

private func installMicroGamepadHandler(_ hub: StreamHub, _ controller: GCController) {
    guard let gamepad = controller.microGamepad else { return }
    hub.install(on: gamepad) {
        let previous = gamepad.valueChangedHandler
        gamepad.valueChangedHandler = { [weak hub] gp, element in
            previous?(gp, element)
            guard let hub else { return }
            var raw = GCMicroGamepadEventRaw(
                button_a: gp.buttonA.value,
                button_x: gp.buttonX.value,
                dpad_x: gp.dpad.xAxis.value,
                dpad_y: gp.dpad.yAxis.value
            )
            withUnsafePointer(to: &raw) { ptr in
                hub.emit(0, UnsafeRawPointer(ptr))
            }
        }
        return { [weak gamepad] in gamepad?.valueChangedHandler = previous }
    }
}

private let microGamepadValueHub = StreamHub(
    connectNotifications: [.GCControllerDidConnect],
    installAll: { hub in
        for controller in GCController.controllers() { installMicroGamepadHandler(hub, controller) }
    },
    installConnected: { hub, note in
        if let controller = note.object as? GCController { installMicroGamepadHandler(hub, controller) }
    }
)

@_cdecl("gc_stream_micro_gamepad_subscribe")
public func gc_stream_micro_gamepad_subscribe(
    _ onEvent: @escaping GCStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: GCContextCallback,
    _ releaseContext: @escaping GCContextCallback
) -> UnsafeMutableRawPointer {
    subscribeToHub(microGamepadValueHub, onEvent, ctx, retainContext, releaseContext)
}

@_cdecl("gc_stream_micro_gamepad_unsubscribe")
public func gc_stream_micro_gamepad_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    releaseSubscription(handle)
}
