//! Compatibility is unknown until evidence exists; never infer playability from a title.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityStatus {
    Unknown, Untested, Experimental, Playable, Broken, BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestEvidence {
    pub game_version: String,
    pub macos_version: String,
    pub host_architecture: String,
    pub runtime_id: String,
    pub runtime_version: String,
    pub graphics_backend: String,
    pub tested_at_utc: String,
    pub outcome_notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityRecord {
    pub game_id: String,
    pub status: CompatibilityStatus,
    pub evidence: Option<TestEvidence>,
    pub notes: Vec<String>,
}

impl CompatibilityRecord {
    pub fn unknown(game_id: impl Into<String>) -> Self {
        Self { game_id: game_id.into(), status: CompatibilityStatus::Unknown,
            evidence: None, notes: Vec::new() }
    }

    pub fn may_be_advertised_as_playable(&self) -> bool {
        self.status == CompatibilityStatus::Playable && self.evidence.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn new_game_is_not_assumed_playable() {
        assert!(!CompatibilityRecord::unknown("example").may_be_advertised_as_playable());
    }
    #[test] fn playable_without_evidence_is_not_advertised() {
        let mut record = CompatibilityRecord::unknown("example");
        record.status = CompatibilityStatus::Playable;
        assert!(!record.may_be_advertised_as_playable());
    }
}
