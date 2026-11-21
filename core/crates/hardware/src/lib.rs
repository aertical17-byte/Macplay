//! Conservative hardware facts. Never substitutes guessed values for unavailable data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitecture { IntelX86_64, AppleSilicon, Unknown }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareSnapshot {
    pub architecture: CpuArchitecture,
    pub logical_cpu_count: Option<u32>,
    pub total_memory_mb: Option<u64>,
    pub macos_version: Option<String>,
}

impl Default for HardwareSnapshot {
    fn default() -> Self {
        Self { architecture: CpuArchitecture::Unknown, logical_cpu_count: None,
            total_memory_mb: None, macos_version: None }
    }
}

impl HardwareSnapshot {
    pub fn is_low_memory(&self) -> Option<bool> {
        self.total_memory_mb.map(|mb| mb <= 8 * 1024)
    }

    pub fn architecture_label(&self) -> &'static str {
        match self.architecture {
            CpuArchitecture::IntelX86_64 => "Intel",
            CpuArchitecture::AppleSilicon => "Apple Silicon",
            CpuArchitecture::Unknown => "Sconosciuta",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn unknown_memory_is_not_guessed() {
        assert_eq!(HardwareSnapshot::default().is_low_memory(), None);
    }
    #[test] fn eight_gibibytes_is_low_memory_tier() {
        let value = HardwareSnapshot { total_memory_mb: Some(8192), ..Default::default() };
        assert_eq!(value.is_low_memory(), Some(true));
    }
    #[test] fn architecture_label_is_explicit() {
        assert_eq!(HardwareSnapshot { architecture: CpuArchitecture::AppleSilicon,
            ..Default::default() }.architecture_label(), "Apple Silicon");
    }
}
