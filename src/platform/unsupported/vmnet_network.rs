//! Stand-ins for `VmnetNetwork.swift`: a `Network?`, `VmnetNetwork` and its
//! interfaces.

use super::CzOutcome;
use super::VmnetMode;
use crate::containerization::network;
use crate::containerization_extras;
use std::convert::Infallible;

/// A `Network?`, which is only ever `nil` here.
pub(crate) struct CzNetwork;

pub(crate) fn cz_no_network() -> CzNetwork {
  CzNetwork
}

taken!(
  vmnet_network -> CzVmnetNetwork,
  optional_vmnet_interface -> Option<network::vmnet_network::Interface>,
);

handles!(CzVmnetNetwork, CzVmnetInterface);

failing!(cz_vmnet_network_new(VmnetMode, Option<u32>, u8, bool, containerization_extras::address::IPv6Address, u8));

impl CzVmnetNetwork {
  pub(crate) fn as_network(&self) -> CzNetwork {
    match self.0 {}
  }

  pub(crate) fn subnet(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn prefix_v6(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ipv4_gateway(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ipv6_gateway(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_interface(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_interface_with_mtu(&self, _id: &str, _mtu: u32) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_interface_without_gateway(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn release_interface(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }
}

impl CzVmnetInterface {
  pub(crate) fn duplicate(&self) -> CzVmnetInterface {
    match self.0 {}
  }

  pub(crate) fn ipv4_address(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ipv4_gateway(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ipv6_address(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ipv6_gateway(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn mac_address(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn mtu(&self) -> u32 {
    match self.0 {}
  }
}
