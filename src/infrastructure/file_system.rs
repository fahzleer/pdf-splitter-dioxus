use std::path::Path;
use crate::domain::errors::SplitError;
use crate::domain::ports::FileSystem;

pub struct NativeFileSystem;

impl NativeFileSystem {
    pub fn new() -> Self {
        NativeFileSystem
    }
}

impl FileSystem for NativeFileSystem {
    fn ensure_dir(&self, path: &Path) -> Result<(), SplitError> {
        std::fs::create_dir_all(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => SplitError::PermissionDenied {
                path: path.to_path_buf(),
            },
            _ => SplitError::WriteError { msg: e.to_string() },
        })
    }

    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn open_in_explorer(&self, path: &Path) {
        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open").arg(path).spawn();
        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("explorer").arg(path).spawn();
        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
}
