// Based on deno implementation

use crate::fs::normalize_p;
use crate::throw;
use anyhow::{anyhow, Result};
use boa_engine::{Context, JsNativeError, JsResult, JsString, JsValue};
use std::env::current_dir;
use std::io;
use std::io::Read;
use std::path::Path;
use tokio::task::spawn_blocking;

pub mod dir;
pub mod files;

pub fn resolve_path_from_args(args: &[JsValue], ctx: &mut Context) -> JsResult<JsString> {
    if args.is_empty() {
        throw!(err, "Expected a path in fs function");
    }
    let path = args.first().unwrap();
    let path = path.to_string(ctx)?;
    Ok(path)
}

pub struct FileSystem {}

impl FileSystem {
    pub fn open_sync(path: &Path) -> anyhow::Result<File> {
        let file = open_file(path)?;
        Ok(File::new(file))
    }

    pub async fn open_async(path: &Path) -> anyhow::Result<File> {
        let file = open_file(path)?;
        Ok(File::new(file))
    }

    pub fn create_dir_all_sync(path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn create_dir_sync(path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir(path)?;
        Ok(())
    }
}

pub struct File {
    file: std::fs::File,
}

impl File {
    pub fn new(file: std::fs::File) -> Self {
        Self { file }
    }

    pub fn from_path(path: &Path) -> Result<Self, io::Error> {
        let file = open_file(path).unwrap();
        Ok(Self::new(file))
    }

    pub fn read_sync(mut self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub async fn read_async(self) -> Result<Vec<u8>, io::Error> {
        let mut file = self.file;
        let buf: Vec<u8> = spawn_blocking(move || {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            Ok::<Vec<u8>, io::Error>(buf)
        })
        .await??;

        Ok(buf)
    }
}

#[inline(always)]
pub fn open_file(path: &Path) -> Result<std::fs::File> {
    let path_bytes = path.as_os_str().as_encoded_bytes();
    let is_windows_device_path =
        cfg!(windows) && path_bytes.starts_with(br"\\.\") && !path_bytes.contains(&b':');
    let path = if is_windows_device_path {
        path.to_owned()
    } else if path.is_absolute() {
        normalize_p(path)
    } else {
        let cwd = current_dir()?;
        normalize_p(cwd.join(path))
    };

    let needs_canonicalization =
        !is_windows_device_path && (!cfg!(target_os = "linux") || path.starts_with("/proc"));
    let path = if needs_canonicalization {
        match path.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                if let (Some(parent), Some(filename)) = (path.parent(), path.file_name()) {
                    parent.canonicalize()?.join(filename)
                } else {
                    return Err(anyhow!("Failed to canonicalize path"));
                }
            }
        }
    } else {
        path
    };

    std::fs::File::open(&path).map_err(Into::into)
}
