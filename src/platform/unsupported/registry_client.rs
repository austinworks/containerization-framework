//! Stand-ins for `RegistryClient.swift`.

use super::CzAuthentication;
use super::CzOutcome;
use crate::containerization_oci;
use crate::platform;
use std::convert::Infallible;

taken!(registry_client -> CzRegistryClient);

handles!(CzRegistryClient);

failing!(
  cz_registry_client_new(&str, bool, CzAuthentication),
  cz_registry_client_with_host(
    &str,
    Option<String>,
    Option<u16>,
    CzAuthentication,
    Option<String>,
    bool,
    isize,
    u64,
    usize,
  ),
);

impl CzRegistryClient {
  pub(crate) fn ping(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn resolve(&self, _name: &str, _tag: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn fetch_data(&self, _name: &str, _descriptor: containerization_oci::image::Descriptor) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn fetch_blob(
    &self,
    _name: &str,
    _descriptor: containerization_oci::image::Descriptor,
    _into: &str,
    _progress: platform::Progress,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn catalog(&self, _prefix: Option<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn referrers(&self, _name: &str, _digest: &str, _artifact_type: Option<String>) -> CzOutcome {
    match self.0 {}
  }
}
