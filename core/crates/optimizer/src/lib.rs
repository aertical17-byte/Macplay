//! Advice-only optimizer. It never kills processes, disables swap, or changes system settings.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    Normal,
    Elevated,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation {
    KeepSettings,
    CloseUnneededApps,
    LowerInGameTextureQuality,
    StopAndRecover,
    CollectMetricsFirst,
}

pub fn recommendation(pressure: MemoryPressure) -> Recommendation {
    match pressure {
        MemoryPressure::Normal => Recommendation::KeepSettings,
        MemoryPressure::Elevated => Recommendation::CloseUnneededApps,
        MemoryPressure::Critical => Recommendation::StopAndRecover,
        MemoryPressure::Unknown => Recommendation::CollectMetricsFirst,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_pressure_prefers_safe_stop() {
        assert_eq!(recommendation(MemoryPressure::Critical), Recommendation::StopAndRecover);
    }

    #[test]
    fn unknown_pressure_does_not_tweak_the_machine() {
        assert_eq!(recommendation(MemoryPressure::Unknown), Recommendation::CollectMetricsFirst);
    }
}
