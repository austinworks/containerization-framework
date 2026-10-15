//! The closures Rust hands Swift, each in an opaque type with a `call` method.

use crate::containerization::container::linux_container::Configuration as RustLinuxContainerConfiguration;
use crate::containerization::container::linux_pod::Configuration as RustPodConfiguration;
use crate::containerization::container::linux_pod::ContainerConfiguration as RustPodContainerConfiguration;
use crate::containerization::process::LinuxProcessConfiguration as RustLinuxProcessConfiguration;
use crate::platform::ConfigureContainer as RustConfigure;
use crate::platform::ConfigurePod as RustConfigurePod;
use crate::platform::ConfigurePodContainer as RustConfigurePodContainer;
use crate::platform::ConfigureProcess as RustConfigureProcess;
use crate::platform::Enumerate as RustEnumerate;
use crate::platform::Progress as RustProgressHandler;
use crate::platform::WithDescriptor as RustWithDescriptor;

use super::ffi::FdOpsEntryType;
use super::ffi::ProgressKind;

#[swift_bridge::bridge]
mod ffi {
  #[swift_bridge(already_declared)]
  enum ProgressKind {}
  #[swift_bridge(already_declared)]
  enum FdOpsEntryType {}

  extern "Rust" {
    #[swift_bridge(already_declared)]
    type RustLinuxContainerConfiguration;
    #[swift_bridge(already_declared)]
    type RustLinuxProcessConfiguration;
    #[swift_bridge(already_declared)]
    type RustPodConfiguration;
    #[swift_bridge(already_declared)]
    type RustPodContainerConfiguration;

    // Each `call` returns whether Rust's closure threw, and Rust keeps what
    // it threw.

    // A `(inout LinuxContainer.Configuration) throws -> Void`: Swift calls
    // it once, with the configuration it seeded.
    type RustConfigure;
    fn call(self: &RustConfigure, configuration: &mut RustLinuxContainerConfiguration) -> bool;

    // A `(inout LinuxProcessConfiguration) throws -> Void`: Swift calls it
    // once, with the configuration it filled.
    type RustConfigureProcess;
    fn call(self: &RustConfigureProcess, process: &mut RustLinuxProcessConfiguration) -> bool;

    // A `(inout LinuxPod.Configuration) throws -> Void`, which Swift calls
    // with the configuration it filled, and a `(inout
    // LinuxPod.ContainerConfiguration) throws -> Void`, which Swift calls once
    // on Rust's default configuration. Swift doesn't fill that one, because
    // Rust's default matches Swift's.
    type RustConfigurePod;
    fn call(self: &RustConfigurePod, configuration: &mut RustPodConfiguration) -> bool;
    type RustConfigurePodContainer;
    fn call(self: &RustConfigurePodContainer, configuration: &mut RustPodContainerConfiguration) -> bool;

    // A `ProgressHandler?`. Swift asks `is_some` before calling it.
    type RustProgressHandler;
    #[swift_bridge(swift_name = "isSome")]
    fn is_some(self: &RustProgressHandler) -> bool;
    fn call(self: &RustProgressHandler, kinds: Vec<ProgressKind>, values: Vec<i64>);

    // A `(FileDescriptor) throws -> Void`, which
    // `FileDescriptorOps.mkdir` calls once with the directory it opened.
    type RustWithDescriptor;
    fn call(self: &RustWithDescriptor, descriptor: i32) -> bool;

    // `FileDescriptorOps.enumerate`'s body, which Swift calls once for each
    // entry with its path, its type and the directory that holds it.
    type RustEnumerate;
    fn call(self: &RustEnumerate, path: String, entry_type: FdOpsEntryType, parent: i32) -> bool;
  }
}
