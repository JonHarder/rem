use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{read_dir, DirEntry, ReadDir};
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

#[derive(Debug)]
struct FileWithModification {
    file: PathBuf,
    is_file: bool,
    modified: SystemTime,
}

impl TryFrom<DirEntry> for FileWithModification {
    type Error = io::Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let metadata = value.metadata()?;
        Ok(FileWithModification {
            file: value.path(),
            is_file: metadata.is_file(),
            modified: metadata.modified()?,
        })
    }
}

fn sort_files_recently_modified(dir: ReadDir) -> io::Result<Vec<FileWithModification>> {
    let mut files: Vec<FileWithModification> = dir
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| FileWithModification::try_from(e).ok())
        })
        .filter(|f| f.is_file)
        .collect();
    files.sort_by_key(|x| x.modified);
    Ok(files)
}

/// Returns the list of files (not directories) in the current directory given by dir, sorted by most recently modified.
fn get_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let dir_entries = read_dir(dir)?;
    let files = sort_files_recently_modified(dir_entries)?;
    Ok(files.into_iter().map(|x| x.file).collect())
}
