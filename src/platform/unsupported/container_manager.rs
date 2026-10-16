//! Stand-ins for `ContainerManager.swift`.

use super::CzImage;
use super::CzImageStore;
use super::CzNetwork;
use super::CzOutcome;
use super::CzVirtualMachineManager;
use crate::containerization::container;
use crate::containerization::vm;
use crate::platform;
use std::convert::Infallible;

taken!(container_manager -> CzContainerManager);

handles!(CzContainerManager);

failing!(
  cz_container_manager_at_root(vm::Kernel, container::Mount, Option<String>, CzNetwork, bool, bool),
  cz_container_manager_at_root_with_initfs_reference(vm::Kernel, &str, Option<String>, CzNetwork, bool, bool),
  cz_container_manager_with_vmm(CzVirtualMachineManager, CzNetwork),
);

/// `ContainerManager`'s inits that take an image store.
impl CzImageStore {
  pub(crate) fn container_manager(
    &self,
    _kernel: vm::Kernel,
    _initfs: container::Mount,
    _network: CzNetwork,
    _rosetta: bool,
    _nested_virtualization: bool,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn container_manager_with_initfs_reference(
    &self,
    _kernel: vm::Kernel,
    _initfs_reference: &str,
    _network: CzNetwork,
    _rosetta: bool,
    _nested_virtualization: bool,
  ) -> CzOutcome {
    match self.0 {}
  }
}

impl CzContainerManager {
  pub(crate) fn image_store(&self) -> CzImageStore {
    match self.0 {}
  }

  pub(crate) fn create_from_reference(
    &self,
    _id: &str,
    _reference: &str,
    _options: container::container_manager::CreateOptions,
    _progress: platform::Progress,
    _seed: container::linux_container::Configuration,
    _configuration: platform::ConfigureContainer,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(
    &self,
    _id: &str,
    _image: CzImage,
    _options: container::container_manager::CreateOptions,
    _progress: platform::Progress,
    _seed: container::linux_container::Configuration,
    _configuration: platform::ConfigureContainer,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_with_rootfs(
    &self,
    _id: &str,
    _image: CzImage,
    _rootfs: container::Mount,
    _options: container::container_manager::CreateWithRootfsOptions,
    _seed: container::linux_container::Configuration,
    _configuration: platform::ConfigureContainer,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn release_network(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }
}
