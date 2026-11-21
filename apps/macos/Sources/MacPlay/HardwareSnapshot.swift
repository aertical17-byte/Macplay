import Foundation

struct MacHardwareSnapshot {
    let architecture: String
    let logicalCPUCount: Int
    let memoryGB: Double
    let macOSVersion: String

    static func detect() -> MacHardwareSnapshot {
        let architecture: String
        #if arch(arm64)
        architecture = "Apple Silicon"
        #elseif arch(x86_64)
        architecture = "Intel"
        #else
        architecture = "Sconosciuta"
        #endif

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
