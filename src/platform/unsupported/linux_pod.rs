//! Stand-ins for `LinuxPod.swift`.

use super::CzOutcome;
use super::CzUnixSocketConfiguration;
use super::CzVirtualMachineManager;
use super::FilesystemOperationKind;
use crate::containerization::container;
use crate::containerization::process;
use crate::platform;
use std::convert::Infallible;

taken!(linux_pod -> CzLinuxPod);

handles!(CzLinuxPod);

failing!(cz_linux_pod_new(
  &str,
  CzVirtualMachineManager,
  u32,
  u64,
  container::linux_pod::Configuration
));

impl CzLinuxPod {
  pub(crate) fn id(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn config(&self, _seed: container::linux_pod::Configuration, _receive: platform::ConfigurePod) {
    match self.0 {}
  }

  pub(crate) fn vm_cpus(&self) -> u32 {
    match self.0 {}
  }

  pub(crate) fn vm_memory_in_bytes(&self) -> u64 {
    match self.0 {}
  }

  pub(crate) fn add_container(
    &self,
    _id: &str,
    _rootfs: container::Mount,
    _seed: container::linux_pod::ContainerConfiguration,
    _configuration: platform::ConfigurePodContainer,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn start_container(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stop_container(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stop(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn kill_container(&self, _id: &str, _signal: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn wait_container(&self, _id: &str, _timeout_in_seconds: Option<i64>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize_container(&self, _id: &str, _width: u16, _height: u16) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn exec_in_container(
    &self,
    _id: &str,
    _process_id: &str,
    _seed: process::LinuxProcessConfiguration,
    _configuration: platform::ConfigureProcess,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn list_containers(&self) -> Vec<String> {
    match self.0 {}
  }

  pub(crate) fn statistics(
    &self,
    _has_container_ids: bool,
    _container_ids: Vec<String>,
    _categories: i64,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn dial_vsock(&self, _port: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn virtual_machine_instance(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn filesystem_operation(&self, _id: &str, _operation: FilesystemOperationKind, _path: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close_container_stdin(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn relay_unix_socket(&self, _id: &str, _socket: CzUnixSocketConfiguration) -> CzOutcome {
    match self.0 {}
  }
}
