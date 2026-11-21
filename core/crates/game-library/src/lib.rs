//! Validated game-library metadata. This crate never executes imported files.
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportSource { Executable, Steam }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameRecord {
    pub id: String,
    pub title: String,
    pub executable_path: String,
    pub source: ImportSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryError {
    EmptyId, EmptyTitle, EmptyPath, PathMustBeAbsolute,
    UnsupportedExecutable, DuplicateId, DuplicatePath,
}

#[derive(Debug, Default)]
pub struct GameLibrary { entries: Vec<GameRecord> }

impl GameLibrary {
    pub fn entries(&self) -> &[GameRecord] { &self.entries }

    pub fn add(&mut self, game: GameRecord) -> Result<(), LibraryError> {
        if game.id.trim().is_empty() { return Err(LibraryError::EmptyId); }
        if game.title.trim().is_empty() { return Err(LibraryError::EmptyTitle); }
        if game.executable_path.trim().is_empty() { return Err(LibraryError::EmptyPath); }
        let path = Path::new(&game.executable_path);
        if !path.is_absolute() { return Err(LibraryError::PathMustBeAbsolute); }
        if path.extension().and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("exe")) != Some(true) {
            return Err(LibraryError::UnsupportedExecutable);
        }
        if self.entries.iter().any(|entry| entry.id == game.id) {
            return Err(LibraryError::DuplicateId);
        }
        if self.entries.iter().any(|entry| entry.executable_path == game.executable_path) {
            return Err(LibraryError::DuplicatePath);
        }
        self.entries.push(game);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let previous_len = self.entries.len();
        self.entries.retain(|entry| entry.id != id);
        self.entries.len() != previous_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn game(id: &str, path: &str) -> GameRecord {
        GameRecord { id: id.into(), title: "Sample".into(),
            executable_path: path.into(), source: ImportSource::Executable }
    }

    #[test] fn accepts_absolute_exe() {
        let mut lib = GameLibrary::default();
        assert_eq!(lib.add(game("one", "/Games/sample.exe")), Ok(()));
        assert_eq!(lib.entries().len(), 1);
    }
    #[test] fn rejects_relative_path() {
        let mut lib = GameLibrary::default();
        assert_eq!(lib.add(game("one", "Games/sample.exe")), Err(LibraryError::PathMustBeAbsolute));
    }
    #[test] fn rejects_non_exe() {
        let mut lib = GameLibrary::default();
        assert_eq!(lib.add(game("one", "/Games/readme.txt")), Err(LibraryError::UnsupportedExecutable));
    }
    #[test] fn rejects_duplicate_id() {
        let mut lib = GameLibrary::default();
        lib.add(game("one", "/Games/one.exe")).unwrap();
        assert_eq!(lib.add(game("one", "/Games/two.exe")), Err(LibraryError::DuplicateId));
    }
    #[test] fn removes_matching_id() {
        let mut lib = GameLibrary::default();
        lib.add(game("one", "/Games/one.exe")).unwrap();
        assert!(lib.remove("one"));
        assert!(!lib.remove("one"));
        assert!(lib.entries().is_empty());
    }
}
