//! Runtime validation and launch-plan construction. Does not spawn processes.
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDescriptor {
    pub id: String,
    pub version: String,
    pub root: PathBuf,
    pub architecture: String,
    pub wine_binary: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    EmptyField,
    UnsafeIdentifier,
    RootMustBeAbsolute,
    BinaryOutsideRuntimeRoot,
    GamePathMustBeAbsolute,
    GameFileMissing,
    RuntimeBinaryMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchPlan {
    pub program: PathBuf,
    pub arguments: Vec<String>,
    pub working_directory: PathBuf,
    pub prefix_directory: PathBuf,
}

impl RuntimeDescriptor {
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.id.trim().is_empty() || self.version.trim().is_empty()
            || self.architecture.trim().is_empty() {
            return Err(RuntimeError::EmptyField);
        }
        if Path::new(&self.id).components()
            .any(|part| !matches!(part, Component::Normal(_))) {
            return Err(RuntimeError::UnsafeIdentifier);
        }
        if !self.root.is_absolute() { return Err(RuntimeError::RootMustBeAbsolute); }
        if !self.wine_binary.starts_with(&self.root) {
            return Err(RuntimeError::BinaryOutsideRuntimeRoot);
        }
        Ok(())
    }

    /// Produce an argument-safe plan; a separate, reviewed supervisor will execute it.
    pub fn plan_launch(
        &self,
        game_executable: &Path,
        prefix_directory: &Path,
    ) -> Result<LaunchPlan, RuntimeError> {
        self.validate()?;
        if !game_executable.is_absolute() { return Err(RuntimeError::GamePathMustBeAbsolute); }
        if !game_executable.is_file() { return Err(RuntimeError::GameFileMissing); }
        if !self.wine_binary.is_file() { return Err(RuntimeError::RuntimeBinaryMissing); }

        let working_directory = game_executable.parent()
            .unwrap_or_else(|| Path::new("/")).to_path_buf();
        Ok(LaunchPlan {
            program: self.wine_binary.clone(),
            arguments: vec![game_executable.to_string_lossy().into_owned()],
            working_directory,
            prefix_directory: prefix_directory.to_path_buf(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runtime() -> RuntimeDescriptor {
        RuntimeDescriptor {
            id: "wine-stable".into(),
            version: "test".into(),
            root: PathBuf::from("/opt/macplay/runtimes/wine-stable"),
            architecture: "arm64".into(),
            wine_binary: PathBuf::from("/opt/macplay/runtimes/wine-stable/bin/wine"),
        }
    }

    #[test]
    fn rejects_relative_runtime_root() {
        let mut value = runtime();
        value.root = PathBuf::from("relative");
        assert_eq!(value.validate(), Err(RuntimeError::RootMustBeAbsolute));
    }

    #[test]
    fn rejects_binary_outside_managed_runtime() {
        let mut value = runtime();
        value.wine_binary = PathBuf::from("/usr/local/bin/wine");
        assert_eq!(value.validate(), Err(RuntimeError::BinaryOutsideRuntimeRoot));
    }

    #[test]
    fn rejects_path_traversal_in_runtime_id() {
        let mut value = runtime();
        value.id = "../other-runtime".into();
        assert_eq!(value.validate(), Err(RuntimeError::UnsafeIdentifier));
    }

    #[test]
    fn launch_plan_requires_existing_executable() {
        let value = runtime();
        assert_eq!(
            value.plan_launch(Path::new("/Games/not-installed.exe"), Path::new("/tmp/prefix")),
            Err(RuntimeError::GameFileMissing)
        );
    }
}
