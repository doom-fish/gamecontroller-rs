// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "GameControllerBridge",
    platforms: [.macOS(.v12)],
    products: [
        .library(name: "GameControllerBridge", type: .static, targets: ["GameControllerBridge"]),
    ],
    targets: [
        .target(name: "GameControllerABI", path: "Sources/GameControllerABI", publicHeadersPath: "include"),
        .target(name: "GameControllerBridge", dependencies: ["GameControllerABI"], path: "Sources/GameControllerBridge"),
    ]
)
