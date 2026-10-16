use crate::platform::ffi;

/// `VirtualMachineInstanceState`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VirtualMachineInstanceState {
  Starting,
  Running,
  Stopped,
  Stopping,
  Unknown,
}

impl From<ffi::InstanceState> for VirtualMachineInstanceState {
  fn from(state: ffi::InstanceState) -> Self {
    match state {
      ffi::InstanceState::Starting => Self::Starting,
      ffi::InstanceState::Running => Self::Running,
      ffi::InstanceState::Stopped => Self::Stopped,
      ffi::InstanceState::Stopping => Self::Stopping,
      ffi::InstanceState::Unknown => Self::Unknown,
    }
  }
}

/// `VirtiofsLayout`, which says how a VM exposes virtiofs devices to its
/// guest.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VirtiofsLayout {
  /// One device, tagged `virtiofs`, carries each share as a subdirectory.
  Unified,
  /// One device per share.
  PerTag,
}

impl From<ffi::VirtiofsLayoutKind> for VirtiofsLayout {
  fn from(layout: ffi::VirtiofsLayoutKind) -> Self {
    match layout {
      ffi::VirtiofsLayoutKind::Unified => Self::Unified,
      ffi::VirtiofsLayoutKind::PerTag => Self::PerTag,
    }
  }
}
