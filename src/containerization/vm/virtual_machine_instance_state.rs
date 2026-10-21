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
