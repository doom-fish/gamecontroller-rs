import Foundation
import GameController

public typealias GCNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void
public typealias GCDiscoveryCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class NotifyState {
    let callback: GCNotificationCallback
    let userInfo: UnsafeMutableRawPointer?
    var connectObserver: NSObjectProtocol?
    var disconnectObserver: NSObjectProtocol?

    init(callback: @escaping GCNotificationCallback, userInfo: UnsafeMutableRawPointer?) {
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

private var notifyStates: [UnsafeMutableRawPointer: NotifyState] = [:]
private var discoveryState: DiscoveryState?

@_cdecl("gc_register_connection_callback")
public func gc_register_connection_callback(
    _ callback: @escaping GCNotificationCallback,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer {
    let state = NotifyState(callback: callback, userInfo: userInfo)
    let notificationCenter = NotificationCenter.default
    state.connectObserver = notificationCenter.addObserver(
        forName: .GCControllerDidConnect,
        object: nil,
        queue: .main
    ) { _ in
        state.callback(state.userInfo, true)
    }
    state.disconnectObserver = notificationCenter.addObserver(
        forName: .GCControllerDidDisconnect,
        object: nil,
        queue: .main
    ) { _ in
        state.callback(state.userInfo, false)
    }

    let token = Unmanaged.passRetained(state).toOpaque()
    notifyStates[token] = state
    return token
}

@_cdecl("gc_unregister_connection_callback")
public func gc_unregister_connection_callback(_ token: UnsafeMutableRawPointer?) {
    guard let token, let state = notifyStates.removeValue(forKey: token) else { return }

    let notificationCenter = NotificationCenter.default
    if let connectObserver = state.connectObserver {
        notificationCenter.removeObserver(connectObserver)
    }
    if let disconnectObserver = state.disconnectObserver {
        notificationCenter.removeObserver(disconnectObserver)
    }
    Unmanaged<NotifyState>.fromOpaque(token).release()
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
