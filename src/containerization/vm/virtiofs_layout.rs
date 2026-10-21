use crate::platform::ffi;

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
