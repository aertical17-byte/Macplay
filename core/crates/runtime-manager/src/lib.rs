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
    EmptyField, UnsafeIdentifier, RootMustBeAbsolute, BinaryOutsideRuntimeRoot,
    UnsafeRuntimePath, GamePathMustBeAbsolute, PrefixMustBeAbsolute,
    GameFileMissing, RuntimeBinaryMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchPlan {
    pub program: PathBuf,
    pub arguments: Vec<String>,
    pub working_directory: PathBuf,
    pub prefix_directory: PathBuf,
}

fn contains_parent_component(path: &Path) -> bool {
    path.components().any(|part| part == Component::ParentDir)
}

impl RuntimeDescriptor {
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.id.trim().is_empty() || self.version.trim().is_empty()
            || self.architecture.trim().is_empty() { return Err(RuntimeError::EmptyField); }
        if Path::new(&self.id).components()
            .any(|part| !matches!(part, Component::Normal(_))) {
            return Err(RuntimeError::UnsafeIdentifier);
        }
        if !self.root.is_absolute() { return Err(RuntimeError::RootMustBeAbsolute); }
        if contains_parent_component(&self.root) || contains_parent_component(&self.wine_binary) {
            return Err(RuntimeError::UnsafeRuntimePath);
        }
        if !self.wine_binary.starts_with(&self.root) {
            return Err(RuntimeError::BinaryOutsideRuntimeRoot);
        }
        Ok(())
    }

    /// Produces an argument-safe plan for a future reviewed process supervisor.
    pub fn plan_launch(&self, game_executable: &Path, prefix_directory: &Path)
        -> Result<LaunchPlan, RuntimeError> {
        self.validate()?;
        if !game_executable.is_absolute() { return Err(RuntimeError::GamePathMustBeAbsolute); }
        if !prefix_directory.is_absolute() { return Err(RuntimeError::PrefixMustBeAbsolute); }
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
            id: "wine-stable".into(), version: "test".into(),
            root: PathBuf::from("/opt/macplay/runtimes/wine-stable"),
            architecture: "arm64".into(),
            wine_binary: PathBuf::from("/opt/macplay/runtimes/wine-stable/bin/wine"),
        }
    }

    #[test] fn rejects_relative_root() {
        let mut value = runtime(); value.root = PathBuf::from("relative");
        assert_eq!(value.validate(), Err(RuntimeError::RootMustBeAbsolute));
    }
    #[test] fn rejects_binary_outside_runtime() {
        let mut value = runtime(); value.wine_binary = PathBuf::from("/usr/local/bin/wine");
        assert_eq!(value.validate(), Err(RuntimeError::BinaryOutsideRuntimeRoot));
    }
    #[test] fn rejects_parent_traversal() {
        let mut value = runtime();
        value.wine_binary = PathBuf::from("/opt/macplay/runtimes/wine-stable/../other/wine");
        assert_eq!(value.validate(), Err(RuntimeError::UnsafeRuntimePath));
    }
    #[test] fn rejects_traversal_in_identifier() {
        let mut value = runtime(); value.id = "../other".into();
        assert_eq!(value.validate(), Err(RuntimeError::UnsafeIdentifier));
    }
    #[test] fn launch_requires_existing_game() {
        let value = runtime();
        assert_eq!(value.plan_launch(Path::new("/Games/missing.exe"), Path::new("/tmp/prefix")),
            Err(RuntimeError::GameFileMissing));
    }
    #[test] fn launch_requires_absolute_prefix() {
        let value = runtime();
        assert_eq!(value.plan_launch(Path::new("/Games/game.exe"), Path::new("prefix")),
            Err(RuntimeError::PrefixMustBeAbsolute));
    }
}
