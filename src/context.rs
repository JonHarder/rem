use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{read_dir, Metadata};
use std::os::unix::fs::DirEntryExt2;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    dir: PathBuf,
    recent_files: Vec<PathBuf>,
}

impl Context {
    pub fn new() -> Self {
        let dir = current_dir().expect("to be able to access the current working directory");
        Context {
            dir: dir.as_path().into(),
            recent_files: get_files(&dir),
        }
    }
}

/// Returns the list of files (not directories) in the current directory given by dir, sorted by most recently modified.
fn get_files(dir: &Path) -> Vec<PathBuf> {
    let dir_entries = match read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };
    let mut files = dir_entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let metadata = e.metadata().ok()?;
                match metadata.is_dir() {
                    true => None,
                    false => Some((e.path(), metadata)),
                }
            })
        })
        .collect::<Vec<(PathBuf, Metadata)>>();
    files.sort_by(|a, b| {
        let a_time = a.1.modified().expect("to get the modified time of a");
        let b_time = b.1.modified().expect("to get the modified time of b");
        b_time.cmp(&a_time)
    });
    files.into_iter().map(|(path, _)| path).collect()
}
