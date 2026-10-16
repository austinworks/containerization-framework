//! Stand-ins for the bridge on every platform but macOS: the same types and
//! functions, so the wrappers are written once and a cross-platform workspace
//! still compiles.
//!
//! Nothing can make a Swift object here: every constructor's outcome is a
//! failure, and every handle is uninhabited, so its methods never run.

const MACOS_ONLY: &str = "Containerization.framework is macOS only";

/// `ProgressEvent`, less its value.
pub(crate) enum ProgressKind {
  Items,
  TotalItems,
  Size,
  TotalSize,
}

/// swift-log's `Logger.Level`.
pub(crate) enum LogLevel {
  Trace,
  Debug,
  Info,
  Notice,
  Warning,
  Error,
  Critical,
}

/// `Hosts.Entry`'s static constructors.
pub(crate) enum HostsEntryName {
  LocalHostIpv4,
  LocalHostIpv6,
  Ipv6LocalNet,
  Ipv6MulticastPrefix,
  Ipv6AllNodes,
  Ipv6AllRouters,
}

/// `FileDescriptorOps.EntryType`.
pub(crate) enum FdOpsEntryType {
  Regular,
  Directory,
  Symlink,
  Other,
}

/// `FileDescriptorOps.SymlinkPolicy`.
pub(crate) enum FdOpsSymlinkPolicy {
  Refuse,
  FollowBeneath,
}

/// `FilesystemOperation`.
pub(crate) enum FilesystemOperationKind {
  Freeze,
  Thaw,
  Trim,
}

/// vmnet's `operating_modes_t`.
pub(crate) enum VmnetMode {
  Shared,
  Host,
  Bridged,
}

/// `UnixSocketConfiguration.Direction`.
pub(crate) enum SocketDirection {
  Into,
  OutOf,
}

/// `VirtualMachineInstanceState`.
pub(crate) enum InstanceState {
  Starting,
  Running,
  Stopped,
  Stopping,
  Unknown,
}

/// `VirtiofsLayout`.
pub(crate) enum VirtiofsLayoutKind {
  Unified,
  PerTag,
}

/// A `Network?`, which is only ever `nil` here.
pub(crate) struct CzNetwork;

pub(crate) fn cz_no_network() -> CzNetwork {
  CzNetwork
}

/// An outcome that is always a failure, so nothing is ever taken from it.
pub(crate) struct CzOutcome;

impl CzOutcome {
  pub(crate) fn error(&self) -> Option<String> {
    Some(MACOS_ONLY.to_string())
  }
}

macro_rules! taken {
  ($($name:ident -> $type:ty),* $(,)?) => {
    impl CzOutcome {
      $(pub(crate) fn $name(&self) -> $type {
        unreachable!("a failed outcome holds nothing")
      })*
    }
  };
}

macro_rules! handles {
  ($($name:ident),* $(,)?) => {
    $(pub(crate) struct $name(Infallible);)*
  };
}

/// Declares free functions, by their argument types, that always fail here.
macro_rules! failing {
  ($($name:ident($($type:ty),* $(,)?)),* $(,)?) => {
    $(pub(crate) fn $name($(_: $type),*) -> CzOutcome {
      CzOutcome
    })*
  };
}

mod archive_ext4_io;
mod containers;
mod images;
mod os;
mod values;
mod vm_network;

pub(crate) use self::archive_ext4_io::*;
pub(crate) use self::containers::*;
pub(crate) use self::images::*;
pub(crate) use self::os::*;
pub(crate) use self::values::*;
pub(crate) use self::vm_network::*;
