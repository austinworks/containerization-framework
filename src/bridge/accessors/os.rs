//! Readers for what ContainerizationOS returns, and getters for the
//! `Binfmt.Entry` Swift reads.

use crate::bridge::ffi;
use crate::containerization_os;
use crate::containerization_os::file_descriptor_ops;
use crate::containerization_os::keychain;
use crate::containerization_os::linux::binfmt;
use crate::error::Error;
use crate::platform;

impl binfmt::Entry {
  pub(crate) fn name(&self) -> &str {
    &self.name
  }

  pub(crate) fn entry_type(&self) -> &str {
    &self.r#type
  }

  pub(crate) fn offset(&self) -> &str {
    &self.offset
  }

  pub(crate) fn magic(&self) -> &str {
    &self.magic
  }

  pub(crate) fn mask(&self) -> &str {
    &self.mask
  }

  pub(crate) fn flags(&self) -> &str {
    &self.flags
  }
}

impl ffi::CzOutcome {
  /// The `Stat` an outcome holds.
  pub(crate) fn stat(&self) -> containerization_os::Stat {
    let [dev, ino, mode, nlink, uid, gid, rdev] = self.stat_unsigned()[..] else {
      panic!("Swift sends a `Stat`'s seven unsigned fields");
    };
    let [
      size,
      blksize,
      blocks,
      atime,
      atime_nanos,
      mtime,
      mtime_nanos,
      ctime,
      ctime_nanos,
    ] = self.stat_signed()[..]
    else {
      panic!("Swift sends a `Stat`'s nine signed fields");
    };
    let time = |seconds, nanoseconds: i64| containerization_os::TimeSpec {
      seconds,
      nanoseconds: nanoseconds as i32,
    };

    containerization_os::Stat {
      dev,
      ino,
      mode: mode as u32,
      nlink,
      uid: uid as u32,
      gid: gid as u32,
      rdev,
      size,
      blksize,
      blocks,
      atime: time(atime, atime_nanos),
      mtime: time(mtime, mtime_nanos),
      ctime: time(ctime, ctime_nanos),
    }
  }

  /// The `KeychainQueryResult` an outcome holds, its dates crossing as
  /// seconds since 1970.
  pub(crate) fn keychain_query_result(&self) -> Result<keychain::KeychainQueryResult, Error> {
    Ok(keychain::KeychainQueryResult {
      username: self.keychain_query_result_username(),
      password: self.keychain_query_result_password(),
      modified_date: platform::system_time(self.keychain_query_result_modified_date())?,
      created_date: platform::system_time(self.keychain_query_result_created_date())?,
    })
  }

  /// The `EntryType?` an outcome holds.
  pub(crate) fn optional_fd_ops_entry_type(&self) -> Option<file_descriptor_ops::EntryType> {
    self.optional(|entry_type| entry_type.held_entry_type().into())
  }

  /// The `FileStatus` an outcome holds.
  pub(crate) fn fd_ops_file_status(&self) -> file_descriptor_ops::FileStatus {
    file_descriptor_ops::FileStatus {
      r#type: self.held_status_type().into(),
      permissions: self.held_status_permissions(),
      size: self.held_status_size(),
      user_id: self.held_status_user_id(),
      group_id: self.held_status_group_id(),
      modification_seconds: self.held_status_modification_seconds(),
      modification_nanoseconds: self.held_status_modification_nanoseconds(),
    }
  }

  /// The `FileStatus?` an outcome holds.
  pub(crate) fn optional_fd_ops_file_status(&self) -> Option<file_descriptor_ops::FileStatus> {
    self.optional(Self::fd_ops_file_status)
  }

  /// The `FileDescriptorOps.Error` a failed outcome threw, if it threw one.
  pub(crate) fn thrown_fd_ops_error(&self) -> Option<file_descriptor_ops::Error> {
    self
      .thrown_fd_ops_error_outcome()
      .optional(|error| match error.held_fd_ops_error_kind() {
        ffi::FdOpsErrorKind::InvalidRelativePath => file_descriptor_ops::Error::InvalidRelativePath,
        ffi::FdOpsErrorKind::InvalidPathComponent => file_descriptor_ops::Error::InvalidPathComponent,
        ffi::FdOpsErrorKind::CannotFollowSymlink => file_descriptor_ops::Error::CannotFollowSymlink,
        ffi::FdOpsErrorKind::NotFound => file_descriptor_ops::Error::NotFound,
        ffi::FdOpsErrorKind::AlreadyExists => file_descriptor_ops::Error::AlreadyExists,
        ffi::FdOpsErrorKind::Conflict => {
          file_descriptor_ops::Error::Conflict(error.held_fd_ops_error_entry_type().into())
        }
        ffi::FdOpsErrorKind::SystemError => {
          file_descriptor_ops::Error::SystemError(error.held_fd_ops_error_operation(), error.held_fd_ops_error_number())
        }
      })
  }
}

#[cfg(test)]
mod tests {
  use crate::bridge::ffi;
  use crate::containerization_os;

  #[test]
  fn copies_swifts_capability_names() {
    let names = containerization_os::linux::CapabilityName::ALL_CASES;

    assert_eq!(
      ffi::cz_capability_name_descriptions(),
      names
        .iter()
        .map(|name| name.description())
        .collect::<Vec<_>>()
    );
    assert_eq!(
      ffi::cz_capability_name_cap_values(),
      names
        .iter()
        .map(|name| name.cap_value())
        .collect::<Vec<_>>()
    );
  }

  #[test]
  fn copies_swifts_binfmt_values() {
    let amd64 = containerization_os::linux::binfmt::Entry::amd64();

    assert_eq!(ffi::cz_binfmt_path(), containerization_os::linux::binfmt::PATH);
    assert_eq!(
      ffi::cz_binfmt_entry_amd64(),
      [
        amd64.name,
        amd64.r#type,
        amd64.offset,
        amd64.magic,
        amd64.mask,
        amd64.flags,
      ]
    );
  }

  #[test]
  fn copies_swifts_symlink_limit() {
    assert_eq!(
      ffi::cz_fd_ops_maximum_symlinks_followed(),
      containerization_os::file_descriptor_ops::MAXIMUM_SYMLINKS_FOLLOWED
    );
  }

  #[test]
  fn copies_swifts_capability_sets() {
    assert_eq!(
      ffi::cz_capability_set_descriptions(),
      containerization_os::linux::CapabilitySet::ALL
        .iter()
        .map(|set| set.description())
        .collect::<Vec<_>>()
    );
  }
}
