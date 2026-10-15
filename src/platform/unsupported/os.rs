//! Terminals, file information, `FileDescriptorOps`, the keychain and sysctl.

use super::CzOutcome;
use super::FdOpsSymlinkPolicy;
use crate::containerization_os;
use crate::containerization_os::file_descriptor_ops;
use crate::platform::Enumerate;
use crate::platform::WithDescriptor;
use std::convert::Infallible;

taken!(
  terminal -> CzTerminal,
  parent_terminal -> CzTerminal,
  child_terminal -> CzTerminal,
  terminal_size_width -> u16,
  terminal_size_height -> u16,
  keychain_query_result -> containerization_os::keychain::KeychainQueryResult,
  file_info -> CzFileInfo,
  stat -> containerization_os::Stat,
  registry_infos -> Vec<containerization_os::keychain::RegistryInfo>,
  optional_fd_ops_entry_type -> Option<file_descriptor_ops::EntryType>,
  fd_ops_file_status -> file_descriptor_ops::FileStatus,
  optional_fd_ops_file_status -> Option<file_descriptor_ops::FileStatus>,
);

handles!(CzTerminal, CzFileInfo,);

failing!(
  cz_keychain_helper_lookup(&str, Option<String>, &str),
  cz_keychain_helper_list(&str, Option<String>),
  cz_keychain_helper_delete(&str, Option<String>, &str),
  cz_keychain_helper_save(&str, Option<String>, &str, &str, &str),
  cz_terminal_new(i32, bool),
  cz_terminal_current(),
  cz_terminal_create(bool, u16, u16),
  cz_keychain_query_save(&str, Option<String>, &str, &str, &str),
  cz_keychain_query_delete(&str, Option<String>, &str),
  cz_keychain_query_get(&str, Option<String>, &str),
  cz_keychain_query_list(&str, Option<String>),
  cz_keychain_query_exists(&str, Option<String>, &str),
  cz_sysctl_by_name(&str),
  cz_file_info(&str),
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

impl CzTerminal {
  pub(crate) fn duplicate(&self) -> CzTerminal {
    match self.0 {}
  }

  pub(crate) fn handle(&self) -> i32 {
    match self.0 {}
  }

  pub(crate) fn write(&self, _data: Vec<u8>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn size(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize_from(&self, _pty: CzTerminal) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize_size(&self, _width: u16, _height: u16) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize(&self, _width: u16, _height: u16) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn setraw(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn enable_echo(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn disable_echo(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn reset(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn try_reset(&self) {
    match self.0 {}
  }
}

impl CzFileInfo {
  pub(crate) fn mode(&self) -> u16 {
    match self.0 {}
  }

  pub(crate) fn uid(&self) -> i64 {
    match self.0 {}
  }

  pub(crate) fn gid(&self) -> i64 {
    match self.0 {}
  }

  pub(crate) fn dev(&self) -> i64 {
    match self.0 {}
  }

  pub(crate) fn ino(&self) -> i64 {
    match self.0 {}
  }

  pub(crate) fn size(&self) -> i64 {
    match self.0 {}
  }

  pub(crate) fn path(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn is_directory(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_pipe(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_socket(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_link(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_regular_file(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_block(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn is_char(&self) -> bool {
    match self.0 {}
  }
}
