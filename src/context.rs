use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{read_dir, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    dir: PathBuf,
    recent_files: Vec<PathBuf>,
}

impl Context {
    pub fn from_current_dir() -> io::Result<Self> {
        let dir = current_dir().expect("to be able to access the current working directory");
        Ok(Context {
            dir: dir.as_path().into(),
            recent_files: get_files(&dir)?,
        })
    }
}

struct FileWithModification {
    file: PathBuf,
    is_file: bool,
    modified: SystemTime,
}

impl FileWithModification {
    fn try_from(entry: DirEntry) -> io::Result<Self> {
        let metadata = entry.metadata()?;
        Ok(FileWithModification {
            file: entry.path(),
            is_file: metadata.is_file(),
            modified: metadata.modified()?,
        })
    }
}

/// Returns the list of files (not directories) in the current directory given by dir, sorted by most recently modified.
fn get_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let dir_entries = read_dir(dir)?;
    let mut files: Vec<FileWithModification> = dir_entries
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| FileWithModification::try_from(e).ok())
        })
        .filter(|f| f.is_file)
        .collect();
    files.sort_by_key(|x| x.modified);
    Ok(files.into_iter().map(|x| x.file).collect())
}
