//! Compatibility is unknown until evidence exists; never infer playability from a title.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityStatus {
    Unknown,
    Untested,
    Experimental,
    Playable,
    Broken,
    BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityRecord {
    pub game_id: String,
    pub status: CompatibilityStatus,
    pub tested_runtime: Option<String>,
    pub notes: Vec<String>,
}

impl CompatibilityRecord {
    pub fn unknown(game_id: impl Into<String>) -> Self {
        Self {
            game_id: game_id.into(),
            status: CompatibilityStatus::Unknown,
            tested_runtime: None,
            notes: Vec::new(),
        }
    }

    pub fn may_be_advertised_as_playable(&self) -> bool {
        self.status == CompatibilityStatus::Playable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_is_never_assumed_playable() {
        assert!(!CompatibilityRecord::unknown("example-game").may_be_advertised_as_playable());
    }
}
