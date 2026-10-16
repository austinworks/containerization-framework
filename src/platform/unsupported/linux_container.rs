//! Stand-ins for `LinuxContainer.swift`: `LinuxContainer`, `LinuxProcess` and
//! `ContainerStatistics`.

use super::CzOutcome;
use super::CzVirtualMachineManager;
use super::FilesystemOperationKind;
use crate::containerization::container;
use crate::containerization::process;
use crate::platform;
use std::convert::Infallible;

taken!(
  linux_container -> CzLinuxContainer,
  linux_process -> CzLinuxProcess,
  container_statistics -> container::ContainerStatistics,
  container_statistics_list -> Vec<container::ContainerStatistics>,
);

handles!(CzLinuxContainer, CzLinuxProcess);

failing!(cz_linux_container_new(
  &str,
  container::Mount,
  bool,
  container::Mount,
  CzVirtualMachineManager,
  u32,
  u64,
  container::linux_container::Configuration,
));

impl CzLinuxContainer {
  pub(crate) fn id(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn virtual_machine_instance(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn rootfs(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn writable_layer(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn config(
    &self,
    _seed: container::linux_container::Configuration,
    _receive: platform::ConfigureContainer,
  ) {
    match self.0 {}
  }

  pub(crate) fn vm_cpus(&self) -> u32 {
    match self.0 {}
  }

  pub(crate) fn vm_memory_in_bytes(&self) -> u64 {
    match self.0 {}
  }

  pub(crate) fn exec_with(
    &self,
    _id: &str,
    _seed: process::LinuxProcessConfiguration,
    _configuration: platform::ConfigureProcess,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn dial_vsock(&self, _port: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn statistics(&self, _categories: i64) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn filesystem_operation(&self, _operation: FilesystemOperationKind, _path: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn copy_in(
    &self,
    _source: &str,
    _destination: &str,
    _mode: u32,
    _create_parents: bool,
    _chunk_size: usize,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn copy_out(
    &self,
    _source: &str,
    _destination: &str,
    _create_parents: bool,
    _chunk_size: usize,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn start(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stop(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn kill(&self, _signal: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn wait(&self, _timeout_in_seconds: Option<i64>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize(&self, _width: u16, _height: u16) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn exec(&self, _id: &str, _configuration: process::LinuxProcessConfiguration) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close_stdin(&self) -> CzOutcome {
    match self.0 {}
  }
}

impl CzLinuxProcess {
  pub(crate) fn id(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn owning_container(&self) -> Option<String> {
    match self.0 {}
  }

  pub(crate) fn pid(&self) -> i32 {
    match self.0 {}
  }

  pub(crate) fn start(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn kill(&self, _signal: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize(&self, _width: u16, _height: u16) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close_stdin(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn wait(&self, _timeout_in_seconds: Option<i64>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete(&self) -> CzOutcome {
    match self.0 {}
  }
}
