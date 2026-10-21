//! The values a VM is configured with, and the emulator its guest agent sets
//! up, which Swift reads.

use crate::containerization::vm::AttachedFilesystem as RustAttachedFilesystem;
use crate::containerization::vm::BootLog as RustBootLog;
use crate::containerization::vm::Kernel as RustKernel;
use crate::containerization::vm::SystemPlatform as RustSystemPlatform;
use crate::containerization_os::linux::binfmt::Entry as RustBinfmtEntry;

use super::ffi::BootLogKind;
use super::ffi::PlatformArchitecture;
use super::ffi::PlatformOs;

#[swift_bridge::bridge]
mod ffi {
  #[swift_bridge(already_declared)]
  enum BootLogKind {}

  #[swift_bridge(already_declared)]
  enum PlatformOs {}

  #[swift_bridge(already_declared)]
  enum PlatformArchitecture {}

  extern "Rust" {
    type RustBootLog;
    fn kind(self: &RustBootLog) -> BootLogKind;
    // `path` is `None` if the path isn't UTF-8, and Swift throws, naming it
    // by `lossyPath`.
    fn path(self: &RustBootLog) -> Option<String>;
    #[swift_bridge(swift_name = "lossyPath")]
    fn lossy_path(self: &RustBootLog) -> String;
    fn append(self: &RustBootLog) -> bool;
    #[swift_bridge(swift_name = "fileHandle")]
    fn file_handle(self: &RustBootLog) -> i32;

    type RustAttachedFilesystem;
    #[swift_bridge(swift_name = "filesystemType")]
    fn filesystem_type(self: &RustAttachedFilesystem) -> &str;
    fn source(self: &RustAttachedFilesystem) -> &str;
    fn destination(self: &RustAttachedFilesystem) -> &str;
    #[swift_bridge(swift_name = "optionsLen")]
    fn options_len(self: &RustAttachedFilesystem) -> usize;
    #[swift_bridge(swift_name = "optionsAt")]
    fn options_at(self: &RustAttachedFilesystem, index: usize) -> &str;

    type RustSystemPlatform;
    fn os(self: &RustSystemPlatform) -> PlatformOs;
    fn architecture(self: &RustSystemPlatform) -> PlatformArchitecture;

    // The emulator `Vminitd.setupEmulator` registers. Its `type` is read as
    // `entryType`, because `type` is a Swift keyword.
    type RustBinfmtEntry;
    fn name(self: &RustBinfmtEntry) -> &str;
    #[swift_bridge(swift_name = "entryType")]
    fn entry_type(self: &RustBinfmtEntry) -> &str;
    fn offset(self: &RustBinfmtEntry) -> &str;
    fn magic(self: &RustBinfmtEntry) -> &str;
    fn mask(self: &RustBinfmtEntry) -> &str;
    fn flags(self: &RustBinfmtEntry) -> &str;

    type RustKernel;
    // `path` is `None` if the path isn't UTF-8, and Swift throws, naming it
    // by `lossyPath`.
    fn path(self: &RustKernel) -> Option<String>;
    #[swift_bridge(swift_name = "lossyPath")]
    fn lossy_path(self: &RustKernel) -> String;
    fn platform(self: &RustKernel) -> &RustSystemPlatform;
    #[swift_bridge(swift_name = "kernelArgsLen")]
    fn kernel_args_len(self: &RustKernel) -> usize;
    #[swift_bridge(swift_name = "kernelArgsAt")]
    fn kernel_args_at(self: &RustKernel, index: usize) -> &str;
    #[swift_bridge(swift_name = "initArgsLen")]
    fn init_args_len(self: &RustKernel) -> usize;
    #[swift_bridge(swift_name = "initArgsAt")]
    fn init_args_at(self: &RustKernel, index: usize) -> &str;
  }
}
