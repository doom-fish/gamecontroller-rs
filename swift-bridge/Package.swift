// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "GameControllerBridge",
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "GameControllerBridge", type: .static, targets: ["GameControllerBridge"]),
    ],
    targets: [
        .target(name: "GameControllerBridge", path: "Sources/GameControllerBridge", publicHeadersPath: "include"),
    ]
)
