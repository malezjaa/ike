use anyhow::{format_err, Result};
use ike_fs::FsError::{FailedToReadFile, FailedToReadFileWithError, FileNotFound};
use logger::{elog, Logger};
use regex::Regex;
use std::path::Component;
use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
};

pub fn read_json<Json, FilePath>(file_path: FilePath) -> Result<Json>
where
    Json: serde::de::DeserializeOwned,
    FilePath: AsRef<Path>,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let serialized_json = serde_json::from_str(&contents);

    match serialized_json {
        Ok(json) => Ok(json),
        Err(error) => Err(FailedToReadFileWithError(error.to_string()).into()),
    }
}

// Determine if a path is a file using regex
pub fn is_file(path: &str) -> bool {
    let re = Regex::new(r"\.[a-zA-Z0-9]+$").unwrap();
    re.is_match(path)
}

pub fn normalize_path(mut path: PathBuf, root: PathBuf) -> Result<PathBuf> {
    if path.is_relative() {
        path = root.join(path);
    }

    path = match path.canonicalize() {
        Ok(path) => path,
        Err(err) => return match err.kind() {
            ErrorKind::NotFound => Err(FileNotFound(path).into()),
            _ => Err(FailedToReadFile(path).into()),
        },
    };
    let root_str = path.to_str().unwrap_or("");
    // On windows, paths can be prefixed with \\?\ to allow longer paths, we need to remove this prefix
    let normalized_root_str = if root_str.starts_with(r"\\?\") {
        root_str.strip_prefix(r"\\?\").unwrap()
    } else {
        root_str
    };
    path = PathBuf::from(normalized_root_str);

    Ok(path)
}

#[inline]
pub fn normalize_p<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
