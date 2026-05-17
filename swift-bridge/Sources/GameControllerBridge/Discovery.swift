import Foundation
import GameController

public typealias GCNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void
public typealias GCVoidNotificationCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias GCDiscoveryCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class BoolNotifyState {
    let callback: GCNotificationCallback
    let userInfo: UnsafeMutableRawPointer?
    var observers: [NSObjectProtocol] = []

    init(callback: @escaping GCNotificationCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
    }
}

private final class VoidNotifyState {
    let callback: GCVoidNotificationCallback
    let userInfo: UnsafeMutableRawPointer?
    var observers: [NSObjectProtocol] = []

    init(callback: @escaping GCVoidNotificationCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
    }
}

private final class DiscoveryState {
    let callback: GCDiscoveryCallback?
    let userInfo: UnsafeMutableRawPointer?

    init(callback: GCDiscoveryCallback?, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
    }

    func complete() {
        callback?(userInfo)
    }
}

private var boolNotifyStates: [UnsafeMutableRawPointer: BoolNotifyState] = [:]
private var voidNotifyStates: [UnsafeMutableRawPointer: VoidNotifyState] = [:]
private var discoveryState: DiscoveryState?

private func registerBoolNotifications(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?,
    _ notifications: [(Notification.Name, Bool)]
) -> UnsafeMutableRawPointer? {
    guard !notifications.isEmpty else { return nil }

    let state = BoolNotifyState(callback: callback, userInfo: userInfo)
    let notificationCenter = NotificationCenter.default
    for (name, value) in notifications {
        let observer = notificationCenter.addObserver(
            forName: name,
            object: nil,
            queue: .main
        ) { _ in
            state.callback(state.userInfo, value)
        }
        state.observers.append(observer)
    }

    let token = Unmanaged.passRetained(state).toOpaque()
    boolNotifyStates[token] = state
    return token
}

private func registerVoidNotifications(
    _ callback: @escaping GCVoidNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?,
    _ names: [Notification.Name]
) -> UnsafeMutableRawPointer? {
    guard !names.isEmpty else { return nil }

    let state = VoidNotifyState(callback: callback, userInfo: userInfo)
    let notificationCenter = NotificationCenter.default
    for name in names {
        let observer = notificationCenter.addObserver(
            forName: name,
            object: nil,
            queue: .main
        ) { _ in
            state.callback(state.userInfo)
        }
        state.observers.append(observer)
    }

    let token = Unmanaged.passRetained(state).toOpaque()
    voidNotifyStates[token] = state
    return token
}

@_cdecl("gc_register_connection_callback")
public func gc_register_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    registerBoolNotifications(callback, userInfo, [
        (.GCControllerDidConnect, true),
        (.GCControllerDidDisconnect, false),
    ])
}

@_cdecl("gc_register_controller_current_callback")
public func gc_register_controller_current_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.0, *) else { return nil }
    return registerBoolNotifications(callback, userInfo, [
        (.GCControllerDidBecomeCurrent, true),
        (.GCControllerDidStopBeingCurrent, false),
    ])
}

@_cdecl("gc_register_keyboard_connection_callback")
public func gc_register_keyboard_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.0, *) else { return nil }
    return registerBoolNotifications(callback, userInfo, [
        (.GCKeyboardDidConnect, true),
        (.GCKeyboardDidDisconnect, false),
    ])
}

@_cdecl("gc_register_mouse_connection_callback")
public func gc_register_mouse_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.0, *) else { return nil }
    return registerBoolNotifications(callback, userInfo, [
        (.GCMouseDidConnect, true),
        (.GCMouseDidDisconnect, false),
    ])
}

@_cdecl("gc_register_mouse_current_callback")
public func gc_register_mouse_current_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.0, *) else { return nil }
    return registerBoolNotifications(callback, userInfo, [
        (.GCMouseDidBecomeCurrent, true),
        (.GCMouseDidStopBeingCurrent, false),
    ])
}

@_cdecl("gc_register_racing_wheel_connection_callback")
public func gc_register_racing_wheel_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else { return nil }
    return registerBoolNotifications(callback, userInfo, [
        (.GCRacingWheelDidConnect, true),
        (.GCRacingWheelDidDisconnect, false),
    ])
}

@_cdecl("gc_unregister_connection_callback")
public func gc_unregister_connection_callback(_ token: UnsafeMutableRawPointer?) {
    guard let token, let state = boolNotifyStates.removeValue(forKey: token) else { return }

    let notificationCenter = NotificationCenter.default
    for observer in state.observers {
        notificationCenter.removeObserver(observer)
    }
    Unmanaged<BoolNotifyState>.fromOpaque(token).release()
}

@_cdecl("gc_register_controller_customizations_callback")
public func gc_register_controller_customizations_callback(
    _ callback: @escaping GCVoidNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else { return nil }
    return registerVoidNotifications(callback, userInfo, [.GCControllerUserCustomizationsDidChange])
}

@_cdecl("gc_unregister_notification_callback")
public func gc_unregister_notification_callback(_ token: UnsafeMutableRawPointer?) {
    guard let token, let state = voidNotifyStates.removeValue(forKey: token) else { return }

    let notificationCenter = NotificationCenter.default
    for observer in state.observers {
        notificationCenter.removeObserver(observer)
    }
    Unmanaged<VoidNotifyState>.fromOpaque(token).release()
}

@_cdecl("gc_start_wireless_controller_discovery")
public func gc_start_wireless_controller_discovery(
    _ callback: GCDiscoveryCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) {
    discoveryState = callback.map { DiscoveryState(callback: $0, userInfo: userInfo) }
    GCController.startWirelessControllerDiscovery {
        let state = discoveryState
        discoveryState = nil
        state?.complete()
    }
}

@_cdecl("gc_stop_wireless_controller_discovery")
public func gc_stop_wireless_controller_discovery() {
    discoveryState = nil
    GCController.stopWirelessControllerDiscovery()
}

@_cdecl("gc_should_monitor_background_events")
public func gc_should_monitor_background_events() -> Bool {
    if #unavailable(macOS 11.3) { return false }
    return GCController.shouldMonitorBackgroundEvents
}

@_cdecl("gc_set_should_monitor_background_events")
public func gc_set_should_monitor_background_events(_ enabled: Bool) {
    if #unavailable(macOS 11.3) { return }
    GCController.shouldMonitorBackgroundEvents = enabled
}
