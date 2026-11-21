//! Runtime metadata and validation only. No downloads or process execution yet.
use std::path::{Component, Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDescriptor {
    pub id: String,
    pub version: String,
    pub root: String,
    pub architecture: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    EmptyField,
    UnsafeIdentifier,
    RootMustBeAbsolute,
}

impl RuntimeDescriptor {
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.id.trim().is_empty()
            || self.version.trim().is_empty()
            || self.architecture.trim().is_empty()
        {
            return Err(RuntimeError::EmptyField);
        }
        let id = Path::new(&self.id);
        if id.components().any(|part| !matches!(part, Component::Normal(_))) {
            return Err(RuntimeError::UnsafeIdentifier);
        }
        if !Path::new(&self.root).is_absolute() {
            return Err(RuntimeError::RootMustBeAbsolute);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_relative_runtime_root() {
        let runtime = RuntimeDescriptor {
            id: "wine-stable".into(),
            version: "0.1".into(),
            root: "runtimes/wine".into(),
            architecture: "arm64".into(),
        };
        assert_eq!(runtime.validate(), Err(RuntimeError::RootMustBeAbsolute));
    }
}
