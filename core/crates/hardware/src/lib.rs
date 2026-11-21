//! Conservative hardware facts. Platform probing will be added behind native adapters.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitecture {
    IntelX86_64,
    AppleSilicon,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareSnapshot {
    pub architecture: CpuArchitecture,
    pub logical_cpu_count: Option<u32>,
    pub total_memory_mb: Option<u64>,
    pub macos_version: Option<String>,
}

impl Default for HardwareSnapshot {
    fn default() -> Self {
        Self {
            architecture: CpuArchitecture::Unknown,
            logical_cpu_count: None,
            total_memory_mb: None,
            macos_version: None,
        }
    }
}

impl HardwareSnapshot {
    /// True only when the available facts are sufficient for a low-memory warning.
    pub fn is_low_memory(&self) -> Option<bool> {
        self.total_memory_mb.map(|mb| mb <= 8 * 1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_memory_is_not_guessed() {
        assert_eq!(HardwareSnapshot::default().is_low_memory(), None);
    }

    #[test]
    fn eight_gibibytes_is_low_memory_tier() {
        let snapshot = HardwareSnapshot {
            total_memory_mb: Some(8192),
            ..HardwareSnapshot::default()
        };
        assert_eq!(snapshot.is_low_memory(), Some(true));
    }
}
