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

func clampUnit(_ value: Float) -> Float {
    max(0, min(1, value))
}

@_cdecl("gc_input_side_button_name")
public func gc_input_side_button_name(_ right: Bool) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 26.4, *),
          let image = dlopen(nil, RTLD_NOW),
          let symbol = dlsym(image, right ? "GCInputRightSideButton" : "GCInputLeftSideButton"),
          let name = symbol.assumingMemoryBound(to: NSString?.self).pointee
    else {
        return nil
    }
    return ffiString(name as String)
}

func controllerFromHandle(_ handle: UnsafeMutableRawPointer?) -> GCController? {
    guard let handle else { return nil }
    return Unmanaged<GCController>.fromOpaque(handle).takeUnretainedValue()
}
