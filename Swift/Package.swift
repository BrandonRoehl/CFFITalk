// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "get_set",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "swift_get_set",
            type: .static,
            targets: ["get_set"]
        ),
        .library(
            name: "swift_next",
            type: .static,
            targets: ["next"]
        ),
        .library(
            name: "swift_step",
            type: .static,
            targets: ["step"]
        ),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "get_set",
            dependencies: ["link"]
        ),
        .target(
            name: "next",
            dependencies: ["link"]
        ),
        .target(
            name: "step",
            dependencies: ["link"]
        ),
        .systemLibrary(name: "link"),
    ]
)
