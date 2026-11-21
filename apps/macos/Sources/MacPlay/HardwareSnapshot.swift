import Darwin
import Foundation

struct MacHardwareSnapshot {
    let architecture: String
    let logicalCPUCount: Int
    let memoryGB: Double
    let macOSVersion: String

    static func detect() -> MacHardwareSnapshot {
        // Query the physical machine, not the current process architecture.
        // This avoids mislabelling Apple Silicon when a universal app runs under Rosetta.
        var arm64Capability: Int32 = 0
        var size = MemoryLayout<Int32>.size
        let queryResult = "hw.optional.arm64".withCString {
            sysctlbyname($0, &arm64Capability, &size, nil, 0)
        }
        let architecture = queryResult == 0 && arm64Capability == 1
            ? "Apple Silicon" : "Intel"

        let processInfo = ProcessInfo.processInfo
        return MacHardwareSnapshot(
            architecture: architecture,
            logicalCPUCount: processInfo.processorCount,
            memoryGB: Double(processInfo.physicalMemory) / 1_073_741_824,
            macOSVersion: processInfo.operatingSystemVersionString
        )
    }

    var memoryTier: String {
        memoryGB <= 8 ? "Memoria limitata · 8 GB o meno" : "Memoria superiore a 8 GB"
    }
}
