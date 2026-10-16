//! Stand-ins for `FileDescriptorOps.swift`.

use super::CzOutcome;
use super::FdOpsSymlinkPolicy;
use crate::containerization_os::file_descriptor_ops;
use crate::platform::Enumerate;
use crate::platform::WithDescriptor;

taken!(
  optional_fd_ops_entry_type -> Option<file_descriptor_ops::EntryType>,
  fd_ops_file_status -> file_descriptor_ops::FileStatus,
  optional_fd_ops_file_status -> Option<file_descriptor_ops::FileStatus>,
);

failing!(
  cz_fd_ops_entry_type(i32, &str),
  cz_fd_ops_open_directory(i32, &str),
  cz_fd_ops_make_directory(i32, &str, bool, u16),
  cz_fd_ops_unlink(i32, &str),
  cz_fd_ops_status_of(i32),
  cz_fd_ops_status(i32, &str),
  cz_fd_ops_status_relative_path(i32, &str, FdOpsSymlinkPolicy),
  cz_fd_ops_open_file(i32, &str),
  cz_fd_ops_open_file_relative_path(i32, &str, FdOpsSymlinkPolicy),
  cz_fd_ops_create_file(i32, &str, bool, u16),
  cz_fd_ops_make_symlink(i32, &str, &str),
  cz_fd_ops_read_symlink(i32, &str),
  cz_fd_ops_unlink_recursive(i32, &str),
  cz_fd_ops_enumerate(i32, Enumerate),
  cz_fd_ops_get_canonical_path(i32),
  cz_fd_ops_mkdir(i32, &str, bool, u16, bool, WithDescriptor),
);
