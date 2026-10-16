//! Stand-ins for `Vminitd.swift`: the guest agent.

use super::CzOutcome;
use super::CzUnixSocketConfiguration;
use super::CzVirtualMachineInstance;
use super::FilesystemOperationKind;
use crate::containerization::network;
use crate::containerization_extras;
use crate::containerization_oci;
use crate::containerization_os;
use std::convert::Infallible;

taken!(vminitd -> CzVminitd);

handles!(CzVminitd);

impl CzVirtualMachineInstance {
  pub(crate) fn dial_agent(&self) -> CzOutcome {
    match self.0 {}
  }
}

impl CzVminitd {
  pub(crate) fn standard_setup(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn filesystem_operation(
    &self,
    _operation: FilesystemOperationKind,
    _path: &str,
    _container_id: Option<String>,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn getenv(&self, _key: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn setenv(&self, _key: &str, _value: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn mount(&self, _mount: containerization_oci::runtime::Mount) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn umount(&self, _path: &str, _flags: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn mkdir(&self, _path: &str, _all: bool, _perms: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn kill(&self, _pid: i32, _signal: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn sync(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_process(
    &self,
    _id: &str,
    _container_id: Option<String>,
    _stdin_port: Option<u32>,
    _stdout_port: Option<u32>,
    _stderr_port: Option<u32>,
    _oci_runtime_path: Option<String>,
    _configuration: containerization_oci::runtime::Spec,
    _has_options: bool,
    _options: Vec<u8>,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn start_process(&self, _id: &str, _container_id: Option<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn signal_process(&self, _id: &str, _container_id: Option<String>, _signal: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resize_process(
    &self,
    _id: &str,
    _container_id: Option<String>,
    _columns: u32,
    _rows: u32,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn wait_process(
    &self,
    _id: &str,
    _container_id: Option<String>,
    _timeout_in_seconds: Option<i64>,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete_process(&self, _id: &str, _container_id: Option<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close_process_stdin(&self, _id: &str, _container_id: Option<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn up(&self, _name: &str, _mtu: Option<u32>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn down(&self, _name: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn address_add(&self, _name: &str, _address: containerization_extras::InterfaceAddress) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn route_add_link(&self, _name: &str, _route: containerization_extras::LinkRoute) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn route_add_default(&self, _name: &str, _route: containerization_extras::DefaultRoute) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn configure_dns(&self, _config: network::DNS, _location: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn configure_hosts(&self, _config: network::Hosts, _location: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn container_statistics(&self, _container_ids: Vec<String>, _categories: i64) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn setup_emulator(
    &self,
    _binary_path: &str,
    _configuration: containerization_os::linux::binfmt::Entry,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn set_time(&self, _sec: i64, _usec: i32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn sysctl(&self, _keys: Vec<String>, _values: Vec<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stat(&self, _root: &str, _path: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn enable_rosetta(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn relay_socket(&self, _port: u32, _configuration: CzUnixSocketConfiguration) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn stop_socket_relay(&self, _configuration: CzUnixSocketConfiguration) -> CzOutcome {
    match self.0 {}
  }
}
