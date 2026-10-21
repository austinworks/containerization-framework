//! `File`.

use super::FileInfo;
use crate::error::Error;
use crate::platform;
use crate::platform::ffi;
use std::path::Path;

/// `File.info(_:)`, which doesn't follow a symlink at `path`.
pub fn info(path: &Path) -> Result<FileInfo, Error> {
  platform::outcome(
    ffi::cz_file_info(platform::path(path)?),
    format!("read {}'s file info", path.display()),
  )
  .map(|outcome| FileInfo {
    handle: outcome.file_info(),
  })
}
