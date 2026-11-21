// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "MacPlay",
    platforms: [.macOS(.v14)],
    products: [.executable(name: "MacPlay", targets: ["MacPlay"])],
    targets: [.executableTarget(name: "MacPlay")]
)
