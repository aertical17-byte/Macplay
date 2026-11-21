//! Advice-only optimizer. No automatic process killing or global system tuning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure { Normal, Elevated, Critical, Unknown }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState { Nominal, Fair, Serious, Critical, Unknown }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation {
    KeepSettings, CloseUnneededApps, LowerInGameTextureQuality,
    ReduceFrameRate, StopAndRecover, CollectMetricsFirst,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Telemetry {
    pub memory_pressure: MemoryPressure,
    pub thermal_state: ThermalState,
    pub average_frame_time_ms: Option<f32>,
    pub target_frame_time_ms: Option<f32>,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self { memory_pressure: MemoryPressure::Unknown, thermal_state: ThermalState::Unknown,
            average_frame_time_ms: None, target_frame_time_ms: None }
    }
}

pub fn recommendation(data: Telemetry) -> Recommendation {
    if data.memory_pressure == MemoryPressure::Critical
        || data.thermal_state == ThermalState::Critical {
        return Recommendation::StopAndRecover;
    }
    if data.memory_pressure == MemoryPressure::Unknown
        || data.thermal_state == ThermalState::Unknown {
        return Recommendation::CollectMetricsFirst;
    }
    if data.memory_pressure == MemoryPressure::Elevated {
        return Recommendation::LowerInGameTextureQuality;
    }
    if matches!(data.thermal_state, ThermalState::Serious | ThermalState::Fair) {
        return Recommendation::ReduceFrameRate;
    }
    match (data.average_frame_time_ms, data.target_frame_time_ms) {
        (Some(actual), Some(target)) if actual.is_finite() && target.is_finite()
            && target > 0.0 && actual > target * 1.15 => Recommendation::ReduceFrameRate,
        (Some(actual), Some(target)) if actual.is_finite() && target.is_finite()
            && target > 0.0 && actual <= target * 1.15 => Recommendation::KeepSettings,
        _ => Recommendation::CollectMetricsFirst,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Telemetry {
        Telemetry { memory_pressure: MemoryPressure::Normal, thermal_state: ThermalState::Nominal,
            average_frame_time_ms: Some(16.7), target_frame_time_ms: Some(16.7) }
    }
    #[test] fn critical_pressure_prefers_safe_stop() {
        let mut data = sample(); data.memory_pressure = MemoryPressure::Critical;
        assert_eq!(recommendation(data), Recommendation::StopAndRecover);
    }
    #[test] fn unknown_data_does_not_tweak_machine() {
        assert_eq!(recommendation(Telemetry::default()), Recommendation::CollectMetricsFirst);
    }
    #[test] fn elevated_memory_suggests_texture_reduction() {
        let mut data = sample(); data.memory_pressure = MemoryPressure::Elevated;
        assert_eq!(recommendation(data), Recommendation::LowerInGameTextureQuality);
    }
    #[test] fn serious_thermals_suggest_frame_limit() {
        let mut data = sample(); data.thermal_state = ThermalState::Serious;
        assert_eq!(recommendation(data), Recommendation::ReduceFrameRate);
    }
    #[test] fn good_frame_time_keeps_settings() {
        assert_eq!(recommendation(sample()), Recommendation::KeepSettings);
    }
}
