//! Getters for what a container manager is asked to create, and the
//! filesystem operations it performs.

use crate::containerization::container;

impl container::container_manager::CreateOptions {
  pub(crate) fn rootfs_size_in_bytes(&self) -> u64 {
    self.rootfs_size_in_bytes
  }

  pub(crate) fn writable_layer_size_in_bytes(&self) -> Option<u64> {
    self.writable_layer_size_in_bytes
  }

  pub(crate) fn read_only(&self) -> bool {
    self.read_only
  }

  pub(crate) fn networking(&self) -> bool {
    self.networking
  }

  pub(crate) fn vm_cpus(&self) -> u32 {
    self.vm.cpus
  }

  pub(crate) fn vm_memory_in_bytes(&self) -> u64 {
    self.vm.memory_in_bytes
  }
}
