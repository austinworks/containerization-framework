//! Stand-ins for `VirtualMachine.swift`: `VZVirtualMachineManager`,
//! `VZVirtualMachineInstance` and `VsockListener`.

use super::CzOutcome;
use super::InstanceState;
use super::VirtiofsLayoutKind;
use crate::containerization::container;
use crate::containerization::vm;
use std::convert::Infallible;

taken!(
  virtual_machine_manager -> CzVirtualMachineManager,
  virtual_machine_instance -> CzVirtualMachineInstance,
  vsock_listener -> CzVsockListener,
);

handles!(CzVirtualMachineManager, CzVirtualMachineInstance, CzVsockListener);

failing!(
  cz_virtual_machine_manager_new(vm::Kernel, container::Mount, bool, bool),
  cz_install_rosetta(),
);

impl CzVirtualMachineManager {
  pub(crate) fn duplicate(&self) -> CzVirtualMachineManager {
    match self.0 {}
  }

  pub(crate) fn create(&self, _config: vm::VMConfiguration) -> CzOutcome {
    match self.0 {}
  }
}

impl CzVirtualMachineInstance {
  pub(crate) fn state(&self) -> InstanceState {
    match self.0 {}
  }

  pub(crate) fn mounts(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn virtiofs_layout(&self) -> VirtiofsLayoutKind {
    match self.0 {}
  }

  pub(crate) fn start(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stop(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn pause(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resume(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn dial(&self, _port: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn listen(&self, _port: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn hotplug(&self, _block: container::Mount, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn register_mounts(
    &self,
    _id: &str,
    _rootfs: vm::AttachedFilesystem,
    _additional_mounts: Vec<container::Mount>,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn release_hotplug(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn hotplug_virtio_fs(&self, _mounts: Vec<container::Mount>, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn release_virtio_fs(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }
}

impl CzVsockListener {
  pub(crate) fn port(&self) -> u32 {
    match self.0 {}
  }

  pub(crate) fn next(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn finish(&self) -> CzOutcome {
    match self.0 {}
  }
}
